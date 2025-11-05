use std::str::FromStr;

use schemars::JsonSchema;
use serde::Deserialize;
use url::ParseError;

/// Data type: URL.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub struct URL(String);

impl URL {
    /// Creates a new URL after validation.
    pub fn new(value: impl Into<String>) -> Result<Self, ParseError> {
        let url_str = value.into();
        let url = url::Url::parse(&url_str)?;
        Ok(Self(url.to_string()))
    }

    /// Returns the URL as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the URL and returns the underlying String.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl FromStr for URL {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for URL {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<URL> for String {
    fn from(url: URL) -> Self {
        url.0
    }
}
