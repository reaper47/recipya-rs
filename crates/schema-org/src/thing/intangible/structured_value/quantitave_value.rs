use schemars::JsonSchema;
use serde::Deserialize;

/// A point value or interval for product characteristics and other purposes.
#[derive(Debug, Deserialize, PartialEq, JsonSchema, Default)]
pub struct QuantitativeValue {
    pub value: i64,
}
