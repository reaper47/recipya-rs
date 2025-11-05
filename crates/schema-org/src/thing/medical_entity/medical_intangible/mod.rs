use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::intangible::defined_term::CategoryCode;
use crate::thing::intangible::enumeration::MedicalEvidenceLevel;
use crate::thing::MedicalEntity;
use crate::types::date::Date;

/// A code for a medical entity.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalCode {
    /// A short textual code that uniquely identifies the value.
    pub code_value: 	String,
    /// The coding system, e.g. 'ICD-10'.
    pub coding_system: 	String,
    #[serde(flatten)]
    pub category_code: CategoryCode,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}

