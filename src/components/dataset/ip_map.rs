use super::super::super::events::field::SiemIp;
use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub enum UpdateIpMap {
    Add((SiemIp, LogString)),
    Remove(SiemIp),
    Replace(IpMapDataset),
}
#[derive(Debug, Clone)]
pub struct IpMapSynDataset {
    dataset: Arc<IpMapDataset>,
    comm: Sender<UpdateIpMap>,
}
impl IpMapSynDataset {
    pub fn new(dataset: Arc<IpMapDataset>, comm: Sender<UpdateIpMap>) -> IpMapSynDataset {
        return IpMapSynDataset { dataset, comm };
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&self, ip: SiemIp, data: LogString) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMap::Add((ip, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&self, ip: SiemIp) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMap::Remove(ip)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data: IpMapDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMap::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&LogString> {
        // Todo improve with cached content
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug)]
pub struct IpMapDataset {
    data4: BTreeMap<u32, LogString>,
    data6: BTreeMap<u128, LogString>,
}

impl IpMapDataset {
    pub fn new() -> IpMapDataset {
        return IpMapDataset {
            data4: BTreeMap::new(),
            data6: BTreeMap::new(),
        };
    }
    pub fn insert<S>(&mut self, ip: SiemIp, data: S)
    where
        S: Into<LogString>,
    {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.insert(ip, data.into());
            }
            SiemIp::V6(ip) => {
                self.data6.insert(ip, data.into());
            }
        }
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&LogString> {
        match ip {
            SiemIp::V4(ip) => self.data4.get(ip),
            SiemIp::V6(ip) => self.data6.get(ip),
        }
    }

    pub fn internal_ref(&self) -> (&BTreeMap<u32, LogString>, &BTreeMap<u128, LogString>) {
        (&self.data4, &self.data6)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_info_of_ip_in_map() {
        let mut dataset = IpMapDataset::new();
        dataset.insert(
            SiemIp::from_ip_str("192.168.1.1").unwrap(),
            LogString::Borrowed("Local IP "),
        );
        assert_eq!(
            dataset.get(&SiemIp::from_ip_str("192.168.1.1").unwrap()),
            Some(&LogString::Borrowed("Local IP "))
        );
    }
}
