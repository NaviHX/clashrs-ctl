use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::{ ClashRequestBuilder, ClashRequest };

use super::ClashStream;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Traffic {
    pub up: usize,
    pub down: usize,
}

pub struct ClashTraffic {
    ip: String,
    port: u16,
    secret: Option<String>,
}

impl From<ClashRequestBuilder> for ClashTraffic {
    fn from(r: ClashRequestBuilder) -> Self {
        Self {
            ip: r.ip.unwrap(),
            port: r.port.unwrap(),
            secret: r.secret,
        }
    }
}

#[async_trait]
impl ClashRequest for ClashTraffic {
    type Response = ClashStream<Traffic>;

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
        "traffic".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use super::get_stream_request;

        let base_stream = get_stream_request(self).await?;
        Ok(ClashStream::new(base_stream))
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_get_traffic() {
        use futures::StreamExt;
        use crate::ClashRequest;
        let mut stream = crate::ClashRequestBuilder::new()
            .secret("test")
            .traffic()
            .send()
            .await
            .unwrap();

        let traffic_1 = stream.next().await.unwrap().unwrap();
        let traffic_2 = stream.next().await.unwrap().unwrap();
        let traffic_3 = stream.next().await.unwrap().unwrap();
        let traffic_4 = stream.next().await.unwrap().unwrap();
        println!("traffic: {:?}", traffic_1);
        println!("traffic: {:?}", traffic_2);
        println!("traffic: {:?}", traffic_3);
        println!("traffic: {:?}", traffic_4);

    }
}

