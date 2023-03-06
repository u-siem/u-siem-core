use crate::prelude::types::LogString;

use super::field::SiemIp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DhcpEvent {
    /// Client IP address assigned, requested or cleaned
    pub source_ip: SiemIp,
    /// Client MAC address
    pub source_mac: u128,
    /// Request or assignation
    pub record_type: DhcpRecordType,
    /// Client HostName
    pub source_hostname: LogString,
    /// DHCP server HostName
    pub hostname: LogString,
}

impl DhcpEvent {
    pub fn source_ip(&self) -> &SiemIp {
        &self.source_ip
    }
    pub fn source_mac(&self) -> &u128 {
        &self.source_mac
    }
    pub fn record_type(&self) -> &DhcpRecordType {
        &self.record_type
    }
    pub fn source_hostname(&self) -> &str {
        &self.source_hostname
    }
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DhcpRecordType {
    Request,
    Assign,
    Release,
}

impl std::fmt::Display for DhcpRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DhcpRecordType {
    pub fn as_cow(&self) -> LogString {
        match self {
            DhcpRecordType::Request => LogString::Borrowed("Request"),
            DhcpRecordType::Assign => LogString::Borrowed("Assign"),
            DhcpRecordType::Release => LogString::Borrowed("Release"),
        }
    }
}
