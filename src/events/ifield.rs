use serde::{Serialize, Deserialize};

use crate::prelude::{SiemField, types::LogString, SiemIp};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum PreStoredField<T> {
    Invalid,
    #[default]
    None,
    Some(T)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InternalField {
    pub original : SiemField,
    pub array : Box<PreStoredField<Vec<LogString>>>,
    pub text : Box<PreStoredField<LogString>>,
    pub nu64 : Box<PreStoredField<u64>>,
    pub ni64 : Box<PreStoredField<i64>>,
    pub nf64 : Box<PreStoredField<f64>>,
    pub ip : Box<PreStoredField<SiemIp>>
}

impl Into<InternalField> for SiemField{
    fn into(self) -> InternalField {
        InternalField {
            original : self,
            ..Default::default()
        }
    }
}