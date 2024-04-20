use crate::alerts::SiemAlert;
use crate::prelude::types::LogString;
use crate::prelude::{SiemDataset, SiemDatasetType};
use crate::runtime::address::Mailed;

use super::super::events::SiemLog;
use super::command::{SiemCommand, SiemResponse};
use super::task::{SiemTask, SiemTaskResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum SiemMessage {
    /// Execute a command in the component
    Command(Mailed<SiemCommand>),
    /// Response to a function call, first element is the ID of the Response
    Response(Mailed<SiemResponse>),
    /// Process a log
    Log(SiemLog),
    /// Local logging system. First element is the ID of the component, to be able to route messages
    Notification(Notification),
    #[serde(skip)]
    /// Dataset updated, this is the last state of it.
    Dataset(SiemDataset),
    /// Alerting
    Alert(SiemAlert),
    Task(Mailed<SiemTask>),
    TaskResult(Mailed<SiemTaskResult>),
}

/// A internal event that occur in a SIEM component such as problems, errors or just information on current operations.
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

impl From<SiemCommand> for SiemMessage {
    fn from(c: SiemCommand) -> Self {
        SiemMessage::Command(Mailed::new(0, 0, c))
    }
}

impl From<SiemResponse> for SiemMessage {
    fn from(c: SiemResponse) -> Self {
        SiemMessage::Response(Mailed::new(0, 0, c))
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
        SiemMessage::Task(Mailed::new(0, 0, c))
    }
}

impl From<SiemTaskResult> for SiemMessage {
    fn from(c: SiemTaskResult) -> Self {
        SiemMessage::TaskResult(Mailed::new(0, 0, c))
    }
}

impl From<Mailed<SiemResponse>> for SiemMessage {
    fn from(value: Mailed<SiemResponse>) -> Self {
        SiemMessage::Response(value)
    }
}
impl From<Mailed<SiemTaskResult>> for SiemMessage {
    fn from(value: Mailed<SiemTaskResult>) -> Self {
        SiemMessage::TaskResult(value)
    }
}
impl From<Mailed<SiemCommand>> for SiemMessage {
    fn from(value: Mailed<SiemCommand>) -> Self {
        SiemMessage::Command(value)
    }
}
impl From<Mailed<SiemTask>> for SiemMessage {
    fn from(value: Mailed<SiemTask>) -> Self {
        SiemMessage::Task(value)
    }
}