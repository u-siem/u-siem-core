use serde::{Deserialize, Serialize};

use crate::{events::ip::SiemIp, prelude::LogString};

use super::{
    common::UserRole,
    mitre::{MitreTactics, MitreTechniques},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParserDefinition {
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDefinition {
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleDefinition {
    pub name: String,
    pub description: String,
    pub mitre: (Vec<MitreTactics>, Vec<MitreTechniques>),
    pub service: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterEmail {
    pub email: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterDomain {
    pub domain: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterIp {
    pub ip: SiemIp,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsolateEndpoint {
    pub hostname: String,
    pub comment: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsolateIp {
    pub ip: SiemIp,
    pub comment: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UseCaseDefinition {
    /// Name of the Use Case
    pub name: LogString,
    /// Description of the Use Case and what is intended
    pub description: LogString,
    /// Abstraction of the logic involved
    pub case_logic: LogString,
    /// What cannot detect this use case
    pub limitations: LogString,
    /// Device requirements: Product, Service, Category => AND conditioned
    pub requirements: Requirements,
    /// Rule for detecting this Use Case. Only the name
    pub rule: LogString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Requirements {
    pub product : Option<LogString>,
    pub service : Option<LogString>,
    pub category : Option<LogString>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum LoginUser {
    Password(LoginUserPass),
    ApiKey(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserPass {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedOnUser {
    pub username: String,
    pub role: UserRole,
}
