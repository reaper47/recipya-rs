mod ddx_element;
mod dose_schedule;
mod drug_strength;

pub use ddx_element::*;
pub use dose_schedule::*;
pub use drug_strength::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::MedicalEntity;
use crate::thing::intangible::defined_term::CategoryCode;

/// A code for a medical entity.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalCode {
    /// A short textual code that uniquely identifies the value.
    pub code_value: String,
    /// The coding system, e.g. 'ICD-10'.
    pub coding_system: String,
    #[serde(flatten)]
    pub category_code: CategoryCode,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
