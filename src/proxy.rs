use std::error::Error;
use super::{get_request, put_request};

#[derive(Debug)]
pub enum ProxyError {
    ProxyNotExisted,
    FormatError,
    TimeOut,
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ProxyError::*;
        write!(f, "{}", match self {
            ProxyNotExisted => "Proxy not existed",
            FormatError => "Request format error",
            TimeOut => "Delay timeout",
        })
    }
}

impl Error for ProxyError {}

