use crate::{ClashRequest, ClashRequestBuilder};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

pub struct ClashConnections {
    ip: String,
    port: u16,
    secret: Option<String>,
}

pub struct ClashCloseConnections {
    ip: String,
    port: u16,
    secret: Option<String>,
}

pub struct ClashCloseID {
    ip: String,
    port: u16,
    secret: Option<String>,

    id: String,
}

impl From<ClashRequestBuilder> for ClashConnections {
    fn from(r: ClashRequestBuilder) -> Self {
        Self {
            ip: r.ip.unwrap(),
            port: r.port.unwrap(),
            secret: r.secret,
        }
    }
}

impl ClashConnections {
    pub fn close(self) -> ClashCloseConnections {
        ClashCloseConnections {
            ip: self.ip,
            port: self.port,
            secret: self.secret,
        }
    }

    pub fn close_id(self, id: &str) -> ClashCloseID {
        ClashCloseID { ip: self.ip, port: self.port, secret: self.secret, id: id.to_owned() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub chains: Vec<String>,
    pub rule: String,
    #[serde(rename(serialize = "rulePayload", deserialize = "rulePayload"))]
    pub rule_payload: String,
    pub upload: usize,
    pub download: usize,
    start: String,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub network: String,
    pub r#type: String,
    #[serde(rename(serialize = "sourceIP", deserialize = "sourceIP"))]
    pub source_ip: String,
    #[serde(rename(serialize = "destinationIP", deserialize = "destinationIP"))]
    pub destination_ip: String,
    #[serde(rename(serialize = "sourcePort", deserialize = "sourcePort"))]
    pub source_port: String,
    #[serde(rename(serialize = "destinationPort", deserialize = "destinationPort"))]
    pub destination_port: String,
    pub host: String,
    #[serde(rename(serialize = "dnsMode", deserialize = "dnsMode"))]
    pub dns_mode: String,
    #[serde(rename(serialize = "processPath", deserialize = "processPath"))]
    pub process_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionVec {
    #[serde(rename(serialize = "downloadTotoal", deserialize = "downloadTotal"))]
    pub download_total: usize,
    #[serde(rename(serialize = "uploadTotal", deserialize = "uploadTotal"))]
    pub upload_total: usize,
    pub connections: Vec<Connection>,
}

impl TryFrom<String> for ConnectionVec {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

#[async_trait]
impl ClashRequest for ClashConnections {
    type Response = ConnectionVec;

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
        "connections".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use crate::get_request;

        get_request(self).await
    }
}

#[async_trait]
impl ClashRequest for ClashCloseConnections {
    type Response = ();

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
        "DELETE".to_owned()
    }

    fn get_path(&self) -> String {
        "conntections".to_owned()
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use crate::get_with_status_code_request;

        let _code = get_with_status_code_request(self).await?;

        // HACK: The official doc says nothing about the return of this request
        Ok(())
    }
}

#[async_trait]
impl ClashRequest for ClashCloseID {
    type Response = ();

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
        "DELETE".to_owned()
    }

    fn get_path(&self) -> String {
        format!("connections/{}", self.id)
    }

    fn get_query_parameter(&self) -> String {
        "".to_owned()
    }

    fn get_body(&self) -> String {
        "".to_owned()
    }

    async fn send(self) -> Result<Self::Response, Box<dyn std::error::Error>> {
        use crate::get_with_status_code_request;

        let _code = get_with_status_code_request(self).await?;

        // HACK: The official doc says nothing about the return of this request
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ClashRequest;

    #[tokio::test]
    async fn test_get_connection_info() {
        use crate::ClashRequestBuilder;

        let connections_info = ClashRequestBuilder::new()
            .secret("test")
            .connections()
            .send()
            .await
            .unwrap();

        println!("{:?}", connections_info);
    }
}

