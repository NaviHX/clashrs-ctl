use crate::{ClashRequest, ClashRequestBuilder};
use serde::{Serialize, Deserialize};

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
    id: String,
    chains: Vec<String>,
    rule: String,
    #[serde(rename(serialize = "rulePayload", deserialize = "rulePayload"))]
    rule_payload: String,
    upload: usize,
    download: usize,
    start: String,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    network: String,
    r#type: String,
    #[serde(rename(serialize = "sourceIP", deserialize = "sourceIP"))]
    source_ip: String,
    #[serde(rename(serialize = "destinationIP", deserialize = "destinationIP"))]
    destination_ip: String,
    #[serde(rename(serialize = "sourcePort", deserialize = "sourcePort"))]
    source_port: String,
    #[serde(rename(serialize = "destinationPort", deserialize = "destinationPort"))]
    destination_port: String,
    host: String,
    #[serde(rename(serialize = "dnsMode", deserialize = "dnsMode"))]
    dns_mode: String,
    #[serde(rename(serialize = "processPath", deserialize = "processPath"))]
    process_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionVec {
    #[serde(rename(serialize = "downloadTotoal", deserialize = "downloadTotal"))]
    download_total: usize,
    #[serde(rename(serialize = "uploadTotal", deserialize = "uploadTotal"))]
    upload_total: usize,
    connections: Vec<Connection>,
}

