
/// Common work datasets that allow a rapid development of rules and that the information of some logs allows enriching others.
/// Other datasets like the ones associated with headquarters is controlled by the CMDB
pub enum SiemDataset {
    /// Map IP to country, city, latitude and longitude
    GEO_IP,
    /// IP associated with a hostname
    IP_HOST,
    /// IP associated with a MAC address
    IP_MAC,
    /// Hostname associated with a MAC address
    HOST_MAC,
    /// Hostname associated with a username
    HOST_USER,
    /// List of IPs in the block list
    BLOCK_IP,
    /// List of domain in the block list
    BLOCK_DOMAIN,
    /// List of countries in the block list
    BLOCK_COUNTRY,
    /// Tag each user with roles => user.roles = [vip, admin, extern, invited, director, super_user, local_user]
    USER_TAG,
    /// Tag each host with categories => [web_server, sec_related, critical, ad_related, net_related]
    ASSET_TAG,
    /// Cloud service => Office 365, G Suit ...
    IP_CLOUD_SERVICE,
    /// Cloud Provider => Azure, Google Cloud, AWS
    IP_CLOUD_PROVIDER,
    /// User associated with a headquarter
    USER_HEADQUARTERS,
    /// IP net associated with a headquarter
    IP_HEADQUARTERS,
    /// Working hours of each headquarter
    HEADQUARTERS_WORKING_HOURS,
    /// User custom dataset IP_NET => Text
    CUSTOM_MAP_IP_NET,
    /// User custom dataset Text => Text
    CUSTOM_MAP_TEXT,
    /// User custom dataset IP list
    CUSTOM_IP_LIST,
    /// User custom dataset Text list
    CUSTOM_TEXT_LIST,
}

