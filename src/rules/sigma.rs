use std::{borrow::Cow, collections::BTreeMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::prelude::{
    mitre::{MitreTactics, MitreTechniques},
    types::LogString,
    AlertSeverity, SiemField,
};

use super::{
    AlertContent, AlertGenerator, MitreInfo, RuleCondition, RuleOperator, SiemRule, SiemSubRule,
};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SigmaRule {
    /// A brief title for the rule that should contain what the rules is supposed to detect (max. 256 characters)
    pub title: LogString,
    /// Sigma rules should be identified by a globally unique identifier in the id attribute. For this purpose random generated UUIDs (version 4) are recommended but not mandatory. An example for this is:
    /// ```yml
    /// title: Test rule
    /// id: 929a690e-bef0-4204-a928-ef5e620d6fcc
    /// ```
    ///
    /// Rule identifiers can and should change for the following reasons:
    /// - Major changes in the rule. E.g. a different rule logic.
    /// - Derivation of a new rule from an existing or refinement of a rule in a way that both are kept active.
    /// - Merge of rules.
    ///
    /// To being able to keep track on relationships between detections, Sigma rules may also contain references to related rule identifiers in the related attribute. This allows to define common relationships between detections as follows:
    ///
    /// ```yml
    /// related:
    /// - id: 08fbc97d-0a2f-491c-ae21-8ffcfd3174e9
    ///   type: derived
    /// - id: 929a690e-bef0-4204-a928-ef5e620d6fcc
    ///   type: obsoletes
    /// ```
    ///Currently the following types are defined:
    /// - derived: Rule was derived from the referred rule or rules, which may remain active.
    /// - obsoletes: Rule obsoletes the referred rule or rules, which aren't used anymore.
    /// - merged: Rule was merged from the referred rules. The rules may be still existing and in use.
    /// - renamed: The rule had previously the referred identifier or identifiers but was renamed for any other reason, e.g. from a private naming scheme to UUIDs, to resolve collisions etc. It's not expected that a rule with this id exists anymore.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LogString>,
    /// A short description of the rule and the malicious activity that can be detected (max. 65,535 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<LogString>,
    /// References to the source that the rule was derived from. These could be blog articles, technical papers, presentations or even tweets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<LogString>>,
    /// Declares the status of the rule:
    /// - stable: the rule is considered as stable and may be used in production systems or dashboards.
    /// - test: an almost stable rule that possibly could require some fine tuning.
    /// - experimental: an experimental rule that could lead to false results or be noisy, but could also identify interesting events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LogString>,
    /// License of the rule according the SPDX ID specification: https://spdx.org/ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LogString>,
    /// Creator of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<LogString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<LogString>,
    ///This section describes the log data on which the detection is meant to be applied to. It describes the log source, the platform, the application and the type that is required in detection.
    ///
    ///It consists of three attributes that are evaluated automatically by the converters and an arbitrary number of optional elements. We recommend using a "definition" value in cases in which further explication is necessary.
    ///
    /// - category - examples: firewall, web, antivirus
    /// - product - examples: windows, apache, check point fw1
    /// - service - examples: sshd, applocker
    ///
    ///The "category" value is used to select all log files written by a certain group of products, like firewalls or web server logs. The automatic conversion will use the keyword as a selector for multiple indices.
    ///
    ///The "product" value is used to select all log outputs of a certain product, e.g. all Windows Eventlog types including "Security", "System", "Application" and the new log types like "AppLocker" and "Windows Defender".
    ///
    ///Use the "service" value to select only a subset of a product's logs, like the "sshd" on Linux or the "Security" Eventlog on Windows systems.
    ///
    ///The "definition" can be used to describe the log source, including some information on the log verbosity level or configurations that have to be applied. It is not automatically evaluated by the converters but gives useful advice to readers on how to configure the source to provide the necessary events used in the detection.
    ///
    ///You can use the values of 'category, 'product' and 'service' to point the converters to a certain index. You could define in the configuration files that the category 'firewall' converts to ( index=fw1* OR index=asa* ) during Splunk search conversion or the product 'windows' converts to "_index":"logstash-windows*" in ElasticSearch queries.
    pub logsource: Cow<'static, SigmaRuleLogSource>,
    /// A set of search-identifiers that represent searches on log data
    pub detection: Cow<'static, SigmaRuleDetection>,
    /// A list of log fields that could be interesting in further analysis of the event and should be displayed to the analyst.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<LogString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falsepositives: Option<Vec<LogString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<LogString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<LogString>>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SigmaRuleDetection {
    #[serde(flatten)]
    pub search_identifiers: BTreeMap<LogString, SigmaRuleCondition>,
    pub condition: LogString,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmaRuleCondition {
    Map(BTreeMap<LogString, SigmaValue>),
    List(Vec<BTreeMap<LogString, SigmaValue>>),
    #[default]
    None,
}

impl From<SigmaRuleCondition> for SiemSubRule {
    fn from(val: SigmaRuleCondition) -> Self {
        match val {
            SigmaRuleCondition::Map(condition_list) => {
                let mut conditions = Vec::with_capacity(16);
                for (field, value) in condition_list {
                    conditions.push(parse_rule_condition(field, value));
                }
                SiemSubRule {
                    conditions,
                    rule_state: None,
                }
            }
            SigmaRuleCondition::List(condition_list) => {
                let mut conditions = Vec::with_capacity(16);
                for condition in condition_list {
                    for (field, value) in condition {
                        conditions.push(parse_rule_condition(field, value))
                    }
                }
                SiemSubRule {
                    conditions,
                    rule_state: None,
                }
            }
            SigmaRuleCondition::None => SiemSubRule {
                conditions: vec![],
                rule_state: None,
            },
        }
    }
}

fn parse_rule_condition(field: LogString, value: SigmaValue) -> RuleCondition {
    let mut iter = field.split('|');
    let field_name = iter.next().unwrap_or("");
    let operator = iter.next();
    let extra = iter.next();
    if let Some(val) = operator {
        RuleCondition {
            field: Cow::Owned(field_name.to_string()),
            operator: translate_operator(val, extra, value),
        }
    } else {
        RuleCondition {
            field: Cow::Owned(field_name.to_string()),
            operator: translate_content_to_operator(value),
        }
    }
}

fn translate_content_to_operator(value: SigmaValue) -> RuleOperator {
    match value {
        SigmaValue::Text(v) => {
            let starts = v.starts_with('*');
            let ends = v.ends_with('*') && !v.ends_with("\\*");
            if starts && ends && v.len() > 2 {
                RuleOperator::Contains(v[1..v.len() - 2].to_string())
            } else if starts {
                RuleOperator::StartsWith(v[1..].to_string())
            } else if ends && v.len() > 1 {
                RuleOperator::StartsWith(v[..v.len() - 1].to_string())
            } else {
                RuleOperator::Equals(SiemField::Text(Cow::Owned(v.to_string())))
            }
        }
        SigmaValue::Int(v) => RuleOperator::Equals(SiemField::I64(v)),
        SigmaValue::Float(v) => RuleOperator::Equals(SiemField::F64(v)),
        SigmaValue::Array(v) => RuleOperator::Any(
            v.into_iter()
                .map(|v| Box::new(RuleOperator::Equals(v.into())))
                .collect(),
        ),
        SigmaValue::None => RuleOperator::IsNull(true),
    }
}

fn translate_operator(operator: &str, extra: Option<&str>, value: SigmaValue) -> RuleOperator {
    match operator {
        "equals" => RuleOperator::Equals(value.into()),
        "contains" => match value {
            SigmaValue::Text(v) => RuleOperator::Contains(v.to_string()),
            SigmaValue::Int(v) => RuleOperator::Contains(format!("{}", v)),
            SigmaValue::Float(v) => RuleOperator::Contains(format!("{}", v)),
            SigmaValue::Array(v) => {
                if let Some(extra) = extra {
                    if extra == "all" {
                        return RuleOperator::All(
                            v.iter()
                                .map(|v| Box::new(RuleOperator::Contains(v.to_string())))
                                .collect(),
                        );
                    }
                }
                RuleOperator::Any(
                    v.iter()
                        .map(|v| Box::new(RuleOperator::Contains(v.to_string())))
                        .collect(),
                )
            }
            SigmaValue::None => RuleOperator::Contains(String::new()),
        },
        "endswith" => RuleOperator::EndsWith(value.to_string()),
        "startswith" => RuleOperator::StartsWith(value.to_string()),
        _ => RuleOperator::All(vec![]),
    }
}

impl Display for SigmaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigmaValue::Text(v) => f.write_str(v),
            SigmaValue::Int(v) => f.write_fmt(format_args!("{}", v)),
            SigmaValue::Float(v) => f.write_fmt(format_args!("{}", v)),
            SigmaValue::Array(list) => {
                f.write_str("[")?;
                for value in list {
                    f.write_fmt(format_args!("{},", value))?;
                }
                f.write_str("]")
            }
            SigmaValue::None => return Err(std::fmt::Error::default()),
        }?;
        Ok(())
    }
}

