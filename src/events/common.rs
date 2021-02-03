/// Common HTTP Request methods
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum HttpMethod {
    UNKNOWN(String),
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    CONNECT,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum WebProtocol {
    UNKNOWN(String),
    HTTP,
    HTTPS,
    FTP
}

impl HttpMethod {
    pub fn from_str(val: &str) -> HttpMethod {
        match &val.to_uppercase()[..] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PATCH" => HttpMethod::PATCH,
            "OPTIONS" => HttpMethod::OPTIONS,
            "CONNECT" => HttpMethod::CONNECT,
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
            _ => return false,
        }
    }
}
