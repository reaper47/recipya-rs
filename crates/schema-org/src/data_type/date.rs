use schemars::JsonSchema;
use serde::Deserialize;

/// A date value in ISO 8601 date format.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Date(String);

impl Date {
    /// Creates a new DateTime from a string adhering to ISO 8601.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let dt = iso8601::date(&value.into())?;
        Ok(Self(dt.to_string()))
    }
}
