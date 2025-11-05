use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Text representing a CSS selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[serde(untagged)]
pub enum CssSelectorType {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for CssSelectorType {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

impl From<String> for CssSelectorType {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl From<Vec<String>> for CssSelectorType {
    fn from(v: Vec<String>) -> Self {
        Self::Multiple(v)
    }
}

impl From<Vec<&str>> for CssSelectorType {
    fn from(v: Vec<&str>) -> Self {
        Self::Multiple(v.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&str> for CssSelectorType {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}
