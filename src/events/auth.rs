use super::field::SiemIp;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub struct AuthEvent {
    /// Login type: local, remote, upgrade (change user)
    pub login_type: AuthLoginType,
    /// Login success or failed
    pub outcome : LoginOutcome,
    /// Host where the login happened
    pub hostname : Cow<'static, str>
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum AuthLoginType {
    Local(LocalLogin),
    Remote(RemoteLogin),
    Upgrade(UpgradeLogin)
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum LoginOutcome {
    /// Login success
    SUCESS,
    /// Login failed
    FAIL,
    /// Account locked out
    LOCKOUT
}
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LocalLogin {
    /// User that logged in
    user_name : Cow<'static, str>,
    /// User domain
    domain : Cow<'static, str>
}
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RemoteLogin {
    /// User that logged in
    user_name : Cow<'static, str>,
    /// User domain
    domain : Cow<'static, str>,
    /// Ip from where the login come
    source_ip : SiemIp
}
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct UpgradeLogin {
    /// Original user name
    source_user : Cow<'static, str>,
    /// User to be logged as
    destination_user : Cow<'static, str>,
    /// Domain of the user to be logged as
    destination_domain : Cow<'static, str>,
}


impl std::fmt::Display for AuthLoginType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
