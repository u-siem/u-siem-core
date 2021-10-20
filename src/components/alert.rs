use super::mitre::{MitreTactics, MitreTechniques};
use std::collections::BTreeMap;
use super::SiemLog;
use super::dataset::{SiemDataset, SiemDatasetType};
use super::actuator::ActuatorRequest;
use serde::Serialize;

/// Common rule format to create rock solid rules
/// The rule must be stateless
pub trait SolidRule {
    /// Checks if the log matches this rule. It can return an alert and/or an action to be executed by the SOAR
    fn match_log(&self, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<ActuatorRequest>)>;
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
    fn mitre(&self) -> (&'static Vec<MitreTactics>,&'static Vec<MitreTechniques>);
    /// Updates the references for the datasets
    fn set_dataset(&mut self, dataset : SiemDataset);
    /// List of datasets needed by this rule
    fn datasets(&self) -> &'static Vec<SiemDatasetType>;
}

#[derive(Serialize, Debug, Clone)]
pub enum AlertSeverity {
    INFORMATIONAL,
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL
}

/// Basic Alert format
#[derive(Serialize, Debug, Clone)]
pub struct SiemAlert {
    pub title : String,
    pub description : String,
    /// Severity of the alert
    pub severity : AlertSeverity,
    /// When the alert was generated
    pub date : i64,
    /// List of tags to be added to the alert
    pub tags : Vec<String>,
    /// Name of the rule that generated the alert
    pub rule : String,
    /// The log that triggered this alert
    pub log : SiemLog
}


#[cfg(test)]
mod tests {
    use super::super::super::events::field::SiemIp;
    use super::*;
    use std::borrow::Cow;
    use lazy_static::lazy_static;


    lazy_static! {
        static ref EXAMPLE_RULE_TACTICS : Vec<MitreTactics>= vec!(MitreTactics::TA0001);
        static ref EXAMPLE_RULE_TECHNIQUES : Vec<MitreTechniques> = vec!(MitreTechniques::T1007);
        static ref EXAMPLE_RULE_DATASETS : Vec<SiemDatasetType> = vec!();
    }

    struct ExampleRule {
        templates : BTreeMap<&'static str,&'static str>,
        mapping : BTreeMap<&'static str,&'static str>,
    }
    impl ExampleRule {
        fn new() -> ExampleRule {
            let mut templates = BTreeMap::new(); 
            templates.insert("en", "Example");
            ExampleRule{
                templates : templates,
                mapping : BTreeMap::new()
            }
        }
    }
    impl SolidRule for ExampleRule {
        fn match_log(&self, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<ActuatorRequest>)> {
            let lang = self.mapping.get(log.tenant()).unwrap_or(&"en");
            let description = self.templates.get(lang).unwrap_or(&"Alert default example").to_string();
            return Some((Some(SiemAlert {
                title : String::from("Alert example"),
                description,
                severity : AlertSeverity::CRITICAL,
                date : chrono::Utc::now().timestamp_millis(),
                tags : vec!(String::from("Critical")),
                rule : String::from("ruleset::example::rule1"),
                log : log.clone()
            }), None));
        }
        fn name(&self) -> &'static str {
            return "ExampleRule"
        }
        fn service(&self) -> &'static str  {
            return "Example"
        }
        fn description(&self) -> &'static str {
            "An example rule"
        }
        fn add_template(&mut self, lang : &'static str, template : &'static str) {
            self.templates.insert(lang, template);
        }
        fn tenants(&mut self, tenants: BTreeMap<&'static str,&'static str>) {
            self.mapping = tenants;
        }
        fn mitre(&self) -> (&'static Vec<MitreTactics>, &'static Vec<MitreTechniques>) {
            return (&EXAMPLE_RULE_TACTICS, &EXAMPLE_RULE_TECHNIQUES)
        }
        fn set_dataset(&mut self, _dataset : SiemDataset) {
        }
        fn datasets(&self) -> &'static Vec<SiemDatasetType> {
            return &EXAMPLE_RULE_DATASETS
        }
    }

    #[test]
    fn templates_and_mappings_in_alert() {
        let mut rule = ExampleRule::new();
        rule.add_template("en", "Template example");
        rule.add_template("es", "Ejemplo plantilla");
        let mut tenants = BTreeMap::new();
        tenants.insert("Contoso", "en");
        tenants.insert("Contosa", "es");
        rule.tenants(tenants);

        let mut log = SiemLog::new(String::from("This is a log example"), 0, SiemIp::V4(0));
        log.set_tenant(Cow::Borrowed("Contoso"));

        match rule.match_log(&log) {
            Some((alert,_)) => {
                let alert = alert.expect("Must have content");
                assert_eq!(alert.description, "Template example");
            },
            None => {
                panic!("Add template test failed")
            }
        }
        let mut log = SiemLog::new(String::from("This is a log example"), 0, SiemIp::V4(0));
        log.set_tenant(Cow::Borrowed("Contosa"));
        
        match rule.match_log(&log) {
            Some((alert,_)) => {
                let alert = alert.expect("Must have content");
                assert_eq!(alert.description, "Ejemplo plantilla");
            },
            None => {
                panic!("Add template test failed")
            }
        }
    }
}