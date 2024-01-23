pub mod calendar;
pub mod geo_ip;
pub mod holder;
pub mod i18n;
pub mod ip_map;
pub mod ip_map_list;
pub mod ip_net;
pub mod ip_set;
pub mod rules;
pub mod text_map;
pub mod text_map_list;
pub mod text_set;

use crate::prelude::types::LogString;
use calendar::{CalendarSynDataset, UpdateCalendar};
use geo_ip::{GeoIpSynDataset, UpdateGeoIp};
use i18n::{I18nSynDataset, UpdateI18n};
use ip_map::{IpMapSynDataset, UpdateIpMap};
use ip_map_list::{IpMapListSynDataset, UpdateIpMapList};
use ip_net::{IpNetSynDataset, UpdateNetIp};
use ip_set::{IpSetSynDataset, UpdateIpSet};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use text_map::{TextMapSynDataset, UpdateTextMap};
use text_map_list::{TextMapListSynDataset, UpdateTextMapList};
use text_set::{TextSetSynDataset, UpdateTextSet};

/// Commonly used datasets. They are filled with the information extracted form logs, from the CMDB, from user commands or from repetitive Task like GeoIP.
/// Dataset are used, but not exclusivally, in the enrichment phase.
///
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SiemDataset {
    /// Correlation Rules that can be updated in real-time
    CorrelationRules(GeoIpSynDataset),
    /// Map IP to country, city, latitude and longitude
    GeoIp(GeoIpSynDataset),
    /// IP associated with a MAC address
    IpMac(IpMapSynDataset),
    /// IP associated with a resolved domain
    IpDNS(IpMapListSynDataset),
    /// MAC address associated with a Hostname
    MacHost(TextMapSynDataset),
    /// Hostname associated with a username
    HostUser(TextMapSynDataset),
    /// List of IPs in the block list
    BlockIp(IpSetSynDataset),
    /// List of domain in the block list
    BlockDomain(TextSetSynDataset),
    /// List of email senders in the block list
    BlockEmailSender(TextSetSynDataset),
    /// List of countries in the block list
    BlockCountry(TextSetSynDataset),
    /// Association of hostname with a vulnerability.
    HostVulnerable(TextMapListSynDataset),
    /// Tag each user with roles => user.roles = [vip, admin, extern, guest, director, super_user, local_user]
    UserTag(TextMapListSynDataset),
    /// Tag each host with categories => [web_server, sec_related, critical, ad_related, net_related]
    AssetTag(TextMapListSynDataset),
    /// Cloud service => Office 365, G Suit ...
    IpCloudService(IpNetSynDataset),
    /// Cloud Provider => Azure, Google Cloud, AWS
    IpCloudProvider(IpNetSynDataset),
    /// User associated with a headquarter
    UserHeadquarters(TextMapSynDataset),
    /// IP net associated with a headquarter
    IpHeadquarters(IpNetSynDataset),
    /// Working hours of each headquarter
    HeadquartersWorkingHours,
    /// User custom dataset IP_NET => Text
    CustomMapIpNet((LogString, IpNetSynDataset)),
    /// User custom dataset Text => Text
    CustomMapText((LogString, TextMapSynDataset)),
    /// User custom dataset Text => Text
    CustomMapTextList((LogString, TextMapListSynDataset)),
    /// User custom dataset IP list
    CustomIpList((LogString, IpSetSynDataset)),
    /// User custom dataset IP list
    CustomIpMap((LogString, IpMapSynDataset)),
    /// User custom dataset Text list
    CustomTextList((LogString, TextSetSynDataset)),
    /// Mantaince Calendar
    MantainceCalendar(CalendarSynDataset),
    /// Configuration of components. Allows modifications of component parameters in real time.
    Configuration(TextMapSynDataset),
    /// Internacionalization of SIEM texts
    I18n(I18nSynDataset),
    /// Secret store. A component will only be able to access his own secrets.
    Secrets((LogString, TextMapSynDataset)),
}

