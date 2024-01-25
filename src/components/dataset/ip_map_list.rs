use crate::prelude::types::LogString;
use crate::prelude::SiemIp;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Serialize, Debug)]
pub enum UpdateIpMapList {
    Add((SiemIp, Vec<LogString>)),
    Remove(SiemIp),
    Replace(IpMapListDataset),
}
#[derive(Debug, Clone)]
pub struct IpMapListSynDataset {
    dataset: Arc<IpMapListDataset>,
    comm: Sender<UpdateIpMapList>,
}
impl IpMapListSynDataset {
    pub fn new(dataset: Arc<IpMapListDataset>, comm: Sender<UpdateIpMapList>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(IpMapListDataset::new()),
            comm: sender,
        }
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&self, ip: SiemIp, data: Vec<LogString>) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMapList::Add((ip, data)));
    }
    pub fn remove(&self, ip: SiemIp) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMapList::Remove(ip));
    }
    pub fn update(&self, data: IpMapListDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMapList::Replace(data));
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&Vec<LogString>> {
        // Todo improve with cached content
        self.dataset.get(ip)
    }
    pub fn apply_updates(&self, updates : Vec<UpdateIpMapList>) -> Self {
        let mut iter = updates.into_iter();
        let first = iter.next().unwrap();
        let mut new = match first {
            UpdateIpMapList::Add((a,b)) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.insert(a, b);
                dataset
            },
            UpdateIpMapList::Remove(v) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.remove(&v);
                dataset
            },
            UpdateIpMapList::Replace(v) => v,
        };
        for update in iter {
            match update {
                UpdateIpMapList::Add((a,b)) => {
                    new.insert(a, b);
                },
                UpdateIpMapList::Remove(v) => {
                    new.remove(&v);
                },
                UpdateIpMapList::Replace(v) => {
                    new = v;
                },
            };
        }
        Self::new(Arc::new(new), self.comm.clone())
    }
}
#[derive(Serialize, Debug, Default, Clone)]
pub struct IpMapListDataset {
    data4: BTreeMap<u32, Vec<LogString>>,
    data6: BTreeMap<u128, Vec<LogString>>,
}

impl IpMapListDataset {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, ip: SiemIp, data: Vec<LogString>) {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.insert(ip, data);
            }
            SiemIp::V6(ip) => {
                self.data6.insert(ip, data);
            }
        }
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&Vec<LogString>> {
        match ip {
            SiemIp::V4(ip) => self.data4.get(ip),
            SiemIp::V6(ip) => self.data6.get(ip),
        }
    }
    pub fn internal_ref(
        &self,
    ) -> (
        &BTreeMap<u32, Vec<LogString>>,
        &BTreeMap<u128, Vec<LogString>>,
    ) {
        (&self.data4, &self.data6)
    }

    pub fn remove(&mut self, ip : &SiemIp) {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.remove(ip);
            }
            SiemIp::V6(ip) => {
                self.data6.remove(ip);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_info_of_ip_in_map() {
        let mut dataset = IpMapListDataset::new();
        dataset.insert(
            SiemIp::from_ip_str("192.168.1.1").unwrap(),
            vec![
                LogString::Borrowed("Local IP "),
                LogString::Borrowed("Remote IP"),
            ],
        );
        assert_eq!(
            dataset.get(&SiemIp::from_ip_str("192.168.1.1").unwrap()),
            Some(
                &(vec![
                    LogString::Borrowed("Local IP "),
                    LogString::Borrowed("Remote IP")
                ])
            )
        );
    }
}
