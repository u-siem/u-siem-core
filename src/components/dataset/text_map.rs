use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
#[derive(Serialize, Debug)]
pub enum UpdateTextMap {
    Add((LogString, LogString)),
    Remove(LogString),
    Replace(TextMapDataset),
}
#[derive(Debug, Clone)]
pub struct TextMapSynDataset {
    dataset: Arc<TextMapDataset>,
    comm: Sender<UpdateTextMap>,
}
impl TextMapSynDataset {
    pub fn new(dataset: Arc<TextMapDataset>, comm: Sender<UpdateTextMap>) -> TextMapSynDataset {
        return TextMapSynDataset { dataset, comm };
    }
    pub fn insert<S>(&self, key: S, data: S)
    where
        S: Into<LogString>,
    {
        match self
            .comm
            .try_send(UpdateTextMap::Add((key.into(), data.into())))
        {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove<S>(&self, key: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMap::Remove(key.into())) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data: TextMapDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMap::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, key: &str) -> Option<&LogString> {
        // Todo improve with cached content
        self.dataset.get(key)
    }
}
#[derive(Serialize, Debug)]
pub struct TextMapDataset {
    data: BTreeMap<LogString, LogString>,
}

impl TextMapDataset {
    pub fn new() -> TextMapDataset {
        return TextMapDataset {
            data: BTreeMap::new(),
        };
    }
    pub fn insert<S>(&mut self, key: S, data: S)
    where
        S: Into<LogString>,
    {
        self.data.insert(key.into(), data.into());
    }
    pub fn get(&self, key: &str) -> Option<&LogString> {
        self.data.get(key)
    }
    pub fn internal_ref(&self) -> &BTreeMap<LogString, LogString> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_data_in_map() {
        let mut dataset = TextMapDataset::new();
        dataset.insert(
            LogString::Borrowed("192.168.1.1"),
            LogString::Borrowed("Local IP"),
        );
        assert_eq!(
            dataset.get("192.168.1.1"),
            Some(&LogString::Borrowed("Local IP"))
        );
    }
}
