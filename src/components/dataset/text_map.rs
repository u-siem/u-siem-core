use crossbeam_channel::Sender;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;
#[derive(Serialize, Debug)]
pub enum UpdateTextMap {
    Add((Cow<'static, str>, Cow<'static, str>)),
    Remove(Cow<'static, str>),
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
    pub fn insert(&mut self, key : Cow<'static, str>, data: Cow<'static, str>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMap::Add((key, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&mut self, key : Cow<'static, str>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMap::Remove(key)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&mut self, data : TextMapDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMap::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, key : Cow<'static, str>) -> Option<&Cow<'static, str>> {
        // Todo improve with cached content
        self.dataset.get(key)
    }
}
#[derive(Serialize, Debug)]
pub struct TextMapDataset {
    data: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}

impl TextMapDataset {
    pub fn new() -> TextMapDataset {
        return TextMapDataset {
            data: BTreeMap::new()
        };
    }
    pub fn insert(&mut self, key : Cow<'static, str>, data: Cow<'static, str>) {
        self.data.insert(key, data);
    }
    pub fn get(&self, key : Cow<'static, str>) -> Option<&Cow<'static, str>> {
        self.data.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_dataset_creation() {
        let mut dataset = TextMapDataset::new();
        dataset.insert(
            Cow::Borrowed("192.168.1.1"),
            Cow::Borrowed("Local IP"),
        );
        assert_eq!(
            dataset.get(Cow::Borrowed("192.168.1.1")),
            Some(&Cow::Borrowed("Local IP"))
        );
    }
}
