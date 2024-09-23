use serde::{Serialize, Deserialize};

use crate::impl_parse_str;

#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash, PartialEq)]
pub struct Domain(pub String);

impl_parse_str!(Domain);