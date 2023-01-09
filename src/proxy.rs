use std::error::Error;
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use urlencoding::encode;

use crate::{ClashRequest, get_with_status_code_request};

use super::{get_request, put_request};

#[derive(Debug)]
pub enum ProxyError {
    ProxyNotExisted,
    FormatError,
    TimeOut,
    UnknownError(u16),
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ProxyError::*;
        // write!(f, "{}", match self {
        //     ProxyNotExisted => "Proxy not existed",
        //     FormatError => "Request format error",
        //     TimeOut => "Delay timeout",
        //     UnknownError(_code) => "Unknown error",
        // })
        match self {
            ProxyNotExisted => write!(f, "Proxy not existed"),
            FormatError => write!(f, "Request format error"),
            TimeOut => write!(f, "Delay timeout" ),
            UnknownError(code) => write!(f, "Unknown error: {}", code),
        }
    }
}

impl Error for ProxyError {}

pub struct ClashProxy {
    ip: String,
    port: u16,
    secret: Option<String>,
}

// HACK: proxy info may not include all of these information
#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyInfo {
    pub all: Option< Vec<String> >,
    pub now: Option< String >,
    pub r#type: Option< String >,
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

// impl std::convert::From<String> for ProxyList {
//     fn from(s: String) -> Self {
//         serde_json::from_str(&s).expect("cannot parse")
//     }
// }
impl TryFrom<String> for ProxyList {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
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

impl ClashProxyInfo {
    pub fn delay(self, url: &str, timeout: u32) -> ClashProxyDelay {
        let url = encode(url).to_string();

        ClashProxyDelay {
            ip: self.ip,
            port: self.port,
            secret: self.secret,
            proxy_name: self.proxy_name,

            url,
            timeout,
        }
    }

    pub fn change(self, new_proxy: &str) -> ClashProxyChange {
        ClashProxyChange {
            ip: self.ip,
            port: self.port,
            secret: self.secret,
            proxy_name: self.proxy_name,

            new_proxy: new_proxy.to_owned(),
        }
    }
}

pub struct ClashProxyChange {
    ip: String,
    port: u16,
    secret: Option<String>,
    proxy_name: String,
    new_proxy: String,
}

#[async_trait]
impl ClashRequest for ClashProxyChange {
    type Response = ();

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
        "PUT".to_owned()
    }

    fn get_path(&self) -> String {
        format!("proxies/{}", self.proxy_name)
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        format!("{{ \"name\": \"{}\"}}", self.new_proxy)
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        let res = put_request(self).await?;
        match res {
            // StatusCode::OK => Ok(()),
            StatusCode::NO_CONTENT => Ok(()), // return 204 for success for graceful shutdown
            StatusCode::BAD_REQUEST => Err(Box::new(ProxyError::FormatError)),
            StatusCode::NOT_FOUND => Err(Box::new(ProxyError::ProxyNotExisted)),
            code => Err(Box::new(ProxyError::UnknownError(code.as_u16()))),
        }
    }
}

pub struct ClashProxyDelay {
    ip: String,
    port: u16,
    secret: Option<String>,
    proxy_name: String,
    url: String,
    timeout: u32,
}

#[async_trait]
impl ClashRequest for ClashProxyDelay {
    type Response = ProxyDelay;

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
        format!("proxies/{}/delay", self.proxy_name)
    }

    fn get_query_parameter(&self) -> String {
        format!("timeout={}&url={}", self.timeout, self.url)
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        let (c, text) = get_with_status_code_request(self).await?;
        if c.is_success() {
            let info = text.try_into()?;
            Ok( info )
        } else if c == StatusCode::BAD_REQUEST {
            Err( Box::new(ProxyError::FormatError) )
        } else if c == StatusCode::REQUEST_TIMEOUT {
            Err( Box::new(ProxyError::TimeOut))
        } else if c == StatusCode::NOT_FOUND {
            Err( Box::new(ProxyError::ProxyNotExisted))
        } else {
            Err( Box::new(ProxyError::UnknownError(c.as_u16())))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProxyDelay {
    pub delay: u64,
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
        let (c, text) = get_with_status_code_request(self).await?;
        if c.is_success() {
            let info = text.try_into()?;
            Ok( info )
        } else {
            Err( Box::new(ProxyError::ProxyNotExisted) )
        }
    }
}

impl TryFrom<String> for ProxyInfo {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

impl TryFrom<String> for ProxyDelay {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

#[cfg(test)]
mod test {
    use crate::ClashRequest;

    #[tokio::test]
    async fn test_get_proxy_list() {
        use crate::ClashRequestBuilder;
        let c = ClashRequestBuilder::new()
            .secret("test")
            .proxies()
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
            .secret("test")
            .proxies()
            .get("GLOBAL")
            .send()
            .await
            .unwrap();

        println!("{:?}", c);
    }

    #[tokio::test]
    async fn test_get_proxy_delay() {
        use crate::ClashRequestBuilder;
        let _c = ClashRequestBuilder::new()
            .secret("test")
            .proxies()
            .get("DIRECT")
            .delay("http://baidu.com", 500)
            .send()
            .await
            .unwrap();

    }

    #[tokio::test]
    async fn test_change_proxy() {
        use crate::ClashRequestBuilder;
        ClashRequestBuilder::new()
            .secret("test")
            .proxies()
            .get("GLOBAL")
            .change("DIRECT")
            .send()
            .await
            .unwrap();
    }
}

