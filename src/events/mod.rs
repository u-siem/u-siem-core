use serde::Serialize;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
pub mod field;
pub mod firewall;
pub mod protocol;
pub mod webproxy;

use field::{SiemField, SiemIp};
use firewall::FirewallEvent;
use webproxy::WebProxyEvent;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum SiemEvent {
    /// Firewall events: connections between IPs, blocked connections...
    Firewall(FirewallEvent),
    /// Intrusion detection/protection systems. Ex: Suricata, Snort, OSSEC, Wazuh, NGFW... 
    Intrusion,
    /// Security related assessment, like the output of vulnerability scanners (Nessus) or policy enforcers (OpenSCAP)
    Assessment,
    /// Web Browsing Proxy
    WebProxy(WebProxyEvent),
    /// Adaptative Distribution Content are Servers or LoadBalancers for HTTP traffic.
    ///
    ///
    /// Ex: Apache, Nginx, Tomact or IIS.
    ADC,
    /// Like an antivirus, a Sandbox retrieves information about a file being malicious or not. Can be used
    /// to extract filenames, hashes or other relevant information to update a dataset of known hashes and
    /// trigger queries.
    ///
    ///  Ex: Wildfire, Mcafee ATD, Cuckoo...
    Sandbox,
    Antivirus,
    /// Data Loss Prevention are devices that detect anomalous behavour related to
    /// data exfiltration.
    ///
    /// Ex: CloudSOC
    DLP,
    /// Some devices like email gateways generates a large number of logs when an email arrives: Header processing, AV scan, attachment information...
    /// In those cases, each log is associated with an action using a trace ID or a transaction ID.
    Partitioned,
    /// Endpoint Detection and Response devices, also EPP.
    EDR,
    /// Mail events, as the name suggest are events generated by an email gateway. Can
    /// contain threat related information if an anomaly was detected.
    /// Note that some devices generate partitioned logs instead of Mail logs.
    /// 
    /// Ex: Microsoft Exchange, IronPort, Office 365...
    Mail,
    /// DNS requests events. To better correlate this type of events, be carefull of checking if it contains a dns_server
    /// tag, because that means that the originator of the request is a Recursive DNS and not an endpoint. It normally
    /// happens if the one generating the log was a firewall (Ex: Palo Alto) and not a DNS server, or if multiple DNS are
    /// used in the organization, like a DNS talking to another DNS.
    DNS,
    /// DHCP logs associating an IP with a MAC address.
    DHCP,
    /// Logs related to authentication, like a user trying to log in to a Router,
    /// a server or any kind of system.
    ///
    /// Ex: RDP, Windows, Linux, Mailbox login...
    Auth,
    /// Local events related to servers or workstations, like OS failed to update,
    /// antivirus outdated, log file cleaned, user or group changes (Including global or universal domain events).
    /// Also events related to network devices: Changes in routing policys, Firewall rules, Shutdown out of mantaince
    Endpoint,
    // Unknown info that must be extracted and added to event fields. JSON format, like Windows events
    Json(serde_json::Value),
    // Unknown info that must be extracted and added to event fields.
    Unknown,
}

/// This is a simple log event. It contains information about the asset that generated
/// this log, the client if we are working in a multi-client environments aka SOC,
/// some fields to facilitate correlation with SIGMA rules, timestamps and tags to
/// better describe the content inside.
#[derive(Serialize, Debug)]
pub struct SiemLog {
    /// IP or Hostname of the server that sended the log.
    origin: SiemIp,
    /// Customer name for SOC environments. Ex: Contoso
    tenant: Cow<'static, str>,
    /// Name of the product for wich the log belongs. Ex: ASA
    product: Cow<'static, str>,
    /// Subset of the product logs. Like a OS that can have multiple programs running inside generating multiple logs.
    service: Cow<'static, str>,
    /// Category of the device: Firewall, web, antivirus
    category: Cow<'static, str>,
    /// Company that created the product. Ex: Cisco
    vendor: Cow<'static, str>,
    /// Categorization of the log. This forces the developer to use
    /// the same naming convention and reduces the number of human errors.
    event: SiemEvent,
    /// Tags to better describe the event.Must be in lowercase. Ex: vip_user, critical_asset, fake_account, honeypot
    tags: BTreeSet<Cow<'static, str>>,
    /// Map of fields extracted or generated for this log. Must follow the Elastic Common Schema (ECS v1.x)
    fields: BTreeMap<Cow<'static, str>, SiemField>,
    /// Original log message including syslog header
    message: String,
    /// Timestamp at witch the log arrived  
    event_received: i64,
    /// Timestamp at witch the log was generated. The clocks at origin must be correctly configured.
    event_created: i64,
}