impl SiemDataset {
    /// Creates an empty CustomMapIpNet. Used only for testing.
    pub fn empty_custom_map_ip_net<S: Into<LogString>>(name: S) -> Self {
        SiemDataset::CustomMapIpNet((name.into(), IpNetSynDataset::empty()))
    }
    /// Creates an empty CustomMapText. Used only for testing.
    pub fn empty_custom_map_text<S: Into<LogString>>(name: S) -> Self {
        SiemDataset::CustomMapText((name.into(), TextMapSynDataset::empty()))
    }
    /// Creates an empty CustomMapTextList. Used only for testing.
    pub fn empty_custom_map_text_list<S: Into<LogString>>(name: S) -> Self {
        SiemDataset::CustomMapTextList((name.into(), TextMapListSynDataset::empty()))
    }
    /// Creates an empty CustomIpList. Used only for testing.
    pub fn empty_custom_ip_list<S: Into<LogString>>(name: S) -> Self {
        SiemDataset::CustomIpList((name.into(), IpSetSynDataset::empty()))
    }
    /// Creates an empty CustomIpMap. Used only for testing.
    pub fn empty_custom_ip_map<S: Into<LogString>>(name: S) -> Self {
        SiemDataset::CustomIpMap((name.into(), IpMapSynDataset::empty()))
    }
    /// Creates an empty CustomTextList. Used only for testing.
    pub fn empty_custom_ip_text_list<S: Into<LogString>>(name: S) -> Self {
        SiemDataset::CustomTextList((name.into(), TextSetSynDataset::empty()))
    }
}

