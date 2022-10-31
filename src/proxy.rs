use std::error::Error;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::ClashRequest;

use super::{get_request, put_request};

#[derive(Debug)]
pub enum ProxyError {
    ProxyNotExisted,
    FormatError,
    TimeOut,
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ProxyError::*;
        write!(f, "{}", match self {
            ProxyNotExisted => "Proxy not existed",
            FormatError => "Request format error",
            TimeOut => "Delay timeout",
        })
    }
}

impl Error for ProxyError {}

pub struct ClashProxy {
    ip: String,
    port: u16,
    secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyInfo {
    all: Vec<String>,
    now: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyList {
    proxies: Map<String, Value>,
}

impl ProxyList {
    pub fn iter(&self) -> serde_json::map::Iter {
        self.proxies.iter()
    }
}

// TODO: use TryFrom here, because the convert can fail and we can catch the error instead of panic
impl std::convert::From<String> for ProxyList {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).expect("cannot parse")
    }
}

impl std::convert::From<crate::ClashRequestBuilder> for ClashProxy {
    fn from(f: crate::ClashRequestBuilder) -> Self {
        Self {
            ip: f.ip.unwrap(),
            port: f.port.unwrap(),
            secret: f.secret,
        }
    }
}

impl ClashProxy {
    pub fn get(self, proxy_name: &str) -> ClashProxyInfo {
        ClashProxyInfo {
            ip: self.ip,
            port: self.port,
            secret: self.secret,

            proxy_name: proxy_name.to_owned(),
        }
    }
}

pub struct ClashProxyInfo {
    ip: String,
    port: u16,
    secret: Option<String>,

    proxy_name: String,
}

#[async_trait]
impl ClashRequest for ClashProxy {
    type Response = ProxyList;

    fn get_dest(&self) -> String {
        self.ip.clone()
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_secret(&self) -> Option<String>  {
        self.secret.clone()
    }

    fn get_method(&self) -> String {
        "GET".to_owned()
    }

    fn get_path(&self) -> String {
        "proxies".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn Error>> {
        get_request(self).await
    }
}

#[async_trait]
impl ClashRequest for ClashProxyInfo {
    type Response = ProxyInfo;

    fn get_dest(&self) -> String {
        self.ip.clone()
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_secret(&self) -> Option<String>  {
        self.secret.clone()
    }

    fn get_method(&self) -> String {
        "GET".to_owned()
    }

    fn get_path(&self) -> String {
        format!("proxies/{}", self.proxy_name)
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn Error>> {
        use reqwest::Client;
        let c = Client::new()
            .get(format!("http://{}:{}/{}?{}", self.ip, self.port, self.get_path(), self.get_query_parameter()))
            .body(self.get_body())
            .send()
            .await?;
        if c.status().is_success() {
            let info = serde_json::from_str::<ProxyInfo>( &c.text().await? )?;
            Ok( info )
        } else {
            Err( Box::new(ProxyError::ProxyNotExisted) )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ClashRequest;

    #[tokio::test]
    async fn test_get_proxy_list() {
        use crate::ClashRequestBuilder;
        let c = ClashRequestBuilder::new()
            .proxy()
            .send()
            .await
            .unwrap();

        for ( proxy, status ) in c.iter() {
            println!("{}: {}", proxy, status["type"]);
        }
    }

    #[tokio::test]
    async fn test_get_proxy_info() {
        use crate::ClashRequestBuilder;
        let c = ClashRequestBuilder::new()
            .proxy()
            .get("GLOBAL")
            .send()
            .await
            .unwrap();

        println!("{:?}", c);
    }
}

