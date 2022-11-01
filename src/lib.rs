pub mod config;
pub mod proxy;
pub mod rule;

use async_trait::async_trait;
use rule::{ClashRule, RuleList};
use config::{ClashConfig, ClashConfigGet};
use proxy::{ClashProxy};
use tokio;

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

async fn get_request<'a, T>(request: T) -> Result<T::Response, Box<dyn std::error::Error + 'a>>
    where T: ClashRequest,
        // T::Response: From<String>
        T::Response: TryFrom<String>,
        <T::Response as TryFrom<String>>::Error: std::error::Error + 'a
{
    let c = Client::new()
        .get(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned())
        .send().await?
        .text().await?;

    match c.try_into() {
        Ok(res) => Ok(res),
        Err(err) => Err(Box::new(err)),
    }
}

async fn put_request<T>(request: T) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>>
    where T: ClashRequest
{
    let c = Client::new()
        .put(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned())
        .send().await?
        .status();
    Ok(c)
}

async fn patch_request<T>(request: T) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>>
    where T: ClashRequest
{
    let c = Client::new()
        .patch(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body().to_owned())
        .send().await?
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
        proxy, ClashProxy;
        config, ClashConfig;
        rule, ClashRule
    ];
}

