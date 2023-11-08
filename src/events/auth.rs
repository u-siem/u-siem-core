use crate::prelude::{types::LogString, SiemLog, SiemField, SiemIp};
use serde::{Deserialize, Serialize};

use super::field_dictionary::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthEvent {
    /// Login type: local, remote, upgrade (change user)
    pub login_type: AuthLoginType,
    /// Login success or failed
    pub outcome: LoginOutcome,
    /// Host where the login happened
    pub hostname: LogString,
}

impl AuthEvent {
    pub fn login_type(&self) -> &AuthLoginType {
        &self.login_type
    }
    pub fn outcome(&self) -> &LoginOutcome {
        &self.outcome
    }
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AuthLoginType {
    Local(LocalLogin),
    Remote(RemoteLogin),
    Upgrade(UpgradeLogin),
    Validation(ValidationLogin),
    Delegation(DelegationLogin),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum LoginOutcome {
    /// Login success
    SUCCESS,
    /// Login failed
    FAIL,
    /// Account locked out
    LOCKOUT,
    /// Pre authentication phase: trying to connect
    ESTABLISH,
}

/// A user is login in locally, in front of the computer (or almost).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LocalLogin {
    /// User that logged in
    pub user_name: LogString,
    /// User domain
    pub domain: LogString,
}

impl LocalLogin {
    pub fn new<S>(user_name: S, domain: S) -> Self
    where
        S: Into<LogString>,
    {
        Self {
            user_name: user_name.into(),
            domain: domain.into(),
        }
    }
}

/// A user uses a Credential Vault like CyberArk to use an account
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DelegationLogin {
    /// Original user name
    pub source_user: LogString,
    pub source_domain: LogString,
    /// User to be logged as
    pub destination_user: LogString,
    /// Domain of the user to be logged as
    pub destination_domain: LogString,
}

impl DelegationLogin {
    pub fn new<S>(
        source_user: S,
        source_domain: S,
        destination_user: S,
        destination_domain: S,
    ) -> Self
    where
        S: Into<LogString>,
    {
        Self {
            source_user: source_user.into(),
            source_domain: source_domain.into(),
            destination_domain: destination_domain.into(),
            destination_user: destination_user.into(),
        }
    }
}

/// Someone tries to login in the system from another computer.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RemoteLogin {
    /// User that logged in
    pub user_name: LogString,
    /// User domain
    pub domain: LogString,
    /// Ip or hostname of the remote location
    pub source_address: LogString,
}

impl RemoteLogin {
    pub fn new<S>(user_name: S, domain: S, source_address: S) -> Self
    where
        S: Into<LogString>,
    {
        Self {
            user_name: user_name.into(),
            domain: domain.into(),
            source_address: source_address.into(),
        }
    }
}

/// A user changes into another account, like a "su" command in linux
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UpgradeLogin {
    /// Original user name
    pub source_user: LogString,
    /// User to be logged as
    pub destination_user: LogString,
    /// Domain of the user to be logged as
    pub destination_domain: LogString,
}

impl UpgradeLogin {
    pub fn new<S>(source_user: S, destination_user: S, destination_domain: S) -> Self
    where
        S: Into<LogString>,
    {
        Self {
            source_user: source_user.into(),
            destination_domain: destination_domain.into(),
            destination_user: destination_user.into(),
        }
    }
}

/// This does not imply a user login in the system, only validation of credentials. Like LoginType=3 in Windows.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ValidationLogin {
    /// User doing the login
    pub user_name: LogString,
    /// Origin og the connection
    pub source_address: LogString,
}

impl ValidationLogin {
    pub fn new<S>(user_name: S, source_address: S) -> Self
    where
        S: Into<LogString>,
    {
        Self {
            user_name: user_name.into(),
            source_address: source_address.into(),
        }
    }
}

impl std::fmt::Display for AuthLoginType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
impl std::fmt::Display for LoginOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


impl Into<SiemLog> for AuthEvent {
    fn into(self) -> SiemLog {
        let mut log = SiemLog::new("", 0, "");
        log.add_field(
            "host.hostname",
            SiemField::Text(self.hostname),
        );
        log.add_field(
            EVENT_OUTCOME,
            SiemField::Text(LogString::Owned(self.outcome.to_string())),
        );
        match self.login_type {
            AuthLoginType::Local(evnt) => {
                log.add_field(
                    USER_NAME,
                    SiemField::User(evnt.user_name.to_string()),
                );
                log.add_field(
                    USER_DOMAIN,
                    SiemField::Domain(evnt.domain.to_string()),
                );
            }
            AuthLoginType::Remote(evnt) => {
                log.add_field(
                    USER_NAME,
                    SiemField::User(evnt.user_name.to_string()),
                );
                log.add_field(
                    USER_DOMAIN,
                    SiemField::Domain(evnt.domain.to_string()),
                );
                match SiemIp::from_ip_str(&evnt.source_address) {
                    Ok(ip) => {
                        log.add_field(SOURCE_IP, SiemField::IP(ip));
                    }
                    Err(_) => {}
                };
                log.add_field(
                    "source.address",
                    SiemField::Text(evnt.source_address),
                );
            }
            AuthLoginType::Upgrade(evnt) => {
                log.add_field(
                    USER_NAME,
                    SiemField::User(evnt.destination_user.to_string()),
                );
                log.add_field(
                    "source.user.name",
                    SiemField::User(evnt.source_user.to_string()),
                );
                log.add_field(
                    USER_DOMAIN,
                    SiemField::Domain(evnt.destination_domain.to_string()),
                );
            }
            AuthLoginType::Validation(evnt) => {
                log.add_field(
                    USER_NAME,
                    SiemField::User(evnt.user_name.to_string()),
                );
                match SiemIp::from_ip_str(&evnt.source_address) {
                    Ok(ip) => {
                        log.add_field("source.ip", SiemField::IP(ip));
                    }
                    Err(_) => {}
                };
                log.add_field(
                    "source.address",
                    SiemField::Text(evnt.source_address),
                );
            }
            AuthLoginType::Delegation(evnt) => {
                log.add_field(
                    USER_NAME,
                    SiemField::User(evnt.destination_user.to_string()),
                );
                log.add_field(
                    "source.user.name",
                    SiemField::User(evnt.source_user.to_string()),
                );
                log.add_field(
                    USER_DOMAIN,
                    SiemField::Domain(evnt.destination_domain.to_string()),
                );
                log.add_field(
                    "source.user.domain",
                    SiemField::Domain(evnt.source_domain.to_string()),
                );
            }
        };
        log
    }
}