use crate::prelude::{SiemField, SiemIp, AlertSeverity, AlertAggregation};

use super::dataset::{ SiemDatasetType};
use super::mitre::{MitreTactics, MitreTechniques};
use regex::Regex;
use crate::prelude::types::LogString;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize, Serializer, de};

pub mod sigma;

#[derive(Clone, Serialize, Deserialize)]
pub struct SiemRule {
    pub id: LogString,
    /// Name of the rule
    pub name: LogString,
    /// A description of the rule to be showed in the UI
    pub description: LogString,
    /// tactics and techniques covered by this rule
    pub mitre: Cow<'static,MitreInfo>,
    /// List of datasets needed by this rule
    pub needed_datasets: Vec<SiemDatasetType>,
    /// List of subrules that this rule is made of
    pub subrules : Cow<'static,BTreeMap<LogString, SiemSubRule>>,
    /// List of subrules that triggers this rule.
    pub conditions : Cow<'static,Vec<Vec<LogString>>>,
    /// Generates the content of the alert
    pub alert : Cow<'static,AlertGenerator>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AlertGenerator {
    pub content : Vec<AlertContent>,
    pub severity : AlertSeverity,
    pub tags : Vec<LogString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation : Option<AlertAggregation>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MitreInfo {
    pub tactics : Vec<MitreTactics>,
    pub techniques : Vec<MitreTechniques>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SiemSubRule {
    pub conditions : Vec<RuleCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_state : Option<RuleState>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field : LogString,
    #[serde(flatten)]
    pub operator : RuleOperator,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AlertContent {
    /// A basic text
    Text(LogString),
    /// Content of a Log field
    Field(LogString),
    /// List of matched rules joined by a string. Ex ("\n", ","...)
    MatchedRules(LogString)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum RuleOperator {
    All(Vec<Box<RuleOperator>>),
    Any(Vec<Box<RuleOperator>>),
    Not(Box<RuleOperator>),
    Equals(SiemField),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    GT(SiemField),
    LT(SiemField),
    GTE(SiemField),
    LTE(SiemField),
    #[serde(serialize_with = "regex_to_string", deserialize_with = "string_to_regex")]
    Matches(Regex),
    SameNet((SiemIp, u8)),
    IsLocalIp(bool),
    IsExternalIp(bool),
    Exists(bool),
    IsNull(bool),
    B64(Box<RuleOperator>),
    InDataset(SiemDatasetType),
    ExistsRuleState(Vec<RuleState>),
    InCountry(String),
}

impl PartialEq for RuleOperator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::All(v1), Self::All(v2)) => v1 == v2,
            (Self::Any(v1), Self::Any(v2)) => v1 == v2,
            (Self::Not(v1), Self::Not(v2)) => v1 == v2,
            (Self::Equals(v1), Self::Equals(v2)) => v1 == v2,
            (Self::StartsWith(v1), Self::StartsWith(v2)) => v1 == v2,
            (Self::EndsWith(v1), Self::EndsWith(v2)) => v1 == v2,
            (Self::Contains(v1), Self::Contains(v2)) => v1 == v2,
            (Self::GT(v1), Self::GT(v2)) => v1 == v2,
            (Self::LT(v1), Self::LT(v2)) => v1 == v2,
            (Self::GTE(v1), Self::GTE(v2)) => v1 == v2,
            (Self::LTE(v1), Self::LTE(v2)) => v1 == v2,
            (Self::Matches(v1), Self::Matches(v2)) => v1.as_str() == v2.as_str(),
            (Self::SameNet((v1,v11)), Self::SameNet((v2,v22))) => v1 == v2 && v11 == v22,
            (Self::IsLocalIp(v1), Self::IsLocalIp(v2)) => v1 == v2,
            (Self::IsExternalIp(v1), Self::IsExternalIp(v2)) => v1 == v2,
            (Self::Exists(v1), Self::Exists(v2)) => v1 == v2,
            (Self::B64(v1), Self::B64(v2)) => v1 == v2,
            (Self::InDataset(v1), Self::InDataset(v2)) => v1 == v2,
            (Self::ExistsRuleState(v1), Self::ExistsRuleState(v2)) => v1 == v2,
            (Self::InCountry(v1), Self::InCountry(v2)) => v1 == v2,
            (Self::IsNull(v1), Self::IsNull(v2)) => v1 == v2,
            _ => false
        }
    }
}

