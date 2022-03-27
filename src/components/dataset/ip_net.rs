use super::super::super::events::field::SiemIp;
use crossbeam_channel::Sender;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum UpdateNetIp {
    Add((SiemIp, u8, Cow<'static, str>)),
    Remove((SiemIp, u8)),
    Replace(IpNetDataset),
}
#[derive(Debug, Clone)]
pub struct IpNetSynDataset {
    dataset: Arc<IpNetDataset>,
    comm: Sender<UpdateNetIp>,
}
impl IpNetSynDataset {
    pub fn new(dataset: Arc<IpNetDataset>, comm: Sender<UpdateNetIp>) -> IpNetSynDataset {
        return IpNetSynDataset { dataset, comm };
    }
    pub fn insert(&self, ip: SiemIp, net: u8, data: Cow<'static, str>) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateNetIp::Add((ip, net, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&self, ip: SiemIp, net: u8) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateNetIp::Remove((ip, net))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data : IpNetDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateNetIp::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&Cow<'static, str>> {
        // Todo improve with cached content
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug)]
pub struct IpNetDataset {
    data4: BTreeMap<u32, BTreeMap<u32, Cow<'static, str>>>,
    data6: BTreeMap<u32, BTreeMap<u128, Cow<'static, str>>>,
}

impl IpNetDataset {
    pub fn new() -> IpNetDataset {
        return IpNetDataset {
            data4: BTreeMap::new(),
            data6: BTreeMap::new(),
        };
    }
    pub fn insert<S>(&mut self, ip: SiemIp, net: u8, data: S) where S: Into<Cow<'static, str>> {
        match ip {
            SiemIp::V4(ip) => {
                let ip_net = ip & std::u32::MAX.checked_shl((32 - net) as u32).unwrap_or(0);
                if self.data4.contains_key(&(net as u32)) {
                    match self.data4.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data.into());
                        }
                        None => {}
                    };
                } else {
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data.into());
                    self.data4.insert(net as u32, new_net);
                }
            }
            SiemIp::V6(ip) => {
                let ip_net = ip & std::u128::MAX.checked_shl((128 - net) as u32).unwrap_or(0);
                if self.data6.contains_key(&(net as u32)) {
                    match self.data6.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data.into());
                        }
                        None => {}
                    };
                } else {
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data.into());
                    self.data6.insert(net as u32, new_net);
                }
            }
        }
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&Cow<'static, str>> {
        match ip {
            SiemIp::V4(ip) => {
                let zeros = ip.trailing_zeros();
                for i in zeros..32 {
                    let ip_net = ip & std::u32::MAX.checked_shl(32 - i).unwrap_or(0);
                    match self.data4.get(&i) {
                        Some(map) => match map.get(&ip_net) {
                            Some(v) => return Some(v),
                            None => {
                                continue;
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                }
                None
            }
            SiemIp::V6(ip) => {
                let zeros = ip.trailing_zeros();
                for i in zeros..128 {
                    let ip_net = ip & std::u128::MAX.checked_shl(128 - i).unwrap_or(0);
                    match self.data6.get(&i) {
                        Some(map) => match map.get(&ip_net) {
                            Some(v) => return Some(v),
                            None => {
                                continue;
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                }
                None
            }
        }
    }
    pub fn internal_ref(&self) -> (&BTreeMap<u32, BTreeMap<u32, Cow<'static, str>>>,&BTreeMap<u32, BTreeMap<u128, Cow<'static, str>>>) {
        (&self.data4, &self.data6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dataset_creation() {
        let mut dataset = IpNetDataset::new();
        dataset.insert(
            SiemIp::from_ip_str("192.168.1.1").unwrap(),
            24,
            Cow::Borrowed("Local IP "),
        );
        assert_eq!(
            dataset.get(&SiemIp::from_ip_str("192.168.1.1").unwrap()),
            Some(&Cow::Borrowed("Local IP "))
        );
    }
}
