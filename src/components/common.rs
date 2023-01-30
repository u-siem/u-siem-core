use crate::prelude::types::LogString;

use super::super::events::SiemLog;
use super::alert::SiemAlert;
use super::command::{CommandDefinition, SiemCommandCall, SiemCommandHeader, SiemCommandResponse};
use super::dataset::{SiemDataset, SiemDatasetType};
use super::metrics::SiemMetricDefinition;
use super::task::{SiemTask, SiemTaskResult, TaskDefinition};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum SiemMessage {
    /// Execute a command in the component
    Command(SiemCommandHeader, SiemCommandCall),
    /// Response to a function call, first element is the ID of the Response
    Response(SiemCommandHeader, SiemCommandResponse),
    /// Process a log
    Log(SiemLog),
    /// Local logging system. First element is the ID of the component, to be able to route messages
    Notification(Notification),
    #[serde(skip)]
    /// Dataset updated, this is the last state of it.
    Dataset(SiemDataset),
    /// Alerting
    Alert(SiemAlert),
    Task(SiemCommandHeader, SiemTask),
    TaskResult(SiemCommandHeader, SiemTaskResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub timestamp: i64,
    pub component: u64,
    pub component_name: LogString,
    pub log: LogString,
    pub level: NotificationLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum NotificationLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Serialize, Debug, Clone)]
pub struct SiemComponentCapabilities {
    name: LogString,
    description: LogString,
    view: LogString,
    datasets: Vec<DatasetDefinition>,
    commands: Vec<CommandDefinition>,
    tasks: Vec<TaskDefinition>,
    metrics: Vec<SiemMetricDefinition>,
}
impl SiemComponentCapabilities {
    pub fn new(
        name: LogString,
        description: LogString,
        view: LogString,
        datasets: Vec<DatasetDefinition>,
        commands: Vec<CommandDefinition>,
        tasks: Vec<TaskDefinition>,
        metrics: Vec<SiemMetricDefinition>,
    ) -> SiemComponentCapabilities {
        return SiemComponentCapabilities {
            name,
            description,
            view,
            datasets,
            commands,
            tasks,
            metrics,
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
    pub fn metrics(&self) -> &Vec<SiemMetricDefinition> {
        &self.metrics
    }
}

/// An easy to use role based system
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DatasetDefinition {
    name: SiemDatasetType,
    description: LogString,
    required_permission: UserRole,
}
impl DatasetDefinition {
    pub fn new(
        name: SiemDatasetType,
        description: LogString,
        required_permission: UserRole,
    ) -> DatasetDefinition {
        DatasetDefinition {
            name,
            description,
            required_permission,
        }
    }
    /// Name of the dataset
    pub fn name(&self) -> &SiemDatasetType {
        &self.name
    }
    /// Description of the dataset
    pub fn description(&self) -> &LogString {
        &self.description
    }
    /// Permission needed to access this dataset
    pub fn required_permission(&self) -> &UserRole {
        &self.required_permission
    }
}

impl From<SiemCommandCall> for SiemMessage {
    fn from(c: SiemCommandCall) -> Self {
        SiemMessage::Command(SiemCommandHeader::default(), c)
    }
}

impl From<SiemCommandResponse> for SiemMessage {
    fn from(c: SiemCommandResponse) -> Self {
        SiemMessage::Response(SiemCommandHeader::default(), c)
    }
}

impl From<SiemLog> for SiemMessage {
    fn from(c: SiemLog) -> Self {
        SiemMessage::Log(c)
    }
}

impl From<Notification> for SiemMessage {
    fn from(c: Notification) -> Self {
        SiemMessage::Notification(c)
    }
}

impl From<SiemAlert> for SiemMessage {
    fn from(c: SiemAlert) -> Self {
        SiemMessage::Alert(c)
    }
}

impl From<SiemDataset> for SiemMessage {
    fn from(c: SiemDataset) -> Self {
        SiemMessage::Dataset(c)
    }
}

impl From<SiemTask> for SiemMessage {
    fn from(c: SiemTask) -> Self {
        SiemMessage::Task(SiemCommandHeader::default(), c)
    }
}

impl From<SiemTaskResult> for SiemMessage {
    fn from(c: SiemTaskResult) -> Self {
        SiemMessage::TaskResult(SiemCommandHeader::default(), c)
    }
}
