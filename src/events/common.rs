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
    pub fn from_str_slice(val: &str) -> HttpMethod {
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
        matches!(
            (val, self),
            ("GET", HttpMethod::GET)
                | ("POST", HttpMethod::POST)
                | ("PUT", HttpMethod::PUT)
                | ("PATCH", HttpMethod::PATCH)
                | ("UNKNOWN", HttpMethod::UNKNOWN(_))
                | ("OPTIONS", HttpMethod::OPTIONS)
                | ("CONNECT", HttpMethod::CONNECT)
                | ("HEAD", HttpMethod::HEAD)
                | ("UNDEFINED", HttpMethod::UNDEFINED)
        )
    }
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
