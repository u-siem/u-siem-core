use std::borrow::Cow;
use serde::{ Serialize};

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum NetworkProtocol {
    UDP,
    TCP,
    ICMP,
    OTHER(Cow<'static, str>)
}