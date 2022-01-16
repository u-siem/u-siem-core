use super::dataset::{SiemDataset, SiemDatasetType};
use super::mitre::{MitreTactics, MitreTechniques};
use super::task::SiemTask;
use super::SiemLog;
use serde::Serialize;
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::Arc;

/// Checks if the log matches this rule. It can return an alert and/or an action to be executed by the SOAR
pub type SiemRuleMatchSync =
    fn(rule: &SiemRule, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<SiemTask>)>;

/// Adds the *timestamp* to the *key_name* key and returns the number of elements stored after removing the elements older than *remove_older*
pub type SharedKeyStore = fn(
    key_name: String,
    timestamp: i64,
    remove_older: i64,
) -> Box<dyn Future<Output = usize> + Unpin>;

pub type SiemRuleMatchAsync =
    fn(
        log : Arc<SiemLog>,
        key_store: SharedKeyStore,
        datasets: Arc<BTreeMap<SiemDatasetType, SiemDataset>>,
    ) -> Box<dyn Future<Output = Option<(Option<SiemAlert>, Option<SiemTask>)>> + Unpin>;

#[derive(Clone)]
pub struct SiemRule {
    /// Name of the rule
    pub name: &'static str,
    /// Name of the Service applied to match this rule
    pub service: &'static str,
    /// A description of the rule to be showed in the UI
    pub description: &'static str,
    /// Includes templateS for this rule. used to generate the alert description
    pub templates: &'static BTreeMap<&'static str, &'static str>,
    /// Sets the mapping of languages to be used in each tenant
    pub tenants: &'static BTreeMap<&'static str, &'static str>,
    /// tactics and techniques covered by this rule
    pub mitre: (&'static Vec<MitreTactics>, &'static Vec<MitreTechniques>),
    /// Datasets to be used by the rules
    pub datasets: BTreeMap<SiemDatasetType, SiemDataset>,
    /// List of datasets needed by this rule
    pub needed_datasets: &'static Vec<SiemDatasetType>,
    /// Checks if the log matches this rule. It can return an alert and/or an action to be executed by the SOAR
    pub rule: SiemRuleMatchSync,
}

impl SiemRule {
    pub fn match_log(&self, log: &SiemLog) -> Option<(Option<SiemAlert>, Option<SiemTask>)> {
        (self.rule)(&self, log)
    }
    /// Updates the references for the datasets
    pub fn set_dataset(&mut self, dataset: SiemDataset) {
        self.datasets.insert(dataset.dataset_type(), dataset);
    }
    /// To be used by the SiemRuleMatch
    pub fn get_dataset(&self, typ: SiemDatasetType) -> Option<&SiemDataset> {
        self.datasets.get(&typ)
    }
    /// To be used by the SiemRuleMatch
    pub fn get_template_for_log(&self, log: &SiemLog) -> &'static str {
        let lang = self.tenants.get(log.tenant()).unwrap_or(&"en");
        match self.templates.get(lang) {
            Some(template) => template,
            None => self.description,
        }
    }
}

pub struct SiemRuleAsync {
    /// Name of the rule
    pub name: &'static str,
    /// Name of the Service applied to match this rule
    pub service: &'static str,
    /// A description of the rule to be showed in the UI
    pub description: &'static str,
    /// Includes templateS for this rule. used to generate the alert description
    pub templates: &'static BTreeMap<&'static str, &'static str>,
    /// Sets the mapping of languages to be used in each tenant
    pub tenants: &'static BTreeMap<&'static str, &'static str>,
    /// tactics and techniques covered by this rule
    pub mitre: (&'static Vec<MitreTactics>, &'static Vec<MitreTechniques>),
    /// List of datasets needed by this rule
    pub needed_datasets: &'static Vec<SiemDatasetType>,
}

