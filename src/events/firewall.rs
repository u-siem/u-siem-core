use crate::prelude::{SiemLog, SiemField};
use crate::prelude::types::LogString;

use super::{ip::SiemIp, field_dictionary::*};
use super::protocol::NetworkProtocol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FirewallEvent {
    /// Ip that started the connection
    pub source_ip: SiemIp,
    /// IP that received the connection
    pub destination_ip: SiemIp,
    /// Source port -> source.port
    pub source_port: u16,
    /// Destination port -> destintion.port
    pub destination_port: u16,
    /// Protocol used for the connection.
    pub network_protocol: NetworkProtocol,
    /// What happened to the connection
    pub outcome: FirewallOutcome,
    /// Bytes received. Equals destination.bytes
    pub in_bytes: u32,
    /// Bytes sended. Equals source.bytes
    pub out_bytes: u32,
    /// Input interface for the connection
    pub in_interface: LogString,
    /// Output interface for the connection
    pub out_interface: LogString,
}
impl FirewallEvent {
    pub fn source_ip(&self) -> &SiemIp {
        &self.source_ip
    }
    pub fn destination_ip(&self) -> &SiemIp {
        &self.destination_ip
    }
    pub fn network_protocol(&self) -> &NetworkProtocol {
        &self.network_protocol
    }
    pub fn outcome(&self) -> &FirewallOutcome {
        &self.outcome
    }
    pub fn in_interface(&self) -> &str {
        &self.in_interface
    }
    pub fn out_interface(&self) -> &str {
        &self.out_interface
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FirewallOutcome {
    /// Connection was blocked
    BLOCK,
    /// Connection was allowed
    ALLOW,
    /// Connection ended, the event contains information about bytes sended/received
    END,
    /// The connection is still ongoing, but we log statistics about it
    STATS,
    /// Oppened connection. Later can be dropped due to policy settings. Ej: Sonicwall or Firepower have this behavior.
    OPEN,
    /// Unknow connection state.
    UNKNOWN,
}

impl FirewallOutcome {
    pub fn equals(&self, val: &str) -> bool {
        match (val, self) {
            ("BLOCK", FirewallOutcome::BLOCK) => return true,
            ("ALLOW", FirewallOutcome::ALLOW) => return true,
            ("END", FirewallOutcome::END) => return true,
            ("STATS", FirewallOutcome::STATS) => return true,
            ("OPEN", FirewallOutcome::OPEN) => return true,
            ("UNKNOWN", FirewallOutcome::UNKNOWN) => return true,
            _ => return false,
        }
    }
}

impl std::fmt::Display for FirewallOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl Into<SiemLog> for FirewallEvent {
    fn into(self) -> SiemLog {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(
            SOURCE_IP,
            SiemField::IP(self.source_ip),
        );
        log.add_field(
            DESTINATION_IP,
            SiemField::IP(self.destination_ip),
        );
        log.add_field(
            SOURCE_PORT,
            SiemField::U64(self.source_port as u64),
        );
        log.add_field(
            DESTINATION_PORT,
            SiemField::U64(self.destination_port as u64),
        );
        log.add_field(
            EVENT_OUTCOME,
            SiemField::Text(LogString::Owned(self.outcome.to_string())),
        );
        log.add_field(
            IN_INTERFACE,
            SiemField::Text(LogString::Owned(self.in_interface.into_owned())),
        );
        log.add_field(
            OUT_INTERFACE,
            SiemField::Text(LogString::Owned(self.out_interface.into_owned())),
        );
        log.add_field(SOURCE_BYTES, SiemField::U64(self.out_bytes as u64));
        log.add_field(
            DESTINATION_BYTES,
            SiemField::U64(self.in_bytes as u64),
        );
        log.add_field(
            NETWORK_TRANSPORT,
            SiemField::Text(LogString::Owned(self.network_protocol.to_string())),
        );
        log
    }
}