use crate::prelude::SiemIp;
use crate::prelude::types::LogString;
use chrono::NaiveDateTime;
use chrono::SecondsFormat;
use serde::Serialize;
use serde::Deserialize;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SiemField {
    #[default]
    Null,
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
    /// unsigned number with 64 bits
    U64(u64),
    /// signed number with 64 bits
    I64(i64),
    /// decimal number with 64 bits
    F64(f64),
    ///A date in a decimal number format with 64 bits
    Date(i64),
    Array(Vec<LogString>),
    Path(PathBuf)
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
            SiemField::U64(txt) => write!(f, "{}", txt.to_string()),
            SiemField::I64(txt) => write!(f, "{}", txt.to_string()),
            SiemField::F64(txt) => write!(f, "{}", txt.to_string()),
            SiemField::Date(ts_millis) => {
                let dt = NaiveDateTime::from_timestamp_millis(*ts_millis).unwrap_or_default();
                write!(f, "{}", dt.and_utc().to_rfc3339_opts(SecondsFormat::Millis, true))
            },
            SiemField::Array(v) => write!(f, "[{}]", v.join(",")),
            _ => write!(f, "")
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

impl std::hash::Hash for SiemField {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SiemField::Text(v) => v.hash(state),
            SiemField::IP(v) => v.hash(state),
            SiemField::Domain(v) => v.hash(state),
            SiemField::User(v) => v.hash(state),
            SiemField::AssetID(v) => v.hash(state),
            SiemField::U64(v) => v.hash(state),
            SiemField::I64(v) => v.hash(state),
            SiemField::F64(v) => (*v as u64).hash(state),
            SiemField::Date(v) => v.hash(state),
            SiemField::Array(v) => v.hash(state),
            _ => {}
        }
    }
}
impl SiemField {
    pub fn from_str<S>(val: S) -> SiemField
    where
        S: Into<LogString>,
    {
        SiemField::Text(val.into())
    }

    pub fn eq_ignore_ascii_case(&self, other : &Self) -> bool {
        let self_txt : &str = match self.try_into() {
            Ok(v) => v,
            Err(_) => return false
        };
        match other {
            SiemField::Text(v) => v.eq_ignore_ascii_case(self_txt),
            SiemField::User(v) => v.eq_ignore_ascii_case(self_txt),
            SiemField::Domain(v) => v.eq_ignore_ascii_case(self_txt),
            SiemField::AssetID(v) => v.eq_ignore_ascii_case(self_txt),
            _ => false
        }
    }
    pub fn eq_ignore_ascii_case_str(&self, other : &str) -> bool {
        let self_txt : &str = match self.try_into() {
            Ok(v) => v,
            Err(_) => return false
        };
        other.eq_ignore_ascii_case(self_txt)
    }
    pub fn contains_str(&self, txt: &str) -> bool {
        match self {
            SiemField::Text(v) => v.contains(txt),
            SiemField::User(v) => v.contains(txt),
            SiemField::Domain(v) => v.contains(txt),
            SiemField::AssetID(v) => v.contains(&v[..]),
            _ => false,
        }
    }
    pub fn contains(&self, other : &Self) -> bool {
        match other {
            SiemField::Text(v) => self.contains_str(&v[..]),
            SiemField::User(v) => self.contains_str(&v[..]),
            SiemField::Domain(v) => self.contains_str(&v[..]),
            SiemField::AssetID(v) => self.contains_str(&v[..]),
            _ => false
        }
    }
    pub fn is_text(&self) -> bool {
        match self {
            SiemField::Array(_) => true,
            SiemField::Text(_) => true,
            SiemField::User(_) => true,
            SiemField::Path(_) => true,
            SiemField::AssetID(_) => true,
            SiemField::Domain(_) => true,
            _ =>false
        }
    }
    pub fn is_numeric(&self) -> bool {
        match self {
            SiemField::U64(_) => true,
            SiemField::I64(_) => true,
            SiemField::F64(_) => true,
            _ => false
        }
    }
}

impl<'a> TryInto<&'a str> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<&'a str, Self::Error> {
        match self {
            SiemField::Text(v) => Ok(&v[..]),
            SiemField::Domain(v) => Ok(&v[..]),
            SiemField::User(v) => Ok(&v[..]),
            SiemField::AssetID(v) => Ok(&v[..]),
            _ => Err("Invalid text type")
        }
    }
}

impl<'a> TryInto<LogString> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<LogString, Self::Error> {
        match self {
            SiemField::Text(v) => Ok(v.clone()),
            SiemField::Domain(v) => Ok(LogString::Owned(v.to_string())),
            SiemField::User(v) => Ok(LogString::Owned(v.to_string())),
            SiemField::AssetID(v) => Ok(LogString::Owned(v.to_string())),
            _ => Err("Invalid type")
        }
    }
}
impl<'a> TryInto<&'a LogString> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<&'a LogString, Self::Error> {
        match self {
            SiemField::Text(v) => Ok(v),
            _ => Err("Invalid type")
        }
    }
}

