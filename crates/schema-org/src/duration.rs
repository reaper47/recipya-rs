use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AtType;
use crate::field::DurationDescriptionFieldEnum;
use crate::helpers::one_or_many;

///<https://schema.org/Duration>
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
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
    pub r#type: AtType,
}

impl From<iso8601::Duration> for Duration {
    fn from(value: iso8601::Duration) -> Self {
        Self {
            name: vec![value.to_string()],
            ..Default::default()
        }
    }
}
