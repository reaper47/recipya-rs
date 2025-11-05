use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::intangible::enumeration::MedicalEvidenceLevel;
use crate::thing::MedicalEntity;
use crate::types::date::Date;

/// Any recommendation made by a standard society (e.g. ACC/AHA) or consensus statement that denotes
/// how to diagnose and treat a particular condition. Note: this type should be used to tag the
/// actual guideline recommendation; if the guideline recommendation occurs in a larger scholarly
/// article, use MedicalScholarlyArticle to tag the overall article, not this type. Note also: the
/// organization making the recommendation should be captured in the recognizingAuthority base
/// property of MedicalEntity.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalGuideline {
    /// Strength of evidence of the data used to formulate the guideline (enumerated).
    pub evidence_level: 	MedicalEvidenceLevel,
    /// Source of the data used to formulate the guidance, e.g. RCT, consensus opinion, etc.
    pub evidence_origin: 	String,
    /// Date on which this guideline's recommendation was made.
    pub guideline_date: 	Date,
    /// The medical conditions, treatments, etc. that are the subject of the guideline.
    pub guideline_subject: 	MedicalEntity,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
