use serde::de::{MapAccess, Visitor};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::{fmt::Debug};

use super::task::{SiemTask, SiemTaskResult};
use super::{
    command_types::{
        FilterDomain, FilterEmail, FilterIp, IsolateEndpoint, IsolateIp, LoggedOnUser, LoginUser,
        ParserDefinition, RuleDefinition, TaskDefinition, UseCaseDefinition,
    },
    common::{DatasetDefinition, UserRole},
};
use crate::events::field::SiemField;
use crate::prelude::types::LogString;

/// Define commands to be used by the users or other components.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    GET_USE_CASE,
    LIST_RULES,
    GET_RULE,
    LIST_TASKS,
    LIST_DATASETS,
    DOWNLOAD_QUERY,
    LOGIN_USER,
    LIST_PARSERS,
    START_TASK,
    GET_TASK_RESULT,
    /// Function name
    OTHER(
        LogString
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandDefinition {
    class: SiemFunctionType,
    name: LogString,
    description: LogString,
    min_permission: UserRole,
}
impl CommandDefinition {
    pub fn new(
        class: SiemFunctionType,
        name: LogString,
        description: LogString,
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
    pub fn name(&self) -> &LogString {
        &self.name
    }
    pub fn description(&self) -> &LogString {
        &self.description
    }
    pub fn min_permission(&self) -> &UserRole {
        &self.min_permission
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SiemCommandHeader {
    /// User that created the command
    pub user: String,
    /// Component ID that created the command or the response
    pub comp_id: u64,
    /// Internal command ID: serves as an internal mapping betwen components as to replay to a specific component
    ///
    /// COMMAND => (COMPONENT) CMP_ID ->(KERNEL)-> CMP_ID<=>CMP_KRNL_ID ->(OTHER COMPONENT) -> CMP_KRNL_ID
    ///
    ///
    /// RESPONSE => (OTHER COMPONENT) RSP_ID=CMP_KRNL_ID ->(KERNEL)-> RSP_ID=CMP_KRNL_ID<=>CMP_ID -> (COMPONENT) -> CMP_ID
    pub comm_id: u64,
}

/// Execute a command with parameters
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    LIST_USE_CASES(Pagination),
    GET_USE_CASE(String),
    /// List rules: offset, limit
    LIST_RULES(Pagination),
    /// Get rule by name
    GET_RULE(String),
    /// List datasets: offset, limit
    LIST_DATASETS(Pagination),
    /// List tasks: offset, limit
    LIST_TASKS(Pagination),
    DOWNLOAD_QUERY(),
    LIST_PARSERS(Pagination),
    LOGIN_USER(LoginUser),
    START_TASK(SiemTask),
    GET_TASK_RESULT(u64),
    /// Allows new components to extend the functionality of uSIEM: Function name, Parameters
    OTHER(
        LogString,
        BTreeMap<LogString, LogString>,
    ),
}

impl SiemCommandCall {
    pub fn get_type(&self) -> SiemFunctionType {
        match self {
            SiemCommandCall::START_COMPONENT(_) => SiemFunctionType::START_COMPONENT,
            SiemCommandCall::STOP_COMPONENT(_) => SiemFunctionType::STOP_COMPONENT,
            SiemCommandCall::LOG_QUERY(_) => SiemFunctionType::LOG_QUERY,
            SiemCommandCall::ISOLATE_IP(_) => SiemFunctionType::ISOLATE_IP,
            SiemCommandCall::ISOLATE_ENDPOINT(_) => SiemFunctionType::ISOLATE_ENDPOINT,
            SiemCommandCall::FILTER_IP(_) => SiemFunctionType::FILTER_IP,
            SiemCommandCall::FILTER_DOMAIN(_) => SiemFunctionType::FILTER_DOMAIN,
            SiemCommandCall::FILTER_EMAIL_SENDER(_) => SiemFunctionType::FILTER_EMAIL_SENDER,
            SiemCommandCall::LIST_USE_CASES(_) => SiemFunctionType::LIST_USE_CASES,
            SiemCommandCall::GET_USE_CASE(_) => SiemFunctionType::GET_USE_CASE,
            SiemCommandCall::LIST_RULES(_) => SiemFunctionType::LIST_RULES,
            SiemCommandCall::GET_RULE(_) => SiemFunctionType::GET_RULE,
            SiemCommandCall::LIST_DATASETS(_) => SiemFunctionType::LIST_DATASETS,
            SiemCommandCall::LIST_TASKS(_) => SiemFunctionType::LIST_TASKS,
            SiemCommandCall::DOWNLOAD_QUERY() => SiemFunctionType::DOWNLOAD_QUERY,
            SiemCommandCall::LIST_PARSERS(_) => SiemFunctionType::LIST_PARSERS,
            SiemCommandCall::LOGIN_USER(_) => SiemFunctionType::LOGIN_USER,
            SiemCommandCall::START_TASK(_) => SiemFunctionType::START_TASK,
            SiemCommandCall::GET_TASK_RESULT(_) => SiemFunctionType::GET_TASK_RESULT,
            SiemCommandCall::OTHER(v, _) => SiemFunctionType::OTHER(v.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum CommandError {
    BadParameters(LogString),
    SyntaxError(LogString),
    NotFound(LogString),
}

/// The response of a command execution
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemCommandResponse {
    START_COMPONENT(CommandResult<String>),
    STOP_COMPONENT(CommandResult<String>),
    /// Query created with an ID
    LOG_QUERY(QueryInfo, CommandResult<Vec<BTreeMap<String, SiemField>>>),
    ISOLATE_IP(CommandResult<String>),
    ISOLATE_ENDPOINT(CommandResult<String>),
    /// (IP, Comment)
    FILTER_IP(CommandResult<String>),
    /// (Domain, Comment)
    FILTER_DOMAIN(CommandResult<String>),
    /// (Email, Comment)
    FILTER_EMAIL_SENDER(CommandResult<String>),
    /// List of UseCases: (Name,Description)
    LIST_USE_CASES(CommandResult<Vec<UseCaseDefinition>>),
    GET_USE_CASE(CommandResult<UseCaseDefinition>),
    LIST_RULES(CommandResult<Vec<RuleDefinition>>),
    GET_RULE(CommandResult<RuleDefinition>),
    LIST_DATASETS(CommandResult<Vec<DatasetDefinition>>),
    LIST_TASKS(CommandResult<Vec<TaskDefinition>>),
    LIST_PARSERS(CommandResult<Vec<ParserDefinition>>),
    LOGIN_USER(CommandResult<LoggedOnUser>),
    START_TASK(CommandResult<u64>),
    GET_TASK_RESULT(CommandResult<SiemTaskResult>),
    OTHER(
        LogString,
        CommandResult<BTreeMap<LogString, LogString>>,
    ),
    //TODO: Authentication command, to allow login using third party systems: LDAP...
}

#[derive(Serialize, Debug, Clone)]
pub enum CommandResult<T>
where
    T: Serialize + DeserializeOwned + std::fmt::Debug + Clone,
{
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "err")]
    Err(CommandError),
}

impl<'de, T: Serialize + Clone + Debug + ?Sized + DeserializeOwned> Deserialize<'de>
    for CommandResult<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(CommandResultVisitor::new())
    }
}

struct CommandResultVisitor<T> {
    marker: PhantomData<fn() -> T>,
}

impl<T> CommandResultVisitor<T> {
    fn new() -> Self {
        CommandResultVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for CommandResultVisitor<T>
where
    T: DeserializeOwned + Debug + Serialize + Clone,
{
    // The type that our Visitor is going to produce.
    type Value = CommandResult<T>;

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        // While there are entries remaining in the input, add them
        // into our map.
        if let Some(key) = access.next_key::<&str>()? {
            if key == "ok" {
                let val: T = access.next_value()?;
                Ok(CommandResult::Ok(val))
            } else if key == "err" {
                let val: CommandError = access.next_value()?;
                Ok(CommandResult::Err(val))
            } else {
                Err(serde::de::Error::missing_field("No ok/err field available"))
            }
        } else {
            Err(serde::de::Error::missing_field("No ok/err field available"))
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A valid command result")
    }
}
// */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryInfo {
    /// The user that created the query pettition
    pub user: String,
    /// Use storage native query language: SQL, Elastic
    pub is_native: bool,
    /// If there are alredy a query resolved, make a query agaist it
    pub query_id: Option<String>,
    /// Starting time for event_created: Unix datetime from 1970
    pub from: i64,
    /// Ending time for event_created: Unix datetime from 1970
    pub to: i64,
    /// Number of rows returned
    pub limit: usize,
    /// Offseting the query
    pub offset: usize,
    /// Time to live of the query results
    pub ttl: i64,
    /// If empty and query_id has something, then return the stored query
    pub query: String,
    /// List of fields to be returned, empty for all
    pub fields: Vec<String>,
}

#[cfg(test)]
mod de_ser {

    use crate::prelude::{DatasetDefinition, types::LogString};

    use super::SiemCommandResponse;

    #[test]
    fn should_serialize_and_deserialize_command_response() {
        let res =
            SiemCommandResponse::FILTER_IP(super::CommandResult::Ok(format!("Ip was filtered")));
        let str = serde_json::to_string(&res).unwrap();
        let res2: SiemCommandResponse = serde_json::from_str(&str).unwrap();

        match (res, res2) {
            (SiemCommandResponse::FILTER_IP(ip1), SiemCommandResponse::FILTER_IP(ip2)) => {
                match (ip1, ip2) {
                    (super::CommandResult::Ok(v1), super::CommandResult::Ok(v2)) => {
                        assert_eq!(v1, v2)
                    }
                    (super::CommandResult::Err(v1), super::CommandResult::Err(v2)) => {
                        match (v1, v2) {
                            (
                                super::CommandError::BadParameters(v1),
                                super::CommandError::BadParameters(v2),
                            ) => assert_eq!(v1, v2),
                            (
                                super::CommandError::SyntaxError(v1),
                                super::CommandError::SyntaxError(v2),
                            ) => assert_eq!(v1, v2),
                            (
                                super::CommandError::NotFound(v1),
                                super::CommandError::NotFound(v2),
                            ) => assert_eq!(v1, v2),
                            _ => panic!("Error must be the same"),
                        }
                    }
                    _ => panic!("Both responses must be the same"),
                }
            }
            _ => panic!("Must not happen"),
        }

        let res = SiemCommandResponse::LIST_DATASETS(super::CommandResult::Ok(vec![
            DatasetDefinition::new(
                crate::prelude::SiemDatasetType::CustomIpMap(LogString::Borrowed("")),
                LogString::Borrowed("Description"),
                crate::prelude::UserRole::Administrator,
            ),
        ]));
        let str = serde_json::to_string(&res).unwrap();
        let res2: SiemCommandResponse = serde_json::from_str(&str).unwrap();

        match (res, res2) {
            (SiemCommandResponse::LIST_DATASETS(ip1), SiemCommandResponse::LIST_DATASETS(ip2)) => {
                match (ip1, ip2) {
                    (super::CommandResult::Ok(v1), super::CommandResult::Ok(v2)) => {
                        assert_eq!(v1, v2)
                    }
                    (super::CommandResult::Err(v1), super::CommandResult::Err(v2)) => {
                        match (v1, v2) {
                            (
                                super::CommandError::BadParameters(v1),
                                super::CommandError::BadParameters(v2),
                            ) => assert_eq!(v1, v2),
                            (
                                super::CommandError::SyntaxError(v1),
                                super::CommandError::SyntaxError(v2),
                            ) => assert_eq!(v1, v2),
                            (
                                super::CommandError::NotFound(v1),
                                super::CommandError::NotFound(v2),
                            ) => assert_eq!(v1, v2),
                            _ => panic!("Error must be the same"),
                        }
                    }
                    _ => panic!("Both responses must be the same"),
                }
            }
            _ => panic!("Must not happen"),
        }
    }
}
