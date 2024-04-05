
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeSet;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub enum UpdateIpSet {
    Add(std::net::IpAddr),
    Remove(std::net::IpAddr),
    Replace(IpSetDataset),
}
#[derive(Debug, Clone)]
pub struct IpSetSynDataset {
    dataset: Arc<IpSetDataset>,
    comm: Sender<UpdateIpSet>,
}
impl IpSetSynDataset {
    pub fn new(dataset: Arc<IpSetDataset>, comm: Sender<UpdateIpSet>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(IpSetDataset::new()),
            comm: sender,
        }
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&self, ip: std::net::IpAddr) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpSet::Add(ip));
    }
    pub fn remove(&self, ip: std::net::IpAddr) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpSet::Remove(ip));
    }
    pub fn update(&self, data: IpSetDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpSet::Replace(data));
    }
    pub fn contains(&self, ip: &std::net::IpAddr) -> bool {
        // Todo improve with cached content
        self.dataset.contains(ip)
    }
    pub fn inner(&self) -> &IpSetDataset {
        self.dataset.as_ref()
    }
    pub fn apply_updates(&self, updates : Vec<UpdateIpSet>) -> Self {
        let mut iter = updates.into_iter();
        let first = iter.next().unwrap();
        let mut new  = match first {
            UpdateIpSet::Add(a) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.insert(a);
                dataset
            },
            UpdateIpSet::Remove(v) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.remove(&v);
                dataset
            },
            UpdateIpSet::Replace(v) => v,
        };
        for update in iter {
            match update {
                UpdateIpSet::Add(a) => {
                    new.insert(a);
                },
                UpdateIpSet::Remove(v) => {
                    new.remove(&v);
                },
                UpdateIpSet::Replace(v) => {
                    new = v;
                },
            };
        }
        Self::new(Arc::new(new), self.comm.clone())
    }
}
#[derive(Serialize, Debug, Clone)]
pub struct IpSetDataset {
    data4: BTreeSet<Ipv4Addr>,
    data6: BTreeSet<Ipv6Addr>,
}

impl Default for IpSetDataset {
    fn default() -> Self {
        Self::new()
    }
}

impl IpSetDataset {
    pub fn new() -> Self {
        Self {
            data4: BTreeSet::new(),
            data6: BTreeSet::new(),
        }
    }
    pub fn insert(&mut self, ip: std::net::IpAddr) {
        match ip {
            std::net::IpAddr::V4(ip) => {
                self.data4.insert(ip.into());
            }
            std::net::IpAddr::V6(ip) => {
                self.data6.insert(ip.into());
            }
        }
    }
    pub fn contains(&self, ip: &std::net::IpAddr) -> bool {
        match ip {
            std::net::IpAddr::V4(ip) => self.data4.contains(ip),
            std::net::IpAddr::V6(ip) => self.data6.contains(ip),
        }
    }
    pub fn internal_ref(&self) -> (&BTreeSet<Ipv4Addr>, &BTreeSet<Ipv6Addr>) {
        (&self.data4, &self.data6)
    }

    pub fn remove(&mut self, ip : &std::net::IpAddr) {
        match ip {
            std::net::IpAddr::V4(ip) => self.data4.remove(&(*ip).into()),
            std::net::IpAddr::V6(ip) => self.data6.remove(&(*ip).into()),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    
    #[test]
    fn should_be_in_set() {
        let mut dataset = IpSetDataset::new();
        dataset.insert(std::net::IpAddr::from_str("192.168.1.1").unwrap());
        assert_eq!(
            dataset.contains(&std::net::IpAddr::from_str("192.168.1.1").unwrap()),
            true
        );
    }
}
