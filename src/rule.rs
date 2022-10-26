use std::borrow::Borrow;

use async_trait::async_trait;

use super::{ClashRequest, ClashRequestBuilder};

pub struct ClashRule {
    ip: String,
    port: String,
    secret: Option<String>,
}

impl From<ClashRequestBuilder> for ClashRule {
    fn from(r: ClashRequestBuilder) -> Self {
        Self {
            ip: r.ip.unwrap(),
            port: r.port.unwrap(),
            secret: r.secret,
        }
    }
}

#[async_trait]
impl ClashRequest for ClashRule {
    type Response = RuleList;

    fn get_dest(&self) -> &str {
        &self.ip
    }

    fn get_port(&self) -> &str {
        &self.port
    }

    fn get_secret(&self) -> Option<&str> {
        (&self.secret).as_ref().map(|r| r.borrow())
    }

    fn get_method(&self) -> &str {
        "GET"
    }

    fn get_path(&self) -> &str {
        "rules"
    }

    fn get_query_parameter(&self) -> &str {
        ""
    }

    fn get_body(&self) -> &str {
        ""
    }

    async fn send(self) -> Result<Self::Response , Box<dyn std::error::Error>> {
        use super::get_request;
        get_request(self).await
    }
}

pub struct RuleList(pub String);

impl From<String> for RuleList {
    fn from(s: String) -> Self {
        Self(s)
    }
}
