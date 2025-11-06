use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::Person;
use crate::thing::place::CivicStructure;

/// An educational organization.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EducationalOrganization {
    /// Alumni of an organization.
    ///
    /// Inverse property: alumniOf
    pub alumni: Person,
    #[serde(flatten)]
    pub civic_structure: CivicStructure,
}
