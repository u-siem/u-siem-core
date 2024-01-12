use crate::prelude::types::LogString;
use crate::prelude::SiemIp;
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
    pub fn insert(&self, ip: SiemIp, data: LogString) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMap::Add((ip, data)));
    }
    pub fn remove(&self, ip: SiemIp) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMap::Remove(ip));
    }
    pub fn update(&self, data: IpMapDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateIpMap::Replace(data));
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&LogString> {
        // Todo improve with cached content
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug, Default)]
pub struct IpMapDataset {
    data4: BTreeMap<u32, LogString>,
    data6: BTreeMap<u128, LogString>,
}

impl IpMapDataset {
    pub fn new() -> IpMapDataset {
        IpMapDataset::default()
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
