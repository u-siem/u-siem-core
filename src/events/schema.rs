use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Data schema that allows indexation of logs with field filtering
#[derive(Serialize, Debug)]
pub struct FieldSchema {
    fields: BTreeMap<&'static str, FieldType>,
    /// When used in table based ddbb, create an extra column to store the rest of the fields. Maybe a JSON file
    pub allow_unknown_fields: bool,
    /// GDPR protection of fields
    gdpr: Option<GdprProtection>,
}

impl FieldSchema {
    pub fn new() -> FieldSchema {
        let mut basic_fields = BTreeMap::new();
        basic_fields.insert("origin", FieldType::Ip);
        basic_fields.insert("tenant", FieldType::Text);
        basic_fields.insert("product", FieldType::Text);
        basic_fields.insert("service", FieldType::Text);
        basic_fields.insert("category", FieldType::Text);
        basic_fields.insert("vendor", FieldType::Text);
        basic_fields.insert("event.type", FieldType::Text);
        basic_fields.insert("tags", FieldType::Text);
        basic_fields.insert("message", FieldType::Text);
        basic_fields.insert("event_received", FieldType::Date);
        basic_fields.insert("event_created", FieldType::Date);
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
                                    for vl in list_val {
                                        alredy_val.insert(vl);
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
    Ip,
    /// A basic String field
    Text,
    /// Signed number with 64 bits
    Numeric,
    /// Decimal number with 64 bits
    Decimal,
    /// Date Type
    Date,
    /// List of posible text values. This is like Text but with autocomplete help
    TextOptions(BTreeSet<&'static str>),
}

#[derive(Serialize, Debug)]
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
