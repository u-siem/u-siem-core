use super::mitre::{MitreTactics, MitreTechniques};
use std::collections::BTreeMap;
use std::sync::Arc;
use super::SiemLog;
use super::soar::SoarRequestAction;
use serde::Serialize;

/// Common rule format to create rock solid rules
pub trait SolidRule {
    /// Checks if the log matches this rule. It can return an alert and/or an action to be executed by the SOAR
    fn match_log(&self, log: &Arc<SiemLog>) -> Result<(Option<SiemAlert>, Option<SoarRequestAction>),()>;
    /// Name of the rule
    fn name(&self) -> &'static str;
    /// Name of the Service applied to match this rule
    fn service(&self) -> &'static str;
    /// A description of the rule to be showed in the UI
    fn description(&self) -> &'static str;
    /// Includes a template for this rule used to generate the alert
    fn add_template(&mut self, lang : &'static str, template : &'static str);
    /// Sets the mapping of languages to be used in each tenant
    fn tenants(&mut self, tenants: BTreeMap<&'static str,&'static str>);
    /// Returns the list of tactics and techniques covered by this rule
    fn mitre(&self) -> (Vec<MitreTactics>, Vec<MitreTechniques>);
}

#[derive(Serialize, Debug)]
pub enum AlertSeverity {
    INFORMATIONAL,
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL
}

/// Basic Alert format
#[derive(Serialize, Debug)]
pub struct SiemAlert {
    pub title : String,
    pub description : String,
    /// Severity of the alert
    pub severity : AlertSeverity,
    /// When the alert was generated
    pub date : i64,
    /// List of tags to be added to the alert
    pub tags : Vec<String>,
    /// Name of the service that generated the alert
    pub service : String,
    /// The log that triggered this alert
    pub log : SiemLog
}