use super::field::SiemIp;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct DhcpEvent {
    /// Client IP address assigned, requested or cleaned
    pub source_ip: SiemIp,
    /// Client MAC address
    pub source_mac: u128,
    /// Request or assignation
    pub record_type: DhcpRecordType,
    /// Client HostName
    pub source_hostname: Cow<'static, str>,
    /// DHCP server HostName
    pub hostname: Cow<'static, str>
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
    pub fn source_hostname(&self) -> &Cow<'static, str> {
        &self.source_hostname
    }
    pub fn hostname(&self) -> &Cow<'static, str> {
        &self.hostname
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum DhcpRecordType {
    Request,
    Assign,
    Release
}

impl std::fmt::Display for DhcpRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DhcpRecordType {
    pub fn as_cow(&self) -> Cow<'static,str> {
        match self {
            DhcpRecordType::Request => Cow::Borrowed("Request"),
            DhcpRecordType::Assign => Cow::Borrowed("Assign"),
            DhcpRecordType::Release => Cow::Borrowed("Release")
        }
    }
}