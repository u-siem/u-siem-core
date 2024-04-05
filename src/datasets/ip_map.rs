use crate::prelude::types::LogString;

use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub enum UpdateIpMap {
    Add((std::net::IpAddr, LogString)),
    Remove(std::net::IpAddr),
    Replace(IpMapDataset),
}
#[derive(Debug, Clone)]
pub struct IpMapSynDataset {
    dataset: Arc<IpMapDataset>,
    comm: Sender<UpdateIpMap>,
}
impl IpMapSynDataset {
    pub fn new(dataset: Arc<IpMapDataset>, comm: Sender<UpdateIpMap>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(IpMapDataset::new()),
            comm: sender,
        }
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&self, ip: std::net::IpAddr, data: LogString) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMap::Add((ip, data)));
    }
    pub fn remove(&self, ip: std::net::IpAddr) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMap::Remove(ip));
    }
    pub fn update(&self, data: IpMapDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMap::Replace(data));
    }
    pub fn get(&self, ip: &std::net::IpAddr) -> Option<&LogString> {
        // Todo improve with cached content
        self.dataset.get(&(*ip).into())
    }
    pub fn inner(&self) -> &IpMapDataset {
        self.dataset.as_ref()
    }
    pub fn apply_updates(&self, updates : Vec<UpdateIpMap>) -> Self {
        let mut iter = updates.into_iter();
        let first = iter.next().unwrap();
        let mut new : IpMapDataset = match first {
            UpdateIpMap::Add((a,b)) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.insert(a, b);
                dataset
            },
            UpdateIpMap::Remove(v) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.remove(&v);
                dataset
            },
            UpdateIpMap::Replace(v) => v,
        };
        for update in iter {
            match update {
                UpdateIpMap::Add((a,b)) => {
                    new.insert(a, b);
                },
                UpdateIpMap::Remove(v) => {
                    new.remove(&v);
                },
                UpdateIpMap::Replace(v) => {
                    new = v;
                },
            };
        }
        Self::new(Arc::new(new), self.comm.clone())
    }
}
#[derive(Serialize, Debug, Default, Clone)]
pub struct IpMapDataset {
    data4: BTreeMap<u32, LogString>,
    data6: BTreeMap<u128, LogString>,
}

impl IpMapDataset {
    pub fn new() -> IpMapDataset {
        IpMapDataset::default()
    }
    pub fn insert<S>(&mut self, ip: std::net::IpAddr, data: S)
    where
        S: Into<LogString>,
    {
        match ip {
            std::net::IpAddr::V4(ip) => {
                self.data4.insert(ip.into(), data.into());
            }
            std::net::IpAddr::V6(ip) => {
                self.data6.insert(ip.into(), data.into());
            }
        }
    }
    pub fn get(&self, ip: &std::net::IpAddr) -> Option<&LogString> {
        match ip {
            std::net::IpAddr::V4(ip) => self.data4.get(&(*ip).into()),
            std::net::IpAddr::V6(ip) => self.data6.get(&(*ip).into()),
        }
    }
    pub fn remove(&mut self, ip : &std::net::IpAddr) {
        match ip {
            std::net::IpAddr::V4(ip) => {
                self.data4.remove(&(*ip).into());
            }
            std::net::IpAddr::V6(ip) => {
                self.data6.remove(&(*ip).into());
            }
        }
    }

    pub fn internal_ref(&self) -> (&BTreeMap<u32, LogString>, &BTreeMap<u128, LogString>) {
        (&self.data4, &self.data6)
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    #[test]
    fn should_find_info_of_ip_in_map() {
        let mut dataset = IpMapDataset::new();
        dataset.insert(
            std::net::IpAddr::from_str("192.168.1.1").unwrap(),
            LogString::Borrowed("Local IP "),
        );
        assert_eq!(
            dataset.get(&std::net::IpAddr::from_str("192.168.1.1").unwrap()),
            Some(&LogString::Borrowed("Local IP "))
        );
    }
}
