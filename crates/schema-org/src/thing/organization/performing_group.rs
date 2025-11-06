use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::Organization;

/// A performance group, such as a band, an orchestra, or a circus.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PerformingGroup {
    pub organization: Organization,
}
