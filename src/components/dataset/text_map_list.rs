use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Serialize, Debug)]
pub enum UpdateTextMapList {
    Add((LogString, Vec<LogString>)),
    Remove(LogString),
    Replace(TextMapListDataset),
}
#[derive(Debug, Clone)]
pub struct TextMapListSynDataset {
    dataset: Arc<TextMapListDataset>,
    comm: Sender<UpdateTextMapList>,
}
impl TextMapListSynDataset {
    pub fn new(dataset: Arc<TextMapListDataset>, comm: Sender<UpdateTextMapList>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(TextMapListDataset::new()),
            comm: sender,
        }
    }
    pub fn insert(&self, key: LogString, data: Vec<LogString>) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextMapList::Add((key, data)));
    }
    pub fn remove(&self, key: LogString) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextMapList::Remove(key));
    }
    pub fn update(&self, data: TextMapListDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextMapList::Replace(data));
    }
    pub fn get(&self, key: &str) -> Option<&Vec<LogString>> {
        // Todo improve with cached content
        self.dataset.get(key)
    }
    pub fn inner(&self) -> &TextMapListDataset {
        self.dataset.as_ref()
    }
}
#[derive(Serialize, Debug, Default)]
pub struct TextMapListDataset {
    data: BTreeMap<LogString, Vec<LogString>>,
}

impl TextMapListDataset {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: LogString, data: Vec<LogString>) {
        self.data.insert(key, data);
    }
    pub fn get(&self, key: &str) -> Option<&Vec<LogString>> {
        self.data.get(key)
    }
    pub fn internal_ref(&self) -> &BTreeMap<LogString, Vec<LogString>> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_data_in_dataset() {
        let mut dataset = TextMapListDataset::new();
        dataset.insert(
            LogString::Borrowed("192.168.1.1"),
            vec![
                LogString::Borrowed("Local IP "),
                LogString::Borrowed("Remote IP"),
            ],
        );
        assert_eq!(
            dataset.get("192.168.1.1"),
            Some(
                &(vec![
                    LogString::Borrowed("Local IP "),
                    LogString::Borrowed("Remote IP")
                ])
            )
        );
    }
}
