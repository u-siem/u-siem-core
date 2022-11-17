use super::super::events::SiemLog;
use super::alert::SiemAlert;
use super::dataset::{SiemDataset, SiemDatasetType};
use super::command::{CommandDefinition, SiemCommandResponse, SiemCommandCall, SiemCommandHeader};
use super::metrics::{SiemMetricDefinition};
use super::task::{SiemTask, SiemTaskResult, TaskDefinition};
use dyn_clone::{clone_trait_object, DynClone};
use serde::Serialize;
use std::borrow::Cow;

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

#[derive(Clone)]
pub struct DummyStateStorage {

}

impl SiemComponentStateStorage for DummyStateStorage {
    fn get_value(&self, _key: Cow<'static, str>) -> Result<String, StorageError> {
        Err(StorageError::NotExists)
    }

    fn set_value(
        &mut self,
        _key: Cow<'static, str>,
        _value: String,
        _replace: bool,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn get_file(&self, _filepath: String) -> Result<Vec<u8>, StorageError> {
        Err(StorageError::NotExists)
    }

    fn get_file_size(&self, _filepath: String) -> Result<u64, StorageError> {
        Err(StorageError::NotExists)
    }

    fn get_file_range(
        &self,
        _filepath: String,
        _start: u64,
        _end: u64,
    ) -> Result<Vec<u8>, StorageError> {
        Err(StorageError::NotExists)
    }

    fn set_file(&mut self, _filepath: String, _content: Vec<u8>) -> Result<(), StorageError> {
        Ok(())
    }

    fn set_file_range(
        &mut self,
        _filepath: String,
        _content: Vec<u8>,
        _start: u64,
        _end: u64,
    ) -> Result<(), StorageError> {
        Err(StorageError::NotExists)
    }

    fn duplicate(&self) -> Box<dyn SiemComponentStateStorage> {
        Box::new(self.clone())
    }
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

#[derive(Serialize, Debug, Clone)]
pub struct SiemComponentCapabilities {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    view: Cow<'static, str>,
    datasets: Vec<DatasetDefinition>,
    commands: Vec<CommandDefinition>,
    tasks: Vec<TaskDefinition>,
    metrics : Vec<SiemMetricDefinition>
}
impl SiemComponentCapabilities {
    pub fn new(
        name: Cow<'static, str>,
        description: Cow<'static, str>,
        view: Cow<'static, str>,
        datasets: Vec<DatasetDefinition>,
        commands: Vec<CommandDefinition>,
        tasks: Vec<TaskDefinition>,
        metrics : Vec<SiemMetricDefinition>
    ) -> SiemComponentCapabilities {
        return SiemComponentCapabilities {
            name,
            description,
            view,
            datasets,
            commands,
            tasks,
            metrics
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



#[derive(Serialize, Debug, Clone)]
pub struct DatasetDefinition {
    name: SiemDatasetType,
    description: Cow<'static, str>,
    required_permission: UserRole,
}
impl DatasetDefinition {
    pub fn new(
        name: SiemDatasetType,
        description: Cow<'static, str>,
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
    pub fn description(&self) -> &Cow<'static, str> {
        &self.description
    }
    /// Permission needed to access this dataset
    pub fn required_permission(&self) -> &UserRole {
        &self.required_permission
    }
}



