use serde::{Deserialize, Serialize};
use crate::prelude::LogString;

pub trait Command {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParserDefinition {
    pub name: LogString,
    pub description: LogString,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDefinition {
    pub name: LogString,
    pub description: LogString,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterEmail {
    pub email: LogString,
    pub comment: LogString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterDomain {
    pub domain: LogString,
    pub comment: LogString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterIp {
    pub ip: std::net::IpAddr,
    pub comment: LogString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsolateEndpoint {
    pub hostname: LogString,
    pub comment: LogString,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsolateIp {
    pub ip: std::net::IpAddr,
    pub comment: LogString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum LogUserIn {
    Password(LogUserInPass),
    ApiKey(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogUserInPass {
    pub username: LogString,
    pub password: LogString,
}

impl Command for LogUserIn {}