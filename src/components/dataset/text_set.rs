use crossbeam_channel::Sender;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::borrow::Cow;

pub enum UpdateTextSet {
    Add(Cow<'static, str>),
    Remove(Cow<'static, str>),
    Replace(TextSetDataset),
}

pub struct TextSetSynDataset {
    dataset: Arc<TextSetDataset>,
    comm: Sender<UpdateTextSet>,
}
impl TextSetSynDataset {
    pub fn new(dataset: Arc<TextSetDataset>, comm: Sender<UpdateTextSet>) -> TextSetSynDataset {
        return TextSetSynDataset { dataset, comm };
    }
    pub fn insert(&mut self, val: Cow<'static, str>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextSet::Add(val)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&mut self, val: Cow<'static, str>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextSet::Remove(val)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&mut self, data : TextSetDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateTextSet::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn contains(&self, val: &Cow<'static, str>) -> bool {
        // Todo improve with cached content
        self.dataset.contains(val)
    }
}

pub struct TextSetDataset {
    data: BTreeSet<Cow<'static, str>>
}

impl TextSetDataset {
    pub fn new() -> TextSetDataset {
        return TextSetDataset {
            data: BTreeSet::new()
        };
    }
    pub fn insert(&mut self, val: Cow<'static, str>) {
        self.data.insert(val);
    }
    pub fn contains(&self, val: &Cow<'static, str>) -> bool {
        self.data.contains(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dataset_creation() {
        let mut dataset = TextSetDataset::new();
        dataset.insert(
            Cow::Borrowed("192.168.1.1")
        );
        assert_eq!(
            dataset.contains(&Cow::Borrowed("192.168.1.1")),
            true
        );
    }
}