/// Tries to cast a SiemDataset into a CustomMapIpNet with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_map_ip_net_ref, ip_net::IpNetSynDataset};
/// let dataset = SiemDataset::empty_custom_map_ip_net("MyDataset");
/// let casted_dataset : &IpNetSynDataset = try_to_custom_map_ip_net_ref(&dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_map_ip_net_ref<'a>(
    dataset: &'a SiemDataset,
    name: &str,
) -> Result<&'a IpNetSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomMapIpNet((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomMapIpNet with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_map_ip_net, ip_net::IpNetSynDataset};
/// let dataset = SiemDataset::empty_custom_map_ip_net("MyDataset");
/// let casted_dataset : IpNetSynDataset = try_to_custom_map_ip_net(dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_map_ip_net(
    dataset: SiemDataset,
    name: &str,
) -> Result<IpNetSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomMapIpNet((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomMapText with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_map_text_ref, text_map::TextMapSynDataset};
/// let dataset = SiemDataset::empty_custom_map_text("MyDataset");
/// let casted_dataset : &TextMapSynDataset = try_to_custom_map_text_ref(&dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_map_text_ref<'a>(
    dataset: &'a SiemDataset,
    name: &str,
) -> Result<&'a TextMapSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomMapText((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomMapText with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_map_text, text_map::TextMapSynDataset};
/// let dataset = SiemDataset::empty_custom_map_text("MyDataset");
/// let casted_dataset : TextMapSynDataset = try_to_custom_map_text(dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_map_text(
    dataset: SiemDataset,
    name: &str,
) -> Result<TextMapSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomMapText((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomMapTextList with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_map_text_list_ref, text_map_list::TextMapListSynDataset};
/// let dataset = SiemDataset::empty_custom_map_text_list("MyDataset");
/// let casted_dataset : &TextMapListSynDataset = try_to_custom_map_text_list_ref(&dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_map_text_list_ref<'a>(
    dataset: &'a SiemDataset,
    name: &str,
) -> Result<&'a TextMapListSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomMapTextList((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomMapTextList with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_map_text_list, text_map_list::TextMapListSynDataset};
/// let dataset = SiemDataset::empty_custom_map_text_list("MyDataset");
/// let casted_dataset : TextMapListSynDataset = try_to_custom_map_text_list(dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_map_text_list(
    dataset: SiemDataset,
    name: &str,
) -> Result<TextMapListSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomMapTextList((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}

/// Tries to cast a SiemDataset into a CustomIpList with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_ip_list_ref, ip_set::IpSetSynDataset};
/// let dataset = SiemDataset::empty_custom_ip_list("MyDataset");
/// let casted_dataset : &IpSetSynDataset = try_to_custom_ip_list_ref(&dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_ip_list_ref<'a>(
    dataset: &'a SiemDataset,
    name: &str,
) -> Result<&'a IpSetSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomIpList((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomIpList with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_ip_list, ip_set::IpSetSynDataset};
/// let dataset = SiemDataset::empty_custom_ip_list("MyDataset");
/// let casted_dataset : IpSetSynDataset = try_to_custom_ip_list(dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_ip_list(
    dataset: SiemDataset,
    name: &str,
) -> Result<IpSetSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomIpList((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomIpMap with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_ip_map_ref, ip_map::IpMapSynDataset};
/// let dataset = SiemDataset::empty_custom_ip_map("MyDataset");
/// let casted_dataset : &IpMapSynDataset = try_to_custom_ip_map_ref(&dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_ip_map_ref<'a>(
    dataset: &'a SiemDataset,
    name: &str,
) -> Result<&'a IpMapSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomIpMap((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomIpMap with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_ip_map, ip_map::IpMapSynDataset};
/// let dataset = SiemDataset::empty_custom_ip_map("MyDataset");
/// let casted_dataset : IpMapSynDataset = try_to_custom_ip_map(dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_ip_map(
    dataset: SiemDataset,
    name: &str,
) -> Result<IpMapSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomIpMap((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomIpMap with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_text_list_ref, text_set::TextSetSynDataset};
/// let dataset = SiemDataset::empty_custom_ip_text_list("MyDataset");
/// let casted_dataset : &TextSetSynDataset = try_to_custom_text_list_ref(&dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_text_list_ref<'a>(
    dataset: &'a SiemDataset,
    name: &str,
) -> Result<&'a TextSetSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomTextList((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}
/// Tries to cast a SiemDataset into a CustomIpMap with the defined name
///
/// # Example
///
/// ```rust
/// use usiem::components::dataset::{SiemDataset, try_to_custom_text_list, text_set::TextSetSynDataset};
/// let dataset = SiemDataset::empty_custom_ip_text_list("MyDataset");
/// let casted_dataset : TextSetSynDataset = try_to_custom_text_list(dataset, "MyDataset").unwrap();
/// ```
pub fn try_to_custom_text_list(
    dataset: SiemDataset,
    name: &str,
) -> Result<TextSetSynDataset, &'static str> {
    match dataset {
        SiemDataset::CustomTextList((n, dataset)) => {
            if n == name {
                Ok(dataset)
            } else {
                Err("Cannot cast dataset")
            }
        }
        _ => Err("Cannot cast dataset"),
    }
}

impl TryFrom<(SiemDatasetType, GeoIpSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, GeoIpSynDataset)) -> Result<Self, Self::Error> {
        if value.0 == SiemDatasetType::GeoIp {
            Err("GeoIPSynDataset is only valid for GeoIP dataset!")
        } else {
            Ok(SiemDataset::GeoIp(value.1))
        }
    }
}
impl TryFrom<SiemDataset> for GeoIpSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        if let SiemDataset::GeoIp(geoip) = value {
            Ok(geoip)
        } else {
            Err("GeoIPSynDataset is only valid for GeoIP dataset!")
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a GeoIpSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        if let SiemDataset::GeoIp(geoip) = value {
            Ok(geoip)
        } else {
            Err("GeoIPSynDataset is only valid for GeoIP dataset!")
        }
    }
}

