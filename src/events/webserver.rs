use super::common::{HttpMethod,WebProtocol};
use super::field::SiemIp;
use serde::Serialize;
use std::borrow::Cow;

/// A typical combined Log format has a source_ip, a user_id, a date, the http method,
/// the path requested, the user agent and the size of the resource returned
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub struct WebServerEvent {
    pub source_ip: SiemIp,
    pub destination_ip: Option<SiemIp>,//Server IP 
    pub destination_port: u16,
    pub in_bytes: u32,
    pub out_bytes: u32,
    pub http_code: u32,
    pub duration : f32,
    pub http_method: HttpMethod,
    pub user_agent: Cow<'static, str>,
    pub url_full: Cow<'static, str>,
    pub url_domain: Cow<'static, str>,
    pub url_path: Cow<'static, str>,
    pub url_query: Cow<'static, str>,
    pub url_extension: Cow<'static, str>,
    pub protocol: WebProtocol,
    pub user_name: Cow<'static, str>,
    pub mime_type: Cow<'static, str>,
    pub outcome: WebServerOutcome
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum WebServerOutcome {
    /// Connection was blocked
    BLOCK,
    /// Connection was allowed
    ALLOW,
    /// Unknow connection state.
    UNKNOWN,
}
