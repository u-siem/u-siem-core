use std::{sync::{Arc, Mutex}, collections::BTreeMap};

use dyn_clone::{clone_trait_object, DynClone};

use crate::prelude::{types::LogString, StorageError};

pub trait SiemComponentStateStorage: DynClone + Send {
    /// Read a key value from the database
    fn get_value(&self, key: &str) -> Result<String, StorageError>;
    /// Write to the database a key/value pair
    fn set_value(
        &mut self,
        key: &str,
        value: LogString,
        replace: bool,
    ) -> Result<(), StorageError>;

    /// Get a file
    fn get_file(&self, filepath: &str) -> Result<Vec<u8>, StorageError>;

    /// Get the size of a file
    fn get_file_size(&self, filepath: &str) -> Result<u64, StorageError>;

    /// Get a file part
    fn get_file_range(
        &self,
        filepath: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<u8>, StorageError>;

    /// Sets the content of a file
    fn set_file(&mut self, filepath: &str, content: Vec<u8>) -> Result<(), StorageError>;

    /// Sets the content of a file
    fn set_file_range(
        &mut self,
        filepath: &str,
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
        _key: &str,
        _value: LogString,
        _replace: bool,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn get_file(&self, _filepath: &str) -> Result<Vec<u8>, StorageError> {
        Err(StorageError::NotExists)
    }

    fn get_file_size(&self, _filepath: &str) -> Result<u64, StorageError> {
        Err(StorageError::NotExists)
    }

    fn get_file_range(
        &self,
        _filepath: &str,
        _start: u64,
        _end: u64,
    ) -> Result<Vec<u8>, StorageError> {
        Err(StorageError::NotExists)
    }

    fn set_file(&mut self, _filepath: &str, _content: Vec<u8>) -> Result<(), StorageError> {
        Ok(())
    }

    fn set_file_range(
        &mut self,
        _filepath: &str,
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


#[derive(Clone)]
pub struct TestingStorage {
    files : Arc<Mutex<BTreeMap<String, Vec<u8>>>>,
    values : Arc<Mutex<BTreeMap<String, LogString>>>
}

impl TestingStorage {
    pub fn new() -> Self {
        Self {
            files : Arc::new(Mutex::new(BTreeMap::new())),
            values : Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

impl SiemComponentStateStorage for TestingStorage {
    fn get_value(&self, key: &str) -> Result<String, StorageError> {
        let values = self.values.lock().unwrap();
        values.get(key).map(|v| v.to_string()).ok_or(StorageError::NotExists)
    }

    fn set_value(
        &mut self,
        key: &str,
        value: LogString,
        replace: bool,
    ) -> Result<(), StorageError> {
        let mut values = self.values.lock().unwrap();
        if !replace && values.contains_key(key) {
            return Ok(())
        }
        values.insert(key.to_string(), value);
        Ok(())
    }

    fn get_file(&self, filepath: &str) -> Result<Vec<u8>, StorageError> {
        let files = self.files.lock().unwrap();
        let file = files.get(filepath).ok_or(StorageError::NotExists)?;
        Ok(file.clone())
    }

    fn get_file_size(&self, filepath: &str) -> Result<u64, StorageError> {
        let files = self.files.lock().unwrap();
        let file = files.get(filepath).ok_or(StorageError::NotExists)?;
        Ok(file.len() as u64)
    }

    fn get_file_range(
        &self,
        filepath: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<u8>, StorageError> {
        let files = self.files.lock().unwrap();
        let start = start as usize;
        let end = end as usize;
        let file = files.get(filepath).ok_or(StorageError::NotExists)?;
        if start >= file.len() || end >= file.len() || start > end {
            return Err(StorageError::ConnectionError)
        }
        Ok(file[start..end].to_vec())
    }

    fn set_file(&mut self, filepath: &str, content: Vec<u8>) -> Result<(), StorageError> {
        let mut files = self.files.lock().unwrap();
        files.insert(filepath.to_string(), content);
        Ok(())
    }

    fn set_file_range(
        &mut self,
        filepath: &str,
        content: Vec<u8>,
        _start: u64,
        _end: u64,
    ) -> Result<(), StorageError> {
        let mut files = self.files.lock().unwrap();
        files.insert(filepath.to_string(), content);
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn SiemComponentStateStorage> {
        Box::new(self.clone())
    }
}