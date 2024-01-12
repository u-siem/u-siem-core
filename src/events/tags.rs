//List of common tags

/// Users that must be protected at all costs. Ex: CEO, CTO, CISO...
pub static VIP_USER: &str = "vip_user";

/// The event involved an administrator.
pub static ADMIN_USER: &str = "admin_user";

/// Unused accounts for detecting intrusions.
pub static FAKE_ACCOUNT: &str = "fake_account";

/// An asset that no user inside the company should be accessing.
pub static HONEYPOT: &str = "honeypot";

/// Servers that must be protected at all costs like Domain Controllers or SCCM servers.
pub static CRITICAL_ASSET: &str = "critical_asset";

///Firewalls, routers, switches and any other traditional device related to network systems.
pub static NETWORK_ASSET: &str = "network_asset";

/// NGFW, SIEMs, WAFs, Anti Virus Products and every other device involved in protection of a network.
pub static SECURITY_ASSET: &str = "security_asset";

/// Server hosting a web application
pub static WEB_SERVER: &str = "web_server";

/// DNS server. Used with DNS events as to not trigger alerts if the originator of the
/// DNS request was a dns_server and not a workstation and the observer of the event is a firewall.
pub static DNS_SERVER: &str = "dns_server";

///  A system on a network specifically designed and configured to withstand attacks, typically insida the DMZ.
///
/// https://en.wikipedia.org/wiki/Bastion_host
pub static BASTION_HOST: &str = "bastion_host";

/// A system on a network used to access and manage devices in a separate security zone.
///
///  https://en.wikipedia.org/wiki/Jump_server
pub static JUMP_HOST: &str = "jump_host";

/// Used to tag events when related to a device being configured / updated ...
pub static MAINTENANCE_PERIOD: &str = "maintenance_period";

/// Used to mark information related to the event that has been found in the MISP dataset.
pub static IOC_MISP: &str = "ioc_misp";

/// Used to mark an IP as never seen before. Use only for small instances.
pub static NEVER_SEEN_IP: &str = "never_seen_ip";

/// Used to mark an IP that is in the block list
pub static BLOCKED_IP: &str = "in_block_list";
