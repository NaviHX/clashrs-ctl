pub mod config;
pub mod proxy;
pub mod rule;
pub mod version;
pub mod connection;
pub mod stream;

use async_trait::async_trait;
use rule::ClashRule;
use config::ClashConfig;
use proxy::ClashProxy;
use stream::{traffic::ClashTraffic, log::ClashLog};
use connection::ClashConnections;

macro_rules! fn_to_specified_request {
    ($func:ident, $to:ident) => {
        pub fn $func(self) -> $to {
            $to::from(self.or_default())
        }
    };
}

macro_rules! request_builders {
    [$($func:ident, $to:ident);*] => {
        $(
            fn_to_specified_request!($func, $to);
        )*
    }
}

#[async_trait]
pub trait ClashRequest {
    type Response;

    fn get_dest(&self) -> String;
    fn get_port(&self) -> u16;
    fn get_secret(&self) -> Option<String>;

    fn get_method(&self) -> String;

    fn get_path(&self) -> String;
    fn get_query_parameter(&self) -> String;
    fn get_body(&self) -> String;

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>>;
}

use reqwest::Client;
use version::ClashVersion;

async fn get_request<'a, T>(request: T) -> Result<T::Response, Box<dyn std::error::Error + 'a>>
    where T: ClashRequest,
        // T::Response: From<String>
        T::Response: TryFrom<String>,
        <T::Response as TryFrom<String>>::Error: std::error::Error + 'a
{
    let mut c = Client::new()
        .get(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned());

    if let Some(secret) = request.get_secret() {
        c = c.header("Authorization", format!("Bearer {}", secret));
    }

    let c = c.send().await?
        .text().await?;

    match c.try_into() {
        Ok(res) => Ok(res),
        Err(err) => Err(Box::new(err)),
    }
}

async fn get_with_status_code_request<T>(request: T) -> Result<(reqwest::StatusCode, String), Box<dyn std::error::Error>>
    where T: ClashRequest
{
    let mut c = Client::new()
        .get(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned());
    if let Some(secret) = request.get_secret() {
        c = c.header("Authorization", format!("Bearer {}", secret));
    }
    let c = c.send().await?;
    Ok((c.status(), c.text().await?))
}

async fn put_request<T>(request: T) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>>
    where T: ClashRequest
{
    let mut c = Client::new()
        .put(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned());
    if let Some(secret) = request.get_secret() {
        c = c.header("Authorization", format!("Bearer {}", secret));
    }
    let c = c.send().await?.status();
    Ok(c)
}

async fn patch_request<T>(request: T) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>>
    where T: ClashRequest
{
    let mut c = Client::new()
        .patch(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned());
    if let Some(secret) = request.get_secret() {
        c = c.header("Authorization", format!("Bearer {}", secret));
    }
    let c = c.send().await?
        .status();
    Ok(c)
}

pub struct ClashRequestBuilder {
    ip: Option<String>,     // default: 127.0.0.1
    port: Option<u16>,   // default: 9090
    secret: Option<String>, // default: None
}

impl ClashRequestBuilder {
    pub fn new() -> Self {
        Self {
            ip: None,
            port: None,
            secret: None,
        }
    }

    fn or_default(self) -> Self {
        Self {
            ip: self.ip.or_else(|| Some("127.0.0.1".to_owned())),
            port: self.port.or(Some(9090)),
            secret: self.secret,
        }
    }

    pub fn ip(self, ip: &str) -> Self {
        Self {
            ip: Some(ip.to_owned()),
            ..self
        }
    }

    pub fn secret(self, secret: &str) -> Self {
        Self {
            secret: Some(secret.to_owned()),
            ..self
        }
    }

    pub fn port(self, port: u16) -> Self {
        Self {
            port: Some(port),
            ..self
        }
    }

    request_builders![
        connections, ClashConnections;
        logs, ClashLog;
        traffic, ClashTraffic;
        version, ClashVersion;
        proxies, ClashProxy;
        config, ClashConfig;
        rule, ClashRule
    ];
}

