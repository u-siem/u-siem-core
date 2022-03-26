use super::super::super::events::field::SiemIp;
use crossbeam_channel::Sender;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum UpdateIpMap {
    Add((SiemIp, Cow<'static, str>)),
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
    pub fn insert(&self, ip: SiemIp, data: Cow<'static, str>) {
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
    pub fn update(&self, data : IpMapDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMap::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, ip: SiemIp) -> Option<&Cow<'static, str>> {
        // Todo improve with cached content
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug)]
pub struct IpMapDataset {
    data4: BTreeMap<u32, Cow<'static, str>>,
    data6: BTreeMap<u128, Cow<'static, str>>,
}

impl IpMapDataset {
    pub fn new() -> IpMapDataset {
        return IpMapDataset {
            data4: BTreeMap::new(),
            data6: BTreeMap::new(),
        };
    }
    pub fn insert<S>(&mut self, ip: SiemIp, data: S) where S: Into<Cow<'static, str>> {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.insert(ip, data.into());
            }
            SiemIp::V6(ip) => {
                self.data6.insert(ip, data.into());
            }
        }
    }
    pub fn get(&self, ip: SiemIp) -> Option<&Cow<'static, str>> {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.get(&ip)
            }
            SiemIp::V6(ip) => {
                self.data6.get(&ip)
            }
        }
    }

    pub fn internal_ref(&self) -> (&BTreeMap<u32, Cow<'static, str>>,&BTreeMap<u128, Cow<'static, str>>) {
        (&self.data4, &self.data6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dataset_creation() {
        let mut dataset = IpMapDataset::new();
        dataset.insert(
            SiemIp::from_ip_str("192.168.1.1").unwrap(),
            Cow::Borrowed("Local IP "),
        );
        assert_eq!(
            dataset.get(SiemIp::from_ip_str("192.168.1.1").unwrap()),
            Some(&Cow::Borrowed("Local IP "))
        );
    }
}
