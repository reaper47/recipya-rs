use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::medical_entity::MedicalCondition;
use crate::thing::medical_entity::therapeutic_procedure::MedicalTherapy;

/// Any feature associated or not with a medical condition. In medicine a symptom is generally
/// subjective while a sign is objective.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalSignOrSymptom {
    /// A possible treatment to address this condition, sign or symptom.
    pub possible_treatment: MedicalTherapy,
    #[serde(flatten)]
    pub medical_condition: MedicalCondition,
}
