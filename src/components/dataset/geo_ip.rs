use super::super::super::events::field::SiemIp;
use crossbeam_channel::Sender;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;

/// Enum used to Add/Remove an IP in the GeoIP dataset or full replace it
#[derive(Serialize, Debug)]
pub enum UpdateGeoIp {
    Add((SiemIp, u8, GeoIpInfo)),
    Remove((SiemIp, u8)),
    Replace(GeoIpDataset),
}
#[derive(Serialize, Debug)]
pub struct GeoIpInfo {
    pub country: Cow<'static, str>,
    pub city: Cow<'static, str>,
    pub latitude: f32,
    pub longitude: f32,
    pub isp: Cow<'static, str>,
}
#[derive(Debug)]
pub struct GeoIpSynDataset {
    dataset: Arc<GeoIpDataset>,
    comm: Sender<UpdateGeoIp>,
}
impl GeoIpSynDataset {
    pub fn new(dataset: Arc<GeoIpDataset>, comm: Sender<UpdateGeoIp>) -> GeoIpSynDataset {
        return GeoIpSynDataset { dataset, comm };
    }
    /// This method must not be used with this dataset, because no source will give you accurate data to update this dataset. Maybe some firewalls, but updating the dataset with each log information is not a good idea.
    pub fn insert(&mut self, ip: SiemIp, net: u8, data: GeoIpInfo) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateGeoIp::Add((ip, net, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, ip: SiemIp) -> Option<&GeoIpInfo> {
        // Todo improve with cached added IPs
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug)]
pub struct GeoIpDataset {
    data4: BTreeMap<u32, BTreeMap<u32, GeoIpInfo>>,
    data6: BTreeMap<u32, BTreeMap<u128, GeoIpInfo>>,
}

impl GeoIpDataset {
    pub fn new() -> GeoIpDataset {
        return GeoIpDataset {
            data4: BTreeMap::new(),
            data6: BTreeMap::new(),
        };
    }
    pub fn insert(&mut self, ip: SiemIp, net: u8, data: GeoIpInfo) {
        match ip {
            SiemIp::V4(ip) => {
                let ip_net = ip & std::u32::MAX.checked_shl((32 - net) as u32).unwrap_or(0);
                if self.data4.contains_key(&(net as u32)) {
                    match self.data4.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data);
                        }
                        None => {}
                    };
                } else {
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data4.insert(net as u32, new_net);
                }
            }
            SiemIp::V6(ip) => {
                let ip_net = ip & std::u128::MAX.checked_shl((128 - net) as u32).unwrap_or(0);
                if self.data6.contains_key(&(net as u32)) {
                    match self.data6.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data);
                        }
                        None => {}
                    };
                } else {
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data6.insert(net as u32, new_net);
                }
            }
        }
    }
    pub fn get(&self, ip: SiemIp) -> Option<&GeoIpInfo> {
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
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dataset_creation() {
        let info = GeoIpInfo{
            city : Cow::Borrowed("LocalCity"),
            country : Cow::Borrowed("LocalCountry"),
            isp : Cow::Borrowed("ISP"),
            latitude : 0.1,
            longitude : 0.2,
        };
        let mut dataset = GeoIpDataset::new();
        dataset.insert(
            SiemIp::from_ip_str("192.168.1.1").unwrap(),
            24,
            info,
        );
        assert_eq!(
            &dataset.get(SiemIp::from_ip_str("192.168.1.1").unwrap()).unwrap().city[..],
            &(&Cow::Borrowed("LocalCity"))[..]
        );
    }
}