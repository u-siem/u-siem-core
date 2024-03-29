use crate::prelude::{mac::mac_u128_to_str, types::LogString, SiemField, SiemLog};

use super::{field_dictionary::*, ip::SiemIp};
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

impl From<DhcpEvent> for SiemLog {
    fn from(val: DhcpEvent) -> Self {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(
            "host.hostname",
            SiemField::Text(LogString::Owned(val.hostname().to_string())),
        );
        log.add_field("server.hostname", SiemField::Text(val.hostname));
        log.add_field("client.hostname", SiemField::Text(val.source_hostname));
        log.add_field("client.ip", SiemField::IP(val.source_ip));
        log.add_field(
            "client.mac",
            SiemField::Text(LogString::Owned(mac_u128_to_str(val.source_mac))),
        );
        log.add_field(DHCP_RECORD_TYPE, val.record_type.to_string().into());
        match val.record_type {
            DhcpRecordType::Assign => {}
            DhcpRecordType::Release => {}
            DhcpRecordType::Request => {
                log.add_field("requested_ip", SiemField::IP(val.source_ip));
            }
        };
        log
    }
}
