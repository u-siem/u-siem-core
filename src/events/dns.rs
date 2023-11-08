use crate::prelude::{types::LogString, SiemLog, SiemField};

use super::{ip::SiemIp, field_dictionary::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DnsEvent {
    /// Client that queried
    pub source_ip: SiemIp,
    /// Server that answered the question
    pub destination_ip: SiemIp,
    /// Answer or question
    pub op_code: DnsEventType,
    /// dns.question.type or dns.answer.type
    pub record_type: DnsRecordType,
    /// dns.question.name or dns.answer.name
    pub record_name: LogString,
    pub data: Option<LogString>,
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
    pub fn data(&self) -> &Option<LogString> {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DnsEventType {
    ANSWER,
    QUERY,
}

impl std::fmt::Display for DnsEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    SOA,
}

impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl DnsRecordType {
    pub fn as_cow(&self) -> LogString {
        match self {
            DnsRecordType::A => LogString::Borrowed("A"),
            DnsRecordType::AAAA => LogString::Borrowed("AAAA"),
            DnsRecordType::CNAME => LogString::Borrowed("CNAME"),
            DnsRecordType::MX => LogString::Borrowed("MX"),
            DnsRecordType::NS => LogString::Borrowed("NS"),
            DnsRecordType::PTR => LogString::Borrowed("PTR"),
            DnsRecordType::CERT => LogString::Borrowed("CERT"),
            DnsRecordType::SRV => LogString::Borrowed("SRV"),
            DnsRecordType::TXT => LogString::Borrowed("TXT"),
            DnsRecordType::SOA => LogString::Borrowed("SOA"),
        }
    }
}
impl Into<SiemLog> for DnsEvent {
    fn into(self) -> SiemLog {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(SOURCE_IP, SiemField::IP(self.source_ip));
        log.add_field(DESTINATION_IP, SiemField::IP(self.destination_ip));
        match self.op_code {
            DnsEventType::ANSWER => {
                log.add_field(DNS_OP_CODE, SiemField::Text(LogString::Borrowed("ANSWER")));
                log.add_field(
                    DNS_ANSWER_NAME,
                    SiemField::Text(self.record_name)
                );
                match self.data {
                    Some(data) => {
                        log.add_field(DNS_ANSWER_DATA, SiemField::from_str(data.to_string()));
                    }
                    None => {}
                };
                log.add_field(DNS_ANSWER_TYPE, SiemField::Text(self.record_type.as_cow()));
            }
            DnsEventType::QUERY => {
                log.add_field(DNS_OP_CODE, SiemField::Text(LogString::Borrowed("QUERY")));
                
                log.add_field(
                    DNS_QUESTION_NAME,
                    SiemField::Text(self.record_name)
                );
                log.add_field(
                    DNS_QUESTION_TYPE,
                    SiemField::Text(self.record_type.as_cow()),
                );
            }
        };
        log
    }
}
