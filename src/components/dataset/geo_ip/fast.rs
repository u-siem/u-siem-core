use crate::prelude::SiemIp;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::GeoIpInfo;

/// Enum used to Add/Remove an IP in the GeoIP dataset or full replace it
#[derive(Serialize, Debug)]
pub enum UpdateGeoIp {
    Add((SiemIp, u8, GeoIpInfo)),
    Remove((SiemIp, u8)),
    Replace(GeoIpDataset),
}

#[derive(Debug, Clone)]
pub struct GeoIpSynDataset {
    dataset: Arc<GeoIpDataset>,
    comm: Sender<UpdateGeoIp>,
}
impl GeoIpSynDataset {
    pub fn new(dataset: Arc<GeoIpDataset>, comm: Sender<UpdateGeoIp>) -> Self {
        Self { dataset, comm }
    }
    pub fn full_update(&self, dataset: GeoIpDataset) {
        let _ = self.comm.send(UpdateGeoIp::Replace(dataset));
    }

    /// This method must not be used with this dataset, because no source will give you accurate data to update this dataset. Maybe some firewalls, but updating the dataset with each log information is not a good idea.
    pub fn insert(&mut self, ip: SiemIp, net: u8, data: GeoIpInfo) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateGeoIp::Add((ip, net, data)));
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&GeoIpInfo> {
        // Todo improve with cached added IPs
        self.dataset.get(ip)
    }
}
#[derive(Serialize, Debug, Default)]
pub struct GeoIpDataset {
    data4: BTreeMap<u32, BTreeMap<u32, GeoIpInfo>>,
    data6: BTreeMap<u32, BTreeMap<u128, GeoIpInfo>>,
}

impl GeoIpDataset {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, ip: SiemIp, net: u8, data: GeoIpInfo) {
        match ip {
            SiemIp::V4(ip) => {
                let ip_net = ip & std::u32::MAX.checked_shl((32 - net) as u32).unwrap_or(0);
                if self.data4.contains_key(&(net as u32)) {
                    if let Some(dataset) = self.data4.get_mut(&(net as u32)) {
                        dataset.insert(ip_net, data);
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
                    if let Some(dataset) = self.data6.get_mut(&(net as u32)) {
                        dataset.insert(ip_net, data);
                    };
                } else {
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data6.insert(net as u32, new_net);
                }
            }
        }
    }
    pub fn get(&self, ip: &SiemIp) -> Option<&GeoIpInfo> {
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
    pub fn internal_ref(
        &self,
    ) -> (
        &BTreeMap<u32, BTreeMap<u32, GeoIpInfo>>,
        &BTreeMap<u32, BTreeMap<u128, GeoIpInfo>>,
    ) {
        (&self.data4, &self.data6)
    }
}
#[cfg(test)]
mod tests {

    use crate::prelude::types::LogString;

    use super::*;
    #[test]
    fn geo_ip_should_find_ip() {
        let info = GeoIpInfo {
            city: LogString::Borrowed("LocalCity"),
            country: LogString::Borrowed("LocalCountry"),
            country_iso: LogString::Borrowed("LC"),
            isp: LogString::Borrowed("ISP"),
            latitude: 0.1,
            longitude: 0.2,
            asn: 1,
        };
        let mut dataset = GeoIpDataset::new();
        dataset.insert(SiemIp::from_ip_str("192.168.1.0").unwrap(), 24, info);
        assert_eq!(
            "LocalCity",
            &dataset
                .get(&SiemIp::from_ip_str("192.168.1.1").unwrap())
                .unwrap()
                .city[..]
        );
    }
}
