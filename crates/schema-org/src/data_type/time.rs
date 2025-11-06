use schemars::JsonSchema;
use serde::Deserialize;

/// A point in time recurring on multiple days in the form hh:mm:ss[Z|(+|-)hh:mm]
/// (see XML schema for details).
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Time(String);
