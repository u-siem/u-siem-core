use std::borrow::Cow;
//use serde::ser::{Serializer, SerializeStruct};
use super::super::utilities::ip_utils::{ipv4_from_str, ipv4_to_str, ipv6_from_str, ipv6_to_str};
use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize, Debug)]
pub enum SiemField {
    /// A basic String field
    Text(Cow<'static, str>),
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
    /// Getting the IP address, the users logged in the system or other information.
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
}
impl SiemField {
    pub fn from_str<S>(val : S) -> SiemField where S: Into<Cow<'static, str>> {
        SiemField::Text(val.into())
    }
}

/// Genetares a User field content. Format: "user_domain|user_name".
/// We use the "|" character as to not to confuse with other formats like
/// the email "@" or windows domain "\\","/" 
pub fn generate_user_id(user_name : &str, user_domain : &str) -> String {
    format!("{}|{}",user_domain,user_name)
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
        }
    }
}

impl PartialEq for SiemField {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SiemField::Domain(v) | SiemField::User(v) | SiemField::AssetID(v) => match other {
                SiemField::Text(txt) => &v[..] == *txt,
                SiemField::IP(ip) => &v[..] == ip.to_string(),
                _ => false,
            },
            SiemField::Text(txt) => match other {
                SiemField::Domain(v) | SiemField::User(v) | SiemField::AssetID(v) => {
                    &v[..] == *txt
                },
                SiemField::IP(ip) => *txt == ip.to_string(),
                _ => *txt == other.to_string(),
            },
            SiemField::IP(ip) => match other {
                SiemField::Domain(v) | SiemField::User(v) | SiemField::AssetID(v) => {
                    &v[..] == ip.to_string()
                }
                SiemField::Text(txt) => ip.to_string() == *txt,
                _ => false,
            },
            _ => {
                match other {
                    SiemField::Domain(_) | SiemField::User(_) | SiemField::AssetID(_) => false,
                    SiemField::Text(txt) => other.to_string() == *txt,
                    SiemField::IP(_) => false,
                    _ => self.to_string() == other.to_string()
                }
            }
        }
        //self.to_string() == other.to_string()
    }
}

#[derive(Serialize, Debug, PartialEq,Clone)]
pub enum SiemIp {
    V4(u32),
    V6(u128),
}

impl SiemIp {
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
    pub fn from_ip_str<S>(val : S) -> Result<SiemIp,Cow<'static, str>> where S: Into<Cow<'static, str>> {
        let posible_ip = val.into();
        match ipv4_from_str(&posible_ip) {
            Ok(val) => Ok(SiemIp::V4(val)),
            Err(_) => match ipv6_from_str(&posible_ip) {
                Ok(val) => Ok(SiemIp::V6(val)),
                Err(_) => Err(Cow::Borrowed("Invalid IP value")),
            }
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
        let field_text = SiemField::Text(Cow::Borrowed("TEXT_001"));
        let field_domain = SiemField::Domain(String::from("TEXT_001"));
        assert_eq!(field_text, field_domain);
        let field_text = SiemField::Text(Cow::Borrowed("0.0.0.0"));
        let field_ip = SiemField::IP(SiemIp::V4(0));
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(Cow::Borrowed("123"));
        let field_ip = SiemField::U32(123);
        assert_eq!(field_text, field_ip);
        let field_ip = SiemField::U64(123);
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(Cow::Borrowed("123.456"));
        let field_ip = SiemField::F64(123.456);
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(Cow::Borrowed("User1234"));
        let field_ip = SiemField::User("User1234".to_string());
        assert_eq!(field_text, field_ip);
        let field_ip = SiemField::AssetID("User1234".to_string());
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(Cow::Borrowed("-1234"));
        let field_ip = SiemField::I64(-1234);
        assert_eq!(field_text, field_ip);
        let field_text = SiemField::Text(Cow::Borrowed("-1234"));
        let field_ip = SiemField::Date(-1234);
        assert_eq!(field_text, field_ip);
    }
}
