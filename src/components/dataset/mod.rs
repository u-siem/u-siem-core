pub mod geo_ip;
pub mod ip_map;
pub mod ip_net;
pub mod text_map;
pub mod ip_set;
pub mod text_set;
pub mod calendar;
use geo_ip::{GeoIpSynDataset, UpdateGeoIp};
use ip_map::{IpMapSynDataset, UpdateIpMap};
use ip_net::{IpNetSynDataset, UpdateNetIp};
use text_map::{TextMapSynDataset, UpdateTextMap};
use ip_set::{IpSetSynDataset, UpdateIpSet};
use text_set::{TextSetSynDataset, UpdateTextSet};
use calendar::{CalendarSynDataset, UpdateCalendar};
use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};
use std::borrow::Cow;

/// Common work datasets that allow a rapid development of rules and that the information of some logs allows enriching others.
/// Other datasets like the ones associated with headquarters is controlled by the CMDB
/// 
/// The custom datasets are associated with the name of the dataset
#[derive(Debug)]
pub enum SiemDataset {
    /// Map IP to country, city, latitude and longitude
    GeoIp(GeoIpSynDataset),
    /// IP associated with a hostname
    IpHost(IpMapSynDataset),
    /// IP associated with a MAC address
    IpMac(IpMapSynDataset),
    /// Hostname associated with a MAC address
    HostMac(TextMapSynDataset),
    /// Hostname associated with a username
    HostUser(TextMapSynDataset),
    /// List of IPs in the block list
    BlockIp(IpSetSynDataset),
    /// List of domain in the block list
    BlockDomain(TextSetSynDataset),
    /// List of countries in the block list
    BlockCountry(TextSetSynDataset),
    /// Tag each user with roles => user.roles = [vip, admin, extern, invited, director, super_user, local_user]
    UserTag(TextMapSynDataset),
    /// Tag each host with categories => [web_server, sec_related, critical, ad_related, net_related]
    AssetTag(TextMapSynDataset),
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
    /// User custom dataset IP list
    CustomIpList((Cow<'static, str>, IpSetSynDataset)),
    /// User custom dataset Text list
    CustomTextList((Cow<'static, str>, TextSetSynDataset)),
    /// Mantaince Calendar
    MantainceCalendar(CalendarSynDataset)
}

pub enum SiemDatasetType {
    /// Map IP to country, city, latitude and longitude
    GeoIp,
    /// IP associated with a hostname
    IpHost,
    /// IP associated with a MAC address
    IpMac,
    /// Hostname associated with a MAC address
    HostMac,
    /// Hostname associated with a username
    HostUser,
    /// List of IPs in the block list
    BlockIp,
    /// List of domain in the block list
    BlockDomain,
    /// List of countries in the block list
    BlockCountry,
    /// Tag each user with roles => user.roles = [vip, admin, extern, invited, director, super_user, local_user]
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
    /// User custom dataset IP_NET => Text
    CustomMapIpNet(Cow<'static, str>),
    /// User custom dataset Text => Text
    CustomMapText(Cow<'static, str>),
    /// User custom dataset IP list
    CustomIpList(Cow<'static, str>),
    /// User custom dataset Text list
    CustomTextList(Cow<'static, str>),
    /// Mantaince Calendar
    MantainceCalendar
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
            SiemDataset::HostMac(_) => "HostMac",
            SiemDataset::HostUser(_) => "HostUser",
            SiemDataset::BlockIp(_) => "BlockIp",
            SiemDataset::BlockDomain(_) => "BlockDomain",
            SiemDataset::BlockCountry(_) => "BlockCountry",
            SiemDataset::UserTag(_) => "UserTag",
            SiemDataset::AssetTag(_) => "AssetTag",
            SiemDataset::IpCloudService(_) => "IpCloudService",
            SiemDataset::IpCloudProvider(_) => "IpCloudProvider",
            SiemDataset::UserHeadquarters(_) => "UserHeadquarters",
            SiemDataset::IpHeadquarters(_) => "IpHeadquarters",
            SiemDataset::HeadquartersWorkingHours => "HeadquartersWorkingHours",
            SiemDataset::MantainceCalendar(_) => "MantainceCalendar",
            SiemDataset::CustomMapIpNet((name,_)) => {
                state.serialize_field("name", name)?;
                "CustomMapIpNet"
            },
            SiemDataset::CustomMapText((name,_)) => {
                state.serialize_field("name", name)?;
                "CustomMapIpNet"
            },
            SiemDataset::CustomIpList((name,_)) => {
                state.serialize_field("name", name)?;
                "CustomIpList"
            },
            SiemDataset::CustomTextList((name,_)) => {
                state.serialize_field("name", name)?;
                "CustomTextList"
            },
        };
        state.serialize_field("type", typ)?;
        state.end()
    }
}

#[derive(Serialize, Debug)]
pub enum UpdateDataset {
    GeoIp(UpdateGeoIp),
    IpCloudService(UpdateNetIp),
    IpCloudProvider(UpdateNetIp),
    IpHeadquarters(UpdateNetIp),
    CustomMapIpNet(UpdateNetIp),
    IpHost(UpdateIpMap),
    IpMac(UpdateIpMap),
    HostMac(UpdateTextMap),
    HostUser(UpdateTextMap),
    UserTag(UpdateTextMap),
    AssetTag(UpdateTextMap),
    UserHeadquarters(UpdateTextMap),
    CustomMapText(UpdateTextMap),
    BlockIp(UpdateIpSet),
    CustomIpList(UpdateIpSet),
    BlockDomain(UpdateTextSet),
    BlockCountry(UpdateTextSet),
    CustomTextList(UpdateTextSet),
    MantainceCalendar(UpdateCalendar)
}
