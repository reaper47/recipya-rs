use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;

/// An online or virtual location for attending events. For example, one may attend an online
/// seminar or educational event. While a virtual location may be used as the location of an event,
/// virtual locations should not be confused with physical locations in the real world.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VirtualLocation {
    pub thing: Thing,
}
