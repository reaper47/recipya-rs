use schemars::JsonSchema;
use serde::Deserialize;

use crate::intangible::structured_value::PostalAddress;

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub enum PostalAddressOrText {
    PostalAddress(PostalAddress),
    Text(String),
}

impl Default for PostalAddressOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

