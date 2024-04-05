use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeSet;
use std::sync::Arc;
#[derive(Serialize, Debug)]
pub enum UpdateTextSet {
    Add(LogString),
    Remove(LogString),
    Replace(TextSetDataset),
}
#[derive(Debug, Clone)]
pub struct TextSetSynDataset {
    dataset: Arc<TextSetDataset>,
    comm: Sender<UpdateTextSet>,
}
impl TextSetSynDataset {
    pub fn new(dataset: Arc<TextSetDataset>, comm: Sender<UpdateTextSet>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(TextSetDataset::new()),
            comm: sender,
        }
    }
    pub fn insert<S>(&self, val: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextSet::Add(val.into()));
    }
    pub fn remove<S>(&self, val: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextSet::Remove(val.into()));
    }
    pub fn update(&self, data: TextSetDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateTextSet::Replace(data));
    }
    pub fn contains(&self, val: &LogString) -> bool {
        // Todo improve with cached content
        self.dataset.contains(val)
    }
    pub fn inner(&self) -> &TextSetDataset {
        self.dataset.as_ref()
    }
    pub fn apply_updates(&self, updates : Vec<UpdateTextSet>) -> Self {
        let mut iter = updates.into_iter();
        let first = iter.next().unwrap();
        let mut new  = match first {
            UpdateTextSet::Replace(v) => v,
            UpdateTextSet::Add(a) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.insert(a);
                dataset
            },
            UpdateTextSet::Remove(a) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.remove(&a);
                dataset
            }
        };
        for update in iter {
            match update {
                UpdateTextSet::Add(a) => {
                    new.insert(a);
                },
                UpdateTextSet::Remove(a) => {
                    new.remove(&a);
                },
                UpdateTextSet::Replace(v) => {
                    new = v;
                },
            };
        }
        Self::new(Arc::new(new), self.comm.clone())
    }
}
#[derive(Serialize, Debug, Default, Clone)]
pub struct TextSetDataset {
    data: BTreeSet<LogString>,
}

impl TextSetDataset {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert<S>(&mut self, val: S)
    where
        S: Into<LogString>,
    {
        self.data.insert(val.into());
    }
    pub fn contains(&self, val: &LogString) -> bool {
        self.data.contains(val)
    }
    pub fn internal_ref(&self) -> &BTreeSet<LogString> {
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
    fn should_be_in_set() {
        let mut dataset = TextSetDataset::new();
        dataset.insert(LogString::Borrowed("192.168.1.1"));
        assert_eq!(dataset.contains(&LogString::Borrowed("192.168.1.1")), true);
    }
}
