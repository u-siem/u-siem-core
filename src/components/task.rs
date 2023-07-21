use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{collections::BTreeMap, future::Future, pin::Pin};

use crate::prelude::{holder::DatasetHolder, types::LogString, SiemResult};

use super::common::UserRole;

pub trait TaskBuilder2: std::fmt::Debug {
    fn build(
        &self,
        task: SiemTask,
    ) -> SiemResult<Pin<Box<dyn Future<Output = SiemTaskResult> + Send>>>
    where
        Self: Sized;
    fn clone(&self) -> Box<dyn TaskBuilder2>;
}

pub type TaskBuilder = fn(
    SiemTask,
    &DatasetHolder,
) -> SiemResult<Pin<Box<dyn Future<Output = SiemTaskResult> + Send>>>;

#[derive(Serialize)]
pub struct TaskDefinition {
    data: SiemTaskData,
    name: LogString,
    description: LogString,
    min_permission: UserRole,
    fire_mode: TaskFireMode,
    /// Time after which the task can be killed
    max_duration: u64,
    #[serde(skip)]
    builder: TaskBuilder,
}

impl TaskDefinition {
    pub fn new(
        data: SiemTaskData,
        name: LogString,
        description: LogString,
        min_permission: UserRole,
        fire_mode: TaskFireMode,
        max_duration: u64,
        builder: TaskBuilder,
    ) -> TaskDefinition {
        TaskDefinition {
            data,
            name,
            description,
            min_permission,
            fire_mode,
            max_duration,
            builder,
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
    pub fn builder(&self) -> TaskBuilder {
        self.builder
    }
}

impl std::fmt::Debug for TaskDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskDefinition")
            .field("data", &self.data)
            .field("name", &self.name)
            .field("description", &self.description)
            .field("min_permission", &self.min_permission)
            .field("fire_mode", &self.fire_mode)
            .field("max_duration", &self.max_duration)
            .finish()
    }
}

impl Clone for TaskDefinition {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            min_permission: self.min_permission.clone(),
            fire_mode: self.fire_mode.clone(),
            max_duration: self.max_duration.clone(),
            builder: self.builder,
        }
    }
}

impl<'de> Deserialize<'de> for TaskDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TaskDefinitionVisitor::new())
    }
}

struct TaskDefinitionVisitor {}

impl TaskDefinitionVisitor {
    fn new() -> Self {
        TaskDefinitionVisitor {}
    }
}

impl<'de> Visitor<'de> for TaskDefinitionVisitor {
    // The type that our Visitor is going to produce.
    type Value = TaskDefinition;

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        // While there are entries remaining in the input, add them
        // into our map.
        let mut data = SiemTaskData::UPDATE_GEOIP;
        let mut name = String::new();
        let mut description = String::new();
        let mut min_permission = UserRole::Administrator;
        let mut fire_mode = TaskFireMode::Inmediate;
        let mut max_duration = 0;
        while let Some(key) = access.next_key::<&str>()? {
            if key == "name" {
                name = access.next_value()?;
            } else if key == "description" {
                description = access.next_value()?;
            } else if key == "min_permission" {
                min_permission = access.next_value()?;
            } else if key == "fire_mode" {
                fire_mode = access.next_value()?;
            } else if key == "max_duration" {
                max_duration = access.next_value()?;
            } else if key == "data" {
                data = access.next_value()?;
            }
        }
        Ok(TaskDefinition::new(
            data,
            LogString::Owned(name),
            LogString::Owned(description),
            min_permission,
            fire_mode,
            max_duration,
            |task: SiemTask, _datasets: &DatasetHolder| {
                Ok(Box::pin(async move {
                    SiemTaskResult {
                        data: Some(Ok(format!("OK"))),
                        id: task.id,
                    }
                }))
            },
        ))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A valid command result")
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
    UPDATE_CLOUD_PROVIDER,
    /// Task name, Map<ParamName, Description>
    OTHER(LogString),
}

impl std::fmt::Display for SiemTaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SiemTaskType::EXECUTE_ENDPOINT_SCRIPT => write!(f, "EXECUTE_ENDPOINT_SCRIPT"),
            SiemTaskType::REMEDIATE_EMAILS => write!(f, "REMEDIATE_EMAILS"),
            SiemTaskType::REPORT_ABUSE => write!(f, "REPORT_ABUSE"),
            SiemTaskType::UPDATE_GEOIP => write!(f, "UPDATE_GEOIP"),
            SiemTaskType::UPDATE_CLOUD_PROVIDER => write!(f, "UPDATE_CLOUD_PROVIDER"),
            SiemTaskType::OTHER(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SiemTaskData {
    /// Script name and Script parameters
    EXECUTE_ENDPOINT_SCRIPT(LogString, BTreeMap<LogString, LogString>),
    /// Remediate a list of emails. List of parameters
    REMEDIATE_EMAILS(BTreeMap<LogString, LogString>),
    /// Report IP, email to abuse mail. Needed provider name and parameters
    REPORT_ABUSE(BTreeMap<LogString, LogString>),
    /// Update GeoIP dataset
    UPDATE_GEOIP,
    /// Update CloudProvider dataset
    UPDATE_CLOUD_PROVIDER,
    /// Task name, Map<ParamName, Description>
    OTHER(LogString, BTreeMap<LogString, LogString>),
}

impl SiemTaskData {
    pub fn class(&self) -> SiemTaskType {
        match self {
            SiemTaskData::EXECUTE_ENDPOINT_SCRIPT(_, _) => SiemTaskType::EXECUTE_ENDPOINT_SCRIPT,
            SiemTaskData::REMEDIATE_EMAILS(_) => SiemTaskType::REMEDIATE_EMAILS,
            SiemTaskData::REPORT_ABUSE(_) => SiemTaskType::REPORT_ABUSE,
            SiemTaskData::UPDATE_GEOIP => SiemTaskType::UPDATE_GEOIP,
            SiemTaskData::UPDATE_CLOUD_PROVIDER => SiemTaskType::UPDATE_CLOUD_PROVIDER,
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
    let builder: TaskBuilder = |task: SiemTask, _datasets: &DatasetHolder| {
        Ok(Box::pin(async move {
            SiemTaskResult {
                data: Some(Ok(format!("OK"))),
                id: task.id,
            }
        }))
    };

    let task = SiemTask {
        created_at: 0,
        enqueued_at: 1,
        origin: format!("123"),
        id: 12345,
        data: SiemTaskData::REPORT_ABUSE(BTreeMap::new()),
    };
    let dataset = DatasetHolder::default();
    let task = builder(task, &dataset).unwrap();

    async_std::task::block_on(async move {
        let result = task.await;
        assert_eq!(12345, result.id);
        assert_eq!(Ok(format!("OK")), result.data.unwrap());
    });
}