impl From<SigmaValue> for SiemField {
    fn from(val: SigmaValue) -> Self {
        match val {
            SigmaValue::Text(v) => SiemField::Text(v),
            SigmaValue::Int(v) => SiemField::I64(v),
            SigmaValue::Float(v) => SiemField::F64(v),
            SigmaValue::Array(v) => {
                SiemField::Array(v.iter().map(|v| LogString::Owned(v.to_string())).collect())
            }
            SigmaValue::None => todo!(),
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SigmaRuleLogSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<LogString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<LogString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<LogString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<LogString>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmaValue {
    Text(LogString),
    Int(i64),
    Float(f64),
    Array(Vec<SigmaValue>),
    #[default]
    None,
}

impl From<SigmaRule> for SiemRule {
    fn from(val: SigmaRule) -> Self {
        let mut slf = val;
        let subrules = parse_subrules(&mut slf);
        let conditions = Vec::with_capacity(16);
        let description = slf.description.unwrap_or_default();
        let alert_content = transform_alert_content(&description);
        SiemRule {
            id: slf.id.unwrap_or_default(),
            name: slf.title,
            mitre: Cow::Owned(MitreInfo {
                tactics: slf
                    .tags
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .filter(|t| t.starts_with("attack."))
                            .map(|t| MitreTactics::try_from(&t[7..]))
                            .filter_map(|v| v.ok())
                            .collect()
                    })
                    .unwrap_or_default(),
                techniques: slf
                    .tags
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .filter(|t| t.starts_with("attack."))
                            .map(|t| MitreTechniques::try_from(&t[7..]))
                            .filter_map(|v| v.ok())
                            .collect()
                    })
                    .unwrap_or_default(),
            }),
            description,
            needed_datasets: vec![],
            subrules: Cow::Owned(subrules),
            conditions: Cow::Owned(conditions),
            alert: Cow::Owned(AlertGenerator {
                content: alert_content,
                severity: level_to_severity(&slf.level.unwrap_or_default()),
                tags: slf.tags.unwrap_or_default(),
                aggregation: None,
            }),
        }
    }
}

