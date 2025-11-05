use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;

/// A class, also often called a 'Type'; equivalent to rdfs:Class.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Class {
    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: 	ClassOrEnumerationOrProperty,
    #[serde(flatten)]
    pub thing: Thing,
}
