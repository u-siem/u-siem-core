use super::super::super::events::field::SiemIp;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Serialize, Debug)]
pub enum UpdateIpMapList {
    Add((SiemIp, Vec<Cow<'static, str>>)),
    Remove(SiemIp),
    Replace(IpMapListDataset),
}
#[derive(Debug, Clone)]
pub struct IpMapListSynDataset {
    dataset: Arc<IpMapListDataset>,
    comm: Sender<UpdateIpMapList>,
}
impl IpMapListSynDataset {
    pub fn new(
        dataset: Arc<IpMapListDataset>,
        comm: Sender<UpdateIpMapList>,
    ) -> IpMapListSynDataset {
        return IpMapListSynDataset { dataset, comm };
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&self, ip: SiemIp, data: Vec<Cow<'static, str>>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMapList::Add((ip, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&self, ip: SiemIp) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMapList::Remove(ip)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data: IpMapListDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateIpMapList::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&Vec<Cow<'static, str>>> {
        // Todo improve with cached content
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug)]
pub struct IpMapListDataset {
    data4: BTreeMap<u32, Vec<Cow<'static, str>>>,
    data6: BTreeMap<u128, Vec<Cow<'static, str>>>,
}

impl IpMapListDataset {
    pub fn new() -> IpMapListDataset {
        return IpMapListDataset {
            data4: BTreeMap::new(),
            data6: BTreeMap::new(),
        };
    }
    pub fn insert(&mut self, ip: SiemIp, data: Vec<Cow<'static, str>>) {
        match ip {
            SiemIp::V4(ip) => {
                self.data4.insert(ip, data);
            }
            SiemIp::V6(ip) => {
                self.data6.insert(ip, data);
            }
        }
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&Vec<Cow<'static, str>>> {
        match ip {
            SiemIp::V4(ip) => self.data4.get(ip),
            SiemIp::V6(ip) => self.data6.get(ip),
        }
    }
    pub fn internal_ref(
        &self,
    ) -> (
        &BTreeMap<u32, Vec<Cow<'static, str>>>,
        &BTreeMap<u128, Vec<Cow<'static, str>>>,
    ) {
        (&self.data4, &self.data6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dataset_creation() {
        let mut dataset = IpMapListDataset::new();
        dataset.insert(
            SiemIp::from_ip_str("192.168.1.1").unwrap(),
            vec![Cow::Borrowed("Local IP "), Cow::Borrowed("Remote IP")],
        );
        assert_eq!(
            dataset.get(&SiemIp::from_ip_str("192.168.1.1").unwrap()),
            Some(&(vec![Cow::Borrowed("Local IP "), Cow::Borrowed("Remote IP")]))
        );
    }
}
