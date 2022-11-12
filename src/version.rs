use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::ClashRequestBuilder;

use super::{ClashRequest, get_request};

pub struct ClashVersion {
    ip: String,
    port: u16,
    secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub version: String,
}

impl TryFrom<String> for Version {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

impl From<ClashRequestBuilder> for ClashVersion {
    fn from(r: ClashRequestBuilder) -> Self {
        Self {
            ip: r.ip.unwrap(),
            port: r.port.unwrap(),
            secret: r.secret,
        }
    }
}

#[async_trait]
impl ClashRequest for ClashVersion {
    type Response = Version;

    fn get_dest(&self) -> String {
        self.ip.clone()
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_secret(&self) -> Option<String>  {
        self.secret.clone()
    }

    fn get_method(&self) -> String {
        "GET".to_owned()
    }

    fn get_path(&self) -> String {
        "version".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        get_request(self).await
    }
}

#[cfg(test)]
mod test {
    use crate::ClashRequest;

    #[tokio::test]
    async fn test_get_version() {
        use crate::ClashRequestBuilder;
        let v = ClashRequestBuilder::new()
            .secret("test")
            .version()
            .send()
            .await
            .unwrap();

        println!("{:?}", v);
    }
}

