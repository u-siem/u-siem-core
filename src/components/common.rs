use super::super::events::field::SiemIp;
use super::super::events::SiemLog;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use super::dataset::SiemDataset;

#[derive(Serialize, Debug)]
pub enum SiemMessage {
    /// Execute a function in the component
    Command(SiemFunctionCall),
    /// Response to a function call
    Response(SiemFunctionResponse),
    /// Process a log
    Log(SiemLog),
    /// Local logging system.
    Notification(Cow<'static, str>),
    /// Dataset updated, this is the last state of it
    Dataset((Cow<'static, str>, SiemDataset)),
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
    view : Cow<'static, str>,
    datasets : Vec<DatasetDefinition>,
    commands: Vec<CommandDefinition>,
}
impl SiemComponentCapabilities {
    pub fn new(
        name: Cow<'static, str>,
        description: Cow<'static, str>,
        view : Cow<'static, str>,
        datasets : Vec<DatasetDefinition>,
        commands: Vec<CommandDefinition>,
    ) -> SiemComponentCapabilities {
        return SiemComponentCapabilities {
            name,
            description,
            view,
            datasets,
            commands
        };
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn view(&self) -> &str {
        &self.view
    }
    pub fn datasets(&self) -> &Vec<DatasetDefinition> {
        &self.datasets
    }
    pub fn commands(&self) -> &Vec<CommandDefinition> {
        &self.commands
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
pub struct CommandDefinition {
    class: SiemFunctionType,
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    min_permission: UserRole,
}
impl CommandDefinition {
    pub fn new(
        class: SiemFunctionType,
        name: Cow<'static, str>,
        description: Cow<'static, str>,
        min_permission: UserRole,
    ) -> CommandDefinition {
        CommandDefinition {
            class,
            name,
            description,
            min_permission
        }
    }

    pub fn class(&self) -> &SiemFunctionType {
        &self.class
    }
    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
    pub fn description(&self) -> &Cow<'static, str> {
        &self.description
    }
    pub fn min_permission(&self) -> &UserRole {
        &self.min_permission
    }
}

#[derive(Serialize, Debug)]
pub struct DatasetDefinition {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    min_permission: UserRole,
}
impl DatasetDefinition {
    pub fn new(
        name: Cow<'static, str>,
        description: Cow<'static, str>,
        min_permission: UserRole,
    ) -> DatasetDefinition {
        DatasetDefinition {
            name,
            description,
            min_permission
        }
    }
    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
    pub fn description(&self) -> &Cow<'static, str> {
        &self.description
    }
    pub fn min_permission(&self) -> &UserRole {
        &self.min_permission
    }
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
    OTHER(
        Cow<'static, str>,
        BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    ),
}

/// A simple object with the logic to parse Logs
pub trait LogParser {
    fn parse_log(log: SiemLog) -> Result<SiemLog, LogParsingError>;
    fn device_match(log: &SiemLog) -> bool;
    fn name() -> Cow<'static, str>;
}

/// Error at parsing a log
pub enum LogParsingError {
    /// The parser can't be used with this log
    NoValidParser(SiemLog),
    /// The parser can be used with this log but has some bug
    ParserError(SiemLog),
}

#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SiemFunctionCall {
    /// Component name
    START_COMPONENT(Cow<'static, str>),
    /// Component name
    STOP_COMPONENT(Cow<'static, str>),
    /// Query in database format
    LOG_QUERY(Cow<'static, str>),
    /// IP of the device to isolate
    ISOLATE_IP(SiemIp),
    /// IP of the device to isolate
    ISOLATE_ENDPOINT(SiemIp),
    /// (IP, Comment)
    FILTER_IP(SiemIp, Cow<'static, str>),
    /// (Domain, Comment)
    FILTER_DOMAIN(Cow<'static, str>, Cow<'static, str>),
    /// Function name, Parameters
    OTHER(Cow<'static, str>, serde_json::Value),
}
#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum SiemFunctionResponse {
    START_COMPONENT(Result<Cow<'static, str>, Cow<'static, str>>),
    STOP_COMPONENT(Result<Cow<'static, str>, Cow<'static, str>>),
    LOG_QUERY(Result<serde_json::Value, Cow<'static, str>>),
    ISOLATE_IP(Result<Cow<'static, str>, Cow<'static, str>>),
    ISOLATE_ENDPOINT(Result<Cow<'static, str>, Cow<'static, str>>),
    /// (IP, Comment)
    FILTER_IP(Result<Cow<'static, str>, Cow<'static, str>>),
    /// (Domain, Comment)
    FILTER_DOMAIN(Result<Cow<'static, str>, Cow<'static, str>>),
    OTHER(
        Cow<'static, str>,
        Result<Cow<'static, str>, Cow<'static, str>>,
    ),
}
