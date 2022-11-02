use std::borrow::Borrow;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use super::{ClashRequest, ClashRequestBuilder};

pub struct ClashRule {
    ip: String,
    port: u16,
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

    fn get_port(&self) -> u16 {
        self.port
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleList {
    rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub r#type: String,
    pub payload: String,
    pub proxy: String,
}

// impl From<String> for RuleList {
//     fn from(s: String) -> Self {
//         serde_json::from_str(&s).expect("cannot parse the rule list")
//     }
// }

impl TryFrom<String> for RuleList {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

impl RuleList {
    pub fn iter(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }
}

#[cfg(test)]
mod tests {
    /// please start a clash server on port 9090

    use super::*;
    #[tokio::test]
    async fn test_get_rule() {
        let req = ClashRequestBuilder::new().secret("test").rule().send();
        let rule_list = req.await.unwrap();
        println!("{:?}", rule_list);
    }
}
