use schemars::JsonSchema;
use serde::Deserialize;

use crate::intangible::enumeration::MedicalProcedureType;
use crate::permutations::medical::MedicalEntityOrText;
use crate::thing::MedicalEntity;

pub mod therapeutic_procedure;

#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalProcedure {
    /// Location in the body of the anatomical structure.
    pub body_location: String,
    /// Typical or recommended followup care after the procedure is performed.
    pub followup: String,
    /// How the procedure is performed.
    pub how_performed: String,
    /// Typical preparation that a patient must undergo before having the procedure performed.
    pub preparation: MedicalEntityOrText,
    /// The type of procedure, for example Surgical, Noninvasive, or Percutaneous.
    pub procedure_type: MedicalProcedureType,
    /// The status of the study (enumerated).
    pub status: EventStatusTypeOrMedicalStudyStatusOrText,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
