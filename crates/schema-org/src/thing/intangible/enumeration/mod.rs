mod enumeration;
mod medical;
mod qualitative_value;

pub mod medical_enumeration;
pub mod status_enumeration;

pub use enumeration::*;
pub use medical::*;
pub use qualitative_value::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::property::ClassOrEnumerationOrProperty;

/// Lists or enumerations—for example, a list of cuisines or music genres, etc.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Enumeration {
    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: Box<ClassOrEnumerationOrProperty>,
    #[serde(flatten)]
    pub thing: Thing,
}
