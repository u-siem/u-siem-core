use std::fmt::Display;

use chrono::{NaiveDateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash, Copy, PartialEq)]
pub struct Date(pub i64);

impl Date {
    pub fn now() -> Self {
        Utc::now().timestamp_millis().into()
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dt = NaiveDateTime::from_timestamp_millis(self.0).unwrap_or_default();
        write!(
            f,
            "{}",
            dt.and_utc().to_rfc3339_opts(SecondsFormat::Millis, true)
        )
    }
}

impl From<&i64> for Date {
    fn from(value: &i64) -> Self {
        Self(*value)
    }
}
impl From<i64> for Date {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl From<Date> for i64 {
    fn from(value: Date) -> Self {
        value.0
    }
}
impl From<&Date> for i64 {
    fn from(value: &Date) -> Self {
        value.0
    }
}
impl From<Date> for f64 {
    fn from(value: Date) -> Self {
        value.0 as f64
    }
}
impl From<&Date> for f64 {
    fn from(value: &Date) -> Self {
        value.0 as f64
    }
}
impl From<Date> for u64 {
    fn from(value: Date) -> Self {
        value.0 as u64
    }
}
impl From<&Date> for u64 {
    fn from(value: &Date) -> Self {
        value.0 as u64
    }
}