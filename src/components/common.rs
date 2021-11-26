use super::super::events::field::SiemIp;
use super::super::events::schema::FieldSchema;
use super::super::events::SiemLog;
use super::alert::SiemAlert;
use super::dataset::SiemDataset;
use super::metrics::SiemMetric;
use super::task::{SiemTask, SiemTaskResult};
use dyn_clone::{clone_trait_object, DynClone};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Serialize, Debug)]
#[non_exhaustive]
pub enum SiemMessage {
    /// Execute a command in the component
    Command(SiemCommandHeader, SiemCommandCall),
    /// Response to a function call, first element is the ID of the Response
    Response(SiemCommandHeader, SiemCommandResponse),
    /// Process a log
    Log(SiemLog),
    /// Local logging system. First element is the ID of the component, to be able to route messages
    Notification(u64, Cow<'static, str>),
    /// Dataset updated, this is the last state of it.
    Dataset(SiemDataset),
    /// Alerting
    Alert(SiemAlert),
    /// Send/Receive Metrics, first element is the ID of the component, second is the name of the metric
    Metrics(u64, Cow<'static, str>, SiemMetric), //TODO: use metrics like prometheus
    Task(SiemCommandHeader, SiemTask),
    TaskResult(SiemCommandHeader, SiemTaskResult),
}

#[derive(Serialize, Debug)]
#[non_exhaustive]
pub enum StorageError {
    NotExists,
    ConnectionError,
    AlredyExists,
}

pub trait SiemComponentStateStorage: DynClone + Send {
    /// Read a key value from the database
    fn get_value(&self, key: Cow<'static, str>) -> Result<String, StorageError>;
    /// Write to the database a key/value pair
    fn set_value(
        &mut self,
        key: Cow<'static, str>,
        value: String,
        replace: bool,
    ) -> Result<(), StorageError>;

    /// Get a file
    fn get_file(&self, filepath: String) -> Result<Vec<u8>, StorageError>;

    /// Get the size of a file
    fn get_file_size(&self, filepath: String) -> Result<u64, StorageError>;

    /// Get a file part
    fn get_file_range(
        &self,
        filepath: String,
        start: u64,
        end: u64,
    ) -> Result<Vec<u8>, StorageError>;

    /// Sets the content of a file
    fn set_file(&mut self, filepath: String, content: Vec<u8>) -> Result<(), StorageError>;

    /// Sets the content of a file
    fn set_file_range(
        &mut self,
        filepath: String,
        content: Vec<u8>,
        start: u64,
        end: u64,
    ) -> Result<(), StorageError>;

    fn duplicate(&self) -> Box<dyn SiemComponentStateStorage>;
}
clone_trait_object!(SiemComponentStateStorage);

#[derive(Serialize, Debug)]
pub struct SiemComponentCapabilities {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    view: Cow<'static, str>,
    datasets: Vec<DatasetDefinition>,
    commands: Vec<CommandDefinition>,
    tasks: Vec<TaskDefinition>,
}
impl SiemComponentCapabilities {
    pub fn new(
        name: Cow<'static, str>,
        description: Cow<'static, str>,
        view: Cow<'static, str>,
        datasets: Vec<DatasetDefinition>,
        commands: Vec<CommandDefinition>,
        tasks: Vec<TaskDefinition>,
    ) -> SiemComponentCapabilities {
        return SiemComponentCapabilities {
            name,
            description,
            view,
            datasets,
            commands,
            tasks,
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
    pub fn tasks(&self) -> &Vec<TaskDefinition> {
        &self.tasks
    }
}

/// An easy to use role based system
#[derive(Serialize, Debug, Clone)]
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
            min_permission,
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
pub struct TaskDefinition {
    class: SiemTaskType,
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    min_permission: UserRole,
}
impl TaskDefinition {
    pub fn new(
        class: SiemTaskType,
        name: Cow<'static, str>,
        description: Cow<'static, str>,
        min_permission: UserRole,
    ) -> TaskDefinition {
        TaskDefinition {
            class,
            name,
            description,
            min_permission,
        }
    }

    pub fn class(&self) -> &SiemTaskType {
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
            min_permission,
        }
    }
    /// Name of the dataset
    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
    /// Description of the dataset
    pub fn description(&self) -> &Cow<'static, str> {
        &self.description
    }
    /// Permission needed to access this dataset
    pub fn min_permission(&self) -> &UserRole {
        &self.min_permission
    }
}

#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemTaskType {
    /// Script name and Script parameters
    EXECUTE_ENDPOINT_SCRIPT(
        Cow<'static, str>,
        BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    ),
    /// Remediate a list of emails. List of parameters
    REMEDIATE_EMAILS(BTreeMap<Cow<'static, str>, Cow<'static, str>>),
    /// Report IP, email to abuse mail. Needed provider name and parameters
    REPORT_ABUSE(BTreeMap<Cow<'static, str>, Cow<'static, str>>),
    UPDATE_GEOIP,
    /// Task name, Map<ParamName, Description>
    OTHER(
        Cow<'static, str>,
        BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    ),
}

/// Define commands to be used by the users or other components.
#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemFunctionType {
    STOP_COMPONENT,
    START_COMPONENT,
    LOG_QUERY,
    ISOLATE_IP,
    ISOLATE_ENDPOINT,
    FILTER_IP,
    FILTER_DOMAIN,
    FILTER_EMAIL_SENDER,
    LIST_USE_CASES,
    GET_USE_CASES,
    LIST_RULES,
    GET_RULE,
    LIST_TASKS,
    LIST_DATASETS,
    DOWNLOAD_QUERY,
    /// Function name, Map<ParamName, Description>
    OTHER(
        Cow<'static, str>,
        BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    ),
}

/// A simple object with the logic to parse Logs.
pub trait LogParser: DynClone + Send {
    /// Parse the log. If it fails it must give a reason why. This allow optimization of the parsing process.
    fn parse_log(&self, log: SiemLog) -> Result<SiemLog, LogParsingError>;
    /// Check if the parser can parse the log. Must be fast.
    fn device_match(&self, log: &SiemLog) -> bool;
    /// Name of the parser
    fn name(&self) -> &str;
    /// Description of the parser
    fn description(&self) -> &str;
    /// Get parser schema
    fn schema(&self) -> &'static FieldSchema;
}
clone_trait_object!(LogParser);

/// This is the most complex type of parser. It's statefull to store past logs.
/// Think of the USB event in linux, we need the rest of the logs to extract all information.
/// The Parser component which uses this parsers must be able to store and load past Logs
/// if the user connects to a different SIEM node (LoadBalancing).
pub trait MultilineLogParser: DynClone + Send {
    /// Parse the log. If it fails it must give a reason why. This allow optimization of the parsing process.
    fn parse_log(&mut self, log: SiemLog) -> Result<Option<SiemLog>, LogParsingError>;
    /// Check if the parser can parse the log. Must be fast.
    fn device_match(&self, log: &SiemLog) -> bool;
    /// Name of the parser
    fn name(&self) -> &str;
    /// Description of the parser
    fn description(&self) -> &str;
    /// The connection with the origin has been closed. We must preserve the logs stored inside this parser
    /// so another node can use them to parse the logs of the same machine.
    fn cleaning(&mut self) -> Vec<SiemLog>;
    /// Return those logs that would not be used by the parser, or are older as to reduce the memmory usage.
    fn unused(&mut self) -> Vec<SiemLog>;
    /// Get parser schema
    fn schema(&self) -> &'static FieldSchema;
}

clone_trait_object!(MultilineLogParser);

/// Error at parsing a log
#[derive(Serialize, Debug)]
pub enum LogParsingError {
    /// The parser can't be used with this log
    NoValidParser(SiemLog),
    /// The parser can be used with this log but has some bug
    ParserError(SiemLog),
}

#[derive(Serialize, Debug)]
pub struct SiemCommandHeader {
    user: String,
    comp_id: u64,
    comm_id: u64,
}

/// Execute a command with parameters
#[derive(Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemCommandCall {
    /// Starts a component. Params: Component name
    START_COMPONENT(Cow<'static, str>),
    /// Stops a component. Params: Component name
    STOP_COMPONENT(Cow<'static, str>),
    /// Query in database format. Ex SQL,  Elastic
    LOG_QUERY(QueryInfo),
    /// IP of the device to isolate
    ISOLATE_IP(SiemIp),
    /// IP of the device to isolate
    ISOLATE_ENDPOINT(SiemIp),
    /// Adds a IP to a BlockList with a comment or reference (IP, Comment)
    FILTER_IP(SiemIp, Cow<'static, str>),
    /// Adds a domain to a BlockList with a comment or reference (Domain, Comment)
    FILTER_DOMAIN(Cow<'static, str>, Cow<'static, str>),
    /// Adds a email to a BlockList with a comment or reference (Email, Comment)
    FILTER_EMAIL_SENDER(Cow<'static, str>, Cow<'static, str>),
    /// List use cases: offset, limit
    LIST_USE_CASES(u32, u32),
    GET_USE_CASE(String),
    LIST_RULES(u32, u32),
    GET_RULE(String),
    LIST_DATASETS(u32, u32),
    LIST_TASKS(u32, u32),
    DOWNLOAD_QUERY(),
    /// Allows new components to extend the functionality of uSIEM: Function name, Parameters
    OTHER(
        Cow<'static, str>,
        BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    ),
}

#[derive(Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum CommandError {
    BadParameters(Cow<'static, str>),
    SyntaxError(Cow<'static, str>),
    NotFound(Cow<'static, str>),
}

/// The response of a command execution
#[derive(Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemCommandResponse {
    START_COMPONENT(Result<Cow<'static, str>, CommandError>),
    STOP_COMPONENT(Result<Cow<'static, str>, CommandError>),
    /// Query created with an ID
    LOG_QUERY(QueryInfo,Result<Vec<SiemLog>, CommandError>),
    ISOLATE_IP(Result<Cow<'static, str>, CommandError>),
    ISOLATE_ENDPOINT(Result<Cow<'static, str>, CommandError>),
    /// (IP, Comment)
    FILTER_IP(Result<Cow<'static, str>, CommandError>),
    /// (Domain, Comment)
    FILTER_DOMAIN(Result<Cow<'static, str>, CommandError>),
    /// (Email, Comment)
    FILTER_EMAIL_SENDER(Result<Cow<'static, str>, CommandError>),
    /// List of UseCases: (Name,Description)
    LIST_USE_CASES(Result<Vec<(Cow<'static, str>, Cow<'static, str>)>, CommandError>),
    GET_USE_CASE(Result<(Cow<'static, str>, Cow<'static, str>), CommandError>),
    LIST_RULES(Result<Vec<(&'static str, &'static str)>, CommandError>),
    GET_RULE(Result<(&'static str, &'static str), CommandError>),
    LIST_DATASETS(Result<Vec<Cow<'static, str>>, CommandError>),
    LIST_TASKS(Result<Vec<Cow<'static, str>>, CommandError>),
    OTHER(
        Cow<'static, str>,
        Result<BTreeMap<Cow<'static, str>, Cow<'static, str>>, CommandError>,
    ),
    //TODO: Authentication command, to allow login using third party systems: LDAP...
}

#[derive(Serialize, Debug, Clone)]
pub struct QueryInfo {
    /// The user that created the query pettition
    pub user : String,
    /// Use storage native query language: SQL, Elastic
    pub is_native : bool,
    /// If there are alredy a query resolved, make a query agaist it
    pub query_id : Option<String>,
    /// Starting time for event_created: Unix datetime from 1970
    pub from : i64,
    /// Ending time for event_created: Unix datetime from 1970
    pub to : i64,
    /// Number of rows returned
    pub limit : usize,
    /// Offseting the query
    pub offset : usize,
    /// Time to live of the query results
    pub ttl : i64,
    /// If empty and query_id has something, then return the stored query
    pub query : String
}
