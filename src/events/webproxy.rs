use super::field::SiemIp;
use serde::Serialize;
use std::borrow::Cow;
use super::common::{HttpMethod, WebProtocol};


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
    /// Web protocol: http, https, ftp...Only UPPERCASE
    pub protocol: WebProtocol,
    pub user_name: Cow<'static, str>,
    pub mime_type: Cow<'static, str>,
    pub outcome: WebProxyOutcome,
    /// Rule that routed/blocked the connection
    pub rule_name: Option<Cow<'static, str>>,
    /// Categorization of the traffic
    pub rule_category: Option<WebProxyRuleCategory>,
}
impl WebProxyEvent {
    pub fn source_ip(&self) -> &SiemIp {
        &self.source_ip
    }
    pub fn destination_ip(&self) -> &SiemIp {
        &self.destination_ip
    }
    pub fn protocol(&self) -> &WebProtocol {
        &self.protocol
    }
    pub fn outcome(&self) -> &WebProxyOutcome {
        &self.outcome
    }
    pub fn http_method(&self) -> &HttpMethod {
        &self.http_method
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn domain(&self) -> &str {
        &self.domain
    }
    pub fn rule_name(&self) -> &Option<Cow<'static, str>> {
        &self.rule_name
    }
    pub fn rule_category(&self) -> &Option<WebProxyRuleCategory> {
        &self.rule_category
    }
    
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
impl std::fmt::Display for WebProxyOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
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

impl std::fmt::Display for WebProxyRuleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}