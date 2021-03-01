pub mod geo_ip;
pub mod ip_map;
pub mod ip_net;
pub mod text_map;
pub mod ip_set;
pub mod text_set;
use geo_ip::{GeoIpSynDataset, UpdateGeoIp};
use ip_map::{IpMapSynDataset, UpdateIpMap};
use ip_net::{IpNetSynDataset, UpdateNetIp};
use text_map::{TextMapSynDataset, UpdateTextMap};
use ip_set::{IpSetSynDataset, UpdateIpSet};
use text_set::{TextSetSynDataset, UpdateTextSet};

/// Common work datasets that allow a rapid development of rules and that the information of some logs allows enriching others.
/// Other datasets like the ones associated with headquarters is controlled by the CMDB
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
    CustomMapIpNet(IpNetSynDataset),
    /// User custom dataset Text => Text
    CustomMapText(TextMapSynDataset),
    /// User custom dataset IP list
    CustomIpList(IpSetSynDataset),
    /// User custom dataset Text list
    CustomTextList(TextSetSynDataset),
}

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
}
