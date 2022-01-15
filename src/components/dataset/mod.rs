pub mod calendar;
pub mod geo_ip;
pub mod ip_map;
pub mod ip_map_list;
pub mod ip_net;
pub mod ip_set;
pub mod text_map;
pub mod text_map_list;
pub mod text_set;

use calendar::{CalendarSynDataset, UpdateCalendar};
use geo_ip::{GeoIpSynDataset, UpdateGeoIp};
use ip_map::{IpMapSynDataset, UpdateIpMap};
use ip_map_list::{IpMapListSynDataset, UpdateIpMapList};
use ip_net::{IpNetSynDataset, UpdateNetIp};
use ip_set::{IpSetSynDataset, UpdateIpSet};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use text_map::{TextMapSynDataset, UpdateTextMap};
use text_map_list::{TextMapListSynDataset, UpdateTextMapList};
use text_set::{TextSetSynDataset, UpdateTextSet};

/// Common work datasets that allow a rapid development of rules and that the information of some logs allows enriching others.
/// Other datasets like the ones associated with headquarters is controlled by the CMDB
///
/// The custom datasets are associated with the name of the dataset
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SiemDataset {
    /// Map IP to country, city, latitude and longitude
    GeoIp(GeoIpSynDataset),
    /// IP associated with a hostname
    IpHost(IpMapSynDataset),
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
    CustomMapIpNet((Cow<'static, str>, IpNetSynDataset)),
    /// User custom dataset Text => Text
    CustomMapText((Cow<'static, str>, TextMapSynDataset)),
    /// User custom dataset Text => Text
    CustomMapTextList((Cow<'static, str>, TextMapListSynDataset)),
    /// User custom dataset IP list
    CustomIpList((Cow<'static, str>, IpSetSynDataset)),
    /// User custom dataset IP list
    CustomIpMap((Cow<'static, str>, IpMapSynDataset)),
    /// User custom dataset Text list
    CustomTextList((Cow<'static, str>, TextSetSynDataset)),
    /// Mantaince Calendar
    MantainceCalendar(CalendarSynDataset),
    /// Configuration of components. Allows modifications of component parameters in real time.
    Configuration(TextMapSynDataset),
    /// Secret store. A component will only be able to access his own secrets.
    Secrets((Cow<'static, str>, TextMapSynDataset)),
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
impl TryFrom<(SiemDatasetType, IpMapSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpMapSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::IpHost => Ok(SiemDataset::IpHost(value.1)),
            SiemDatasetType::IpMac => Ok(SiemDataset::IpMac(value.1)),
            SiemDatasetType::CustomIpMap(name) => {
                Ok(SiemDataset::CustomIpMap((name.clone(), value.1)))
            }
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
impl TryFrom<(SiemDatasetType, IpMapListSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpMapListSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::IpDNS => Ok(SiemDataset::IpDNS(value.1)),
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}

impl TryFrom<(SiemDatasetType, IpSetSynDataset)> for SiemDataset {
    type Error = &'static str;

    fn try_from(value: (SiemDatasetType, IpSetSynDataset)) -> Result<Self, Self::Error> {
        match value.0 {
            SiemDatasetType::BlockIp => Ok(SiemDataset::BlockIp(value.1)),
            SiemDatasetType::CustomIpList(name) => Ok(SiemDataset::CustomIpList((name.clone(), value.1))),
            _ => Err("IpMapSynDataset not valid for this type"),
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
            SiemDatasetType::CustomTextList(name) => Ok(SiemDataset::CustomTextList((name.clone(), value.1))),
            _ => Err("IpMapSynDataset not valid for this type"),
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
            SiemDatasetType::CustomMapTextList(name) => Ok(SiemDataset::CustomMapTextList((name.clone(), value.1))),
            _ => Err("IpMapSynDataset not valid for this type"),
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
            SiemDatasetType::CustomMapIpNet(name) => Ok(SiemDataset::CustomMapIpNet((name.clone(), value.1))),
            _ => Err("IpMapSynDataset not valid for this type"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum SiemDatasetType {
    /// Map IP to country, city, latitude and longitude
    GeoIp,
    /// IP associated with a hostname
    IpHost,
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
    /// User custom dataset IP_NET => Text
    CustomMapIpNet(Cow<'static, str>),
    /// User custom dataset Text => Text
    CustomMapText(Cow<'static, str>),
    /// User custom dataset Text => Text[]
    CustomMapTextList(Cow<'static, str>),
    /// User custom dataset IP list
    CustomIpList(Cow<'static, str>),
    CustomIpMap(Cow<'static, str>),
    /// User custom dataset Text list
    CustomTextList(Cow<'static, str>),
    /// Mantaince Calendar
    MantainceCalendar,
    Configuration,
    Secrets(Cow<'static, str>),
}

impl SiemDataset {
    pub fn dataset_type(&self) -> SiemDatasetType {
        match self {
            SiemDataset::GeoIp(_) => SiemDatasetType::GeoIp,
            SiemDataset::IpHost(_) => SiemDatasetType::IpHost,
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
            SiemDataset::GeoIp(_) => "GeoIp",
            SiemDataset::IpHost(_) => "IpHost",
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
    IpHost(UpdateIpMap),
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
}