impl<'a> TryInto<&'a Vec<LogString>> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<&'a Vec<LogString>, Self::Error> {
        match self {
            SiemField::Array(v) => Ok(v),
            _ => Err("Invalid type")
        }
    }
}

impl<'a> TryInto<Vec<LogString>> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<Vec<LogString>, Self::Error> {
        let value = match self {
            SiemField::Array(v) => return Ok(v.clone()),
            SiemField::AssetID(v) => LogString::Owned(v.clone()),
            SiemField::Text(v) => v.clone(),
            SiemField::Domain(v) => LogString::Owned(v.clone()),
            SiemField::User(v) => LogString::Owned(v.clone()),
            SiemField::I64(v) => LogString::Owned(v.to_string()),
            SiemField::F64(v) => LogString::Owned(v.to_string()),
            SiemField::U64(v) => LogString::Owned(v.to_string()),
            SiemField::Date(v) => LogString::Owned(v.to_string()),
            SiemField::IP(v) => LogString::Owned(v.to_string()),
            SiemField::Null => LogString::Borrowed(""),
            SiemField::Path(v) => LogString::Owned(v.to_string_lossy().to_string()),
        };
        Ok(vec![value])
    }
}

impl<'a> TryInto<u64> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<u64, Self::Error> {
        Ok(match self {
            SiemField::F64(v) => *v as u64,
            SiemField::I64(v) => *v as u64,
            SiemField::U64(v) => *v,
            SiemField::Date(v) => *v as u64,
            _ => return Err("Invalid type")
        })
    }
}
impl<'a> TryInto<i64> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<i64, Self::Error> {
        Ok(match self {
            SiemField::F64(v) => *v as i64,
            SiemField::I64(v) => *v as i64,
            SiemField::U64(v) => *v as i64,
            SiemField::Date(v) => *v as i64,
            _ => return Err("Invalid type")
        })
    }
}
impl<'a> TryInto<f64> for &'a SiemField {
    type Error = &'static str;

    fn try_into(self) -> Result<f64, Self::Error> {
        Ok(match self {
            SiemField::F64(v) => *v as f64,
            SiemField::I64(v) => *v as f64,
            SiemField::U64(v) => *v as f64,
            SiemField::Date(v) => *v as f64,
            _ => return Err("Invalid type")
        })
    }
}

impl<'a> TryInto<SiemIp> for &'a SiemField {
    type Error = &'static str;
    fn try_into(self) -> Result<SiemIp, Self::Error> {
        Ok(match self {
            SiemField::Text(v) => SiemIp::from_ip_str(&v).map_err(|_e| "Invalud ip format")?,
            SiemField::IP(v) => v.clone(),
            _ => return Err("Type cannot be converted to Ip")
        })
    }
}

impl From<&'static str> for SiemField {
    fn from(v : &'static str) -> SiemField {
        SiemField::Text(LogString::Borrowed(v))
    }
}
impl From<&String> for SiemField {
    fn from(v : &String) -> SiemField {
        SiemField::Text(LogString::Owned(v.to_string()))
    }
}
impl From<String> for SiemField {
    fn from(v : String) -> SiemField {
        SiemField::Text(LogString::Owned(v))
    }
}
impl From<LogString> for SiemField {
    fn from(v : LogString) -> SiemField {
        SiemField::Text(v)
    }
}
impl From<&LogString> for SiemField {
    fn from(v : &LogString) -> SiemField {
        SiemField::Text(v.clone())
    }
}

impl From<&u64> for SiemField {
    fn from(v : &u64) -> SiemField {
        SiemField::U64(*v)
    }
}
impl From<u64> for SiemField {
    fn from(v : u64) -> SiemField {
        SiemField::U64(v)
    }
}
impl From<&i64> for SiemField {
    fn from(v : &i64) -> SiemField {
        SiemField::I64(*v)
    }
}
impl From<i64> for SiemField {
    fn from(v : i64) -> SiemField {
        SiemField::I64(v)
    }
}

impl From<&f64> for SiemField {
    fn from(v : &f64) -> SiemField {
        SiemField::F64(*v)
    }
}
impl From<f64> for SiemField {
    fn from(v : f64) -> SiemField {
        SiemField::F64(v)
    }
}
impl From<SiemIp> for SiemField {
    fn from(v : SiemIp) -> SiemField {
        SiemField::IP(v)
    }
}
impl From<&SiemIp> for SiemField {
    fn from(v : &SiemIp) -> SiemField {
        SiemField::IP(*v)
    }
}
impl From<Vec<LogString>> for SiemField {
    fn from(v : Vec<LogString>) -> SiemField {
        SiemField::Array(v)
    }
}
impl From<&Vec<LogString>> for SiemField {
    fn from(v : &Vec<LogString>) -> SiemField {
        SiemField::Array(v.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::SiemIp;

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
        let field_ip = SiemField::U64(123);
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
        let date = SiemField::Date(0);
        assert_eq!("1970-01-01T00:00:00.000Z", date.to_string());
    }
}
