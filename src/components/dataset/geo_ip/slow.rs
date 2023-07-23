use crate::prelude::SiemIp;
use crossbeam_channel::Sender;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::sync::Arc;

use super::GeoIpInfo;

/// Enum used to Add/Remove an IP in the GeoIP dataset or full replace it
#[derive(Serialize, Debug)]
pub enum UpdateSlowGeoIp {
    Add((SiemIp, u8, GeoIpInfo)),
    Remove((SiemIp, u8)),
    Replace(SlowGeoIpDataset),
}

#[derive(Debug, Clone)]
pub struct SlowGeoIpSynDataset {
    dataset: Arc<SlowGeoIpDataset>,
    comm: Sender<UpdateSlowGeoIp>,
}
impl SlowGeoIpSynDataset {
    pub fn new(dataset: Arc<SlowGeoIpDataset>, comm: Sender<UpdateSlowGeoIp>) -> Self {
        return Self { dataset, comm };
    }
    pub fn full_update(&self, dataset: SlowGeoIpDataset) {
        match self.comm.send(UpdateSlowGeoIp::Replace(dataset)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    /// This method must not be used with this dataset, because no source will give you accurate data to update this dataset. Maybe some firewalls, but updating the dataset with each log information is not a good idea.
    pub fn insert(&mut self, ip: SiemIp, net: u8, data: GeoIpInfo) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateSlowGeoIp::Add((ip, net, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, ip: &SiemIp) -> Option<GeoIpInfo> {
        // Todo improve with cached added IPs
        self.dataset.get(ip)
    }
}
#[derive(Debug)]
pub struct SlowGeoIpDataset {
    tree: sled::Db
}
impl Serialize for SlowGeoIpDataset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = serializer.serialize_struct("GeoIpDataset", 0)?;
        s.end()
    }
}
impl SlowGeoIpDataset {
    pub fn new() -> Self {
        let path = match std::env::var("SLOW_GEO_IP_LOCATION") {
            Ok(v) => v,
            Err(_) => format!("./slow_geo_ip"),
        };
        let tree = sled::open(&path).expect("open");
        return Self { tree };
    }
    pub fn insert(&mut self, ip: SiemIp, net: u8, data: GeoIpInfo) {
        let _ = self.tree.insert(Self::get_key(&ip, net), data);
    }
    fn get_key(ip: &SiemIp, net: u8) -> [u8; 18] {
        let mut ret = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, net];
        match ip {
            SiemIp::V4(v) => {
                let v = v & std::u32::MAX.checked_shl((32 - net) as u32).unwrap_or(0);
                let mut i = 0;
                for byt in v.to_be_bytes() {
                    ret[i] = byt;
                    i += 1;
                }
            },
            SiemIp::V6(v) => {
                let v  = v & std::u128::MAX.checked_shl((128 - net) as u32).unwrap_or(0);
                ret[17] = 1;
                let mut i = 0;
                for byt in v.to_be_bytes() {
                    ret[i] = byt;
                    i += 1;
                }
            },
        }
        ret
    }
    pub fn get(&self, ip: &SiemIp) -> Option<GeoIpInfo> {
        let (zeros, max_net) = match ip {
            SiemIp::V4(ip) => (ip.trailing_zeros() as u8, 32u8),
            SiemIp::V6(ip) => (ip.trailing_zeros() as u8, 128u8)
        };
        for net in zeros..max_net {
            let key = Self::get_key(ip, net);
            match self.tree.get(key) {
                Ok(v) => {
                    let v = match v {
                        None => continue,
                        Some(v) => v,
                    };
                    let geo_ip : GeoIpInfo = v.into();
                    return Some(geo_ip)
                },
                Err(err) => {
                    crate::warn!("Error getting value in SLED: {:?}", err);
                },
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::types::LogString;

    use super::*;
    #[test]
    fn geo_ip_should_find_ip() {
        let tmp = std::env::temp_dir().join("slow_geo_ip").to_string_lossy().to_string();
        std::env::set_var("SLOW_GEO_IP_LOCATION", &tmp[..]);
        let info = GeoIpInfo {
            city: LogString::Borrowed("LocalCity"),
            country: LogString::Borrowed("LocalCountry"),
            country_iso: LogString::Borrowed("LC"),
            isp: LogString::Borrowed("ISP"),
            latitude: 0.1,
            longitude: 0.2,
            asn: 1,
        };
        let mut dataset = SlowGeoIpDataset::new();
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
