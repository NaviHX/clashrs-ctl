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
    pub port: Option<u16>,
    #[serde(rename(serialize = "socks-port", deserialize = "socks-port"), skip_serializing_if = "Option::is_none")]
    pub socks_port: Option<u16>,
    #[serde(rename(serialize = "redir-port", deserialize = "redir-port"), skip_serializing_if = "Option::is_none")]
    pub redir_port: Option<u16>,
    #[serde(rename(serialize = "allow-lan", deserialize = "allow-lan"), skip_serializing_if = "Option::is_none")]
    pub allow_lan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ConfigMode>,
    #[serde(rename(serialize = "log-level", deserialize = "log-level"), skip_serializing_if = "Option::is_none")]
    pub log_level: Option<ConfigLogLevel>,

    #[serde(rename(serialize = "tproxy-port", deserialize = "tproxy-port"), skip_serializing_if = "Option::is_none")]
    pub tproxy_port: Option<u16>,
    #[serde(rename(serialize = "bind-address", deserialize = "bind-address"), skip_serializing_if = "Option::is_none")]
    pub bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<bool>,

    // BUG: Don't know what this field exists for
    // authentication: Vec<?>,
}

// pub struct LoadResult(StatusCode);
// pub struct PatchResult(StatusCode);
//
// impl LoadResult {
//     pub fn ok(&self) -> bool {
//         self.0 == StatusCode::NO_CONTENT
//     }
// }
//
// impl PatchResult {
//     pub fn ok(&self) -> bool {
//         self.0 == StatusCode::NO_CONTENT
//     }
// }

#[derive(Debug)]
pub struct LoadError;
#[derive(Debug)]
pub struct PatchError;
impl std::error::Error for LoadError {}
impl std::error::Error for PatchError {}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Load configuration failed")
    }
}

impl std::fmt::Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration patch error")
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
    type Response = ();

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
        let code = put_request(self).await?;
        if let StatusCode::NO_CONTENT = code {
            Ok(())
        } else {
            Err(Box::new(LoadError))
        }
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
            tproxy_port: None,
            bind_address: None,
            ipv6: None,
        }
    }

    pub fn tproxy_port(self, port: u16) -> Self {
        Self { tproxy_port: Some(port), ..self }
    }

    pub fn bind_address(self, address: &str) -> Self {
        Self { bind_address: Some(address.to_owned()), ..self }
    }

    pub fn ipv6(self, flag: bool) -> Self {
        Self { ipv6: Some(flag), ..self }
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
    pub fn tproxy_port(self, port: u16) -> Self {
        Self { config: self.config.tproxy_port(port), ..self }
    }

    pub fn bind_address(self, address: &str) -> Self {
        Self { config: self.config.bind_address(address), ..self }
    }

    pub fn ipv6(self, flag: bool) -> Self {
        Self { config: self.config.ipv6(flag), ..self }
    }
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
    type Response = ();

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
        let code = patch_request(self).await?;
        if let StatusCode::NO_CONTENT = code {
            Ok(()) // According to the doc on `clash.gitbook.io`,
                   // this should return status code `200`. But it
                   // actually return `204` after reloading the 
                   // configuration file.

        } else {
            Err(Box::new(PatchError))
        }
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

        ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .port(9090)
            .config()
            .load("./clash-profile.reload")
            .force()
            .send()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_patch_config() {
        use crate::ClashRequestBuilder;
        use super::ConfigMode;
        use super::ConfigLogLevel;

        ClashRequestBuilder::new()
            .ip("127.0.0.1")
            .config()
            .patch()
            .port(9999)
            .redir_port(8888)
            .socks_port(7777)
            .allow_lan(true)
            .ipv6(false)
            .tproxy_port(6666)
            .mode(ConfigMode::Global)
            .log_level(ConfigLogLevel::Debug)
            .send()
            .await
            .unwrap();
    }
}
