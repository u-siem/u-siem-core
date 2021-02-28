use super::super::super::events::field::SiemIp;
use std::collections::BTreeMap;
use std::borrow::Cow;

pub struct GeoIpInfo {
    pub country : Cow<'static, str>,
    pub city : Cow<'static, str>,
    pub latitude : i32,
    pub longitude : i32,
    pub isp : Cow<'static, str>
}

pub struct GeoIpDataset {
    data4 : BTreeMap<u32,BTreeMap<u32, GeoIpInfo>>,
    data6 : BTreeMap<u32,BTreeMap<u128, GeoIpInfo>>,

}

impl GeoIpDataset {
    pub fn new() -> GeoIpDataset {
        return GeoIpDataset {
            data4 : BTreeMap::new(),
            data6 : BTreeMap::new()
        }
    }
    pub fn add_ip(&mut self, ip : SiemIp, net : u8, data : GeoIpInfo){
        match ip {
            SiemIp::V4(ip) => {
                let ip_net = ip & std::u32::MAX.checked_shl((32-net) as u32).unwrap_or(0);
                if self.data4.contains_key(&(net as u32)) {
                    match self.data4.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data);
                        },
                        None => {}
                    };
                }else{
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data4.insert(net as u32, new_net);
                }
            },
            SiemIp::V6(ip) => {
                let ip_net = ip & std::u128::MAX.checked_shl((128-net) as u32).unwrap_or(0);
                if self.data6.contains_key(&(net as u32)) {
                    match self.data6.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data);
                        },
                        None => {}
                    };
                }else{
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data6.insert(net as u32, new_net);
                }
            }
        }
    }
    pub fn lookup(&self, ip : SiemIp) -> Option<&GeoIpInfo> {
        match ip {
            SiemIp::V4(ip) => {
                let zeros = ip.trailing_zeros();
                for i in zeros..32 {
                    let ip_net = ip & std::u32::MAX.checked_shl(32-i).unwrap_or(0);
                    match self.data4.get(&i){
                        Some(map) => {
                            match map.get(&ip_net){
                                Some(v) => return Some(v),
                                None => {continue;}
                            }
                        },
                        None => {continue;}
                    }
                }
                None
            },
            SiemIp::V6(ip) => {
                let zeros = ip.trailing_zeros();
                for i in zeros..128 {
                    let ip_net = ip & std::u128::MAX.checked_shl(128-i).unwrap_or(0);
                    match self.data6.get(&i){
                        Some(map) => {
                            match map.get(&ip_net){
                                Some(v) => return Some(v),
                                None => {continue;}
                            }
                        },
                        None => {continue;}
                    }
                }
                None
            }
        }
    }
}

pub struct IpNetDataset {
    data4 : BTreeMap<u32, BTreeMap<u32, Cow<'static,str>>>,
    data6 : BTreeMap<u32, BTreeMap<u128, Cow<'static,str>>>,

}

impl IpNetDataset {
    pub fn new() -> IpNetDataset {
        return IpNetDataset {
            data4 : BTreeMap::new(),
            data6 : BTreeMap::new()
        }
    }
    pub fn add_ip(&mut self, ip : SiemIp, net : u8, data : Cow<'static,str>){
        match ip {
            SiemIp::V4(ip) => {
                let ip_net = ip & std::u32::MAX.checked_shl((32-net) as u32).unwrap_or(0);
                if self.data4.contains_key(&(net as u32)) {
                    match self.data4.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data);
                        },
                        None => {}
                    };
                }else{
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data4.insert(net as u32, new_net);
                }
            },
            SiemIp::V6(ip) => {
                let ip_net = ip & std::u128::MAX.checked_shl((128-net) as u32).unwrap_or(0);
                if self.data6.contains_key(&(net as u32)) {
                    match self.data6.get_mut(&(net as u32)) {
                        Some(dataset) => {
                            dataset.insert(ip_net, data);
                        },
                        None => {}
                    };
                }else{
                    let mut new_net = BTreeMap::new();
                    new_net.insert(ip_net, data);
                    self.data6.insert(net as u32, new_net);
                }
            }
        }
    }
    pub fn lookup(&self, ip : SiemIp) -> Option<&Cow<'static,str>> {
        match ip {
            SiemIp::V4(ip) => {
                let zeros = ip.trailing_zeros();
                for i in zeros..32 {
                    let ip_net = ip & std::u32::MAX.checked_shl(32-i).unwrap_or(0);
                    match self.data4.get(&i){
                        Some(map) => {
                            match map.get(&ip_net){
                                Some(v) => return Some(v),
                                None => {continue;}
                            }
                        },
                        None => {continue;}
                    }
                }
                None
            },
            SiemIp::V6(ip) => {
                let zeros = ip.trailing_zeros();
                for i in zeros..128 {
                    let ip_net = ip & std::u128::MAX.checked_shl(128-i).unwrap_or(0);
                    match self.data6.get(&i){
                        Some(map) => {
                            match map.get(&ip_net){
                                Some(v) => return Some(v),
                                None => {continue;}
                            }
                        },
                        None => {continue;}
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
        let mut dataset = IpNetDataset::new();
        dataset.add_ip(SiemIp::from_ip_str("192.168.1.1").unwrap(), 24, Cow::Borrowed("Local IP "));
        assert_eq!(dataset.lookup(SiemIp::from_ip_str("192.168.1.1").unwrap()), Some(&Cow::Borrowed("Local IP ")));
    }
}
