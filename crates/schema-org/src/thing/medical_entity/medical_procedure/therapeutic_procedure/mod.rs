mod medical_therapy;

pub use medical_therapy::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::MedicalEntity;

/// A medical procedure intended primarily for therapeutic purposes, aimed at improving
/// a health condition.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TherapeuticProcedure {
    /// A possible complication and/or side effect of this therapy. If it is known that an adverse
    /// outcome is serious (resulting in death, disability, or permanent damage; requiring
    /// hospitalization; or otherwise life-threatening or requiring immediate medical attention),
    /// tag it as a seriousAdverseOutcome instead.
    pub adverse_outcome: MedicalEntity,
    /// A dosing schedule for the drug for a given population, either observed, recommended, or
    /// maximum dose based on the type used.
    pub dose_schedule: DoseSchedule,
    /// Specifying a drug or medicine used in a medication procedure.
    pub drug: Drug,
    #[serde(flatten)]
    pub medical_procedure: MedicalProcedure,
}
