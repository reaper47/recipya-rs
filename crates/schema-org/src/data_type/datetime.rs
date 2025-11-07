use schemars::JsonSchema;
use serde::Deserialize;

/// A combination of date and time of day in the form [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm]
/// (see Chapter 5.4 of ISO 8601).
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub struct DateTime(String);

impl DateTime {
    /// Creates a new DateTime from a string adhering to ISO 8601.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let dt = iso8601::datetime(&value.into())?;
        Ok(Self(dt.to_string()))
    }
}