impl SiemRuleAsync {
    /// To be used by the SiemRuleMatch
    pub fn get_template_for_log(&self, log: &SiemLog) -> &'static str {
        let lang = self.tenants.get(log.tenant()).unwrap_or(&"en");
        match self.templates.get(lang) {
            Some(template) => template,
            None => self.description,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum AlertSeverity {
    INFORMATIONAL,
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

/// Basic Alert format
#[derive(Serialize, Debug, Clone)]
pub struct SiemAlert {
    pub title: String,
    pub description: String,
    /// Severity of the alert
    pub severity: AlertSeverity,
    /// When the alert was generated
    pub date: i64,
    /// List of tags to be added to the alert
    pub tags: Vec<String>,
    /// Name of the rule that generated the alert
    pub rule: String,
    /// The log that triggered this alert
    pub log: SiemLog,
    /// Time at witch the Alert system must create a new case
    pub aggr_limit: i64,
    /// Key to be used in the aggregation of alerts as to join multiple alerts into one
    pub aggr_key: String,
}

#[cfg(test)]
mod tests {
    use super::super::super::events::auth::{AuthEvent, AuthLoginType, LoginOutcome, RemoteLogin};
    use super::super::super::events::SiemEvent;
    use super::*;
    use lazy_static::lazy_static;
    use std::borrow::Cow;
    use std::future::Future;
    use std::sync::{Arc, Mutex};

    fn example_rule_matcher(
        rule: &SiemRule,
        log: &SiemLog,
    ) -> Option<(Option<SiemAlert>, Option<SiemTask>)> {
        let description = rule.get_template_for_log(log);
        return Some((
            Some(SiemAlert {
                title: String::from("Alert example"),
                description: String::from(description),
                severity: AlertSeverity::CRITICAL,
                date: chrono::Utc::now().timestamp_millis(),
                tags: vec![String::from("Critical")],
                rule: String::from("ruleset::example::rule1"),
                log: log.clone(),
                aggr_limit: 0,
                aggr_key: String::from("example::rule"),
            }),
            None,
        ));
    }

    fn example_stateful_rule_matcher(
        rule: &SiemRule,
        log: &SiemLog,
    ) -> Option<(Option<SiemAlert>, Option<SiemTask>)> {
        let description = rule.get_template_for_log(log);
        if let SiemEvent::Auth(auth) = log.event() {
            if auth.outcome() != &LoginOutcome::FAIL {
                return None;
            }
            if let AuthLoginType::Remote(rmt) = auth.login_type() {
                let key = format!("{}|{}", rmt.domain, rmt.user_name);
                match RULE_STATE.lock() {
                    Ok(mut guard) => {
                        if guard.contains_key(&key) {
                            match guard.get_mut(&key) {
                                Some(v) => {
                                    let timestamp = log.event_created();
                                    v.push(timestamp + 60000);
                                    v.retain(|value| *value > timestamp);
                                    if v.len() >= 3 {
                                        let description = description
                                            .replace("$domain", &rmt.domain)
                                            .replace("$username", &rmt.user_name)
                                            .replace("$number", &v.len().to_string());
                                        return Some((
                                            Some(SiemAlert {
                                                title: String::from("Stateful example"),
                                                description,
                                                severity: AlertSeverity::CRITICAL,
                                                date: chrono::Utc::now().timestamp_millis(),
                                                tags: vec![String::from("Critical")],
                                                rule: String::from("ruleset::example::rule1"),
                                                log: log.clone(),
                                                aggr_limit: chrono::Utc::now().timestamp_millis()
                                                    + 60000,
                                                aggr_key: key,
                                            }),
                                            None,
                                        ));
                                    }
                                }
                                None => {}
                            }
                        } else {
                            guard.insert(key, vec![log.event_created() + 60000]);
                        }
                    }
                    Err(_) => {
                        let description = description
                            .replace("$domain", &rmt.domain)
                            .replace("$username", &rmt.user_name)
                            .replace("$number", "1");
                        return Some((
                            Some(SiemAlert {
                                title: String::from("Alert example"),
                                description,
                                severity: AlertSeverity::CRITICAL,
                                date: chrono::Utc::now().timestamp_millis(),
                                tags: vec![String::from("Critical")],
                                rule: String::from("ruleset::example::rule1"),
                                log: log.clone(),
                                aggr_limit: chrono::Utc::now().timestamp_millis() + 60000,
                                aggr_key: key,
                            }),
                            None,
                        ));
                    }
                }
            }
        }
        return None;
    }

    lazy_static! {
        static ref EXAMPLE_RULE_TACTICS: Vec<MitreTactics> = vec!(MitreTactics::TA0001);
        static ref EXAMPLE_RULE_TECHNIQUES: Vec<MitreTechniques> = vec!(MitreTechniques::T1007);
        static ref EXAMPLE_RULE_DATASETS: Vec<SiemDatasetType> = vec!();
        static ref RULE_STATE: Arc<Mutex<BTreeMap<String, Vec<i64>>>> =
            Arc::new(Mutex::new(BTreeMap::new()));
        static ref EXAMPLE_RULE_TEMPLATES: BTreeMap<&'static str, &'static str> = {
            let mut templates = BTreeMap::new();
            templates.insert("es", "Plantilla en español");
            templates.insert("en", "English template");
            templates
        };
        static ref EXAMPLE_STATEFUL_RULE_TEMPLATES: BTreeMap<&'static str, &'static str> = {
            let mut templates = BTreeMap::new();
            templates.insert(
                "es",
                "Alert for user $domain\\$username. $number login errors in less than a minute",
            );
            templates.insert(
                "en",
                "Alert for user $domain\\$username. $number login errors in less than a minute",
            );
            templates
        };
        static ref EXAMPLE_RULE_TENANTS: BTreeMap<&'static str, &'static str> = {
            let mut tenants = BTreeMap::new();
            tenants.insert("Contoso", "en");
            tenants.insert("Cancamusa", "es");
            tenants
        };
        static ref EXAMPLE_RULE: SiemRule = {
            SiemRule {
                name: "ExampleRule",
                description: "An example rule",
                service: "Example",
                templates: &EXAMPLE_RULE_TEMPLATES,
                tenants: &EXAMPLE_RULE_TENANTS,
                datasets: BTreeMap::new(),
                mitre: (&EXAMPLE_RULE_TACTICS, &EXAMPLE_RULE_TECHNIQUES),
                rule: example_rule_matcher,
                needed_datasets: &EXAMPLE_RULE_DATASETS,
            }
        };
        static ref EXAMPLE_STATEFUL_RULE: SiemRule = {
            SiemRule {
                name: "ExampleRule",
                description: "An example rule",
                service: "Example",
                templates: &EXAMPLE_STATEFUL_RULE_TEMPLATES,
                tenants: &EXAMPLE_RULE_TENANTS,
                datasets: BTreeMap::new(),
                mitre: (&EXAMPLE_RULE_TACTICS, &EXAMPLE_RULE_TECHNIQUES),
                rule: example_stateful_rule_matcher,
                needed_datasets: &EXAMPLE_RULE_DATASETS,
            }
        };
        static ref EXAMPLE_STATEFUL_RULE_SHARED: SiemRuleAsync = SiemRuleAsync {
            name: "ExampleRuleAsync",
            description: "An example rule",
            service: "Example",
            templates: &EXAMPLE_STATEFUL_RULE_TEMPLATES,
            tenants: &EXAMPLE_RULE_TENANTS,
            mitre: (&EXAMPLE_RULE_TACTICS, &EXAMPLE_RULE_TECHNIQUES),
            needed_datasets: &EXAMPLE_RULE_DATASETS,
        };
    }
    fn key_store_example(_a: String, b: i64, _c: i64) -> Box<dyn Future<Output = usize> + Unpin> {
        Box::new(Box::pin(async move {
            if b > 4 {
                b as usize
            } else {
                0
            }
        }))
    }
    fn example_async_rule(
        log: Arc<SiemLog>,
        key_store: SharedKeyStore,
        _datasets: Arc<BTreeMap<SiemDatasetType, SiemDataset>>,
    ) -> Box<dyn Future<Output = Option<(Option<SiemAlert>, Option<SiemTask>)>> + Unpin> {
        Box::new(Box::pin(async move {
            if let SiemEvent::Auth(auth) = log.event() {
                if auth.outcome() != &LoginOutcome::FAIL {
                    return None;
                }
                if let AuthLoginType::Remote(rmt) = auth.login_type() {
                    let timestamp = log.event_created();
                    let remove_older = timestamp - 60000;
                    let key = format!(
                        "{}|{}|{}",
                        EXAMPLE_STATEFUL_RULE_SHARED.name, rmt.domain, rmt.user_name
                    );
                    let n_elements = key_store(key.clone(), timestamp, remove_older).await;
                    if n_elements > 4 {
                        let description = EXAMPLE_STATEFUL_RULE_SHARED
                            .get_template_for_log(&log)
                            .replace("$domain", &rmt.domain)
                            .replace("$username", &rmt.user_name)
                            .replace("$number", &n_elements.to_string());
                        return Some((
                            Some(SiemAlert {
                                title: String::from("Stateful example"),
                                description,
                                severity: AlertSeverity::CRITICAL,
                                date: chrono::Utc::now().timestamp_millis(),
                                tags: vec![String::from("Critical")],
                                rule: String::from("ruleset::example::rule1"),
                                log: (*log).clone(),
                                aggr_limit: chrono::Utc::now().timestamp_millis() + 60000,
                                aggr_key: key,
                            }),
                            None, // No automatic Task, only the alert
                        ));
                    }
                }
            }
            return None;
        }))
    }

    #[async_std::test]
    async fn test_async_rule() {
        let async_rule : SiemRuleMatchAsync = example_async_rule;
        let mut log = SiemLog::new(String::from("This is a log example"), 0, "localhost");
        log.set_tenant(Cow::Borrowed("Contoso"));
        log.set_event(SiemEvent::Auth(AuthEvent {
            hostname: Cow::Borrowed("hostname1"),
            outcome: LoginOutcome::FAIL,
            login_type: AuthLoginType::Remote(RemoteLogin {
                domain: Cow::Borrowed("CNMS"),
                source_address: Cow::Borrowed("10.10.10.10"),
                user_name: Cow::Borrowed("cancamusa"),
            }),
        }));
        let res =
        async_rule(Arc::new(log.clone()), key_store_example, Arc::new(BTreeMap::new()))
                .await;
        if let Some(_) = res {
            panic!("Must not match now");
        }
        log.set_event_created(5);
        let res1 =
        async_rule(Arc::new(log), key_store_example, Arc::new(BTreeMap::new())).await;
        if let Some(res) = res1 {
            if let Some(alert) = res.0 {
                assert_eq!(
                    alert.description,
                    "Alert for user CNMS\\cancamusa. 5 login errors in less than a minute"
                );
            } else {
                panic!("Must return an alert");
            }
        } else {
            panic!("Rule must match");
        }
    }

    #[test]
    fn templates_and_mappings_in_alert() {
        let rule = EXAMPLE_RULE.clone();
        let mut log = SiemLog::new(String::from("This is a log example"), 0, "localhost");
        log.set_tenant(Cow::Borrowed("Contoso"));

        match rule.match_log(&log) {
            Some((alert, _)) => {
                let alert = alert.expect("Must have content");
                assert_eq!(alert.description, "English template");
            }
            None => {
                panic!("Add template test failed")
            }
        }
        let mut log = SiemLog::new(String::from("This is a log example"), 0, "localhost");
        log.set_tenant(Cow::Borrowed("Cancamusa"));

        match rule.match_log(&log) {
            Some((alert, _)) => {
                let alert = alert.expect("Must have content");
                assert_eq!(alert.description, "Plantilla en español");
            }
            None => {
                panic!("Add template test failed")
            }
        }
    }

    #[test]
    fn check_stateful_rule() {
        let rule = EXAMPLE_STATEFUL_RULE.clone();
        let mut log = SiemLog::new(String::from("This is a log example"), 0, "localhost");
        log.set_event(SiemEvent::Auth(AuthEvent {
            hostname: Cow::Borrowed("hostname1"),
            outcome: LoginOutcome::FAIL,
            login_type: AuthLoginType::Remote(RemoteLogin {
                domain: Cow::Borrowed("CNMS"),
                source_address: Cow::Borrowed("10.10.10.10"),
                user_name: Cow::Borrowed("cancamusa"),
            }),
        }));

        match rule.match_log(&log) {
            Some((_, _)) => {
                panic!("Should not fire an alert")
            }
            None => {}
        }
        match rule.match_log(&log) {
            Some((_, _)) => {
                panic!("Should not fire an alert")
            }
            None => {}
        }
        log.set_event_created(2);
        match rule.match_log(&log) {
            Some((_, _)) => {}
            None => {
                panic!("Should fire an alert")
            }
        }
        log.set_event_created(3);
        match rule.match_log(&log) {
            Some((alert, _)) => {
                let alert = alert.expect("Must have content");
                assert_eq!(
                    alert.description,
                    "Alert for user CNMS\\cancamusa. 4 login errors in less than a minute"
                );
            }
            None => {
                panic!("Should fire an alert")
            }
        }
    }
}
