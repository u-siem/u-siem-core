use serde::{Deserialize, Serialize};

use crate::prelude::{types::LogString, SiemField, SiemIp};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum PreStoredField<T> {
    Invalid,
    #[default]
    None,
    Some(T),
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct InternalField {
    pub original: SiemField,
    #[serde(skip)]
    pub array: Box<PreStoredField<Vec<LogString>>>,
    #[serde(skip)]
    pub text: Box<PreStoredField<LogString>>,
    #[serde(skip)]
    pub nu64: Box<PreStoredField<u64>>,
    #[serde(skip)]
    pub ni64: Box<PreStoredField<i64>>,
    #[serde(skip)]
    pub nf64: Box<PreStoredField<f64>>,
    #[serde(skip)]
    pub ip: Box<PreStoredField<SiemIp>>,
}

impl From<SiemField> for InternalField {
    fn from(val: SiemField) -> Self {
        let mut ifield = InternalField {
            original: val,
            ..Default::default()
        };
        match &ifield.original {
            SiemField::F64(v) => {
                ifield.nf64 = Box::new(PreStoredField::Some(*v));
            }
            SiemField::I64(v) => {
                ifield.ni64 = Box::new(PreStoredField::Some(*v));
            }
            SiemField::Date(v) => {
                ifield.ni64 = Box::new(PreStoredField::Some(*v));
            }
            SiemField::U64(v) => {
                ifield.nu64 = Box::new(PreStoredField::Some(*v));
            }
            SiemField::IP(v) => {
                ifield.ip = Box::new(PreStoredField::Some(*v));
            }
            _ => {}
        }
        ifield
    }
}

impl Serialize for InternalField {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match &self.original {
            SiemField::Null => s.serialize_none(),
            SiemField::Text(v) => s.serialize_str(&v),
            SiemField::IP(v) => v.serialize(s),
            SiemField::Domain(v) => v.serialize(s),
            SiemField::User(v) => v.serialize(s),
            SiemField::AssetID(v) => v.serialize(s),
            SiemField::U64(v) => v.serialize(s),
            SiemField::I64(v) => v.serialize(s),
            SiemField::F64(v) => v.serialize(s),
            SiemField::Date(v) => v.serialize(s),
            SiemField::Array(v) => v.serialize(s),
            SiemField::Path(v) => v.serialize(s),
        }
    }
}