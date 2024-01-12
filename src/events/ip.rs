use std::fmt::Display;

use serde::{Deserialize, Serialize, Serializer};

use crate::prelude::{
    ip_utils::{
        ipv4_from_str, ipv4_to_str, ipv6_from_str, ipv6_to_str, is_local_ipv4, is_local_ipv6,
    },
    SiemField,
};

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum SiemIp {
    V4(u32),
    V6(u128),
}
impl PartialEq for SiemIp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SiemIp::V4(v1), SiemIp::V4(v2)) => v1 == v2,
            (SiemIp::V6(v1), SiemIp::V6(v2)) => v1 == v2,
            //TODO: IPv4 in IPV6
            _ => false,
        }
    }
}

impl SiemIp {
    pub fn is_local(&self) -> bool {
        match self {
            SiemIp::V4(ip) => is_local_ipv4(*ip),
            SiemIp::V6(ip) => is_local_ipv6(*ip),
        }
    }
    pub fn equals(&self, val: &str) -> bool {
        match self {
            SiemIp::V4(ip1) => match ipv4_from_str(val) {
                Ok(ip2) => *ip1 == ip2,
                Err(_) => false,
            },
            SiemIp::V6(ip1) => match ipv6_from_str(val) {
                Ok(ip2) => *ip1 == ip2,
                Err(_) => false,
            },
        }
    }
    pub fn from_ip_str(val: &str) -> Result<SiemIp, &'static str> {
        match ipv4_from_str(val) {
            Ok(val) => Ok(SiemIp::V4(val)),
            Err(_) => {
                let ip = ipv6_from_str(val)?;
                Ok(SiemIp::V6(ip))
            }
        }
    }
}
impl Serialize for SiemIp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&(&self.to_string())[..])
    }
}

impl From<[u32; 4]> for SiemIp {
    fn from(v: [u32; 4]) -> Self {
        Self::V4(
            ((v[0] & 0xff) << 24) + ((v[1] & 0xff) << 16) + ((v[2] & 0xff) << 8) + (v[3] & 0xff),
        )
    }
}
impl From<[u32; 16]> for SiemIp {
    fn from(v: [u32; 16]) -> Self {
        Self::V6(
            ((v[0] as u128 & 0xffu128) << 120)
                + ((v[1] as u128 & 0xffu128) << 112)
                + ((v[2] as u128 & 0xffu128) << 104)
                + ((v[3] as u128 & 0xffu128) << 96)
                + ((v[4] as u128 & 0xffu128) << 88)
                + ((v[5] as u128 & 0xffu128) << 80)
                + ((v[6] as u128 & 0xffu128) << 72)
                + ((v[7] as u128 & 0xffu128) << 64)
                + ((v[8] as u128 & 0xffu128) << 56)
                + ((v[9] as u128 & 0xffu128) << 48)
                + ((v[10] as u128 & 0xffu128) << 40)
                + ((v[11] as u128 & 0xffu128) << 32)
                + ((v[12] as u128 & 0xffu128) << 24)
                + ((v[13] as u128 & 0xffu128) << 16)
                + ((v[14] as u128 & 0xffu128) << 8)
                + (v[15] as u128 & 0xffu128),
        )
    }
}
impl From<&u32> for SiemIp {
    fn from(v: &u32) -> Self {
        Self::V4(*v)
    }
}
impl From<u32> for SiemIp {
    fn from(v: u32) -> Self {
        Self::V4(v)
    }
}
impl From<&u128> for SiemIp {
    fn from(v: &u128) -> Self {
        Self::V6(*v)
    }
}
impl From<u128> for SiemIp {
    fn from(v: u128) -> Self {
        Self::V6(v)
    }
}

impl<'a> TryFrom<&'a SiemField> for &'a SiemIp {
    type Error = &'static str;

    fn try_from(value: &SiemField) -> Result<&SiemIp, Self::Error> {
        match value {
            SiemField::IP(ip) => Ok(ip),
            _ => Err("Not an IP"),
        }
    }
}

impl TryFrom<SiemField> for SiemIp {
    type Error = &'static str;

    fn try_from(value: SiemField) -> Result<Self, Self::Error> {
        match value {
            SiemField::IP(ip) => Ok(ip),
            _ => Err("Not an IP"),
        }
    }
}

impl Display for SiemIp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ip = match self {
            SiemIp::V4(ip1) => ipv4_to_str(*ip1),
            SiemIp::V6(ip1) => ipv6_to_str(*ip1),
        };
        write!(f, "{}", ip)
    }
}
impl std::hash::Hash for SiemIp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SiemIp::V4(v) => v.hash(state),
            SiemIp::V6(v) => v.hash(state),
        }
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    fn test_equals_between_ips() {
        assert_eq!(SiemIp::V4(111), SiemIp::V4(111));
        assert_eq!(SiemIp::V6(111), SiemIp::V6(111));
        assert_eq!(Some(SiemIp::V6(111)), Some(SiemIp::V6(111)));
    }
    #[test]
    fn test_serialize_ip_field() {
        assert_eq!(SiemIp::V4(111).to_string(), "0.0.0.111");
    }

    #[test]
    fn from_u32_vec() {
        let ip: SiemIp = [192, 168, 1, 1].into();
        assert_eq!(ip.to_string(), "192.168.1.1");
    }

    #[test]
    fn from_u128_vec() {
        let ip: SiemIp = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into();
        assert_eq!(ip.to_string(), "::1");
    }
}
