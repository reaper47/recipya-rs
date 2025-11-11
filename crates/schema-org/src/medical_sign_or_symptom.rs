use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::d_dx_element::DDxElement;
use crate::drug::Drug;
use crate::enums::{MedicalSpecialtyEnum, MedicineSystemEnum};
use crate::helpers::one_or_many;
use crate::field::*;
use crate::grant::Grant;
use crate::medical_code::MedicalCode;
use crate::medical_condition_stage::MedicalConditionStage;
use crate::medical_guideline::MedicalGuideline;
use crate::medical_risk_factor::MedicalRiskFactor;
use crate::medical_study::MedicalStudy;
use crate::medical_test::MedicalTest;
use crate::medical_therapy::MedicalTherapy;
use crate::organization::Organization;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MedicalSignOrSymptomAdditionalTypeFieldEnum = String;

///<https://schema.org/MedicalSignOrSymptom>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalSignOrSymptom {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/possibleTreatment>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub possible_treatment: Vec<MedicalTherapy>,
    ///<https://schema.org/differentialDiagnosis>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub differential_diagnosis: Vec<DDxElement>,
    ///<https://schema.org/possibleComplication>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub possible_complication: Vec<String>,
    ///<https://schema.org/status>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub status: Vec<MedicalSignOrSymptomStatusFieldEnum>,
    ///<https://schema.org/drug>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub drug: Vec<Drug>,
    ///<https://schema.org/stage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stage: Vec<MedicalConditionStage>,
    ///<https://schema.org/pathophysiology>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pathophysiology: Vec<String>,
    ///<https://schema.org/primaryPrevention>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub primary_prevention: Vec<MedicalTherapy>,
    ///<https://schema.org/epidemiology>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub epidemiology: Vec<String>,
    ///<https://schema.org/secondaryPrevention>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secondary_prevention: Vec<MedicalTherapy>,
    ///<https://schema.org/associatedAnatomy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_anatomy: Vec<MedicalSignOrSymptomAssociatedAnatomyFieldEnum>,
    ///<https://schema.org/riskFactor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub risk_factor: Vec<MedicalRiskFactor>,
    ///<https://schema.org/naturalProgression>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub natural_progression: Vec<String>,
    ///<https://schema.org/expectedPrognosis>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub expected_prognosis: Vec<String>,
    ///<https://schema.org/typicalTest>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub typical_test: Vec<MedicalTest>,
    ///<https://schema.org/signOrSymptom>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sign_or_symptom: Vec<MedicalSignOrSymptom>,
    ///<https://schema.org/code>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<MedicalCode>,
    ///<https://schema.org/study>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub study: Vec<MedicalStudy>,
    ///<https://schema.org/funding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/recognizingAuthority>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recognizing_authority: Vec<Organization>,
    ///<https://schema.org/guideline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub guideline: Vec<MedicalGuideline>,
    ///<https://schema.org/medicineSystem>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub medicine_system: Vec<MedicineSystemEnum>,
    ///<https://schema.org/relevantSpecialty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub relevant_specialty: Vec<MedicalSpecialtyEnum>,
    ///<https://schema.org/legalStatus>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub legal_status: Vec<MedicalSignOrSymptomLegalStatusFieldEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<MedicalSignOrSymptomAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MedicalSignOrSymptomIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MedicalSignOrSymptomImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MedicalSignOrSymptomDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<MedicalSignOrSymptomSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<MedicalSignOrSymptomMainEntityOfPageFieldEnum>,
}
