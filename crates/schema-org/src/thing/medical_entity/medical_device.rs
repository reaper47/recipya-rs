use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::medical::MedicalContraindicationOrText;
use crate::thing::MedicalEntity;

/// Any object used in a medical capacity, such as to diagnose or treat a patient.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalDevice {
    /// A possible complication and/or side effect of this therapy. If it is known that an adverse
    /// outcome is serious (resulting in death, disability, or permanent damage; requiring
    /// hospitalization; or otherwise life-threatening or requiring immediate medical attention),
    /// tag it as a seriousAdverseOutcome instead.
    pub adverse_outcome: MedicalEntity,
    /// A contraindication for this therapy.
    pub contraindication: MedicalContraindicationOrText,
    /// A description of the postoperative procedures, care, and/or followups for this device.
    pub post_op: String,
    /// A description of the workup, testing, and other preparations required before implanting
    /// this device.
    pub pre_op: String,
    /// A description of the procedure involved in setting up, using, and/or installing the device.
    pub procedure: String,
    /// A possible serious complication and/or serious side effect of this therapy. Serious adverse
    /// outcomes include those that are life-threatening; result in death, disability, or permanent
    /// damage; require hospitalization or prolong existing hospitalization; cause congenital
    /// anomalies or birth defects; or jeopardize the patient and may require medical or surgical
    /// intervention to prevent one of the outcomes in this definition.
    pub serious_adverse_outcome: MedicalEntity,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
