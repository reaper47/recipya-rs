use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Quantity: Duration (use ISO 8601 duration format).
#[derive(Debug, Default, PartialEq, JsonSchema)]
pub struct Duration {
    #[serde(skip)]
    #[schemars(skip)]
    parsed: iso8601::Duration,
    original: String,
}

impl Duration {
    /// Create a new Duration from an ISO 8601 duration string
    pub fn new(value: impl AsRef<str>) -> Result<Self, String> {
        let s = value.as_ref();
        let parsed = iso8601::duration(s)?;
        Ok(Self {
            parsed,
            original: s.into(),
        })
    }

    /// Get the parsed duration
    pub fn parsed(&self) -> &iso8601::Duration {
        &self.parsed
    }

    /// Get the original duration string
    pub fn as_str(&self) -> &str {
        &self.original
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.original)
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<String> for Duration {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Duration {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<str> for Duration {
    fn as_ref(&self) -> &str {
        &self.original
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}
