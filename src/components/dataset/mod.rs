pub mod ip_net;
use ip_net::{GeoIpDataset, IpNetDataset};
/// Common work datasets that allow a rapid development of rules and that the information of some logs allows enriching others.
/// Other datasets like the ones associated with headquarters is controlled by the CMDB
pub enum SiemDataset {
    /// Map IP to country, city, latitude and longitude
    GeoIp(GeoIpDataset),
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
    IpCloudService(IpNetDataset),
    /// Cloud Provider => Azure, Google Cloud, AWS
    IpCloudProvider(IpNetDataset),
    /// User associated with a headquarter
    UserHeadquarters,
    /// IP net associated with a headquarter
    IpHeadquarters(IpNetDataset),
    /// Working hours of each headquarter
    HeadquartersWorkingHours,
    /// User custom dataset IP_NET => Text
    CustomMapIpNet(IpNetDataset),
    /// User custom dataset Text => Text
    CustomMapText,
    /// User custom dataset IP list
    CustomIpList,
    /// User custom dataset Text list
    CustomTextList,
}

