use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::enums::{MedicalSpecialtyEnum, MedicineSystemEnum};
use crate::helpers::one_or_many;
use crate::field::*;
use crate::grant::Grant;
use crate::medical_code::MedicalCode;
use crate::medical_condition::MedicalCondition;
use crate::medical_guideline::MedicalGuideline;
use crate::medical_study::MedicalStudy;
use crate::medical_therapy::MedicalTherapy;
use crate::organization::Organization;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type SuperficialAnatomyAdditionalTypeFieldEnum = String;
///<https://schema.org/SuperficialAnatomy>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SuperficialAnatomy {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/associatedPathophysiology>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_pathophysiology: Vec<String>,
    ///<https://schema.org/relatedTherapy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related_therapy: Vec<MedicalTherapy>,
    ///<https://schema.org/relatedAnatomy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related_anatomy: Vec<SuperficialAnatomyRelatedAnatomyFieldEnum>,
    ///<https://schema.org/relatedCondition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related_condition: Vec<MedicalCondition>,
    ///<https://schema.org/significance>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub significance: Vec<String>,
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
    pub legal_status: Vec<SuperficialAnatomyLegalStatusFieldEnum>,
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
    pub additional_type: Vec<SuperficialAnatomyAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<SuperficialAnatomyIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<SuperficialAnatomyImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<SuperficialAnatomyDescriptionFieldEnum>,
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
    pub subject_of: Vec<SuperficialAnatomySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<SuperficialAnatomyMainEntityOfPageFieldEnum>,
}
