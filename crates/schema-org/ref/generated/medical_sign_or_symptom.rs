use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MedicalSignOrSymptomAdditionalTypeFieldEnum = String;
///<https://schema.org/MedicalSignOrSymptom>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MedicalSignOrSymptom {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/possibleTreatment>
    #[serde(rename = "possibleTreatment")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub possible_treatment: Vec<MedicalTherapy>,
    ///<https://schema.org/differentialDiagnosis>
    #[serde(rename = "differentialDiagnosis")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub differential_diagnosis: Vec<DDxElement>,
    ///<https://schema.org/possibleComplication>
    #[serde(rename = "possibleComplication")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub possible_complication: Vec<String>,
    ///<https://schema.org/status>
    #[serde(rename = "status")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub status: Vec<MedicalSignOrSymptomStatusFieldEnum>,
    ///<https://schema.org/drug>
    #[serde(rename = "drug")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub drug: Vec<Drug>,
    ///<https://schema.org/stage>
    #[serde(rename = "stage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub stage: Vec<MedicalConditionStage>,
    ///<https://schema.org/pathophysiology>
    #[serde(rename = "pathophysiology")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pathophysiology: Vec<String>,
    ///<https://schema.org/primaryPrevention>
    #[serde(rename = "primaryPrevention")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub primary_prevention: Vec<MedicalTherapy>,
    ///<https://schema.org/epidemiology>
    #[serde(rename = "epidemiology")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub epidemiology: Vec<String>,
    ///<https://schema.org/secondaryPrevention>
    #[serde(rename = "secondaryPrevention")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub secondary_prevention: Vec<MedicalTherapy>,
    ///<https://schema.org/associatedAnatomy>
    #[serde(rename = "associatedAnatomy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub associated_anatomy: Vec<MedicalSignOrSymptomAssociatedAnatomyFieldEnum>,
    ///<https://schema.org/riskFactor>
    #[serde(rename = "riskFactor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub risk_factor: Vec<MedicalRiskFactor>,
    ///<https://schema.org/naturalProgression>
    #[serde(rename = "naturalProgression")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub natural_progression: Vec<String>,
    ///<https://schema.org/expectedPrognosis>
    #[serde(rename = "expectedPrognosis")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub expected_prognosis: Vec<String>,
    ///<https://schema.org/typicalTest>
    #[serde(rename = "typicalTest")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub typical_test: Vec<MedicalTest>,
    ///<https://schema.org/signOrSymptom>
    #[serde(rename = "signOrSymptom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sign_or_symptom: Vec<MedicalSignOrSymptom>,
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
    pub legal_status: Vec<MedicalSignOrSymptomLegalStatusFieldEnum>,
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
    pub additional_type: Vec<MedicalSignOrSymptomAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<MedicalSignOrSymptomIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<MedicalSignOrSymptomImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<MedicalSignOrSymptomDescriptionFieldEnum>,
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
    pub subject_of: Vec<MedicalSignOrSymptomSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<MedicalSignOrSymptomMainEntityOfPageFieldEnum>,
}
