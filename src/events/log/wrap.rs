use std::collections::{BTreeMap, BTreeSet};
use crate::prelude::{AssetId, Domain, LogString, SiemField, User};

use super::SiemLog;


/// Speeds up correlation of rules by reusing transformations and casting of fields
#[derive(Debug, Clone)]
pub struct SiemLogWrapper {
    log: SiemLog,
    /// Transformed fields
    tfields: BTreeMap<LogString, TransformedField>,
}

#[derive(Debug, Clone, Default)]
pub enum PreStoredField<T> {
    Invalid,
    #[default]
    None,
    Some(T),
}

#[derive(Debug, Clone)]
pub struct TransformedField {
    pub u64 : PreStoredField<u64>,
    pub i64 : PreStoredField<i64>,
    pub f64 : PreStoredField<f64>,
    pub ip : PreStoredField<std::net::IpAddr>,
    pub domain : PreStoredField<Domain>,
    pub user : PreStoredField<User>,
    pub asset_id : PreStoredField<AssetId>,
    pub text : PreStoredField<LogString>,
    pub array: PreStoredField<Vec<LogString>>
}

impl SiemLogWrapper {
    pub fn new(log : SiemLog) -> Self {
        Self {
            log,
            tfields : BTreeMap::new()
        }
    }
}

impl From<SiemLogWrapper> for SiemLog {
    fn from(value: SiemLogWrapper) -> Self {
        value.log
    }
}