impl<'a> SiemLog {
    pub fn new(message: String, received: i64, origin: SiemIp) -> SiemLog {
        SiemLog {
            message,
            event_received: received,
            origin,
            tenant: Cow::default(),
            product: Cow::default(),
            service: Cow::default(),
            category: Cow::default(),
            vendor: Cow::default(),
            event: SiemEvent::Unknown,
            tags: BTreeSet::default(),
            fields: BTreeMap::default(),
            event_created: received,
        }
    }

    pub fn message(&'a self) -> &'a str {
        &self.message
    }
    pub fn origin(&'a self) -> &'a SiemIp {
        &self.origin
    }
    pub fn tenant(&'a self) -> &'a str {
        &self.tenant
    }
    pub fn set_tenant(&mut self, tenant: Cow<'static, str>) {
        self.tenant = tenant;
    }
    pub fn product(&'a self) -> &'a str {
        &self.product
    }
    pub fn set_product(&mut self, val: Cow<'static, str>) {
        self.product = val;
    }
    pub fn service(&'a self) -> &'a str {
        &self.service
    }
    pub fn set_service(&mut self, val: Cow<'static, str>) {
        self.service = val;
    }
    pub fn category(&'a self) -> &'a str {
        &self.category
    }
    pub fn set_category(&mut self, val: Cow<'static, str>) {
        self.category = val;
    }
    pub fn vendor(&'a self) -> &'a str {
        &self.vendor
    }
    pub fn set_vendor(&mut self, val: Cow<'static, str>) {
        self.vendor = val;
    }
    pub fn event_received(&'a self) -> i64 {
        self.event_received
    }
    pub fn event_created(&'a self) -> i64 {
        self.event_created
    }
    pub fn set_event_created(&mut self, date : i64) {
        self.event_created = date;
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.insert(Cow::Owned(tag.to_lowercase()));
    }
    pub fn tags(&'a self) -> &'a BTreeSet<Cow<'static, str>> {
        &self.tags
    }
    pub fn field(&'a self, field_name: &str) -> Option<&SiemField> {
        self.fields.get(field_name)
    }
    pub fn add_field(&mut self, field_name: &str, field_value: SiemField) {
        self.fields
            .insert(Cow::Owned(field_name.to_owned()), field_value);
    }
    pub fn has_field(&self, field_name: &str) -> bool {
        self.fields.contains_key(field_name)
    }

    pub fn event(&self) -> &SiemEvent {
        &self.event
    }
    pub fn set_event(&mut self, event: SiemEvent) {
        self.event = event;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::firewall::{FirewallOutcome};
    use super::protocol::NetworkProtocol;

    #[test]
    fn check_log() {
        let message = "<134>Aug 23 20:30:25 OPNsense.localdomain filterlog[21853]: 82,,,0,igb0,match,pass,out,4,0x0,,62,25678,0,DF,17,udp,60,192.168.1.8,8.8.8.8,5074,53,40";
        let mut log = SiemLog::new(message.to_owned(), 0, SiemIp::V4(0));
        log.set_event(SiemEvent::Firewall(FirewallEvent{
            source_ip : SiemIp::V4(0),
            destination_ip : SiemIp::V4(10000),
            source_port : 10000,
            destination_port : 443,
            outcome : FirewallOutcome::ALLOW,
            in_bytes : 0,
            out_bytes : 0,
            in_interface : Cow::Borrowed("in123"),
            out_interface : Cow::Borrowed("out123"),
            network_protocol : NetworkProtocol::TCP
        }));
        log.add_field("event.dataset", SiemField::Text(Cow::Borrowed("filterlog")));
        match log.field("event.dataset") {
            Some(val) => {
                match val {
                    SiemField::Text(val) => {
                        assert_eq!(val,"filterlog")
                    },
                    _ => assert_eq!(1,2)
                }
            },
            _ => assert_eq!(1,2)
            
        };
    }
}