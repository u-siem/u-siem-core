use super::super::super::events::field::SiemIp;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub enum UpdateIpSet {
    Add(SiemIp),
    Remove(SiemIp),
    Replace(IpSetDataset),
}
#[derive(Debug, Clone)]
pub struct IpSetSynDataset {
    dataset: Arc<IpSetDataset>,
    comm: Sender<UpdateIpSet>,
}
impl IpSetSynDataset {
    pub fn new(dataset: Arc<IpSetDataset>, comm: Sender<UpdateIpSet>) -> IpSetSynDataset {
        return IpSetSynDataset { dataset, comm };
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&self, ip: SiemIp) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpSet::Add(ip)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&self, ip: SiemIp) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpSet::Remove(ip)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data: IpSetDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpSet::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn contains(&self, ip: &SiemIp) -> bool {
        // Todo improve with cached content
        self.dataset.contains(ip)
    }
}
#[derive(Serialize, Debug)]
pub struct IpSetDataset {
    data4: BTreeSet<u32>,
    data6: BTreeSet<u128>,
}

impl IpSetDataset {
    pub fn new() -> IpSetDataset {
        return IpSetDataset {
            data4: BTreeSet::new(),
            data6: BTreeSet::new(),
        };
    }
    pub fn insert(&mut self, ip: SiemIp) {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.insert(ip);
            }
            SiemIp::V6(ip) => {
                self.data6.insert(ip);
            }
        }
    }
    pub fn contains(&self, ip: &SiemIp) -> bool {
        match ip {
            SiemIp::V4(ip) => self.data4.contains(ip),
            SiemIp::V6(ip) => self.data6.contains(ip),
        }
    }
    pub fn internal_ref(&self) -> (&BTreeSet<u32>, &BTreeSet<u128>) {
        (&self.data4, &self.data6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_be_in_set() {
        let mut dataset = IpSetDataset::new();
        dataset.insert(SiemIp::from_ip_str("192.168.1.1").unwrap());
        assert_eq!(
            dataset.contains(&SiemIp::from_ip_str("192.168.1.1").unwrap()),
            true
        );
    }
}
