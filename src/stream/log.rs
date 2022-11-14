use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::{ ClashRequestBuilder, ClashRequest, config::ConfigLogLevel };

use super::ClashStream;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Log {
    pub r#type: ConfigLogLevel,
    pub payload: String,
}

pub struct ClashLog {
    ip: String,
    port: u16,
    secret: Option<String>,

    r#type: Option<ConfigLogLevel>,
}

impl From<ClashRequestBuilder> for ClashLog {
    fn from(r: ClashRequestBuilder) -> Self {
        Self {
            ip: r.ip.unwrap(),
            port: r.port.unwrap(),
            secret: r.secret,
            r#type: None,
        }
    }
}

impl ClashLog {
    pub fn level(self, level: ConfigLogLevel) -> Self {
        Self {
            r#type: Some(level),
            ..self
        }
    }
}

#[async_trait]
impl ClashRequest for ClashLog {
    type Response = ClashStream<Log>;

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
        "logs".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        format!("type={}", match self.r#type.as_ref() {
            None => "",
            Some(ConfigLogLevel::Info) => "info",
            Some(ConfigLogLevel::Error) => "error",
            Some(ConfigLogLevel::Debug) => "debug",
            Some(ConfigLogLevel::Warning) => "waring",
        })
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use super::get_stream_request;

        let stream = get_stream_request(self).await?;
        Ok(ClashStream::new(stream))
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_parse_log_from_stream() {
        use futures::stream::{self, StreamExt};
        use super::ClashStream;
        use super::Log;
        
        let source_stream = Box::pin(
            stream::once(async {
                reqwest::Result::Ok( bytes::Bytes::from("{\"type\":\"info\",\"payload\":\"nothing\"}") )
            }) 
        );
        let mut stream: ClashStream<Log> = ClashStream::new(source_stream);

        let log = stream.next().await.unwrap().unwrap();
        println!("log: {:?}", log);
    }
}

