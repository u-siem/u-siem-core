use crate::prelude::types::LogString;
use crate::prelude::{SiemField, SiemLog};

use super::protocol::NetworkProtocol;
use super::{field_dictionary::*, ip::SiemIp};
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
        matches!(
            (val, self),
            ("BLOCK", FirewallOutcome::BLOCK)
                | ("ALLOW", FirewallOutcome::ALLOW)
                | ("END", FirewallOutcome::END)
                | ("STATS", FirewallOutcome::STATS)
                | ("OPEN", FirewallOutcome::OPEN)
                | ("UNKNOWN", FirewallOutcome::UNKNOWN)
        )
    }
}

impl std::fmt::Display for FirewallOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl From<FirewallEvent> for SiemLog {
    fn from(val: FirewallEvent) -> Self {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(SOURCE_IP, SiemField::IP(val.source_ip));
        log.add_field(DESTINATION_IP, SiemField::IP(val.destination_ip));
        log.add_field(SOURCE_PORT, SiemField::U64(val.source_port as u64));
        log.add_field(
            DESTINATION_PORT,
            SiemField::U64(val.destination_port as u64),
        );
        log.add_field(
            EVENT_OUTCOME,
            SiemField::Text(LogString::Owned(val.outcome.to_string())),
        );
        log.add_field(
            IN_INTERFACE,
            SiemField::Text(LogString::Owned(val.in_interface.into_owned())),
        );
        log.add_field(
            OUT_INTERFACE,
            SiemField::Text(LogString::Owned(val.out_interface.into_owned())),
        );
        log.add_field(SOURCE_BYTES, SiemField::U64(val.out_bytes as u64));
        log.add_field(DESTINATION_BYTES, SiemField::U64(val.in_bytes as u64));
        log.add_field(
            NETWORK_TRANSPORT,
            SiemField::Text(LogString::Owned(val.network_protocol.to_string())),
        );
        log
    }
}
