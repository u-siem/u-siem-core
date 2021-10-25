use super::mitre::{MitreTactics, MitreTechniques};
use std::collections::BTreeMap;
use super::SiemLog;
use super::dataset::{SiemDataset, SiemDatasetType};
use super::task::{SiemTask};
use serde::Serialize;
use dyn_clone::{clone_trait_object, DynClone};

/// Common rule format to create rock solid rules
/// The rule must be stateless
pub trait SolidRule : DynClone + Send{
    /// Checks if the log matches this rule. It can return an alert and/or an action to be executed by the SOAR
    fn match_log(&self, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<SiemTask>)>;
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
clone_trait_object!(SolidRule);

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
    pub log : SiemLog,
    /// Time at witch the Alert system must create a new case
    pub aggr_limit : i64,
    /// Key to be used in the aggregation of alerts as to join multiple alerts into one
    pub aggr_key : String
}


#[cfg(test)]
mod tests {
    use super::super::super::events::field::{SiemIp};
    use super::super::super::events::{SiemEvent};
    use super::super::super::events::auth::{AuthLoginType, LoginOutcome, AuthEvent, RemoteLogin};
    use super::*;
    use std::borrow::Cow;
    use lazy_static::lazy_static;
    use std::sync::{Mutex, Arc};


    lazy_static! {
        static ref EXAMPLE_RULE_TACTICS : Vec<MitreTactics>= vec!(MitreTactics::TA0001);
        static ref EXAMPLE_RULE_TECHNIQUES : Vec<MitreTechniques> = vec!(MitreTechniques::T1007);
        static ref EXAMPLE_RULE_DATASETS : Vec<SiemDatasetType> = vec!();
        static ref RULE_STATE : Arc<Mutex<BTreeMap<String, Vec<i64>>>> = Arc::new(Mutex::new(BTreeMap::new()));
    }
    #[derive(Clone)]
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
        fn match_log(&self, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<SiemTask>)> {
            let lang = self.mapping.get(log.tenant()).unwrap_or(&"en");
            let description = self.templates.get(lang).unwrap_or(&"Alert default example").to_string();
            return Some((Some(SiemAlert {
                title : String::from("Alert example"),
                description,
                severity : AlertSeverity::CRITICAL,
                date : chrono::Utc::now().timestamp_millis(),
                tags : vec!(String::from("Critical")),
                rule : String::from("ruleset::example::rule1"),
                log : log.clone(),
                aggr_limit : 0,
                aggr_key : String::from("example::rule")
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
    #[derive(Clone)]
    struct StatefulRule {
        templates : BTreeMap<&'static str,&'static str>,
        mapping : BTreeMap<&'static str,&'static str>,
    }
    impl StatefulRule {
        fn new() -> StatefulRule {
            StatefulRule{
                templates : BTreeMap::new(),
                mapping : BTreeMap::new()
            }
        }
    }
    impl SolidRule for StatefulRule {
        fn match_log(&self, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<SiemTask>)> {
            let lang = self.mapping.get(log.tenant()).unwrap_or(&"en");
            let description = self.templates.get(lang).unwrap_or(&"Alert for user $domain\\$username. $number login errors in less than a minute");

            match log.event() {
                SiemEvent::Auth(auth) => {
                    if auth.outcome() != &LoginOutcome::FAIL {
                        return None
                    }
                    match auth.login_type() {
                        AuthLoginType::Remote(rmt) => {
                            let key = format!("{}|{}",rmt.domain,rmt.user_name);
                            match RULE_STATE.lock() {
                                Ok(mut guard) => {
                                    if guard.contains_key(&key) {
                                        match guard.get_mut(&key) {
                                            Some(v) => {
                                                let timestamp = log.event_created();
                                                v.push(timestamp + 60000);
                                                v.retain(|value| {
                                                    *value > timestamp
                                                });
                                                if v.len() >= 3 {
                                                    let description = description.replace("$domain",&rmt.domain).replace("$username",&rmt.user_name).replace("$number",&v.len().to_string());
                                                    return Some((Some(SiemAlert {
                                                        title : String::from("Stateful example"),
                                                        description,
                                                        severity : AlertSeverity::CRITICAL,
                                                        date : chrono::Utc::now().timestamp_millis(),
                                                        tags : vec!(String::from("Critical")),
                                                        rule : String::from("ruleset::example::rule1"),
                                                        log : log.clone(),
                                                        aggr_limit : chrono::Utc::now().timestamp_millis() + 60000,
                                                        aggr_key : key
                                                    }), None));
                                                }
                                            }
                                            None => {}
                                        }
                                    } else {
                                        guard.insert(key, vec![log.event_created() + 60000]);
                                    }
                                },
                                Err(_) => {
                                    let description = description.replace("$domain",&rmt.domain).replace("$username",&rmt.user_name).replace("$number","1");
                                    return Some((Some(SiemAlert {
                                        title : String::from("Alert example"),
                                        description,
                                        severity : AlertSeverity::CRITICAL,
                                        date : chrono::Utc::now().timestamp_millis(),
                                        tags : vec!(String::from("Critical")),
                                        rule : String::from("ruleset::example::rule1"),
                                        log : log.clone(),
                                        aggr_limit : chrono::Utc::now().timestamp_millis() + 60000,
                                        aggr_key : key
                                    }), None));
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            };
            return None

            
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

    #[test]
    fn check_stateful_rule() {
        let rule = StatefulRule::new();
        let mut log = SiemLog::new(String::from("This is a log example"), 0, SiemIp::V4(0));
        log.set_event(SiemEvent::Auth(AuthEvent {
            hostname : Cow::Borrowed("hostname1"),
            outcome : LoginOutcome::FAIL,
            login_type : AuthLoginType::Remote(RemoteLogin {
                domain : Cow::Borrowed("CNMS"),
                source_address : Cow::Borrowed("10.10.10.10"),
                user_name : Cow::Borrowed("cancamusa")
            })
        }));

        match rule.match_log(&log) {
            Some((_,_)) => {
                panic!("Should not fire an alert")
            },
            None => {}
        }
        match rule.match_log(&log) {
            Some((_,_)) => {
                panic!("Should not fire an alert")
            },
            None => {}
        }
        log.set_event_created(2);
        match rule.match_log(&log) {
            Some((_,_)) => {},
            None => {
                panic!("Should fire an alert")
            }
        }
        log.set_event_created(3);
        match rule.match_log(&log) {
            Some((alert,_)) => {
                let alert = alert.expect("Must have content");
                assert_eq!(alert.description, "Alert for user CNMS\\cancamusa. 4 login errors in less than a minute");
            },
            None => {
                panic!("Should fire an alert")
            }
        }
    }
}