fn parse_subrules(rule: &mut SigmaRule) -> BTreeMap<LogString, SiemSubRule> {
    let mut ret = BTreeMap::new();
    for (id, condition) in &rule.detection.search_identifiers {
        ret.insert(Cow::Owned(id.to_string()), condition.clone().into());
    }
    ret
}

fn level_to_severity(level: &str) -> AlertSeverity {
    match level {
        "info" => AlertSeverity::INFORMATIONAL,
        "informational" => AlertSeverity::INFORMATIONAL,
        "INFORMATIONAL" => AlertSeverity::INFORMATIONAL,
        "low" => AlertSeverity::LOW,
        "LOW" => AlertSeverity::LOW,
        "medium" => AlertSeverity::MEDIUM,
        "MEDIUM" => AlertSeverity::MEDIUM,
        "high" => AlertSeverity::HIGH,
        "HIGH" => AlertSeverity::HIGH,
        "critical" => AlertSeverity::CRITICAL,
        "CRITICAL" => AlertSeverity::CRITICAL,
        "critic" => AlertSeverity::CRITICAL,
        "CRITIC" => AlertSeverity::CRITICAL,
        _ => AlertSeverity::LOW,
    }
}

fn transform_alert_content(description: &str) -> Vec<AlertContent> {
    let mut to_return = Vec::with_capacity(16);
    let mut l = SigmaDescriptionLexer::new(description.chars().collect());
    l.read_char();
    loop {
        match l.next_token() {
            Token::EOF => break,
            Token::FIELD(field) => {
                to_return.push(AlertContent::Field(field));
            }
            Token::Text(text) => to_return.push(AlertContent::Text(text)),
        }
    }
    if to_return.is_empty() {
        to_return.push(AlertContent::Text(Cow::Owned(description.to_string())));
    }
    to_return
}

