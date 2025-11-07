use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::MedicalEntity;
use crate::thing::medical_entity::medical_condition::{MedicalCondition, MedicalSignOrSymptom};

/// An alternative, closely-related condition typically considered later in the differential
/// diagnosis process along with the signs that are used to distinguish it.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DDxElement {
    /// One or more alternative conditions considered in the differential diagnosis process as
    /// output of a diagnosis process.
    pub diagnosis: MedicalCondition,
    /// One of a set of signs and symptoms that can be used to distinguish this diagnosis from
    /// others in the differential diagnosis.
    pub distinguishing_sign: MedicalSignOrSymptom,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
