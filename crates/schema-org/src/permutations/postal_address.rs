use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::intangible::structured_value::contact_point::PostalAddress;

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub enum PostalAddressOrText {
    PostalAddress(Box<PostalAddress>),
    Text(String),
}

impl Default for PostalAddressOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
