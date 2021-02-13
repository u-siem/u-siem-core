use super::super::events::{SiemLog};
use super::super::events::field::SiemIp;
use serde::Serialize;
use std::collections::BTreeMap;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
pub enum SiemMessage {
    /// Execute a function in the component
    Command(SiemFunctionCall),
    /// Response to a function call
    Response(SiemFunctionResponse),
    /// Process a log
    Log(SiemLog),
    Dataset,
}

pub trait SiemComponentStateStorage {
    /// Read a key value from the database
    fn read_value(key: Cow<'static, str>) -> Result<serde_json::Value, Cow<'static, str>>;
    /// Write to the database a key/value pair
    fn set_value(key: Cow<'static, str>, value: serde_json::Value)
        -> Result<(), Cow<'static, str>>;
}

#[derive(Serialize, Debug)]
pub struct SiemComponentCapabilities {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    capabilities: Vec<ComponentCapabilities>,
}
impl SiemComponentCapabilities {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn capabilities(&self) -> &Vec<ComponentCapabilities> {
        &self.capabilities
    }
}

/// An easy to use role based system
#[derive(Serialize, Debug)]
pub enum UserRole {
    /// Review the system (Read-Only configuration: rules, use-cases, Sources with parsers)
    Compliance,
    /// Do searchs (Read-Only configuration and information)
    Analyst,
    /// Launch postproceses (Read-only configuration and Read-Write information)
    Engineer,
    /// Configure the system (Full Read-Write access)
    Administrator,
}

#[derive(Serialize, Debug)]
pub enum ComponentCapabilities {
    /// Actions for the component
    Command(CommandDefinition),
    /// The component uses datasets or updates them
    Dataset(DatasetDefinition),
    /// HTML and javascript components that modifies the SIEM UI.
    View(Cow<'static, str>),
}

#[derive(Serialize, Debug)]
pub struct CommandDefinition {
    class: SiemFunctionType,
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    min_permission: UserRole,
}

#[derive(Serialize, Debug)]
pub struct DatasetDefinition {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    min_permission: UserRole,
}

/// Function launch and forget
#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SiemFunctionType {
    STOP_COMPONENT,
    START_COMPONENT,
    LOG_QUERY,
    ISOLATE_IP,
    ISOLATE_ENDPOINT,
    FILTER_IP,
    FILTER_DOMAIN,
    /// Function name, Map<ParamName, Description>
    OTHER(Cow<'static, str>,BTreeMap<Cow<'static,str>,Cow<'static,str>>),
}

/// A simple object with the logic to parse Logs
pub trait LogParser {
    fn parse_log(log : SiemLog) -> Result<SiemLog, LogParsingError>;
    fn device_match(log : &SiemLog) -> bool;
    fn name() -> Cow<'static, str>;
}

/// Error at parsing a log
pub enum LogParsingError {
    /// The parser can't be used with this log
    NoValidParser(SiemLog),
    /// The parser can be used with this log but has some bug
    ParserError(SiemLog)
}

#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SiemFunctionCall {
    /// Component name
    START_COMPONENT(Cow<'static,str>),
    /// Component name
    STOP_COMPONENT(Cow<'static,str>),
    /// Query in database format
    LOG_QUERY(Cow<'static,str>),
    /// IP of the device to isolate
    ISOLATE_IP(SiemIp),
    /// IP of the device to isolate
    ISOLATE_ENDPOINT(SiemIp),
    /// (IP, Comment)
    FILTER_IP(SiemIp, Cow<'static,str>),
    /// (Domain, Comment)
    FILTER_DOMAIN(Cow<'static,str>, Cow<'static,str>),
    /// Function name, Parameters
    OTHER(Cow<'static, str>, serde_json::Value)
}
#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SiemFunctionResponse {
    START_COMPONENT(Result<Cow<'static,str>,Cow<'static,str>>),
    STOP_COMPONENT(Result<Cow<'static,str>,Cow<'static,str>>),
    LOG_QUERY(serde_json::Value),
    ISOLATE_IP(Result<Cow<'static,str>,Cow<'static,str>>),
    ISOLATE_ENDPOINT(Result<Cow<'static,str>,Cow<'static,str>>),
    /// (IP, Comment)
    FILTER_IP(Result<Cow<'static,str>,Cow<'static,str>>),
    /// (Domain, Comment)
    FILTER_DOMAIN(Result<Cow<'static,str>,Cow<'static,str>>),
    OTHER(Cow<'static, str>,Result<Cow<'static,str>,Cow<'static,str>>)
}