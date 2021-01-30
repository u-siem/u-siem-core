//List of common tags

/// Users that must be protected at all costs. Ex: CEO, CTO, CISO...
pub static VIP_USER: &'static str = "vip_user";

/// The event involved an administrator.
pub static ADMIN_USER: &'static str = "admin_user";

/// Unused accounts used to detect intrusions.
pub static FAKE_ACCOUNT: &'static str = "fake_account";

/// An asset that no user inside the company should be accessing.
pub static HONEYPOT: &'static str = "honeypot";

/// Servers that must be protected at all costs like Domain Controllers or SCCM servers.
pub static CRITICAL_ASSET: &'static str = "critical_asset";

/// Traditional firewalls, routers, switches and every other device used to configure or mantain a network.
pub static NETWORK_ASSET: &'static str = "network_asset";

/// NGFW, SIEMs, WAFs, Anti Virus Products and every other device involved in protection of a network.
pub static SECURITY_ASSET: &'static str = "security_asset";

/// Server hosting a web application
pub static WEB_SERVER: &'static str = "web_server";

/// DNS server. Used with DNS events as to not trigger alerts if the originator of the
/// DNS request was a dns_server and not a workstation and the observer of the event is a firewall.
pub static DNS_SERVER: &'static str = "dns_server";

///  A system on a network specifically designed and configured to withstand attacks, typically insida the DMZ.
///
/// https://en.wikipedia.org/wiki/Bastion_host
pub static BASTION_HOST: &'static str = "bastion_host";

/// A system on a network used to access and manage devices in a separate security zone.
///
///  https://en.wikipedia.org/wiki/Jump_server
pub static JUMP_HOST: &'static str = "jump_host";
