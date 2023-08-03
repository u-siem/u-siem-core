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
    pub fn new(dataset: Arc<TextSetDataset>, comm: Sender<UpdateTextSet>) -> TextSetSynDataset {
        return TextSetSynDataset { dataset, comm };
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);

        return Self { dataset : Arc::new(TextSetDataset::new()), comm : sender };
    }
    pub fn insert<S>(&self, val: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextSet::Add(val.into())) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove<S>(&self, val: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextSet::Remove(val.into())) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data: TextSetDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextSet::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn contains(&self, val: &LogString) -> bool {
        // Todo improve with cached content
        self.dataset.contains(val)
    }
}
#[derive(Serialize, Debug)]
pub struct TextSetDataset {
    data: BTreeSet<LogString>,
}

impl TextSetDataset {
    pub fn new() -> TextSetDataset {
        return TextSetDataset {
            data: BTreeSet::new(),
        };
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
