use std::io::Error as IoError;

use serde::{Deserialize, Serialize};

use crate::prelude::{LogString, SiemLog};
pub type SiemResult<T> = Result<T, SiemError>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub enum SiemError {
    /// Io Error
    Io(String),
    /// Seriaization/Deserialization error
    Serialization(String),
    /// Error parsing a log
    Parsing(LogParsingError),
    /// Error indexing a log
    Indexing(String),
    /// Error accessing the storage system
    Storage(StorageError),
    /// A task execution failed
    Task(String),
    /// A command executed failed
    Command(CommandExecutionError),
    /// A component sufered an error during the startup process
    Configuration(String),
    Messaging(MessagingError),
    Other(String),
    Component(ComponentError),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub enum CommandExecutionError {
    Communication(String),
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum StorageError {
    NotExists,
    ConnectionError,
    AlredyExists,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum MessagingError {
    Disconnected,
    TimeoutReached,
    Full,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum ComponentError {
    StopRequested,
    ExecutionError(LogString)
}


/// Error at parsing a log
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogParsingError {
    /// The parser can't be used with this log
    NoValidParser(SiemLog),
    /// The log is for this parser but there is a bug in the code
    ParserError(SiemLog, String),
    /// The log is for this parser but the submodule has not been implemented.
    NotImplemented(SiemLog),
    /// The log has change format the parser cant process it.
    FormatError(SiemLog, String),
    /// Log was discarded. It does not have utility or there are storage limitations.
    Discard,
}

/// Error at parsing a log
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogEnrichmentError {
    DatasetError(LogString),
    /// Log was discarded.
    Discard
}

impl From<MessagingError> for SiemError {
    fn from(e: MessagingError) -> Self {
        Self::Messaging(e)
    }
}
impl From<ComponentError> for SiemError {
    fn from(e: ComponentError) -> Self {
        Self::Component(e)
    }
}
impl From<StorageError> for SiemError {
    fn from(e: StorageError) -> Self {
        Self::Storage(e)
    }
}
impl From<CommandExecutionError> for SiemError {
    fn from(e: CommandExecutionError) -> Self {
        Self::Command(e)
    }
}

impl From<IoError> for SiemError {
    fn from(e: IoError) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<serde_json::Error> for SiemError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

impl<T> From<crossbeam_channel::TrySendError<T>> for SiemError {
    fn from(e: crossbeam_channel::TrySendError<T>) -> Self {
        match e {
            crossbeam_channel::TrySendError::Full(_) => Self::Messaging(MessagingError::Full),
            crossbeam_channel::TrySendError::Disconnected(_) => {
                Self::Messaging(MessagingError::Disconnected)
            }
        }
    }
}
impl<T> From<crossbeam_channel::SendError<T>> for SiemError {
    fn from(_e: crossbeam_channel::SendError<T>) -> Self {
        Self::Messaging(MessagingError::Disconnected)
    }
}
impl<T> From<crossbeam_channel::SendTimeoutError<T>> for SiemError {
    fn from(e: crossbeam_channel::SendTimeoutError<T>) -> Self {
        match e {
            crossbeam_channel::SendTimeoutError::Timeout(_) => {
                Self::Messaging(MessagingError::TimeoutReached)
            }
            crossbeam_channel::SendTimeoutError::Disconnected(_) => {
                Self::Messaging(MessagingError::Disconnected)
            }
        }
    }
}