pub struct SigmaDescriptionLexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl SigmaDescriptionLexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let ch = self.ch;
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
            } else {
                return;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let read_text = |l: &mut SigmaDescriptionLexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch != '$' {
                l.read_char();
            }
            let dt = l.input[position..l.position].to_vec();
            l.read_position -= 1;
            dt
        };

        let read_field = |l: &mut SigmaDescriptionLexer| -> Vec<char> {
            let mut to_ret = Vec::with_capacity(32);
            while l.position < l.input.len() && l.ch != ' ' {
                to_ret.push(l.ch);
                l.read_char();
            }
            l.read_position -= 1;
            to_ret
        };

        let tok: Token = match self.ch {
            '$' => {
                self.read_char();
                let data = read_field(self);
                Token::FIELD(Cow::Owned(data.iter().collect()))
            }
            '\0' => Token::EOF,
            _ => {
                let data = read_text(self);
                Token::Text(Cow::Owned(data.iter().collect()))
            }
        };
        self.read_char();
        tok
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(LogString),
    FIELD(LogString),
    EOF,
}

#[test]
fn should_translate_condition() {
    let input = String::from(
        "This is a basic description with source.ip=$source.ip and this a text $field123",
    );
    let mut l = SigmaDescriptionLexer::new(input.chars().collect());
    l.read_char();
    assert_eq!(
        Token::Text(LogString::Borrowed(
            "This is a basic description with source.ip="
        )),
        l.next_token()
    );
    assert_eq!(
        Token::FIELD(LogString::Borrowed("source.ip")),
        l.next_token()
    );
    assert_eq!(
        Token::Text(LogString::Borrowed(" and this a text ")),
        l.next_token()
    );
    assert_eq!(
        Token::FIELD(LogString::Borrowed("field123")),
        l.next_token()
    );
    assert_eq!(Token::EOF, l.next_token());
}

#[test]
fn should_be_deserialized() {
    let rule = include_str!("simple_sigma_rule.yml");
    let yml_test: SigmaRule = serde_yaml::from_str(&rule).unwrap();
    let _yml_text = serde_yaml::to_string(&yml_test).unwrap();
    let rule = include_str!("c2_sigma_rule.yml");
    let yml_test: SigmaRule = serde_yaml::from_str(&rule).unwrap();
    let _yml_text = serde_json::to_string_pretty(&yml_test).unwrap();
    let rule = include_str!("7zip_sigma_rule.yml");
    let yml_test: SigmaRule = serde_yaml::from_str(&rule).unwrap();
    let _yml_text = serde_yaml::to_string(&yml_test).unwrap();
}

