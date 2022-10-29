use std::{borrow::Borrow, path::PathBuf};

use crate::ClashRequestBuilder;
use async_trait::async_trait;
use reqwest::StatusCode;
use path_absolutize::Absolutize;

use super::ClashRequest;
pub struct ClashConfig {
    ip: String,
    port: String,
    secret: Option<String>,
}
pub struct ClashConfigGet {
    ip: String,
    port: String,
    secret: Option<String>,
}
pub struct ClashConfigPatch;
pub struct ClashConfigLoad {
    force: bool,
    config_path: PathBuf,

    ip: String,
    port: String,
    secret: Option<String>,
}

pub struct Config(String);
pub struct LoadResult(StatusCode);
pub struct PatchResult;

impl From<ClashRequestBuilder> for ClashConfig {
    fn from(r: ClashRequestBuilder) -> Self {
        Self {
            ip: r.ip.unwrap(),
            port: r.port.unwrap(),
            secret: r.secret,
        }
    }
}

impl ClashConfig {
    fn get(self) -> ClashConfigGet {
        self.into()
    }

    fn patch(self) -> ClashConfigPatch {
        todo!()
    }

    fn load(self, path: &str) -> ClashConfigLoad {
        ClashConfigLoad {
            force: false,
            config_path: path.to_owned().into(),
            ip: self.ip,
            port: self.port,
            secret: self.secret,
        }
    }
}

#[async_trait]
impl ClashRequest for ClashConfigGet {
    type Response = Config;

    fn get_dest(&self) ->  String {
        self.ip.clone()
    }

    fn get_port(&self) ->  String {
        self.port.clone()
    }

    fn get_secret(&self) -> Option< String>  {
        self.secret.clone()
    }

    fn get_method(&self) ->  String {
        "GET".to_owned()
    }

    fn get_path(&self) ->  String {
        "configs".to_owned()
    }

    fn get_query_parameter(&self) ->  String {
        "".to_owned()
    }

    fn get_body(&self) ->  String {
        "".to_owned()
    }

    async fn send(self) -> Result< Self::Response, Box<dyn std::error::Error> > {
        use super::get_request;
        get_request(self).await
    }
}

impl From<ClashConfig> for ClashConfigGet {
    fn from(c: ClashConfig) -> Self {
        Self {
            ip: c.ip,
            port: c.port,
            secret: c.secret,
        }
    }
}

impl From<String> for Config {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[async_trait]
impl ClashRequest for ClashConfigLoad {
    type Response = LoadResult;

    fn get_dest(&self) ->  String {
        self.ip.clone()
    }

    fn get_port(&self) ->  String {
        self.port.clone()
    }

    fn get_secret(&self) -> Option< String>  {
        self.secret.clone()
    }

    fn get_method(&self) ->  String {
        "PUT".to_owned()
    }

    fn get_path(&self) ->  String {
        "configs".to_owned()
    }

    fn get_query_parameter(&self) ->  String {
        format!("force={}", self.force)
    }

    fn get_body(&self) ->  String {
        format!("{{\"path\":\"{}\"}}", self.config_path.absolutize().expect("config path error").to_str().unwrap())
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use super::put_request;
        put_request(self).await.map(LoadResult)
    }
}

impl ClashConfigLoad {
    pub fn force(self) -> Self {
        Self {
            force: true,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ClashRequest, config::Config};

    #[tokio::test]
    async fn test_get_config() {
        use crate::ClashRequestBuilder;
        let Config(res) = ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .port("9090")
            .config()
            .get()
            .send()
            .await
            .unwrap();

        println!("config: {}", res);
    }

    #[tokio::test]
    async fn test_load_config() {
        use crate::ClashRequestBuilder;
        use super::LoadResult;
        use reqwest::StatusCode;

        let LoadResult(code) = ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .port("9090")
            .config()
            .load("./clash-profile.reload")
            .force()
            .send()
            .await
            .unwrap();

        assert!(code == StatusCode::NO_CONTENT); // According to the doc on `clash.gitbook.io`,
                                                 // this should return status code `200`. But it
                                                 // actually return `204` after reloading the 
                                                 // configuration file.
    }
}
