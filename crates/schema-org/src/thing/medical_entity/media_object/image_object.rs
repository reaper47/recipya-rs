use schemars::JsonSchema;
use serde::Deserialize;

/// An image file.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageObject {}