#[test]
fn should_transform_c2_sigma_to_siem_rule() {
    let rule = include_str!("c2_sigma_rule.yml");
    let yml_test: SigmaRule = serde_yaml::from_str(&rule).unwrap();
    let siem_rule: SiemRule = yml_test.into();
    assert_eq!(
        &MitreTechniques::T1041,
        siem_rule.mitre.techniques.get(0).unwrap()
    );
    assert_eq!(&AlertContent::Text(LogString::Borrowed("Detects communication to C2 servers mentioned in the operational notes of the ShadowBroker leak of EquationGroup C2 tools")), siem_rule.alert.content.get(0).unwrap());
    let select_incoming = siem_rule
        .subrules
        .get("select_incoming")
        .unwrap()
        .conditions
        .get(0)
        .unwrap();
    assert_eq!("src_ip", select_incoming.field);
    assert_eq!(
        RuleOperator::Any(vec![
            Box::new(RuleOperator::Equals(SiemField::IP([69, 42, 98, 86].into()))),
            Box::new(RuleOperator::Equals(SiemField::IP(
                [89, 185, 234, 145].into()
            )))
        ]),
        select_incoming.operator
    );
    let select_outgoing = siem_rule
        .subrules
        .get("select_outgoing")
        .unwrap()
        .conditions
        .get(0)
        .unwrap();
    assert_eq!("dst_ip", select_outgoing.field);
    assert_eq!(
        RuleOperator::Any(vec![
            Box::new(RuleOperator::Equals(SiemField::IP([69, 42, 98, 86].into()))),
            Box::new(RuleOperator::Equals(SiemField::IP(
                [89, 185, 234, 145].into()
            )))
        ]),
        select_outgoing.operator
    );
}

#[test]
fn should_transform_7zip_sigma_to_siem_rule() {
    let rule = include_str!("7zip_sigma_rule.yml");
    let yml_test: SigmaRule = serde_yaml::from_str(&rule).unwrap();
    let siem_rule: SiemRule = yml_test.into();
    assert_eq!(&AlertContent::Text(LogString::Borrowed("7-Zip through 21.07 on Windows allows privilege escalation (CVE-2022-29072) and command execution when a file with the .7z extension is dragged to the Help>Contents area. This is caused by misconfiguration of 7z.dll and a heap overflow. The command runs in a child process under the 7zFM.exe process.")), siem_rule.alert.content.get(0).unwrap());

    let img_ends_with = siem_rule
        .subrules
        .get("selection_img")
        .unwrap()
        .conditions
        .get(0)
        .unwrap();
    assert_eq!("Image", img_ends_with.field);
    assert_eq!(
        RuleOperator::EndsWith(format!("\\cmd.exe")),
        img_ends_with.operator
    );
    let original_file_name = siem_rule
        .subrules
        .get("selection_img")
        .unwrap()
        .conditions
        .get(1)
        .unwrap();
    assert_eq!("OriginalFileName", original_file_name.field);
    assert_eq!(
        RuleOperator::Equals("Cmd.Exe".into()),
        original_file_name.operator
    );

    let parent_image = siem_rule
        .subrules
        .get("selection_parent")
        .unwrap()
        .conditions
        .get(0)
        .unwrap();
    assert_eq!("ParentImage", parent_image.field);
    assert_eq!(
        RuleOperator::EndsWith(format!("\\7zFM.exe")),
        parent_image.operator
    );

    let bat_command_line = siem_rule
        .subrules
        .get("filter_bat")
        .unwrap()
        .conditions
        .get(0)
        .unwrap();
    assert_eq!("CommandLine", bat_command_line.field);
    assert_eq!(
        RuleOperator::Any(vec![
            Box::new(RuleOperator::Contains(format!(" /c "))),
            Box::new(RuleOperator::Contains(format!(" /k "))),
            Box::new(RuleOperator::Contains(format!(" /r "))),
        ]),
        bat_command_line.operator
    );

    let filter_null = siem_rule
        .subrules
        .get("filter_null")
        .unwrap()
        .conditions
        .get(0)
        .unwrap();
    assert_eq!("CommandLine", filter_null.field);
    assert_eq!(RuleOperator::IsNull(true), filter_null.operator);
}
