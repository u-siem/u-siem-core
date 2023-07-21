use dyn_clone::{clone_trait_object, DynClone};

use crate::prelude::{types::LogString, StorageError};

pub trait SiemComponentStateStorage: DynClone + Send {
    /// Read a key value from the database
    fn get_value(&self, key: &str) -> Result<String, StorageError>;
    /// Write to the database a key/value pair
    fn set_value(
        &mut self,
        key: LogString,
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

#[derive(Clone)]
pub struct DummyStateStorage {}

impl SiemComponentStateStorage for DummyStateStorage {
    fn get_value(&self, _key: &str) -> Result<String, StorageError> {
        Err(StorageError::NotExists)
    }

    fn set_value(
        &mut self,
        _key: LogString,
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
