use super::field::SiemIp;
use super::protocol::NetworkProtocol;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub struct IntrusionEvent {
    /// Ip that started the connection
    pub source_ip: SiemIp,
    /// IP that received the connection
    pub destination_ip: SiemIp,
    pub source_port: u16,
    pub destination_port: u16,
    pub network_protocol: NetworkProtocol,
    pub outcome: IntrusionOutcome, //Allowed, Deny...
    pub rule_name: Cow<'static, str>,
    pub rule_category: IntrusionCategory, //Allowed, Deny...
    pub rule_id: u32,
}

impl IntrusionEvent {
    pub fn source_ip(&self) -> &SiemIp {
        &self.source_ip
    }
    pub fn destination_ip(&self) -> &SiemIp {
        &self.destination_ip
    }
    pub fn network_protocol(&self) -> &NetworkProtocol {
        &self.network_protocol
    }
    pub fn outcome(&self) -> &IntrusionOutcome {
        &self.outcome
    }
    pub fn rule_category(&self) -> &IntrusionCategory {
        &self.rule_category
    }
    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum IntrusionOutcome {
    /// The attack has not been prevented and may affect systems 
    DETECTED,
    /// The attack was prevented
    BLOCKED,
    /// The attack was not prevented but it does not affect assets 
    MONITOR,
    /// The attack has not been prevented and has affected assets
    IMPACTED
}
impl std::fmt::Display for IntrusionOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum IntrusionCategory {
    /// Cross site scripting
    XSS,
    /// SQL Injection
    SQL_INJECTION,
    /// Path Transversal attack
    PATH_TRANSVERSAL,
    /// Common web attacks
    WEB_ATTACK,
    /// Denial of service
    DOS,
    /// Scan or Surveillance
    SURVEILLANCE,
    /// Trojan horse 
    TROJAN,
    /// Simple virus
    VIRUS,
    /// Remote administration tools
    RAT_TOOL,
    /// Ransomware attacks
    RANSOMWARE,
    /// Hacktools like mimikatz
    HACKTOOL,
    /// TOR or Web proxies
    PROXY_AVOIDANCE,
    /// Remote execution exploit
    REMOTE_EXPLOIT,
    /// Local execution exploit
    LOCAL_EXPLOIT,
    /// Botnet related
    BOTNET,
    /// Spam
    SPAM,
    /// Phising
    PHISHING,
    /// Email scam
    CEO_FRAUD,
    /// Email scam
    SCAM,
    /// Exfiltration of information
    DATA_THEFT,
    /// Brute force: Password/users guessing
    BRUTE_FORCE,
    /// Clear text passwords...
    MISCONFIGURATION,
    /// Active Directory attack: kerberoasting, DC Sync, Golden Ticket, Pass the Hash, Pass the ticket...
    AD_ATTACK

    //TODO: Add more categories

}
impl std::fmt::Display for IntrusionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}