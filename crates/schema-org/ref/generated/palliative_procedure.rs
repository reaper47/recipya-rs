use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PalliativeProcedureAdditionalTypeFieldEnum = String;
///<https://schema.org/PalliativeProcedure>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PalliativeProcedure {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/procedureType>
    #[serde(rename = "procedureType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub procedure_type: Vec<MedicalProcedureTypeEnum>,
    ///<https://schema.org/status>
    #[serde(rename = "status")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub status: Vec<PalliativeProcedureStatusFieldEnum>,
    ///<https://schema.org/howPerformed>
    #[serde(rename = "howPerformed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub how_performed: Vec<String>,
    ///<https://schema.org/bodyLocation>
    #[serde(rename = "bodyLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub body_location: Vec<String>,
    ///<https://schema.org/preparation>
    #[serde(rename = "preparation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub preparation: Vec<PalliativeProcedurePreparationFieldEnum>,
    ///<https://schema.org/followup>
    #[serde(rename = "followup")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub followup: Vec<String>,
    ///<https://schema.org/code>
    #[serde(rename = "code")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub code: Vec<MedicalCode>,
    ///<https://schema.org/study>
    #[serde(rename = "study")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub study: Vec<MedicalStudy>,
    ///<https://schema.org/funding>
    #[serde(rename = "funding")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub funding: Vec<Grant>,
    ///<https://schema.org/recognizingAuthority>
    #[serde(rename = "recognizingAuthority")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub recognizing_authority: Vec<Organization>,
    ///<https://schema.org/guideline>
    #[serde(rename = "guideline")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub guideline: Vec<MedicalGuideline>,
    ///<https://schema.org/medicineSystem>
    #[serde(rename = "medicineSystem")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub medicine_system: Vec<MedicineSystemEnum>,
    ///<https://schema.org/relevantSpecialty>
    #[serde(rename = "relevantSpecialty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub relevant_specialty: Vec<MedicalSpecialtyEnum>,
    ///<https://schema.org/legalStatus>
    #[serde(rename = "legalStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub legal_status: Vec<PalliativeProcedureLegalStatusFieldEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_type: Vec<PalliativeProcedureAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<PalliativeProcedureIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<PalliativeProcedureImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<PalliativeProcedureDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub subject_of: Vec<PalliativeProcedureSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<PalliativeProcedureMainEntityOfPageFieldEnum>,
    ///<https://schema.org/contraindication>
    #[serde(rename = "contraindication")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contraindication: Vec<PalliativeProcedureContraindicationFieldEnum>,
    ///<https://schema.org/duplicateTherapy>
    #[serde(rename = "duplicateTherapy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub duplicate_therapy: Vec<MedicalTherapy>,
    ///<https://schema.org/seriousAdverseOutcome>
    #[serde(rename = "seriousAdverseOutcome")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub serious_adverse_outcome: Vec<MedicalEntity>,
    ///<https://schema.org/drug>
    #[serde(rename = "drug")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub drug: Vec<Drug>,
    ///<https://schema.org/adverseOutcome>
    #[serde(rename = "adverseOutcome")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub adverse_outcome: Vec<MedicalEntity>,
    ///<https://schema.org/doseSchedule>
    #[serde(rename = "doseSchedule")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub dose_schedule: Vec<DoseSchedule>,
}
