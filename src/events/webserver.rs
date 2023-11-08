use super::common::{HttpMethod, WebProtocol};
use super::field_dictionary::*;
use super::ip::SiemIp;
use crate::prelude::{SiemLog, SiemField};
use crate::prelude::types::LogString;
use serde::{Deserialize, Serialize};

/// A typical combined Log format has a source_ip, a user_id, a date, the http method,
/// the path requested, the user agent and the size of the resource returned
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebServerEvent {
    pub source_ip: SiemIp,
    pub destination_ip: Option<SiemIp>, //Server IP
    pub destination_port: u16,
    pub in_bytes: u32,
    pub out_bytes: u32,
    pub http_code: u32,
    pub duration: f32,
    pub http_method: HttpMethod,
    pub user_agent: LogString,
    pub url_full: LogString,
    pub url_domain: LogString,
    pub url_path: LogString,
    pub url_query: LogString,
    pub url_extension: LogString,
    pub protocol: WebProtocol,
    pub user_name: LogString,
    pub mime_type: LogString,
    pub outcome: WebServerOutcome,
}
impl WebServerEvent {
    pub fn source_ip(&self) -> &SiemIp {
        &self.source_ip
    }
    pub fn destination_ip(&self) -> &Option<SiemIp> {
        &self.destination_ip
    }
    pub fn destination_port(&self) -> u16 {
        self.destination_port
    }
    pub fn in_bytes(&self) -> u32 {
        self.in_bytes
    }
    pub fn out_bytes(&self) -> u32 {
        self.out_bytes
    }
    pub fn http_code(&self) -> u32 {
        self.http_code
    }
    pub fn duration(&self) -> f32 {
        self.duration
    }
    pub fn protocol(&self) -> &WebProtocol {
        &self.protocol
    }
    pub fn outcome(&self) -> &WebServerOutcome {
        &self.outcome
    }
    pub fn http_method(&self) -> &HttpMethod {
        &self.http_method
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }
    pub fn url_full(&self) -> &str {
        &self.url_full
    }
    pub fn url_domain(&self) -> &str {
        &self.url_domain
    }
    pub fn url_path(&self) -> &str {
        &self.url_path
    }
    pub fn url_query(&self) -> &str {
        &self.url_query
    }
    pub fn url_extension(&self) -> &str {
        &self.url_extension
    }
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum WebServerOutcome {
    /// Connection was blocked
    BLOCK,
    /// Connection was allowed
    ALLOW,
    /// Unknow connection state.
    UNKNOWN,
}
impl std::fmt::Display for WebServerOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl Into<SiemLog> for WebServerEvent {
    fn into(self) -> SiemLog {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(
            SOURCE_IP,
            SiemField::IP(self.source_ip),
        );
        match self.destination_ip {
            Some(ip) => {
                log.add_field(DESTINATION_IP, SiemField::IP(ip));
            }
            None => {}
        };

        log.add_field(
            DESTINATION_PORT,
            SiemField::U64(self.destination_port as u64),
        );
        log.add_field(
            EVENT_OUTCOME,
            SiemField::Text(LogString::Owned(self.outcome.to_string())),
        );
        log.add_field(SOURCE_BYTES, SiemField::U64(self.out_bytes as u64));
        log.add_field(
            DESTINATION_BYTES,
            SiemField::U64(self.in_bytes as u64),
        );
        log.add_field(
            NETWORK_PROTOCOL,
            SiemField::Text(LogString::Owned(self.protocol.to_string())),
        );
        log.add_field(
            HTTP_RESPONSE_STATUS_CODE,
            SiemField::U64(self.http_code as u64),
        );
        log.add_field(
            HTTP_REQUEST_METHOD,
            SiemField::Text(LogString::Owned(self.http_method.to_string())),
        );
        log.add_field(
            URL_FULL,
            SiemField::Text(self.url_full),
        );
        log.add_field(
            URL_DOMAIN,
            SiemField::Text(self.url_domain),
        );
        log.add_field(
            URL_PATH,
            SiemField::Text(self.url_path),
        );
        log.add_field(
            URL_QUERY,
            SiemField::Text(self.url_query),
        );
        log.add_field(
            "url.extension",
            SiemField::Text(self.url_extension),
        );
        log.add_field(
            USER_NAME,
            SiemField::User(self.user_name.to_string()),
        );
        log.add_field(
            HTTP_RESPONSE_MIME_TYPE,
            SiemField::Text(self.mime_type)
        );
        log.add_field(
            NETWORK_DURATION,
            SiemField::F64(self.duration as f64),
        );
        log.add_field(
            "user_agent.original",
            SiemField::Text(self.user_agent),
        );
        log
    }
}