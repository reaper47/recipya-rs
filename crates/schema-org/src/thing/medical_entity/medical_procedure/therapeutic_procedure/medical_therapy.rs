use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::medical::MedicalContraindicationOrText;
use crate::thing::medical_entity::medical_procedure::therapeutic_procedure::TherapeuticProcedure;
use crate::thing::MedicalEntity;

/// Any medical intervention designed to prevent, treat, and cure human diseases and medical
/// conditions, including both curative and palliative therapies. Medical therapies are typically
/// processes of care relying upon pharmacotherapy, behavioral therapy, supportive therapy (with
/// fluid or nutrition for example), or detoxification (e.g. hemodialysis) aimed at improving or
/// preventing a health condition.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalTherapy {
    /// A contraindication for this therapy.
    pub contraindication: 	MedicalContraindicationOrText,
    /// A therapy that duplicates or overlaps this one.
    pub duplicate_therapy: 	Box<MedicalTherapy>,
    /// A possible serious complication and/or serious side effect of this therapy. Serious adverse
    /// outcomes include those that are life-threatening; result in death, disability, or permanent
    /// damage; require hospitalization or prolong existing hospitalization; cause congenital
    /// anomalies or birth defects; or jeopardize the patient and may require medical or surgical
    /// intervention to prevent one of the outcomes in this definition.
    pub serious_adverse_outcome: 	MedicalEntity,
    #[serde(flatten)]
    pub therapeutic_procedure: TherapeuticProcedure,
}
