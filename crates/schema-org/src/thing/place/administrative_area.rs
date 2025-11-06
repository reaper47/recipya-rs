use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::thing::Place;

/// A geographical region, typically under the jurisdiction of a particular government.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AdministrativeArea {
    #[serde(flatten)]
    pub place: Place,
    #[serde(flatten)]
    pub thing: Thing,
}

/// A country.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Country {
    #[serde(flatten)]
    pub place: Place,
    #[serde(flatten)]
    pub thing: Thing,
}
