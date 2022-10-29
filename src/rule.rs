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

    fn get_dest(&self) -> String {
        self.ip.clone()
    }

    fn get_port(&self) -> String {
        self.port.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_method(&self) -> String {
        "GET".to_owned()
    }

    fn get_path(&self) -> String {
        "rules".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
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

#[cfg(test)]
mod tests {
    /// please start a clash server on port 9090

    use super::*;
    #[tokio::test]
    async fn test_get_rule() {
        let req = ClashRequestBuilder::new().rule().send();
        let RuleList(rule) = req.await.unwrap();
        println!("{}", rule);
    }
}
