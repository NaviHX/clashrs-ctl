use std::borrow::Borrow;

use crate::ClashRequestBuilder;
use async_trait::async_trait;

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
pub struct ClashConfigLoad;

pub struct Config(String);
pub struct LoadResult;
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
        todo!()
    }
}

#[async_trait]
impl ClashRequest for ClashConfigGet {
    type Response = Config;

    fn get_dest(&self) ->  &str {
        &self.ip
    }

    fn get_port(&self) ->  &str {
        &self.port
    }

    fn get_secret(&self) -> Option< &str>  {
        (&self.secret).as_ref().map(|r| r.borrow())
    }

    fn get_method(&self) ->  &str {
        "GET"
    }

    fn get_path(&self) ->  &str {
        "configs"
    }

    fn get_query_parameter(&self) ->  &str {
        ""
    }

    fn get_body(&self) ->  &str {
        ""
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

#[cfg(test)]
mod test {
    use crate::{ClashRequest, config::Config};

    #[tokio::test]
    async fn test_get_config() {
        use crate::ClashRequestBuilder;
            let Config(res) = ClashRequestBuilder::new()
                .ip("127.0.0.1".to_owned())
                .port("9090".to_owned())
                .config()
                .get()
                .send()
                .await
                .unwrap();

            println!("config: {}", res);
    }
}
