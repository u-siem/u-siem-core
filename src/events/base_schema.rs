use crate::events::field_dictionary::*;
use crate::events::schema::{FieldSchema, FieldType};
use std::collections::BTreeMap;

pub fn get_default_schema() -> FieldSchema {
    let mut fields = BTreeMap::new();
    fields.insert(
        "origin",
        FieldType::Text("IP or Hostname of the server that sent the log"),
    );
    fields.insert(
        "tenant",
        FieldType::Text("Customer name for SOC environments. Ex: Contoso"),
    );
    fields.insert(
        "product",
        FieldType::Text("Name of the product for wich the log belongs. Ex: ASA"),
    );
    fields.insert("service", FieldType::Text("Subset of the product logs. Like a OS that can have multiple programs running inside generating multiple logs."));
    fields.insert(
        "category",
        FieldType::Text("Category of the device: Firewall, web, antivirus"),
    );
    fields.insert(
        "vendor",
        FieldType::Text("Company that created the product. Ex: Cisco"),
    );
    fields.insert("event.type", FieldType::Text("uSIEM log type: SiemEvent"));
    fields.insert("tags", FieldType::Text("Tags to better describe the event"));
    fields.insert(
        "message",
        FieldType::Text("Original log message including syslog header"),
    );
    fields.insert(
        "event_received",
        FieldType::Date("Timestamp at witch the log arrived  "),
    );
    fields.insert(
        "event_created",
        FieldType::Date("Timestamp at witch the log was generated"),
    );
    fields.insert("host.hostname", FieldType::Text("Hostname of the host"));
    fields.insert("server.hostname", FieldType::Text("Hostname of the server"));
    fields.insert("client.hostname", FieldType::Text("Hostname of the client"));
    fields.insert("client.ip", FieldType::Ip("Ip of the client"));
    fields.insert("client.mac", FieldType::Text("MAC address of the client"));
    fields.insert(DHCP_RECORD_TYPE, FieldType::Text("DHCP record type"));
    fields.insert("dhcp.requested_ip", FieldType::Ip("DHCP requested IP"));
    fields.insert(USER_NAME, FieldType::Text("User name"));
    fields.insert(USER_DOMAIN, FieldType::Text("Domain of the user"));
    fields.insert(
        "source.user.name",
        FieldType::Text("Username of the initiator of an action"),
    );
    fields.insert(
        "source.user.domain",
        FieldType::Text("Domain of the initiator of an action"),
    );
    fields.insert(
        SOURCE_IP,
        FieldType::Ip("IP of the initiator of a connection"),
    );
    fields.insert(
        DESTINATION_IP,
        FieldType::Ip("IP of the target of a conector"),
    );
    fields.insert(SOURCE_PORT, FieldType::Numeric("Port of the source"));
    fields.insert(
        DESTINATION_PORT,
        FieldType::Numeric("Port of the destination"),
    );
    fields.insert(
        SOURCE_BYTES,
        FieldType::Numeric("Bytes sent from the source to the destination."),
    );
    fields.insert(
        DESTINATION_BYTES,
        FieldType::Numeric("Bytes sent from the destination to the source"),
    );
    fields.insert(
        IN_INTERFACE,
        FieldType::Text("Name of the interface receiving the connection"),
    );
    fields.insert(
        OUT_INTERFACE,
        FieldType::Text("Name of the egress interface"),
    );
    let mut web_protocol = BTreeMap::new();
    web_protocol.insert(
        "HTTP",
        "HyperText Transfer Protocol. HTTP is the underlying protocol used by the World Wide Web. ",
    );
    web_protocol.insert("HTTPS", "Secured HTTP protocol");
    web_protocol.insert("FTP", "The File Transfer Protocol is a standard communication protocol used for the transfer of computer files from a server to a client on a computer network.");
    web_protocol.insert("WS", "WebSocket is a computer communications protocol, providing full-duplex communication channels over a single TCP connection.");
    web_protocol.insert("WSS", "Secured WebSocket protocol");
    let mut web_cat = BTreeMap::new();
    web_cat.insert("Abortion", "Abortion");
    web_cat.insert("MatureContent", "MatureContent");
    web_cat.insert("Alcohol", "Alcohol");
    web_cat.insert("AlternativeSpirituality", "AlternativeSpirituality");
    web_cat.insert("ArtCulture", "ArtCulture");
    web_cat.insert("Auctions", "Auctions");
    web_cat.insert("AudioVideoClips", "AudioVideoClips");
    web_cat.insert("Trading", "Trading");
    web_cat.insert("Economy", "Economy");
    web_cat.insert("Charitable", "Charitable");
    web_cat.insert("OnlineChat", "OnlineChat");
    web_cat.insert("ChildPornography", "ChildPornography");
    web_cat.insert("CloudInfrastructure", "CloudInfrastructure");
    web_cat.insert("CompromisedSites", "CompromisedSites");
    web_cat.insert("InformationSecurity", "InformationSecurity");
    web_cat.insert("ContentDeliveryNetworks", "ContentDeliveryNetworks");
    web_cat.insert("ControlledSubstances", "ControlledSubstances");
    web_cat.insert("Cryptocurrency", "Cryptocurrency");
    web_cat.insert("DynamicDNSHost", "DynamicDNSHost");
    web_cat.insert("ECardInvitations", "ECardInvitations");
    web_cat.insert("Education", "Education");
    web_cat.insert("Email", "Email");
    web_cat.insert("EmailMarketing", "EmailMarketing");
    web_cat.insert("Entertainment", "Entertainment");
    web_cat.insert("FileStorage", "FileStorage");
    web_cat.insert("Finance", "Finance");
    web_cat.insert("ForKids", "ForKids");
    web_cat.insert("Gambling", "Gambling");
    web_cat.insert("Games", "Games");
    web_cat.insert("Gore", "Gore");
    web_cat.insert("Government", "Government");
    web_cat.insert("Hacking", "Hacking");
    web_cat.insert("Health", "Health");
    web_cat.insert("HumorJokes", "HumorJokes");
    web_cat.insert("Informational", "Informational");
    web_cat.insert("InternetConnectedDevices", "InternetConnectedDevices");
    web_cat.insert("InternetTelephony", "InternetTelephony");
    web_cat.insert("IntimateApparel", "IntimateApparel");
    web_cat.insert("JobSearch", "JobSearch");
    web_cat.insert(
        "MaliciousOutboundDataBotnets",
        "MaliciousOutboundDataBotnets",
    );
    web_cat.insert("MaliciousSources", "MaliciousSources");
    web_cat.insert("Marijuana", "Marijuana");
    web_cat.insert("MediaSharing", "MediaSharing");
    web_cat.insert("Military", "Military");
    web_cat.insert("PotentiallyAdult", "PotentiallyAdult");
    web_cat.insert("News", "News");
    web_cat.insert("Forums", "Forums");
    web_cat.insert("Nudity", "Nudity");
    web_cat.insert("BusinessApplications", "BusinessApplications");
    web_cat.insert("OnlineMeetings", "OnlineMeetings");
    web_cat.insert("P2P", "P2P");
    web_cat.insert("PersonalSites", "PersonalSites");
    web_cat.insert("PersonalsDating", "PersonalsDating");
    web_cat.insert("Phishing", "Phishing");
    web_cat.insert("CopyrightConcerns", "CopyrightConcerns");
    web_cat.insert("Placeholders", "Placeholders");
    web_cat.insert("PoliticalAdvocacy", "PoliticalAdvocacy");
    web_cat.insert("Pornography", "Pornography");
    web_cat.insert("PotentiallyUnwantedSoftware", "PotentiallyUnwantedSoftware");
    web_cat.insert("ProxyAvoidance", "ProxyAvoidance");
    web_cat.insert("RadioAudioStreams", "RadioAudioStreams");
    web_cat.insert("RealEstate", "RealEstate");
    web_cat.insert("Reference", "Reference");
    web_cat.insert("Religion", "Religion");
    web_cat.insert("RemoteAccess", "RemoteAccess");
    web_cat.insert("Restaurants", "Restaurants");
    web_cat.insert("QuestionableLegality", "QuestionableLegality");
    web_cat.insert("SearchEngines", "SearchEngines");
    web_cat.insert("SexEducation", "SexEducation");
    web_cat.insert("SexualExpression", "SexualExpression");
    web_cat.insert("Shopping", "Shopping");
    web_cat.insert("SocialNetworking", "SocialNetworking");
    web_cat.insert("DailyLiving", "DailyLiving");
    web_cat.insert("SoftwareDownloads", "SoftwareDownloads");
    web_cat.insert("Spam", "Spam");
    web_cat.insert("Sports", "Sports");
    web_cat.insert("Suspicious", "Suspicious");
    web_cat.insert("Technology", "Technology");
    web_cat.insert("Tobacco", "Tobacco");
    web_cat.insert("Translation", "Translation");
    web_cat.insert("Travel", "Travel");
    web_cat.insert("VideoStreams", "VideoStreams");
    web_cat.insert("Uncategorized", "Uncategorized");
    web_cat.insert("URLShorteners", "URLShorteners");
    web_cat.insert("Vehicles", "Vehicles");
    web_cat.insert("Violence", "Violence");
    web_cat.insert("Weapons", "Weapons");
    web_cat.insert("WebAds", "WebAds");
    web_cat.insert("WebHosting", "WebHosting");
    web_cat.insert("WebInfrastructure", "WebInfrastructure");
    fields.insert(
        RULE_CATEGORY,
        FieldType::TextOptions(web_cat, "Category of the rule"),
    );
    let mut net_protocol = BTreeMap::new();
    net_protocol.insert("HOPOPT", "HOPOPT");
    net_protocol.insert("ICMP", "ICMP");
    net_protocol.insert("IGMP", "IGMP");
    net_protocol.insert("GGP", "GGP");
    net_protocol.insert("IPV4", "IPV4");
    net_protocol.insert("ST", "ST");
    net_protocol.insert("TCP", "TCP");
    net_protocol.insert("CBT", "CBT");
    net_protocol.insert("EGP", "EGP");
    net_protocol.insert("IGP", "IGP");
    net_protocol.insert("BBN_RCC_MON", "BBN_RCC_MON");
    net_protocol.insert("NVP_II", "NVP_II");
    net_protocol.insert("PUP", "PUP");
    net_protocol.insert("ARGUS", "ARGUS");
    net_protocol.insert("EMCON", "EMCON");
    net_protocol.insert("XNET", "XNET");
    net_protocol.insert("CHAOS", "CHAOS");
    net_protocol.insert("UDP", "UDP");
    net_protocol.insert("MUX", "MUX");
    net_protocol.insert("DCN_MEAS", "DCN_MEAS");
    net_protocol.insert("HMP", "HMP");
    net_protocol.insert("PRM", "PRM");
    net_protocol.insert("XNS_IDP", "XNS_IDP");
    net_protocol.insert("TRUNK_1", "TRUNK_1");
    net_protocol.insert("TRUNK_2", "TRUNK_2");
    net_protocol.insert("LEAF_1", "LEAF_1");
    net_protocol.insert("LEAF_2", "LEAF_2");
    net_protocol.insert("RDP", "RDP");
    net_protocol.insert("IRTP", "IRTP");
    net_protocol.insert("ISO_TP4", "ISO_TP4");
    net_protocol.insert("NETBLT", "NETBLT");
    net_protocol.insert("MFE_NSP", "MFE_NSP");
    net_protocol.insert("MERIT_INP", "MERIT_INP");
    net_protocol.insert("DCCP", "DCCP");
    net_protocol.insert("ThirdPC", "ThirdPC");
    net_protocol.insert("IDPR", "IDPR");
    net_protocol.insert("XTP", "XTP");
    net_protocol.insert("DDP", "DDP");
    net_protocol.insert("IDPR_CMTP", "IDPR_CMTP");
    net_protocol.insert("TPpp", "TPpp");
    net_protocol.insert("IL", "IL");
    net_protocol.insert("IPV6", "IPV6");
    net_protocol.insert("SDRP", "SDRP");
    net_protocol.insert("IPV6_ROUTE", "IPV6_ROUTE");
    net_protocol.insert("IPV6_FRAG", "IPV6_FRAG");
    net_protocol.insert("IDRP", "IDRP");
    net_protocol.insert("RSVP", "RSVP");
    net_protocol.insert("GRE", "GRE");
    net_protocol.insert("DSR", "DSR");
    net_protocol.insert("BNA", "BNA");
    net_protocol.insert("ESP", "ESP");
    net_protocol.insert("AH", "AH");
    net_protocol.insert("I_NLSP", "I_NLSP");
    net_protocol.insert("SWIPE", "SWIPE");
    net_protocol.insert("NARP", "NARP");
    net_protocol.insert("MOBILE", "MOBILE");
    net_protocol.insert("TLSP", "TLSP");
    net_protocol.insert("SKIP", "SKIP");
    net_protocol.insert("IPV6_ICMP", "IPV6_ICMP");
    net_protocol.insert("IPV6_NONXT", "IPV6_NONXT");
    net_protocol.insert("IPV6_OPTS", "IPV6_OPTS");
    net_protocol.insert("CFTP", "CFTP");
    net_protocol.insert("SAT_EXPAK", "SAT_EXPAK");
    net_protocol.insert("KRYPTOLAN", "KRYPTOLAN");
    net_protocol.insert("RVD", "RVD");
    net_protocol.insert("IPPC", "IPPC");
    net_protocol.insert("SAT_MON", "SAT_MON");
    net_protocol.insert("VISA", "VISA");
    net_protocol.insert("IPCV", "IPCV");
    net_protocol.insert("CPNX", "CPNX");
    net_protocol.insert("CPHB", "CPHB");
    net_protocol.insert("WSN", "WSN");
    net_protocol.insert("PVP", "PVP");
    net_protocol.insert("BR_SAT_MON", "BR_SAT_MON");
    net_protocol.insert("SUN_ND", "SUN_ND");
    net_protocol.insert("WB_MON", "WB_MON");
    net_protocol.insert("WB_EXPAK", "WB_EXPAK");
    net_protocol.insert("ISO_IP", "ISO_IP");
    net_protocol.insert("VMTP", "VMTP");
    net_protocol.insert("SECURE_VMTP", "SECURE_VMTP");
    net_protocol.insert("VINES", "VINES");
    net_protocol.insert("TTP", "TTP");
    net_protocol.insert("IPTM", "IPTM");
    net_protocol.insert("NSFNET_IGP", "NSFNET_IGP");
    net_protocol.insert("DGP", "DGP");
    net_protocol.insert("TCF", "TCF");
    net_protocol.insert("EIGRP", "EIGRP");
    net_protocol.insert("OSPFIGP", "OSPFIGP");
    net_protocol.insert("SPRITE_RPC", "SPRITE_RPC");
    net_protocol.insert("LARP", "LARP");
    net_protocol.insert("MTP", "MTP");
    net_protocol.insert("AX_25", "AX_25");
    net_protocol.insert("IPIP", "IPIP");
    net_protocol.insert("MICP", "MICP");
    net_protocol.insert("SCC_SP", "SCC_SP");
    net_protocol.insert("ETHERIP", "ETHERIP");
    net_protocol.insert("ENCAP", "ENCAP");
    net_protocol.insert("GMTP", "GMTP");
    net_protocol.insert("IFMP", "IFMP");
    net_protocol.insert("PNNI", "PNNI");
    net_protocol.insert("PIM", "PIM");
    net_protocol.insert("ARIS", "ARIS");
    net_protocol.insert("SCPS", "SCPS");
    net_protocol.insert("QNX", "QNX");
    net_protocol.insert("A_N", "A_N");
    net_protocol.insert("IPCOMP", "IPCOMP");
    net_protocol.insert("SNP", "SNP");
    net_protocol.insert("COMPAQ_PEER", "COMPAQ_PEER");
    net_protocol.insert("IPX_IN_IP", "IPX_IN_IP");
    net_protocol.insert("VRRP", "VRRP");
    net_protocol.insert("PGM", "PGM");
    net_protocol.insert("L2TP", "L2TP");
    net_protocol.insert("DDX", "DDX");
    net_protocol.insert("IATP", "IATP");
    net_protocol.insert("STP", "STP");
    net_protocol.insert("SRP", "SRP");
    net_protocol.insert("UTI", "UTI");
    net_protocol.insert("SMP", "SMP");
    net_protocol.insert("SM", "SM");
    net_protocol.insert("PTP", "PTP");
    net_protocol.insert("ISIS", "ISIS");
    net_protocol.insert("FIRE", "FIRE");
    net_protocol.insert("CRTP", "CRTP");
    net_protocol.insert("CRUDP", "CRUDP");
    net_protocol.insert("SSCOPMCE", "SSCOPMCE");
    net_protocol.insert("IPLT", "IPLT");
    net_protocol.insert("SPS", "SPS");
    net_protocol.insert("PIPE", "PIPE");
    net_protocol.insert("SCTP", "SCTP");
    net_protocol.insert("FC", "FC");
    net_protocol.insert("RSVP_E2E_IGNORE", "RSVP_E2E_IGNORE");
    net_protocol.insert("MOBILITY", "MOBILITY");
    net_protocol.insert("UDPLITE", "UDPLITE");
    net_protocol.insert("MPLS_IN_IP", "MPLS_IN_IP");
    net_protocol.insert("MANET", "MANET");
    net_protocol.insert("HIP", "HIP");
    net_protocol.insert("SHIM6", "SHIM6");
    net_protocol.insert("WESP", "WESP");
    net_protocol.insert("ROHC", "ROHC");
    net_protocol.insert("ETHERNET", "ETHERNET");
    net_protocol.insert("USE", "USE");
    net_protocol.insert("RESERVED", "RESERVED");
    net_protocol.insert("UNKNOWN", "UNKNOWN");
    fields.insert(
        NETWORK_TRANSPORT,
        FieldType::TextOptions(net_protocol, "Network transport protocol: TCP, UDP..."),
    );
    fields.insert(
        NETWORK_PROTOCOL,
        FieldType::TextOptions(web_protocol, "Network protocol: http, ftp, snmp..."),
    );
    fields.insert(
        "source.address",
        FieldType::Text("An IP, a domain or a unix socket."),
    );
    fields.insert(
        HTTP_RESPONSE_STATUS_CODE,
        FieldType::Numeric("HTTP Status code: 404, 200..."),
    );
    let mut http_request_method = BTreeMap::new();
    http_request_method.insert(
        "GET",
        "The GET method requests that the target resource transfers a representation of its state.",
    );
    http_request_method.insert("HEAD", "The HEAD method requests that the target resource transfers a representation of its state, like for a GET request, but without the representation data enclosed in the response body.");
    http_request_method.insert("POST", "The POST method requests that the target resource processes the representation enclosed in the request according to the semantics of the target resource.");
    http_request_method.insert("PUT", "The PUT method requests that the target resource creates or updates its state with the state defined by the representation enclosed in the request.");
    http_request_method.insert("PATCH", "The PATCH method requests that the target resource modifies its state according to the partial update defined in the representation enclosed in the request.");
    http_request_method.insert("OPTIONS", "The OPTIONS method requests that the target resource transfers the HTTP methods that it supports.");
    http_request_method.insert("CONNECT", "The CONNECT method request that the intermediary establishes a TCP/IP tunnel to the origin server identified by the request target.");
    fields.insert(
        HTTP_REQUEST_METHOD,
        FieldType::TextOptions(http_request_method, "HTTP Request method: get, post..."),
    );

    fields.insert(URL_FULL, FieldType::Text("Full url"));
    fields.insert(URL_DOMAIN, FieldType::Text("Domain of the url"));
    fields.insert(
        HTTP_RESPONSE_MIME_TYPE,
        FieldType::Text("HTTP response mime type"),
    );
    fields.insert(RULE_NAME, FieldType::Text("Name of the rule"));
    fields.insert(DNS_OP_CODE, FieldType::Text("Operation code in DNS"));
    fields.insert(DNS_ANSWER_NAME, FieldType::Text("DNS answer name"));
    fields.insert(DNS_ANSWER_TYPE, FieldType::Text("DNS answer type"));
    fields.insert(DNS_QUESTION_NAME, FieldType::Text("DNS question name"));
    fields.insert(DNS_QUESTION_TYPE, FieldType::Text("DNS question type"));
    fields.insert(RULE_ID, FieldType::Text("Identifier of the rule"));
    fields.insert(URL_PATH, FieldType::Text("URL path: /api/v1"));
    fields.insert(URL_QUERY, FieldType::Text("URL query: ?a=b&c=d"));
    fields.insert("url.extension", FieldType::Text("URL extension: exe, html"));
    fields.insert(
        NETWORK_DURATION,
        FieldType::Decimal("Duration of the communication"),
    );
    fields.insert("user_agent.original", FieldType::Text("Full user agent"));

    let mut event_outcome = BTreeMap::new();
    // Login
    event_outcome.insert("SUCCESS", "Login success");
    event_outcome.insert("FAIL", "Login failed");
    event_outcome.insert("LOCKOUT", "Account locked out");
    event_outcome.insert("ESTABLISH", "Pre authentication phase: trying to connect");
    // Firewall, WebProxy and WebServer
    event_outcome.insert("BLOCK", "Connection was blocked");
    event_outcome.insert("ALLOW", "Connection was allowed");
    event_outcome.insert(
        "END",
        "Connection ended, the event contains information about bytes sended/received",
    );
    event_outcome.insert(
        "STATS",
        "The connection is still ongoing, but we log statistics about it",
    );
    event_outcome.insert("OPEN", "Oppened connection. Later can be dropped due to policy settings. Ej: Sonicwall or Firepower have this behavior.");
    event_outcome.insert("UNKNOWN", "Unknow connection state.");
    // Intrusion
    event_outcome.insert(
        "DETECTED",
        "The attack has not been prevented and may affect systems",
    );
    event_outcome.insert("BLOCKED", "The attack was prevented");
    event_outcome.insert(
        "MONITOR",
        "The attack was not prevented but it does not affect assets",
    );
    event_outcome.insert(
        "IMPACTED",
        "The attack has not been prevented and has affected assets",
    );

    fields.insert(
        EVENT_OUTCOME,
        FieldType::TextOptions(event_outcome, "Outcome of the event"),
    );
    FieldSchema {
        fields,
        allow_unknown_fields: false,
        gdpr: None,
    }
}
