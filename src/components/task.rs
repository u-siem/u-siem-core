use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, future::Future, pin::Pin};

use crate::prelude::{types::LogString, SiemResult};

use super::common::UserRole;

pub trait TaskBuilder {
    fn build(&self, task : SiemTask) -> SiemResult<Pin<Box<dyn Future<Output = SiemTaskResult> + Send + '_>>>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDefinition {
    data: SiemTaskData,
    name: LogString,
    description: LogString,
    min_permission: UserRole,
    fire_mode: TaskFireMode,
    /// Time after which the task can be killed
    max_duration : u64
}
impl TaskDefinition {
    pub fn new(
        data: SiemTaskData,
        name: LogString,
        description: LogString,
        min_permission: UserRole,
        fire_mode: TaskFireMode,
        max_duration : u64
    ) -> TaskDefinition {
        TaskDefinition {
            data,
            name,
            description,
            min_permission,
            fire_mode,
            max_duration
        }
    }

    pub fn data(&self) -> &SiemTaskData {
        &self.data
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
    pub fn max_duration(&self) -> u64 {
        self.max_duration
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemTaskType {
    /// Script name and Script parameters
    EXECUTE_ENDPOINT_SCRIPT,
    /// Remediate a list of emails. List of parameters
    REMEDIATE_EMAILS,
    /// Report IP, email to abuse mail. Needed provider name and parameters
    REPORT_ABUSE,
    /// Update GeoIP database
    UPDATE_GEOIP,
    /// Task name, Map<ParamName, Description>
    OTHER(LogString)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemTaskData {
    /// Script name and Script parameters
    EXECUTE_ENDPOINT_SCRIPT(
        LogString,
        BTreeMap<LogString, LogString>,
    ),
    /// Remediate a list of emails. List of parameters
    REMEDIATE_EMAILS(BTreeMap<LogString, LogString>),
    /// Report IP, email to abuse mail. Needed provider name and parameters
    REPORT_ABUSE(BTreeMap<LogString, LogString>),
    /// Update GeoIP database
    UPDATE_GEOIP,
    /// Task name, Map<ParamName, Description>
    OTHER(
        LogString,
        BTreeMap<LogString, LogString>,
    ),
}

impl SiemTaskData {
    pub fn class(&self) -> SiemTaskType {
        match self {
            SiemTaskData::EXECUTE_ENDPOINT_SCRIPT(_, _) => SiemTaskType::EXECUTE_ENDPOINT_SCRIPT,
            SiemTaskData::REMEDIATE_EMAILS(_) => SiemTaskType::REMEDIATE_EMAILS,
            SiemTaskData::REPORT_ABUSE(_) => SiemTaskType::REPORT_ABUSE,
            SiemTaskData::UPDATE_GEOIP => SiemTaskType::UPDATE_GEOIP,
            SiemTaskData::OTHER(v, _) => SiemTaskType::OTHER(v.clone()),
        }
    }
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
    pub data: SiemTaskData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiemTaskResult {
    pub id: u64,
    pub data: Option<Result<String, String>>,
}



#[test]
fn task_builder_should_generate_async_task() {
    struct DummyTaskBuilder{}
    impl TaskBuilder for DummyTaskBuilder {
        fn build(&self, task : SiemTask) -> SiemResult<Pin<Box<dyn Future<Output = SiemTaskResult> + Send + '_>>> {
            Ok(Box::pin(async move {
                SiemTaskResult {
                    data : Some(Ok(format!("OK"))),
                    id : task.id
                }
            }))
        }
    }

    let builder = DummyTaskBuilder{};
    let task = SiemTask { created_at: 0, enqueued_at: 1, origin: format!("123"), id: 12345, data: SiemTaskData::REPORT_ABUSE(BTreeMap::new()) };
    let task = builder.build(task);
    async_std::task::block_on(async move {
        let result = task.unwrap().await;
        assert_eq!(12345, result.id);
        assert_eq!(Ok(format!("OK")),result.data.unwrap());
    });
}