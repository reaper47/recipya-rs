use schemars::JsonSchema;
use serde::Deserialize;

/// A point value or interval for product characteristics and other purposes.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub struct QuantitativeValue {
    pub value: i64,
}
