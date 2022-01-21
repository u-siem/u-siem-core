use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::events::field::{SiemField};

use super::{common::{UserRole, DatasetDefinition}, command_types::{ParserDefinition, TaskDefinition, RuleDefinition, FilterEmail, FilterDomain, FilterIp, IsolateEndpoint, IsolateIp, UseCaseDefinition, LoginUser, LoggedOnUser}};

/// Define commands to be used by the users or other components.
#[derive(Serialize, Debug, Clone)]
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
    LOGIN_USER,
    LIST_PARSERS,
    /// Function name, Map<ParamName, Description>
    OTHER(
        Cow<'static, str>,
        BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    ),
}


#[derive(Serialize, Debug, Clone)]
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
pub struct SiemCommandHeader {
    pub user: String,
    pub comp_id: u64,
    pub comm_id: u64,
}

/// Execute a command with parameters
#[derive(Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemCommandCall {
    /// Starts a component. Params: Component name
    START_COMPONENT(String),
    /// Stops a component. Params: Component name
    STOP_COMPONENT(String),
    /// Query in database format. Ex SQL,  Elastic
    LOG_QUERY(QueryInfo),
    /// IP of the device to isolate
    ISOLATE_IP(IsolateIp),
    /// IP of the device to isolate
    ISOLATE_ENDPOINT(IsolateEndpoint),
    /// Adds a IP to a BlockList with a comment or reference (IP, Comment)
    FILTER_IP(FilterIp),
    /// Adds a domain to a BlockList with a comment or reference (Domain, Comment)
    FILTER_DOMAIN(FilterDomain),
    /// Adds a email to a BlockList with a comment or reference (Email, Comment)
    FILTER_EMAIL_SENDER(FilterEmail),
    /// List use cases: offset, limit
    LIST_USE_CASES(u32, u32),
    GET_USE_CASE(String),
    /// List rules: offset, limit
    LIST_RULES(u32, u32),
    /// Get rule by name
    GET_RULE(String),
    /// List datasets: offset, limit
    LIST_DATASETS(u32, u32),
    /// List tasks: offset, limit
    LIST_TASKS(u32, u32),
    DOWNLOAD_QUERY(),
    LIST_PARSERS(u32,u32),
    LOGIN_USER(LoginUser),
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
    START_COMPONENT(Result<String, CommandError>),
    STOP_COMPONENT(Result<String, CommandError>),
    /// Query created with an ID
    LOG_QUERY(QueryInfo,Result<Vec<BTreeMap<String,SiemField>>, CommandError>),
    ISOLATE_IP(Result<String, CommandError>),
    ISOLATE_ENDPOINT(Result<String, CommandError>),
    /// (IP, Comment)
    FILTER_IP(Result<String, CommandError>),
    /// (Domain, Comment)
    FILTER_DOMAIN(Result<String, CommandError>),
    /// (Email, Comment)
    FILTER_EMAIL_SENDER(Result<String, CommandError>),
    /// List of UseCases: (Name,Description)
    LIST_USE_CASES(Result<Vec<UseCaseDefinition>, CommandError>),
    GET_USE_CASE(Result<UseCaseDefinition, CommandError>),
    LIST_RULES(Result<Vec<RuleDefinition>, CommandError>),
    GET_RULE(Result<RuleDefinition, CommandError>),
    LIST_DATASETS(Result<Vec<DatasetDefinition>, CommandError>),
    LIST_TASKS(Result<Vec<TaskDefinition>, CommandError>),
    LIST_PARSERS(Result<Vec<ParserDefinition>, CommandError>),
    LOGIN_USER(Result<LoggedOnUser, CommandError>),
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
    pub query : String,
    /// List of fields to be returned, empty for all
    pub fields : Vec<String>
}
