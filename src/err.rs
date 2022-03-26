use std::io::Error as IoError;
pub type SiemResult<T> = Result<T, SiemError>;


#[derive(Debug)]
#[non_exhaustive]
pub enum SiemError {
    Io(IoError),
    Serialization(String),
    Parsing(String),
    Indexing(String),
    Task(String),
    Command(CommandExecutionError),
    Other(String)
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CommandExecutionError {
    Communication(String),
    Other(String)
}