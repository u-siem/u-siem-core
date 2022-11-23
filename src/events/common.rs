/// Common HTTP Request methods
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum HttpMethod {
    UNKNOWN(String),
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    CONNECT,
    HEAD,
    UNDEFINED,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum WebProtocol {
    UNKNOWN(String),
    HTTP,
    HTTPS,
    FTP,
    WS,
    WSS,
}
impl std::fmt::Display for WebProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
impl HttpMethod {
    pub fn from_str(val: &str) -> HttpMethod {
        match &val.to_uppercase()[..] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PATCH" => HttpMethod::PATCH,
            "OPTIONS" => HttpMethod::OPTIONS,
            "CONNECT" => HttpMethod::CONNECT,
            "HEAD" => HttpMethod::HEAD,
            "UNDEFINED" => HttpMethod::UNDEFINED,
            val => HttpMethod::UNKNOWN(val.to_string()),
        }
    }

    pub fn equals(&self, val: &str) -> bool {
        match (val, self) {
            ("GET", HttpMethod::GET) => return true,
            ("POST", HttpMethod::POST) => return true,
            ("PUT", HttpMethod::PUT) => return true,
            ("PATCH", HttpMethod::PATCH) => return true,
            ("UNKNOWN", HttpMethod::UNKNOWN(_)) => return true,
            ("OPTIONS", HttpMethod::OPTIONS) => return true,
            ("CONNECT", HttpMethod::CONNECT) => return true,
            ("HEAD", HttpMethod::HEAD) => return true,
            ("UNDEFINED", HttpMethod::UNDEFINED) => return true,
            _ => return false,
        }
    }
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
