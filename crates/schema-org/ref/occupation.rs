use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type OccupationAdditionalTypeFieldEnum = String;
///<https://schema.org/Occupation>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Occupation {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/experienceRequirements>
    #[serde(rename = "experienceRequirements")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub experience_requirements: Vec<OccupationExperienceRequirementsFieldEnum>,
    ///<https://schema.org/responsibilities>
    #[serde(rename = "responsibilities")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub responsibilities: Vec<String>,
    ///<https://schema.org/educationRequirements>
    #[serde(rename = "educationRequirements")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub education_requirements: Vec<OccupationEducationRequirementsFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub skills: Vec<OccupationSkillsFieldEnum>,
    ///<https://schema.org/occupationalCategory>
    #[serde(rename = "occupationalCategory")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub occupational_category: Vec<OccupationOccupationalCategoryFieldEnum>,
    ///<https://schema.org/estimatedSalary>
    #[serde(rename = "estimatedSalary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub estimated_salary: Vec<OccupationEstimatedSalaryFieldEnum>,
    ///<https://schema.org/qualifications>
    #[serde(rename = "qualifications")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub qualifications: Vec<OccupationQualificationsFieldEnum>,
    ///<https://schema.org/occupationLocation>
    #[serde(rename = "occupationLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub occupation_location: Vec<AdministrativeArea>,
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
    pub additional_type: Vec<OccupationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<OccupationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<OccupationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<OccupationDescriptionFieldEnum>,
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
    pub subject_of: Vec<OccupationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<OccupationMainEntityOfPageFieldEnum>,
}
