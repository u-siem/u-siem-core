use crate::{events::SiemLog, utils::types::LogString};

use super::mitre::MitreTechniques;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlertSeverity {
    INFORMATIONAL,
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

/// Basic Alert format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiemAlert {
    pub title: LogString,
    pub description: LogString,
    /// Severity of the alert
    pub severity: AlertSeverity,
    /// When the alert was generated
    pub date: i64,
    /// List of tags to be added to the alert
    pub tags: Vec<LogString>,
    /// List of MitreAtack Techniques
    pub techniques: Vec<MitreTechniques>,
    /// Name of the rule that generated the alert
    pub rule: LogString,
    /// List of logs that triggered this alert
    pub log: Vec<SiemLog>,
    pub aggregation: Option<AlertAggregation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlertAggregation {
    /// Time at witch the Alert system must create a new case
    pub limit: i64,
    /// Key to be used in the aggregation of alerts as to join multiple alerts into one
    pub key: String,
}
