use std::{borrow::Cow, collections::BTreeMap};
use serde::Serialize;

use super::common::UserRole;

#[derive(Serialize, Debug, Clone)]
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


#[derive(Serialize, Debug, Clone)]
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

/// Enqueued task with data.
/// If the Task has finished then the result has Some data.
/// This data can be a Ok with the output (not the data) or the error.
/// The ID is to get the Task result
#[derive(Serialize, Debug, Clone)]
pub struct SiemTask {
    pub created_at : i64,
    pub enqueued_at : i64,
    pub origin : String,
    pub id : u64,
    pub data : String
}

#[derive(Serialize, Debug, Clone)]
pub struct SiemTaskResult {
    pub id : u64,
    pub data : Option<Result<String, String>>
}