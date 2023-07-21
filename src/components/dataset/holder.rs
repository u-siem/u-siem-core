use super::{SiemDataset, SiemDatasetType};
use std::collections::BTreeMap;

/// The dataset holder allows access to the latest version of a dataset almost instantly without the need to check if there is an update of a dataset using a channel as was done previously.
#[derive(Clone, Default)]
pub struct DatasetHolder {
    datasets: BTreeMap<SiemDatasetType, SiemDataset>,
}

impl DatasetHolder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_datasets(list: Vec<SiemDataset>) -> Self {
        let mut datasets = BTreeMap::new();
        for dataset in list {
            datasets.insert(dataset.dataset_type(), dataset);
        }

        Self { datasets }
    }

    pub fn insert(&mut self, dataset: SiemDataset) {
        self.datasets.insert(dataset.dataset_type(), dataset);
    }

    pub fn get(&self, key: &SiemDatasetType) -> Option<&SiemDataset> {
        self.datasets.get(key)
    }
    pub fn subset(&self, list: Vec<SiemDatasetType>) -> Self {
        let mut datasets = BTreeMap::new();
        for typ in list {
            let dataset = match self.datasets.get(&typ) {
                Some(d) => d,
                None => continue,
            };
            datasets.insert(typ, dataset.clone());
        }

        Self { datasets }
    }
}
