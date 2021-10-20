use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Data schema that allows indexation of logs with field filtering
#[derive(Serialize, Debug, Clone)]
pub struct FieldSchema {
    pub fields: BTreeMap<&'static str, FieldType>,
    /// When used in table based ddbb, create an extra column to store the rest of the fields. Maybe a JSON file
    pub allow_unknown_fields: bool,
    /// GDPR protection of fields
    pub gdpr: Option<GdprProtection>,
}

impl FieldSchema {
    pub fn new() -> FieldSchema {
        let mut basic_fields = BTreeMap::new();
        basic_fields.insert("origin", FieldType::Ip("IP or Hostname of the server that sent the log"));
        basic_fields.insert("tenant", FieldType::Text("Customer name for SOC environments. Ex: Contoso"));
        basic_fields.insert("product", FieldType::Text("Name of the product for wich the log belongs. Ex: ASA"));
        basic_fields.insert("service", FieldType::Text("Subset of the product logs. Like a OS that can have multiple programs running inside generating multiple logs."));
        basic_fields.insert("category", FieldType::Text("Category of the device: Firewall, web, antivirus"));
        basic_fields.insert("vendor", FieldType::Text("Company that created the product. Ex: Cisco"));
        basic_fields.insert("event.type", FieldType::Text("uSIEM log type: SiemEvent"));
        basic_fields.insert("tags", FieldType::Text("Tags to better describe the event"));
        basic_fields.insert("message", FieldType::Text("Original log message including syslog header"));
        basic_fields.insert("event_received", FieldType::Date("Timestamp at witch the log arrived  "));
        basic_fields.insert("event_created", FieldType::Date("Timestamp at witch the log was generated"));
        FieldSchema {
            fields: basic_fields,
            allow_unknown_fields: false,
            gdpr: None,
        }
    }
    pub fn add_schema(&mut self, schema: &FieldSchema) {
        for (name, element) in &schema.fields {
            match element {
                FieldType::TextOptions(list_val) => {
                    match self.fields.get_mut(name) {
                        Some(alredy_val) => {
                            match alredy_val {
                                FieldType::TextOptions(alredy_val) => {
                                    for (vl_1, vl_2) in list_val {
                                        alredy_val.insert(vl_1, vl_2);
                                    }
                                },
                                _ => {
                                    self.fields.insert(name, element.clone());
                                }
                            }
                        },
                        None => {
                            self.fields.insert(name, element.clone());
                        }
                    }
                    
                }
                _ => {
                    self.fields.insert(name, element.clone());
                }
            }
        }
    }
    pub fn set_gdpr(&mut self, protection: Option<GdprProtection>) {
        self.gdpr = protection;
    }
    pub fn protected_field(&self, field: &str) -> bool {
        match &self.gdpr {
            Some(val) => val.fields.contains(field),
            None => false,
        }
    }
    pub fn get_field(&self, field: &str) -> Option<&FieldType> {
        self.fields.get(field)
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum FieldType {
    /// Save IP as text
    Ip(&'static str),
    /// A basic String field
    Text(&'static str),
    /// Signed number with 64 bits
    Numeric(&'static str),
    /// Decimal number with 64 bits
    Decimal(&'static str),
    /// Date Type
    Date(&'static str),
    /// List of posible text values. This is like Text but with autocomplete help
    TextOptions(BTreeMap<&'static str, &'static str>),
}

#[derive(Serialize, Debug, Clone)]
pub struct GdprProtection {
    /// List of fields that must be protected
    pub fields: BTreeSet<&'static str>,
    pub method: GdprProtectionMethod,
}

#[derive(Serialize, Debug, Clone)]
pub enum GdprProtectionMethod {
    /// Encrypted at REST or similar
    Storage,
    /// Hide field from analysts
    ApiProtected,
}
