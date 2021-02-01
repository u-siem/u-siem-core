use super::field::SiemIp;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub struct WebProxyEvent {
    pub source_ip: SiemIp,
    pub destination_ip: SiemIp,
    pub destination_port: u16,
    pub in_bytes: u32,
    pub out_bytes: u32,
    pub http_code: u32,
    pub http_method: HttpMethod,
    pub url: Cow<'static, str>,
    pub domain: Cow<'static, str>,
    /// Web protocol: http, https, ftp... Use lowercase only
    pub protocol: Cow<'static, str>,
    pub user_name: Cow<'static, str>,
    pub mime_type: Cow<'static, str>,
    pub outcome: WebProxyOutcome,
    /// Rule that routed/blocked the connection
    pub rule_name: Option<Cow<'static, str>>,
    /// Categorization of the traffic
    pub rule_category: Option<WebProxyRuleCategory>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum WebProxyOutcome {
    /// Connection was blocked
    BLOCK,
    /// Connection was allowed
    ALLOW,
    /// Unknow connection state.
    UNKNOWN,
}

/// Based on Bluecoat categories http://sitereview.bluecoat.com/#/category-descriptions
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum WebProxyRuleCategory {
    Abortion,
    MatureContent,
    Alcohol,
    AlternativeSpirituality,
    ArtCulture,
    Auctions,
    AudioVideoClips,
    Trading,
    Economy,
    Charitable,
    OnlineChat,
    ChildPornography,
    CloudInfrastructure,
    CompromisedSites,
    InformationSecurity,
    ContentDeliveryNetworks,
    ControlledSubstances,
    Cryptocurrency,
    DynamicDNSHost,
    ECardInvitations,
    Education,
    Email,
    EmailMarketing,
    Entertainment,
    FileStorage,
    Finance,
    ForKids,
    Gambling,
    Games,
    Gore,
    Government,
    Hacking,
    Health,
    HumorJokes,
    Informational,
    InternetConnectedDevices,
    InternetTelephony,
    IntimateApparel,
    JobSearch,
    MaliciousOutboundDataBotnets,
    MaliciousSources,
    Marijuana,
    MediaSharing,
    Military,
    PotentiallyAdult,
    News,
    Forums,
    Nudity,
    BusinessApplications,
    OnlineMeetings,
    P2P,
    PersonalSites,
    PersonalsDating,
    Phishing,
    CopyrightConcerns,
    Placeholders,
    PoliticalAdvocacy,
    Pornography,
    PotentiallyUnwantedSoftware,
    ProxyAvoidance,
    RadioAudioStreams,
    RealEstate,
    Reference,
    Religion,
    RemoteAccess,
    Restaurants,
    QuestionableLegality,
    SearchEngines,
    SexEducation,
    SexualExpression,
    Shopping,
    SocialNetworking,
    DailyLiving,
    SoftwareDownloads,
    Spam,
    Sports,
    Suspicious,
    Technology,
    Tobacco,
    Translation,
    Travel,
    VideoStreams,
    Uncategorized,
    URLShorteners,
    Vehicles,
    Violence,
    Weapons,
    WebAds,
    WebHosting,
    WebInfrastructure,
    Others(String),
}

/// Common HTTP Request methods
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum HttpMethod {
    UNKNOWN(String),
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    CONNECT,
}

impl HttpMethod {
    pub fn from_str(val: &str) -> HttpMethod {
        match &val.to_uppercase()[..] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PATCH" => HttpMethod::PATCH,
            "OPTIONS" => HttpMethod::OPTIONS,
            "CONNECT" => HttpMethod::CONNECT,
            val => HttpMethod::UNKNOWN(val.to_string()),
        }
    }

    pub fn equals(&self, val: &str) -> bool {
        match (val, self) {
            ("GET", HttpMethod::GET) => return true,
            ("POST", HttpMethod::POST) => return true,
            ("PUT", HttpMethod::PUT) => return true,
            ("PATCH", HttpMethod::PATCH) => return true,
            ("UNKNOWN", HttpMethod::UNKNOWN(_)) => return true,
            ("OPTIONS", HttpMethod::OPTIONS) => return true,
            ("CONNECT", HttpMethod::CONNECT) => return true,
            _ => return false,
        }
    }
}
