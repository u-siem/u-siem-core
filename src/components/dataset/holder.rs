use super::{SiemDataset, SiemDatasetType};
use std::sync::atomic::AtomicPtr;
use std::{collections::BTreeMap, sync::Arc};

/// The dataset holder allows access to the latest version of a dataset almost instantly without the need to check if there is an update of a dataset using a channel as was done previously. Although you have to resort to two calls to unsafe since pointers are used
#[derive(Clone)]
pub struct DatasetHolder {
    datasets: BTreeMap<SiemDatasetType, Arc<AtomicPtr<SiemDataset>>>,
}

impl DatasetHolder {
    pub fn from_datasets(list: Vec<Arc<AtomicPtr<SiemDataset>>>) -> Self {
        let mut datasets = BTreeMap::new();
        for dataset in list {
            let data = dataset.load(std::sync::atomic::Ordering::Relaxed);
            datasets.insert(unsafe { (*data).dataset_type().clone() }, dataset);
        }

        Self { datasets }
    }

    pub fn get(&self, key: &SiemDatasetType) -> Option<&SiemDataset> {
        let dataset = self.datasets.get(key)?;
        let data = dataset.load(std::sync::atomic::Ordering::Relaxed);
        Some(unsafe { &(*data) })
    }
    pub fn subset(&self, list : Vec<SiemDatasetType>) -> Self {
        let mut datasets = BTreeMap::new();
        for typ in list {
            let dataset  = match self.datasets.get(&typ) {
                Some(d) => d,
                None => continue
            };
            datasets.insert(typ, dataset.clone());
        }

        Self { datasets }
    }
}

#[cfg(test)]
mod holder_test {
    use std::{convert::TryInto, time::Duration};

    use crate::{components::dataset::ip_set::{IpSetSynDataset, IpSetDataset}, events::field::SiemIp};

    use super::*;
    #[test]
    fn test_atomic() {
        let (s_c,_r_c) = crossbeam_channel::bounded(1024);
        let mut ip_set = IpSetDataset::new();
        ip_set.insert(SiemIp::V4(100_222));
        let mut dataset1 = SiemDataset::BlockIp(IpSetSynDataset::new(Arc::new(ip_set), s_c.clone()));
        let dataset1_ref = Arc::new(AtomicPtr::new(&mut dataset1));
        let datasets = vec![dataset1_ref.clone()];
        let holder = DatasetHolder::from_datasets(datasets);
        let hodler_th1 = holder.clone();
        let th1 = std::thread::spawn(move|| {
            let val = hodler_th1.get(&SiemDatasetType::BlockIp).unwrap();
            let val : &IpSetSynDataset = val.try_into().unwrap();
            assert!(val.contains(&SiemIp::V4(100_222)));
            std::thread::sleep(Duration::from_millis(100));
            let val = hodler_th1.get(&SiemDatasetType::BlockIp).unwrap();
            let val : &IpSetSynDataset = val.try_into().unwrap();
            assert!(val.contains(&SiemIp::V4(100_223)));
        });
        std::thread::sleep(Duration::from_millis(10));
        let mut ip_set2 = IpSetDataset::new();
        ip_set2.insert(SiemIp::V4(100_223));
        let mut dataset2 = SiemDataset::BlockIp(IpSetSynDataset::new(Arc::new(ip_set2), s_c.clone()));
        dataset1_ref.store(&mut dataset2, std::sync::atomic::Ordering::Relaxed);
        let _ = th1.join();
    }

    #[test]
    fn test_atomic_performance() {
        let (s_c,_r_c) = crossbeam_channel::bounded(1024);
        let mut ip_set = IpSetDataset::new();
        ip_set.insert(SiemIp::V4(100_222));
        let mut dataset1 = SiemDataset::BlockIp(IpSetSynDataset::new(Arc::new(ip_set), s_c.clone()));
        let dataset1_ref = Arc::new(AtomicPtr::new(&mut dataset1));
        let datasets = vec![dataset1_ref.clone()];
        let holder = DatasetHolder::from_datasets(datasets);
        let hodler_th1 = holder.clone();
        let th1 = std::thread::spawn(move|| {
            let now = std::time::Instant::now();
            let ipcheck = SiemIp::V4(100_222);
            for _ in 0..1_000_000 {
                let val = hodler_th1.get(&SiemDatasetType::BlockIp).unwrap();
                let val : &IpSetSynDataset = val.try_into().unwrap();
                let _ = val.contains(&ipcheck);
            }
            //println!("DPS = {}", 1_000_000_000 / now.elapsed().as_millis());
            // Aprox 2.500.000 Datasets per second
            assert!(1_000_000_000 / now.elapsed().as_millis()  > 1_500_000);
            
        });
        let _ = th1.join();
    }
}
