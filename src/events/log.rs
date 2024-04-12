use crate::prelude::{types::LogString, SiemField};
use serde::{Deserialize, Serialize};
use std::{collections::{BTreeMap, BTreeSet}, net::IpAddr};

//use serde::ser::{Serializer, SerializeStruct};

/// This is a simple log event. It contains information about the asset that generated
/// this log, the client if we are working in a multi-client environments aka SOC,
/// some fields to facilitate correlation with SIGMA rules, timestamps and tags to
/// better describe the content inside.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiemLog {
    #[serde(skip, default)]
    /// Tags to better describe the event.Must be in lowercase. Ex: vip_user, critical_asset, fake_account, honeypot
    tags: BTreeSet<LogString>,
    /// Map of fields extracted or generated for this log. Must follow the Elastic Common Schema (ECS v1.x)
    #[serde(flatten)]
    pub(crate) fields: BTreeMap<LogString, SiemField>,
}

impl<'a> SiemLog {
    pub fn new<S, M>(message: M, received: i64, origin: S) -> SiemLog
    where
        S: Into<LogString>,
        M: Into<String>,
    {
        let cw = origin.into();
        let ms = message.into();
        let mut fields = BTreeMap::new();
        fields.insert(
            LogString::Borrowed("message"),
            SiemField::Text(LogString::Owned(ms)).into(),
        );
        fields.insert(LogString::Borrowed("origin"), SiemField::Text(cw).into());
        fields.insert(
            LogString::Borrowed("event.created"),
            SiemField::Date(received).into(),
        );
        fields.insert(
            LogString::Borrowed("event.received"),
            SiemField::Date(received).into(),
        );
        SiemLog {
            tags: BTreeSet::default(),
            fields
        }
    }

    pub fn message(&'a self) -> &'a str {
        match self.field("message") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }
    pub fn set_message(&mut self, msg: String) {
        self.fields.insert(
            LogString::Borrowed("message"),
            SiemField::Text(LogString::Owned(msg)).into(),
        );
    }
    pub fn origin(&'a self) -> &'a str {
        match self.field("origin") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }
    pub fn set_origin(&mut self, msg: LogString) {
        self.fields
            .insert(LogString::Borrowed("origin"), SiemField::Text(msg).into());
    }
    pub fn tenant(&'a self) -> &'a str {
        match self.field("tenant") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }
    pub fn set_tenant<S>(&mut self, tenant: S)
    where
        S: Into<LogString>,
    {
        self.fields.insert(
            LogString::Borrowed("tenant"),
            SiemField::Text(tenant.into()).into(),
        );
    }
    /// Name of the product for wich the log belongs. Ex: ASA
    pub fn product(&'a self) -> &'a str {
        match self.field("product") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }
    pub fn set_product<S>(&mut self, product: S)
    where
        S: Into<LogString>,
    {
        let product = product.into();
        self.fields.insert(
            LogString::Borrowed("product"),
            SiemField::Text(product.clone()).into(),
        );
    }
    /// Subset of the product logs. Like a OS that can have multiple programs running inside generating multiple logs.
    pub fn service(&'a self) -> &'a str {
        match self.field("service") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }

    pub fn set_service<S>(&mut self, service: S)
    where
        S: Into<LogString>,
    {
        let service = service.into();
        self.fields.insert(
            LogString::Borrowed("service"),
            SiemField::Text(service.clone()).into(),
        );
    }
    /// Category of the device: Firewall, web, antivirus
    pub fn category(&'a self) -> &'a str {
        match self.field("category") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }
    pub fn set_category<S>(&mut self, category: S)
    where
        S: Into<LogString>,
    {
        let category = category.into();
        self.fields.insert(
            LogString::Borrowed("category"),
            SiemField::Text(category.clone()).into(),
        );
    }
    /// Company that created the product. Ex: Cisco
    pub fn vendor(&'a self) -> &'a str {
        self.field("vendor")
            .map(|v| match v {
                SiemField::Text(v) => v,
                _ => "",
            })
            .unwrap_or("")
    }
    pub fn set_vendor<S>(&mut self, vendor: S)
    where
        S: Into<LogString>,
    {
        let vendor = vendor.into();
        self.fields.insert(
            LogString::Borrowed("vendor"),
            SiemField::Text(vendor.clone()).into(),
        );
    }
    /// Timestamp at witch the log arrived in milliseconds since UNIX
    pub fn event_received(&'a self) -> i64 {
        match self.field("event.received") {
            Some(SiemField::Date(v)) => *v,
            _ => 0,
        }
    }
    /// Timestamp at witch the log was generated. The clocks at origin must be correctly configured.
    pub fn event_created(&'a self) -> i64 {
        match self.field("event.created") {
            Some(SiemField::Date(v)) => *v,
            _ => 0,
        }
    }
    pub fn set_event_created(&mut self, date: i64) {
        self.fields.insert(
            LogString::Borrowed("event.created"),
            SiemField::I64(date).into(),
        );
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.insert(LogString::Owned(tag.to_lowercase()));
        self.fields.insert(
            LogString::Borrowed("tags"),
            SiemField::Array(
                self.tags
                    .iter()
                    .map(|x| LogString::Owned(x.to_lowercase()))
                    .collect::<Vec<LogString>>(),
            )
            .into(),
        );
    }
    pub fn tags(&'a self) -> &'a BTreeSet<LogString> {
        &self.tags
    }
    pub fn field(&'a self, field_name: &str) -> Option<&SiemField> {
        self.fields.get(field_name)
    }
    pub fn field_mut(&'a mut self, field_name: &str) -> Option<&mut SiemField> {
        self.fields.get_mut(field_name)
    }
    pub fn add_field(&mut self, field_name: &str, field_value: SiemField) {
        let field_name = LogString::Owned(field_name.to_owned());
        self.insert(field_name, field_value);
    }
    pub fn insert(&mut self, field_name: LogString, field_value: SiemField) {
        self.fields.insert(field_name, field_value.into());
    }
    pub fn has_field(&self, field_name: &str) -> bool {
        self.fields.contains_key(field_name)
    }
    pub fn fields(&self) -> EventIter<'_> {
        EventIter {
            children: self.fields.iter(),
        }
    }
    pub fn iter(&self) -> EventIter<'_> {
        EventIter {
            children: self.fields.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> EventIterMut<'_> {
        EventIterMut {
            children: self.fields.iter_mut(),
        }
    }
    pub fn ip_fields(&self) -> IpFieldIter<'_> {
        IpFieldIter {
            children: self.fields.iter(),
        }
    }
}

pub struct EventIter<'a> {
    children: std::collections::btree_map::Iter<'a, LogString, SiemField>,
}
pub struct IpFieldIter<'a> {
    children: std::collections::btree_map::Iter<'a, LogString, SiemField>,
}

pub struct EventIterMut<'a> {
    children: std::collections::btree_map::IterMut<'a, LogString, SiemField>,
}

impl<'a> Iterator for EventIter<'a> {
    type Item = (&'a LogString, &'a SiemField);

    fn next(&mut self) -> Option<Self::Item> {
        let evt = self.children.next()?;
        Some((evt.0, &evt.1))
    }
}
impl<'a> Iterator for EventIterMut<'a> {
    type Item = (&'a LogString, &'a mut SiemField);

    fn next(&mut self) -> Option<Self::Item> {
        let evt = self.children.next()?;
        Some((evt.0, evt.1))
    }
}
impl<'a> Iterator for IpFieldIter<'a> {
    type Item = (&'a LogString, &'a IpAddr);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let nxt = self.children.next()?;
            if let SiemField::IP(ip) = nxt.1 {
                return Some((nxt.0, ip));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;
    use crate::prelude::event::SiemEvent;
    use crate::prelude::{FirewallEvent, FirewallOutcome, NetworkProtocol};

    #[test]
    fn check_log() {
        let event = SiemEvent::Firewall(FirewallEvent {
            source_ip: Ipv4Addr::new(0,0,0,0).into(),
            destination_ip: Ipv4Addr::new(1,0,0,0).into(),
            source_port: 10000,
            destination_port: 443,
            outcome: FirewallOutcome::ALLOW,
            in_bytes: 0,
            out_bytes: 0,
            in_interface: LogString::Borrowed("in123"),
            out_interface: LogString::Borrowed("out123"),
            network_protocol: NetworkProtocol::TCP,
        });
        let mut log: SiemLog = event.into();
        log.set_message("<134>Aug 23 20:30:25 OPNsense.localdomain filterlog[21853]: 82,,,0,igb0,match,pass,out,4,0x0,,62,25678,0,DF,17,udp,60,192.168.1.8,8.8.8.8,5074,53,40".to_string());
        log.set_origin("localhost".into());
        log.add_field(
            "event.dataset",
            SiemField::Text(LogString::Borrowed("filterlog")),
        );
        let val: &str = log.field("event.dataset").unwrap().try_into().unwrap();
        assert_eq!("filterlog", val);
    }
}
