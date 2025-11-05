use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::medical_entity::medical_condition::MedicalCondition;
use crate::thing::medical_entity::MedicalDevice;
use crate::thing::MedicalEntity;

/// Any medical test, typically performed for diagnostic purposes.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalTest {
    /// Drugs that affect the test's results.
    pub affected_by: 	Drug,
    /// Range of acceptable values for a typical patient, when applicable.
    pub normal_range: 	MedicalEnumerationOrText,
    /// A sign detected by the test.
    pub sign_detected: 	MedicalSign,
    /// A condition the test is used to diagnose.
    pub used_to_diagnose: 	MedicalCondition,
    /// Device used to perform the test.
    pub uses_device: 	MedicalDevice,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
