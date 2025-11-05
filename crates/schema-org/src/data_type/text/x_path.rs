use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Text representing an XPath (typically but not necessarily version 1.0).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum XPathType {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for XPathType {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

impl From<String> for XPathType {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl From<Vec<String>> for XPathType {
    fn from(v: Vec<String>) -> Self {
        Self::Multiple(v)
    }
}

impl From<Vec<&str>> for XPathType {
    fn from(v: Vec<&str>) -> Self {
        Self::Multiple(v.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&str> for XPathType {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}