impl TryFrom<SiemDataset> for I18nSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        if let SiemDataset::I18n(v) = value {
            Ok(v)
        } else {
            Err("I18nSynDataset is only valid for I18n dataset!")
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a I18nSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        if let SiemDataset::I18n(v) = value {
            Ok(v)
        } else {
            Err("I18nSynDataset is only valid for I18n dataset!")
        }
    }
}

impl TryFrom<(SiemDatasetType, IpMapSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpMapSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::IpMac => Ok(SiemDataset::IpMac(value.1)),
            SiemDatasetType::CustomIpMap(name) => {
                Ok(SiemDataset::CustomIpMap((name.clone(), value.1)))
            }
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for IpMapSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::IpMac(v) => Ok(v),
            SiemDataset::CustomIpMap((_name, v)) => Ok(v),
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a IpMapSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::IpMac(v) => Ok(v),
            SiemDataset::CustomIpMap((_name, v)) => Ok(v),
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, TextMapSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, TextMapSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::MacHost => Ok(SiemDataset::MacHost(value.1)),
            SiemDatasetType::HostUser => Ok(SiemDataset::HostUser(value.1)),
            SiemDatasetType::UserHeadquarters => Ok(SiemDataset::UserHeadquarters(value.1)),
            SiemDatasetType::Configuration => Ok(SiemDataset::Configuration(value.1)),
            SiemDatasetType::CustomMapText(name) => {
                Ok(SiemDataset::CustomMapText((name.clone(), value.1)))
            }
            SiemDatasetType::Secrets(name) => Ok(SiemDataset::Secrets((name.clone(), value.1))),
            _ => Err("TextMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for TextMapSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::MacHost(v) => Ok(v),
            SiemDataset::HostUser(v) => Ok(v),
            SiemDataset::UserHeadquarters(v) => Ok(v),
            SiemDataset::Configuration(v) => Ok(v),
            SiemDataset::CustomMapText((_name, v)) => Ok(v),
            SiemDataset::Secrets((_name, v)) => Ok(v),
            _ => Err("TextMapSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a TextMapSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::MacHost(v) => Ok(v),
            SiemDataset::HostUser(v) => Ok(v),
            SiemDataset::UserHeadquarters(v) => Ok(v),
            SiemDataset::Configuration(v) => Ok(v),
            SiemDataset::CustomMapText((_name, v)) => Ok(v),
            SiemDataset::Secrets((_name, v)) => Ok(v),
            _ => Err("TextMapSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, IpMapListSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpMapListSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::IpDNS => Ok(SiemDataset::IpDNS(value.1)),
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for IpMapListSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::IpDNS(v) => Ok(v),
            _ => Err("IpMapListSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a IpMapListSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::IpDNS(v) => Ok(v),
            _ => Err("IpMapListSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, IpSetSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpSetSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::BlockIp => Ok(SiemDataset::BlockIp(value.1)),
            SiemDatasetType::CustomIpList(name) => {
                Ok(SiemDataset::CustomIpList((name.clone(), value.1)))
            }
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for IpSetSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::BlockIp(v) => Ok(v),
            SiemDataset::CustomIpList((_name, v)) => Ok(v),
            _ => Err("IpSetSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a IpSetSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::BlockIp(v) => Ok(v),
            SiemDataset::CustomIpList((_name, v)) => Ok(v),
            _ => Err("IpSetSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, TextSetSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, TextSetSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::BlockDomain => Ok(SiemDataset::BlockDomain(value.1)),
            SiemDatasetType::BlockEmailSender => Ok(SiemDataset::BlockEmailSender(value.1)),
            SiemDatasetType::BlockCountry => Ok(SiemDataset::BlockCountry(value.1)),
            SiemDatasetType::CustomTextList(name) => {
                Ok(SiemDataset::CustomTextList((name.clone(), value.1)))
            }
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for TextSetSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::BlockDomain(v) => Ok(v),
            SiemDataset::BlockEmailSender(v) => Ok(v),
            SiemDataset::BlockCountry(v) => Ok(v),
            SiemDataset::CustomTextList((_name, v)) => Ok(v),
            _ => Err("TextSetSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a TextSetSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::BlockDomain(v) => Ok(v),
            SiemDataset::BlockEmailSender(v) => Ok(v),
            SiemDataset::BlockCountry(v) => Ok(v),
            SiemDataset::CustomTextList((_name, v)) => Ok(v),
            _ => Err("TextSetSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, TextMapListSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, TextMapListSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::HostVulnerable => Ok(SiemDataset::HostVulnerable(value.1)),
            SiemDatasetType::UserTag => Ok(SiemDataset::UserTag(value.1)),
            SiemDatasetType::AssetTag => Ok(SiemDataset::AssetTag(value.1)),
            SiemDatasetType::CustomMapTextList(name) => {
                Ok(SiemDataset::CustomMapTextList((name.clone(), value.1)))
            }
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for TextMapListSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::HostVulnerable(v) => Ok(v),
            SiemDataset::UserTag(v) => Ok(v),
            SiemDataset::AssetTag(v) => Ok(v),
            SiemDataset::CustomMapTextList((_name, v)) => Ok(v),
            _ => Err("TextMapListSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a TextMapListSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::HostVulnerable(v) => Ok(v),
            SiemDataset::UserTag(v) => Ok(v),
            SiemDataset::AssetTag(v) => Ok(v),
            SiemDataset::CustomMapTextList((_name, v)) => Ok(v),
            _ => Err("TextMapListSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, IpNetSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpNetSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::IpCloudService => Ok(SiemDataset::IpCloudService(value.1)),
            SiemDatasetType::IpCloudProvider => Ok(SiemDataset::IpCloudProvider(value.1)),
            SiemDatasetType::IpHeadquarters => Ok(SiemDataset::IpHeadquarters(value.1)),
            SiemDatasetType::CustomMapIpNet(name) => {
                Ok(SiemDataset::CustomMapIpNet((name.clone(), value.1)))
            }
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}
impl TryFrom<SiemDataset> for IpNetSynDataset {
    type Error = &'static str;

    fn try_from(value: SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::IpCloudService(v) => Ok(v),
            SiemDataset::IpCloudProvider(v) => Ok(v),
            SiemDataset::IpHeadquarters(v) => Ok(v),
            SiemDataset::CustomMapIpNet((_name, v)) => Ok(v),
            _ => Err("IpNetSynDataset not valid for this type"),
        }
    }
}
impl<'a> TryFrom<&'a SiemDataset> for &'a IpNetSynDataset {
    type Error = &'static str;

    fn try_from(value: &'a SiemDataset) -> Result<Self, Self::Error> {
        match value {
            SiemDataset::IpCloudService(v) => Ok(v),
            SiemDataset::IpCloudProvider(v) => Ok(v),
            SiemDataset::IpHeadquarters(v) => Ok(v),
            SiemDataset::CustomMapIpNet((_name, v)) => Ok(v),
            _ => Err("IpNetSynDataset not valid for this type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SiemDatasetType {
    /// Map IP to country, city, latitude and longitude
    GeoIp,
    /// IP associated with a MAC address
    IpMac,
    /// IP associated with a resolved domain
    IpDNS,
    /// MAC address associated with a Hostname
    MacHost,
    /// Hostname associated with a username
    HostUser,
    /// List of IPs in the block list
    BlockIp,
    /// List of domain in the block list
    BlockDomain,
    /// List of email senders in the block list
    BlockEmailSender,
    /// List of countries in the block list
    BlockCountry,
    /// Tag each user with roles => user.roles = [vip, admin, extern, guest, director, super_user, local_user]
    UserTag,
    /// Tag each host with categories => [web_server, sec_related, critical, ad_related, net_related]
    AssetTag,
    /// Cloud service => Office 365, G Suit ...
    IpCloudService,
    /// Cloud Provider => Azure, Google Cloud, AWS
    IpCloudProvider,
    /// User associated with a headquarter
    UserHeadquarters,
    /// IP net associated with a headquarter
    IpHeadquarters,
    /// Working hours of each headquarter
    HeadquartersWorkingHours,
    /// Vulnerabilities on a computer  
    HostVulnerable,
    CorrelationRules,
    /// User custom dataset IP_NET => Text
    CustomMapIpNet(LogString),
    /// User custom dataset Text => Text
    CustomMapText(LogString),
    /// User custom dataset Text => Text[]
    CustomMapTextList(LogString),
    /// User custom dataset IP list
    CustomIpList(LogString),
    CustomIpMap(LogString),
    /// User custom dataset Text list
    CustomTextList(LogString),
    /// Mantaince Calendar
    MantainceCalendar,
    Configuration,
    Secrets(LogString),
    /// Internacionalization
    I18n,
}

impl SiemDataset {
    pub fn dataset_type(&self) -> SiemDatasetType {
        match self {
            SiemDataset::CorrelationRules(_) => SiemDatasetType::CorrelationRules,
            SiemDataset::GeoIp(_) => SiemDatasetType::GeoIp,
            SiemDataset::IpMac(_) => SiemDatasetType::IpMac,
            SiemDataset::IpDNS(_) => SiemDatasetType::IpDNS,
            SiemDataset::MacHost(_) => SiemDatasetType::MacHost,
            SiemDataset::HostUser(_) => SiemDatasetType::HostUser,
            SiemDataset::BlockIp(_) => SiemDatasetType::BlockIp,
            SiemDataset::BlockDomain(_) => SiemDatasetType::BlockDomain,
            SiemDataset::BlockEmailSender(_) => SiemDatasetType::BlockEmailSender,
            SiemDataset::BlockCountry(_) => SiemDatasetType::BlockCountry,
            SiemDataset::UserTag(_) => SiemDatasetType::UserTag,
            SiemDataset::AssetTag(_) => SiemDatasetType::AssetTag,
            SiemDataset::IpCloudService(_) => SiemDatasetType::IpCloudService,
            SiemDataset::IpCloudProvider(_) => SiemDatasetType::IpCloudProvider,
            SiemDataset::UserHeadquarters(_) => SiemDatasetType::UserHeadquarters,
            SiemDataset::IpHeadquarters(_) => SiemDatasetType::IpHeadquarters,
            SiemDataset::HeadquartersWorkingHours => SiemDatasetType::HeadquartersWorkingHours,
            SiemDataset::MantainceCalendar(_) => SiemDatasetType::MantainceCalendar,
            SiemDataset::Configuration(_) => SiemDatasetType::Configuration,
            SiemDataset::HostVulnerable(_) => SiemDatasetType::HostVulnerable,
            SiemDataset::CustomMapIpNet((name, _)) => SiemDatasetType::CustomMapIpNet(name.clone()),
            SiemDataset::CustomMapText((name, _)) => SiemDatasetType::CustomMapText(name.clone()),
            SiemDataset::CustomIpList((name, _)) => SiemDatasetType::CustomIpList(name.clone()),
            SiemDataset::CustomIpMap((name, _)) => SiemDatasetType::CustomIpMap(name.clone()),
            SiemDataset::CustomTextList((name, _)) => SiemDatasetType::CustomTextList(name.clone()),
            SiemDataset::CustomMapTextList((name, _)) => {
                SiemDatasetType::CustomMapTextList(name.clone())
            }
            SiemDataset::I18n(_) => SiemDatasetType::I18n,
            SiemDataset::Secrets((name, _)) => SiemDatasetType::Secrets(name.clone()),
        }
    }
}
impl fmt::Display for SiemDataset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
impl PartialEq for SiemDataset {
    fn eq(&self, other: &Self) -> bool {
        self.dataset_type() == other.dataset_type()
    }
}
impl Eq for SiemDataset {}

impl PartialOrd for SiemDataset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SiemDataset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dataset_type().cmp(&other.dataset_type())
    }
}

impl Serialize for SiemDataset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SiemDataset", 2)?;
        let typ = match self {
            SiemDataset::CorrelationRules(_) => "CorrelationRules",
            SiemDataset::GeoIp(_) => "GeoIp",
            SiemDataset::IpMac(_) => "IpMac",
            SiemDataset::IpDNS(_) => "IpDNS",
            SiemDataset::MacHost(_) => "MacHost",
            SiemDataset::HostUser(_) => "HostUser",
            SiemDataset::BlockIp(_) => "BlockIp",
            SiemDataset::BlockDomain(_) => "BlockDomain",
            SiemDataset::BlockEmailSender(_) => "BlockEmailSender",
            SiemDataset::BlockCountry(_) => "BlockCountry",
            SiemDataset::UserTag(_) => "UserTag",
            SiemDataset::AssetTag(_) => "AssetTag",
            SiemDataset::IpCloudService(_) => "IpCloudService",
            SiemDataset::IpCloudProvider(_) => "IpCloudProvider",
            SiemDataset::UserHeadquarters(_) => "UserHeadquarters",
            SiemDataset::IpHeadquarters(_) => "IpHeadquarters",
            SiemDataset::HeadquartersWorkingHours => "HeadquartersWorkingHours",
            SiemDataset::MantainceCalendar(_) => "MantainceCalendar",
            SiemDataset::Configuration(_) => "Configuration",
            SiemDataset::HostVulnerable(_) => "HostVulnerable",
            SiemDataset::I18n(_) => "I18n",
            SiemDataset::CustomMapIpNet((name, _)) => {
                state.serialize_field("name", name)?;
                "CustomMapIpNet"
            }
            SiemDataset::CustomMapText((name, _)) => {
                state.serialize_field("name", name)?;
                "CustomMapText"
            }
            SiemDataset::CustomIpList((name, _)) => {
                state.serialize_field("name", name)?;
                "CustomIpList"
            }
            SiemDataset::CustomIpMap((name, _)) => {
                state.serialize_field("name", name)?;
                "CustomIpMap"
            }
            SiemDataset::CustomTextList((name, _)) => {
                state.serialize_field("name", name)?;
                "CustomTextList"
            }
            SiemDataset::CustomMapTextList((name, _)) => {
                state.serialize_field("name", name)?;
                "CustomMapTextList"
            }
            SiemDataset::Secrets((name, _)) => {
                state.serialize_field("name", name)?;
                "Secrets"
            }
        };
        state.serialize_field("type", typ)?;
        state.end()
    }
}

#[derive(Serialize, Debug)]
#[non_exhaustive]
pub enum UpdateDataset {
    GeoIp(UpdateGeoIp),
    IpCloudService(UpdateNetIp),
    IpCloudProvider(UpdateNetIp),
    IpHeadquarters(UpdateNetIp),
    CustomMapIpNet(UpdateNetIp),
    IpMac(UpdateIpMap),
    IpDNS(UpdateIpMapList),
    MacHost(UpdateTextMap),
    HostUser(UpdateTextMap),
    UserTag(UpdateTextMap),
    AssetTag(UpdateTextMap),
    UserHeadquarters(UpdateTextMap),
    CustomMapText(UpdateTextMap),
    BlockIp(UpdateIpSet),
    CustomIpList(UpdateIpSet),
    BlockDomain(UpdateTextSet),
    BlockEmailSender(UpdateTextSet),
    BlockCountry(UpdateTextSet),
    CustomTextList(UpdateTextSet),
    CustomMapTextList(UpdateTextMapList),
    MantainceCalendar(UpdateCalendar),
    Configuration(UpdateTextMap),
    Secrets(UpdateTextMap),
    HostVulnerable(UpdateTextMapList),
    CorrelationRules(UpdateGeoIp),
    I18n(UpdateI18n),
}