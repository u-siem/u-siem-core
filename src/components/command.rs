use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::events::field::{SiemIp, SiemField};

use super::common::UserRole;

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
    LOG_QUERY(QueryInfo,Result<Vec<BTreeMap<String,SiemField>>, CommandError>),
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
    pub query : String,
    /// List of fields to be returned, empty for all
    pub fields : Vec<String>
}
