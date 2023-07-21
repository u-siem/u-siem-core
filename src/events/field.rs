use crate::prelude::ip_utils::{is_local_ipv4, is_local_ipv6};
use crate::prelude::types::LogString;

use super::super::utilities::ip_utils::{ipv4_from_str, ipv4_to_str, ipv6_from_str, ipv6_to_str};
use serde::Serialize;
use serde::{ser::Serializer, Deserialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SiemField {
    /// A basic String field
    Text(LogString),
    /// IPv4 or IPv6
    IP(SiemIp),
    //Domain like contoso.com
    Domain(String),
    User(String),
    ///This is a special field. Uniquely identifies an asset like a system, a
    /// computer or a mobile phone. Reason: the network is dynamic, the IP address
    /// is not fixed certain devices and the hostname of a system can be changed.
    ///
    /// This field should be used with a dataset to recover information about an asset
    /// during the enchance phase:
    /// Getting the IP address, the users logged in the system or another information.
    ///
    /// Can be multiple AssetsID associated with the same event because multiple virtual
    /// machines can be running in the same asset.
    AssetID(String),
    /// unsigned number with 32 bits
    U32(u32),
    /// unsigned number with 64 bits
    U64(u64),
    /// signed number with 64 bits
    I64(i64),
    /// decimal number with 64 bits
    F64(f64),
    ///A date in a decimal number format with 64 bits
    Date(i64),
    Array(Vec<LogString>),
}
impl SiemField {
    pub fn from_str<S>(val: S) -> SiemField
    where
        S: Into<LogString>,
    {
        SiemField::Text(val.into())
    }
}

/// Genetares a User field content. Format: "user_domain|user_name".
/// We use the "|" character as to not to confuse with other formats like
/// the email "@" or windows domain "\\","/"
pub fn generate_user_id(user_name: &str, user_domain: &str) -> String {
    format!("{}|{}", user_domain, user_name)
}

impl Display for SiemField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SiemField::Text(txt) => write!(f, "{}", txt.to_string()),
            SiemField::IP(txt) => write!(f, "{}", txt.to_string()),
            SiemField::Domain(txt) => write!(f, "{}", txt.to_string()),
            SiemField::User(txt) => write!(f, "{}", txt.to_string()),
            SiemField::AssetID(txt) => write!(f, "{}", txt.to_string()),
            SiemField::U32(txt) => write!(f, "{}", txt.to_string()),
            SiemField::U64(txt) => write!(f, "{}", txt.to_string()),
            SiemField::I64(txt) => write!(f, "{}", txt.to_string()),
            SiemField::F64(txt) => write!(f, "{}", txt.to_string()),
            SiemField::Date(txt) => write!(f, "{}", txt.to_string()),
            SiemField::Array(v) => write!(f, "[{}]", v.join(",")),
        }
    }
}

impl PartialEq for SiemField {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SiemField::Domain(v) | SiemField::User(v) | SiemField::AssetID(v) => match other {
                SiemField::Text(txt) => &v[..] == *txt,
                SiemField::IP(ip) => &v[..] == ip.to_string(),
                SiemField::User(txt) => &v[..] == *txt,
                SiemField::Domain(txt) => &v[..] == *txt,
                SiemField::AssetID(txt) => &v[..] == *txt,
                _ => false,
            },
            SiemField::Text(txt) => match other {
                SiemField::Domain(v) | SiemField::User(v) | SiemField::AssetID(v) => &v[..] == *txt,
                SiemField::IP(ip) => *txt == ip.to_string(),
                _ => *txt == other.to_string(),
            },
            SiemField::IP(ip) => match other {
                SiemField::Domain(v) | SiemField::User(v) | SiemField::AssetID(v) => {
                    &v[..] == ip.to_string()
                }
                SiemField::Text(txt) => ip.to_string() == *txt,
                SiemField::IP(ip2) => ip == ip2,
                _ => false,
            },
            _ => match other {
                SiemField::Domain(_) | SiemField::User(_) | SiemField::AssetID(_) => false,
                SiemField::Text(txt) => other.to_string() == *txt,
                SiemField::IP(_) => false,
                _ => self.to_string() == other.to_string(),
            },
        }
        //self.to_string() == other.to_string()
    }
}

#[derive(Deserialize, Debug, Clone)]
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
                Ok(ip2) => return *ip1 == ip2,
                Err(_) => false,
            },
            SiemIp::V6(ip1) => match ipv6_from_str(val) {
                Ok(ip2) => return *ip1 == ip2,
                Err(_) => false,
            },
        }
    }
    pub fn from_ip_str(val: &str) -> Result<SiemIp, LogString> {
        match ipv4_from_str(&val) {
            Ok(val) => Ok(SiemIp::V4(val)),
            Err(_) => match ipv6_from_str(&val) {
                Ok(val) => Ok(SiemIp::V6(val)),
                Err(_) => Err(LogString::Borrowed("Invalid IP value")),
            },
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equals_between_fields() {
        let field_text = SiemField::Text(LogString::Borrowed("TEXT_001"));
        let field_domain = SiemField::Domain(String::from("TEXT_001"));
        assert_eq!(field_text, field_domain);
        let field_text = SiemField::Text(LogString::Borrowed("0.0.0.0"));
        let field_ip = SiemField::IP(SiemIp::V4(0));
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(LogString::Borrowed("123"));
        let field_ip = SiemField::U32(123);
        assert_eq!(field_text, field_ip);
        let field_ip = SiemField::U64(123);
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(LogString::Borrowed("123.456"));
        let field_ip = SiemField::F64(123.456);
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(LogString::Borrowed("User1234"));
        let field_ip = SiemField::User("User1234".to_string());
        assert_eq!(field_text, field_ip);
        let field_ip = SiemField::AssetID("User1234".to_string());
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(LogString::Borrowed("-1234"));
        let field_ip = SiemField::I64(-1234);
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(LogString::Borrowed("-1234"));
        let field_ip = SiemField::Date(-1234);
        assert_eq!(field_text, field_ip);
    }
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
