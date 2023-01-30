use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap};

use crate::prelude::types::LogString;

use super::common::UserRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDefinition {
    class: SiemTaskType,
    name: LogString,
    description: LogString,
    min_permission: UserRole,
    fire_mode: TaskFireMode,
}
impl TaskDefinition {
    pub fn new(
        class: SiemTaskType,
        name: LogString,
        description: LogString,
        min_permission: UserRole,
        fire_mode: TaskFireMode,
    ) -> TaskDefinition {
        TaskDefinition {
            class,
            name,
            description,
            min_permission,
            fire_mode,
        }
    }

    pub fn class(&self) -> &SiemTaskType {
        &self.class
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn min_permission(&self) -> &UserRole {
        &self.min_permission
    }
    pub fn fire_mode(&self) -> &TaskFireMode {
        &self.fire_mode
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskFireMode {
    /// Execute this tasks as soon as posible
    Inmediate,
    /// Execute this taks using a cron definition
    Cron(u32, u32, u32, u32, u32),
    /// Execute each X miliseconds
    Repetitive(u64),
    /// Execute this task once in the future
    Future(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemTaskType {
    /// Script name and Script parameters
    EXECUTE_ENDPOINT_SCRIPT(
        LogString,
        BTreeMap<LogString, LogString>,
    ),
    /// Remediate a list of emails. List of parameters
    REMEDIATE_EMAILS(BTreeMap<LogString, LogString>),
    /// Report IP, email to abuse mail. Needed provider name and parameters
    REPORT_ABUSE(BTreeMap<LogString, LogString>),
    UPDATE_GEOIP,
    /// Task name, Map<ParamName, Description>
    OTHER(
        LogString,
        BTreeMap<LogString, LogString>,
    ),
}

/// Enqueued task with data.
/// If the Task has finished then the result has Some data.
/// This data can be a Ok with the output (not the data) or the error.
/// The ID is to get the Task result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiemTask {
    pub created_at: i64,
    pub enqueued_at: i64,
    pub origin: String,
    pub id: u64,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiemTaskResult {
    pub id: u64,
    pub data: Option<Result<String, String>>,
}
