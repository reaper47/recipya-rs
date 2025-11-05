mod enumeration;
mod medical;
mod qualitative_value;

pub use enumeration::*;
pub use medical::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::property::ClassOrEnumerationOrProperty;
use crate::Thing;

/// Lists or enumerations—for example, a list of cuisines or music genres, etc.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Enumeration {
    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: Box<ClassOrEnumerationOrProperty>,
    #[serde(flatten)]
    pub thing: Thing,
}
