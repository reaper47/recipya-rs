use serde::{Deserialize, Serialize};

use crate::field::DurationDescriptionFieldEnum;
use crate::helpers::one_or_many;
use crate::{AtType, at_context};

/// Converts a value to an ISO 8601 duration string representation.
pub trait ToIso8601 {
    fn to_is8601_duration(&self) -> Option<iso8601::Duration>;
}

impl ToIso8601 for Vec<DurationOrText> {
    fn to_is8601_duration(&self) -> Option<iso8601::Duration> {
        self.first().map(|d| d.to_iso8601()).unwrap_or_default()
    }
}

/// A flexible duration representation that can be either a structured duration or a text string.
///
/// This enum allows for parsing duration values that may come in different formats,
/// such as from configuration files or API responses where durations might be
/// specified either as structured data or as ISO 8601 duration strings.
///
/// # Variants
///
/// * `Duration` - A structured duration value
/// * `Text` - A string representation of a duration (e.g. ISO 8601 format)
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
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
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
            r#type: AtType::Duration.to_opt(),
            context: at_context(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_text() {
        let s = "20250607";

        let got = DurationOrText::new_text(s);

        assert_eq!(got, DurationOrText::Text(s.into()));
    }

    #[test]
    fn test_to_iso8601() {
        let s = "P1Y2M3DT4H5M6S";
        let d = DurationOrText::new_text(s);

        let got = d.to_iso8601();

        assert_eq!(got, iso8601::duration(s).ok());
    }
}
