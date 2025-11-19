use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::field::DurationDescriptionFieldEnum;
use crate::helpers::one_or_many;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum DurationOrText {
    Duration(Duration),
    Text(String),
}

impl DurationOrText {
    pub fn new_text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }
}

///<https://schema.org/Duration>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<DurationDescriptionFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
}

impl From<iso8601::Duration> for Duration {
    fn from(value: iso8601::Duration) -> Self {
        Self {
            name: vec![value.to_string()],
            ..Default::default()
        }
    }
}

impl From<iso8601::Duration> for DurationOrText {
    fn from(value: iso8601::Duration) -> Self {
        Self::new_text(value.to_string())
    }
}
