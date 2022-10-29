pub mod config;
pub mod proxy;
pub mod rule;

use async_trait::async_trait;
use rule::{ClashRule, RuleList};
use config::{ClashConfig, ClashConfigGet};
use tokio;

macro_rules! fn_set_field {
    ($id:ident) => {
        pub fn $id(self, $id: &str) -> Self {
            Self {
                $id: Some($id.to_owned()),
                ..self
            }
        }
    };
}

macro_rules! field_setters {
    [$($id:ident),*] => {
        $(
            fn_set_field!($id);
        )*
    }
}

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
    fn get_port(&self) -> String;
    fn get_secret(&self) -> Option<String>;

    fn get_method(&self) -> String;

    fn get_path(&self) -> String;
    fn get_query_parameter(&self) -> String;
    fn get_body(&self) -> String;

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>>;
}

use reqwest::Client;

async fn get_request<T>(request: T) -> Result<T::Response, Box<dyn std::error::Error>>
    where T: ClashRequest,
        T::Response: From<String>
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
    Ok(c.into())
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
    port: Option<String>,   // default: 9090
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
            port: self.port.or_else(|| Some("9090".to_owned())),
            secret: self.secret,
        }
    }

    field_setters![ip, port, secret];
    request_builders![
        // proxy, ClashProxy;
        config, ClashConfig;
        rule, ClashRule
    ];
}

