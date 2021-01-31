use std::borrow::Cow;
use serde::{ Serialize};

#[derive(Serialize, Debug, PartialEq,Clone)]
#[serde(tag = "type")]
pub enum NetworkProtocol {
    UDP,
    TCP,
    ICMP,
    OTHER(Cow<'static, str>)
}