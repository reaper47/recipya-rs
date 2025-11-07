use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::thing::intangible::enumeration::{MedicalSpecialty, MedicineSystem};
use crate::thing::intangible::grant::Grant;
use crate::thing::medical_entity::MedicalStudy;
use crate::thing::medical_entity::medical_guideline::MedicalGuideline;
use crate::thing::medical_entity::medical_intangible::MedicalCode;
use crate::thing::organization::Organization;

/// The most generic type of entity related to health and the practice of medicine.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalEntity {
    /// A medical code for the entity, taken from a controlled vocabulary or ontology such as
    /// ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
    pub code: Box<MedicalCode>,
    /// A Grant that directly or indirectly provide funding or sponsorship for this item. See
    /// also ownershipFundingInfo.
    ///
    /// Inverse property: fundedItem
    pub funding: Grant,
    /// A medical guideline related to this entity.
    pub guideline: Box<MedicalGuideline>,
    /// The drug or supplement's legal status, including any controlled substance schedules that
    /// apply.
    pub legal_status: DrugLegalStatusOrMedicalEnumerationOrText,
    /// The system of medicine that includes this MedicalEntity, for example 'evidence-based',
    /// 'homeopathic', 'chiropractic', etc.
    pub medicine_system: MedicineSystem,
    /// If applicable, the organization that officially recognizes this entity as part of its
    /// endorsed system of medicine.
    pub recognizing_authority: Organization,
    /// If applicable, a medical specialty in which this entity is relevant.
    pub relevant_specialty: MedicalSpecialty,
    /// A medical study or trial related to this entity.
    pub study: MedicalStudy,
    #[serde(flatten)]
    pub thing: Thing,
}