fn regex_to_string<S>(x: &Regex, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

fn string_to_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: de::Deserializer<'de>,
{
    // define a visitor that deserializes
    // `ActualData` encoded as json within a string
    struct RegexVisitor;

    impl<'de> de::Visitor<'de> for RegexVisitor {
        type Value = Regex;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing json data")
        }
    
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // unfortunately we lose some typed information
            // from errors deserializing the json string
            Regex::from_str(v).map_err(E::custom)
        }
    }
    
    // use our visitor to deserialize an `ActualValue`
    deserializer.deserialize_any(RegexVisitor)
}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RuleState {
    pub states : RuleStateValue
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RuleStateValue {
    Text(LogString),
    Field(LogString)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AlertDictionary {
    pub language_map : BTreeMap<LogString, BTreeMap<LogString, LogString>>
}
impl AlertDictionary {
    pub fn get_mappings_for(&self, language : &str) -> Option<&BTreeMap<LogString, LogString>> {
        self.language_map.get(language)
    }

    pub fn get_mappings_for_id(&self, language : &str, id : &str) -> Option<&LogString> {
        self.language_map.get(language).and_then(|v| v.get(id))
    }
}

#[test]
fn should_be_serialized_and_deserialize() {
    let superrule = SiemRule {
        id : LogString::Borrowed("id001"),
        name: LogString::Borrowed("Rule001"),
        description: LogString::Borrowed("descripcion"),
        mitre: Cow::Owned(MitreInfo { tactics: vec![MitreTactics::TA0001], techniques: vec![] }),
        needed_datasets: vec![SiemDatasetType::BlockIp],
        subrules: {
            let mut map = BTreeMap::new();
            map.insert(LogString::Borrowed("rule_source_ip"), SiemSubRule { 
                conditions: vec![
                    RuleCondition {
                        field : LogString::Borrowed("source.ip"),
                        operator : RuleOperator::Not(Box::new(RuleOperator::Equals(SiemField::IP([192,168,1,1].into()))))
                    }
                ], 
                rule_state: None 
            });
            map.insert(LogString::Borrowed("rule_destination_ip"), SiemSubRule { 
                conditions: vec![
                    RuleCondition {
                        field : LogString::Borrowed("destination.ip"),
                        operator : RuleOperator::All(vec![
                            Box::new(RuleOperator::IsExternalIp(true)),
                            Box::new(RuleOperator::InDataset(SiemDatasetType::BlockIp))
                        ])
                    }
                ], 
                rule_state: None 
            });
            Cow::Owned(map)
        },
        conditions: Cow::Owned(vec![vec![LogString::Borrowed("rule_source_ip"), LogString::Borrowed("rule_destination_ip")]]),
        alert: Cow::Owned(AlertGenerator {
            content : vec![
                AlertContent::Text(LogString::Borrowed("A local ip tried to connect to a a malicious IP: source.ip=")),
                AlertContent::Field(LogString::Borrowed("source.ip")),
                AlertContent::Text(LogString::Borrowed(", destination.ip=")),
                AlertContent::Field(LogString::Borrowed("destination.ip")),
            ],
            severity : AlertSeverity::HIGH,
            tags : vec![LogString::Borrowed("external_attack")],
            aggregation : None
        })
    };
    let json_txt = serde_json::to_string_pretty(&superrule).unwrap();
    let _v : SiemRule = serde_json::from_str(&json_txt).unwrap();

    let new_superrule = superrule.clone();
    match new_superrule.name {
        Cow::Borrowed(_) => {},
        _ => {unreachable!("Should not be owned")}
    };
}