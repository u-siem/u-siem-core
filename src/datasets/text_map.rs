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
    pub fn new(dataset: Arc<TextMapDataset>, comm: Sender<UpdateTextMap>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(TextMapDataset::new()),
            comm: sender,
        }
    }
    pub fn insert<S>(&self, key: S, data: S)
    where
        S: Into<LogString>,
    {
        let _ = self
            .comm
            .try_send(UpdateTextMap::Add((key.into(), data.into())));
    }
    pub fn remove<S>(&self, key: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextMap::Remove(key.into()));
    }
    pub fn update(&self, data: TextMapDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextMap::Replace(data));
    }
    pub fn get(&self, key: &str) -> Option<&LogString> {
        // Todo improve with cached content
        self.dataset.get(key)
    }
    pub fn inner(&self) -> &TextMapDataset {
        self.dataset.as_ref()
    }
    pub fn apply_updates(&self, updates : Vec<UpdateTextMap>) -> Self {
        let mut iter = updates.into_iter();
        let first = iter.next().unwrap();
        let mut new  = match first {
            UpdateTextMap::Replace(v) => v,
            UpdateTextMap::Add((a,b)) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.insert(a, b);
                dataset
            },
            UpdateTextMap::Remove(a) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.remove(&a);
                dataset
            }
        };
        for update in iter {
            match update {
                UpdateTextMap::Add((a,b)) => {
                    new.insert(a, b);
                },
                UpdateTextMap::Remove(a) => {
                    new.remove(&a);
                },
                UpdateTextMap::Replace(v) => {
                    new = v;
                },
            };
        }
        Self::new(Arc::new(new), self.comm.clone())
    }
}
#[derive(Serialize, Debug, Default, Clone)]
pub struct TextMapDataset {
    data: BTreeMap<LogString, LogString>,
}

impl TextMapDataset {
    pub fn new() -> TextMapDataset {
        Self::default()
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
    pub fn remove(&mut self, key : &str) {
        self.data.remove(key);
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
