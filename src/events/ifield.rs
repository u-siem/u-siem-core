use serde::{Deserialize, Serialize};

use crate::prelude::{types::LogString, SiemField, SiemIp};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum PreStoredField<T> {
    Invalid,
    #[default]
    None,
    Some(T),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InternalField {
    pub original: SiemField,
    pub array: Box<PreStoredField<Vec<LogString>>>,
    pub text: Box<PreStoredField<LogString>>,
    pub nu64: Box<PreStoredField<u64>>,
    pub ni64: Box<PreStoredField<i64>>,
    pub nf64: Box<PreStoredField<f64>>,
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
