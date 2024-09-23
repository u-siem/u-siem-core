use std::collections::{BTreeMap, BTreeSet};

use crate::prelude::{Date, LogString, SiemField};

use super::SiemLog;

#[derive(Debug, Clone)]
pub struct SiemLogEdit {
    log: SiemLog,
    tags: BTreeSet<LogString>,
    fields: BTreeMap<LogString, SiemField>,
}

impl SiemLogEdit {
    pub fn new(log: SiemLog) -> Self {
        Self {
            log,
            tags : BTreeSet::new(),
            fields: BTreeMap::new(),
        }
    }

    pub fn message(&self) -> &str {
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
    pub fn origin(&self) -> &str {
        match self.field("origin") {
            Some(SiemField::Text(v)) => v,
            _ => "",
        }
    }
    pub fn set_origin(&mut self, msg: LogString) {
        self.fields
            .insert(LogString::Borrowed("origin"), SiemField::Text(msg).into());
    }
    pub fn tenant(&self) -> &str {
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
    pub fn product(&self) -> &str {
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
    pub fn service(&self) -> &str {
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
    pub fn category(&self) -> &str {
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
    pub fn vendor(&self) -> &str {
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
    pub fn event_received(&self) -> Date {
        match self.field("event.received") {
            Some(SiemField::Date(v)) => *v,
            _ => Date::default(),
        }
    }
    /// Timestamp at witch the log was generated. The clocks at origin must be correctly configured.
    pub fn event_created(&self) -> Date {
        match self.field("event.created") {
            Some(SiemField::Date(v)) => *v,
            _ => Date::default(),
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
    pub fn tags(&self) -> &BTreeSet<LogString> {
        &self.tags
    }
    pub fn field(&self, field_name: &str) -> Option<&SiemField> {
        self.fields.get(field_name)
    }
    pub fn field_mut(&mut self, field_name: &str) -> Option<&mut SiemField> {
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
    pub fn apply_changes(self) -> SiemLog {
        let mut log = self.log;
        for (key, field) in self.fields {
            log.fields.insert(key, field);
        }
        for tag in self.tags {
            log.tags.insert(tag);
        }
        log
    }
    pub fn discard_changes(self) -> SiemLog {
        self.log
    }
}
