use super::field::SiemIp;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct DnsEvent {
    /// Client that queried
    pub source_ip: SiemIp,
    /// Server that answered the question
    pub destination_ip: SiemIp,
    /// Answer or question
    pub op_code : DnsEventType,
    /// dns.question.type or dns.answer.type
    pub record_type: DnsRecordType,
    /// dns.question.name or dns.answer.name
    pub record_name: Cow<'static, str>,
    pub data : Option<Cow<'static, str>>
}

impl DnsEvent {
    pub fn source_ip(&self) -> &SiemIp {
        &self.source_ip
    }
    pub fn destination_ip(&self) -> &SiemIp {
        &self.destination_ip
    }
    pub fn op_code(&self) -> &DnsEventType {
        &self.op_code
    }
    pub fn record_type(&self) -> &DnsRecordType {
        &self.record_type
    }
    pub fn record_name(&self) -> &str {
        &self.record_name
    }
    pub fn data(&self) -> &Option<Cow<'static, str>> {
        &self.data
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum DnsEventType {
    ANSWER,
    QUERY
}

impl std::fmt::Display for DnsEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum DnsRecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    PTR,
    CERT,
    SRV,
    TXT,
    SOA
}

impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl DnsRecordType {
    pub fn as_cow(&self) -> Cow<'static,str> {
        match self {
            DnsRecordType::A => Cow::Borrowed("A"),
            DnsRecordType::AAAA => Cow::Borrowed("AAAA"),
            DnsRecordType::CNAME => Cow::Borrowed("CNAME"),
            DnsRecordType::MX => Cow::Borrowed("MX"),
            DnsRecordType::NS => Cow::Borrowed("NS"),
            DnsRecordType::PTR => Cow::Borrowed("PTR"),
            DnsRecordType::CERT => Cow::Borrowed("CERT"),
            DnsRecordType::SRV => Cow::Borrowed("SRV"),
            DnsRecordType::TXT => Cow::Borrowed("TXT"),
            DnsRecordType::SOA => Cow::Borrowed("SOA"),
        }
    }
}