use crate::{ClashRequest, ClashRequestBuilder};

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

