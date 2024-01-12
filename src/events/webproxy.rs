use super::common::{HttpMethod, WebProtocol};
use super::field_dictionary::*;
use super::ip::SiemIp;
use crate::prelude::types::LogString;
use crate::prelude::{SiemField, SiemLog};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebProxyEvent {
    pub source_ip: SiemIp,
    pub destination_ip: SiemIp,
    pub destination_port: u16,
    pub in_bytes: u32,
    pub out_bytes: u32,
    pub http_code: u32,
    pub http_method: HttpMethod,
    pub url: LogString,
    pub domain: LogString,
    /// Web protocol: http, https, ftp...Only UPPERCASE
    pub protocol: WebProtocol,
    pub user_name: LogString,
    pub mime_type: LogString,
    pub outcome: WebProxyOutcome,
    /// Rule that routed/blocked the connection
    pub rule_name: Option<LogString>,
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
    pub fn rule_name(&self) -> &Option<LogString> {
        &self.rule_name
    }
    pub fn rule_category(&self) -> &Option<WebProxyRuleCategory> {
        &self.rule_category
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

impl From<WebProxyEvent> for SiemLog {
    fn from(val: WebProxyEvent) -> Self {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(SOURCE_IP, SiemField::IP(val.source_ip));
        log.add_field(DESTINATION_IP, SiemField::IP(val.destination_ip));
        log.add_field(
            DESTINATION_PORT,
            SiemField::U64(val.destination_port as u64),
        );
        log.add_field(
            EVENT_OUTCOME,
            SiemField::Text(LogString::Owned(val.outcome.to_string())),
        );
        log.add_field(SOURCE_BYTES, SiemField::U64(val.out_bytes as u64));
        log.add_field(DESTINATION_BYTES, SiemField::U64(val.in_bytes as u64));
        log.add_field(
            NETWORK_PROTOCOL,
            SiemField::Text(LogString::Owned(val.protocol.to_string())),
        );
        log.add_field(
            HTTP_RESPONSE_STATUS_CODE,
            SiemField::U64(val.http_code as u64),
        );
        log.add_field(
            HTTP_REQUEST_METHOD,
            SiemField::Text(LogString::Owned(val.http_method.to_string())),
        );
        log.add_field(URL_FULL, SiemField::Text(val.url));
        log.add_field(URL_DOMAIN, SiemField::Text(val.domain));
        log.add_field(USER_NAME, SiemField::User(val.user_name.to_string()));
        log.add_field(HTTP_RESPONSE_MIME_TYPE, SiemField::Text(val.mime_type));
        if let Some(rule_category) = val.rule_category {
            log.add_field(
                RULE_CATEGORY,
                SiemField::Text(LogString::Owned(rule_category.to_string())),
            );
        }
        if let Some(rule_name) = val.rule_name {
            log.add_field(
                RULE_NAME,
                SiemField::Text(LogString::Owned(rule_name.to_string())),
            );
        }
        log
    }
}
