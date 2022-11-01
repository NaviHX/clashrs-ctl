use std::{borrow::Borrow, path::PathBuf};

use crate::ClashRequestBuilder;
use async_trait::async_trait;
use reqwest::StatusCode;
use path_absolutize::Absolutize;
use serde::{Serialize, Deserialize};
use serde_json;

use super::ClashRequest;
pub struct ClashConfig {
    ip: String,
    port: u16,
    secret: Option<String>,
}
pub struct ClashConfigGet {
    ip: String,
    port: u16,
    secret: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ClashConfigPatch {
    #[serde(skip)]
    api_ip: String,
    #[serde(skip)]
    api_port: u16,
    #[serde(skip)]
    secret: Option<String>,

    #[serde(flatten)]
    config: Config,
}

use clap::ValueEnum;

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ConfigMode {
    Global,
    Rule,
    Direct,
}

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ConfigLogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

pub struct ClashConfigLoad {
    force: bool,
    config_path: PathBuf,

    ip: String,
    port: u16,
    secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
    #[serde(rename(serialize = "socks-port", deserialize = "socks-port"), skip_serializing_if = "Option::is_none")]
    socks_port: Option<u16>,
    #[serde(rename(serialize = "socks-port", deserialize = "socks-port"), skip_serializing_if = "Option::is_none")]
    redir_port: Option<u16>,
    #[serde(rename(serialize = "allow-lan", deserialize = "allow-lan"), skip_serializing_if = "Option::is_none")]
    allow_lan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<ConfigMode>,
    #[serde(rename(serialize = "log-level", deserialize = "log-level"), skip_serializing_if = "Option::is_none")]
    log_level: Option<ConfigLogLevel>,
}
pub struct LoadResult(StatusCode);
pub struct PatchResult(StatusCode);

impl LoadResult {
    pub fn ok(&self) -> bool {
        self.0 == StatusCode::NO_CONTENT
    }
}

impl PatchResult {
    pub fn ok(&self) -> bool {
        self.0 == StatusCode::NO_CONTENT
    }
}

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
    pub fn get(self) -> ClashConfigGet {
        self.into()
    }

    pub fn patch(self) -> ClashConfigPatch {
        ClashConfigPatch {
            api_ip: self.ip,
            api_port: self.port,
            secret: self.secret,
            config: Config::new(),
        }
    }

    pub fn load(self, path: &str) -> ClashConfigLoad {
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

    fn get_port(&self) ->  u16 {
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

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
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

// impl From<String> for Config {
//     fn from(s: String) -> Self {
//         serde_json::from_str(&s).expect("cannot parse the configuration")
//     }
// }

impl TryFrom<String> for Config {
    type Error = serde_json::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&s)
    }
}

#[async_trait]
impl ClashRequest for ClashConfigLoad {
    type Response = LoadResult;

    fn get_dest(&self) ->  String {
        self.ip.clone()
    }

    fn get_port(&self) -> u16 {
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

impl Config {
    pub fn new() -> Self {
        Self {
            port: None,
            socks_port: None,
            redir_port: None,
            allow_lan: None,
            mode: None,
            log_level: None,
        }
    }
    pub fn port(self, port: u16) -> Self {
        Self {
            port: Some(port),
            ..self
        }
    }

    pub fn socks_port(self, port: u16) -> Self {
        Self {
            socks_port: Some(port),
            ..self
        }
    }

    pub fn redir_port(self, port: u16) -> Self {
        Self {
            redir_port: Some(port),
            ..self
        }
    }

    pub fn allow_lan(self, allow: bool) -> Self {
        Self {
            allow_lan: Some(allow),
            ..self
        }
    }

    pub fn mode(self, mode: ConfigMode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }

    pub fn log_level(self, level: ConfigLogLevel) -> Self {
        Self {
            log_level: Some(level),
            ..self
        }
    }
}
impl ClashConfigPatch {
    pub fn port(self, port: u16) -> Self {
        Self {
            config: self.config.port(port),
            ..self
        }
    }

    pub fn socks_port(self, port: u16) -> Self {
        Self {
            config: self.config.socks_port(port),
            ..self
        }
    }

    pub fn redir_port(self, port: u16) -> Self {
        Self {
            config: self.config.redir_port(port),
            ..self
        }
    }

    pub fn allow_lan(self, allow: bool) -> Self {
        Self {
            config: self.config.allow_lan(allow),
            ..self
        }
    }

    pub fn mode(self, mode: ConfigMode) -> Self {
        Self {
            config: self.config.mode(mode),
            ..self
        }
    }

    pub fn log_level(self, level: ConfigLogLevel) -> Self {
        Self {
            config: self.config.log_level(level),
            ..self
        }
    }
}

#[async_trait]
impl ClashRequest for ClashConfigPatch {
    type Response = PatchResult;

    fn get_dest(&self) -> String {
        self.api_ip.clone()
    }

    fn get_port(&self) -> u16 {
        self.api_port.clone()
    }

    fn get_secret(&self) -> Option<String>  {
        self.secret.clone()
    }

    fn get_method(&self) -> String {
        "PATCH".to_owned()
    }

    fn get_path(&self) -> String {
        "configs".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use super::patch_request;
        patch_request(self).await.map(PatchResult)
    }
}

#[cfg(test)]
mod test {
    use crate::{ClashRequest, config::Config};

    #[tokio::test]
    async fn test_get_config() {
        use crate::ClashRequestBuilder;
        let res = ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .port(9090)
            .config()
            .get()
            .send()
            .await
            .unwrap();

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_load_config() {
        use crate::ClashRequestBuilder;
        use super::LoadResult;
        use reqwest::StatusCode;

        let LoadResult(code) = ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .port(9090)
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

    #[tokio::test]
    async fn test_patch_config() {
        use crate::ClashRequestBuilder;
        use super::PatchResult;
        use super::ConfigMode;
        use super::ConfigLogLevel;

        let res: PatchResult = ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .config()
            .patch()
            .port(9999)
            .redir_port(8888)
            .socks_port(7777)
            .allow_lan(true)
            .mode(ConfigMode::Global)
            .log_level(ConfigLogLevel::Debug)
            .send()
            .await
            .unwrap();

        assert!(res.ok());
    }
}
