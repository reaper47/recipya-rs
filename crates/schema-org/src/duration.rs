use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

use crate::field::DurationDescriptionFieldEnum;
use crate::helpers::{is_smallvec_empty, one_or_many};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum DurationOrText {
    Duration(Duration),
    Text(String),
}

impl DurationOrText {
    /// Creates a new DurationOrText::Text.
    pub fn new_text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }

    /// Converts the DurationOrText to iso8601.
    pub fn to_iso8601(&self) -> Option<iso8601::Duration> {
        match self {
            Self::Duration(d) => d
                .name
                .first()
                .map(|d| iso8601::duration(d).ok())
                .unwrap_or_default(),
            Self::Text(s) => iso8601::duration(s).ok(),
        }
    }
}

///<https://schema.org/Duration>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[DurationDescriptionFieldEnum; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
}

impl From<iso8601::Duration> for Duration {
    fn from(value: iso8601::Duration) -> Self {
        Self {
            name: smallvec![value.to_string()],
            ..Default::default()
        }
    }
}

impl From<iso8601::Duration> for DurationOrText {
    fn from(value: iso8601::Duration) -> Self {
        Self::new_text(value.to_string())
    }
}
