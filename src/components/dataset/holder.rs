use std::collections::BTreeMap;
use super::{SiemDataset, SiemDatasetType};

pub struct DatasetHolder {
    datasets : BTreeMap<SiemDatasetType, SiemDataset>
}

impl DatasetHolder {
    pub fn new() -> Self {
        Self {
            datasets : BTreeMap::new()
        }
    }
    pub fn add(&mut self, dataset : SiemDataset) {
        self.datasets.insert(dataset.dataset_type().clone(), dataset);
    }

    pub fn get(&self, key : &SiemDatasetType) -> Option<&SiemDataset> {
        self.datasets.get(key)
    }
}