use crossbeam_channel::Sender;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;
use std::vec::Vec;

#[derive(Serialize, Debug)]
pub enum UpdateTextMapList {
    Add((Cow<'static, str>, Vec<Cow<'static, str>>)),
    Remove(Cow<'static, str>),
    Replace(TextMapListDataset),
}
#[derive(Debug,Clone)]
pub struct TextMapListSynDataset {
    dataset: Arc<TextMapListDataset>,
    comm: Sender<UpdateTextMapList>,
}
impl TextMapListSynDataset {
    pub fn new(dataset: Arc<TextMapListDataset>, comm: Sender<UpdateTextMapList>) -> TextMapListSynDataset {
        return TextMapListSynDataset { dataset, comm };
    }
    pub fn insert(&mut self, key : Cow<'static, str>, data: Vec<Cow<'static, str>>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMapList::Add((key, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&mut self, key : Cow<'static, str>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMapList::Remove(key)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&mut self, data : TextMapListDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextMapList::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, key : Cow<'static, str>) -> Option<&Vec<Cow<'static, str>>> {
        // Todo improve with cached content
        self.dataset.get(key)
    }
}
#[derive(Serialize, Debug)]
pub struct TextMapListDataset {
    data: BTreeMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
}

impl TextMapListDataset {
    pub fn new() -> TextMapListDataset {
        return TextMapListDataset {
            data: BTreeMap::new()
        };
    }
    pub fn insert(&mut self, key : Cow<'static, str>, data: Vec<Cow<'static, str>>) {
        self.data.insert(key, data);
    }
    pub fn get(&self, key : Cow<'static, str>) -> Option<&Vec<Cow<'static, str>>> {
        self.data.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_dataset_creation() {
        let mut dataset = TextMapListDataset::new();
        dataset.insert(
            Cow::Borrowed("192.168.1.1"),
            vec![Cow::Borrowed("Local IP "), Cow::Borrowed("Remote IP")],
        );
        assert_eq!(
            dataset.get(Cow::Borrowed("192.168.1.1")),
            Some(&(vec![Cow::Borrowed("Local IP "),Cow::Borrowed("Remote IP")]))
        );
    }
}
