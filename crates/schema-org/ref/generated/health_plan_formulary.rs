use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type HealthPlanFormularyAdditionalTypeFieldEnum = String;
///<https://schema.org/HealthPlanFormulary>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct HealthPlanFormulary {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/healthPlanDrugTier>
    #[serde(rename = "healthPlanDrugTier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub health_plan_drug_tier: Vec<String>,
    ///<https://schema.org/offersPrescriptionByMail>
    #[serde(rename = "offersPrescriptionByMail")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers_prescription_by_mail: Vec<String>,
    ///<https://schema.org/healthPlanCostSharing>
    #[serde(rename = "healthPlanCostSharing")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub health_plan_cost_sharing: Vec<String>,
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
    pub additional_type: Vec<HealthPlanFormularyAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<HealthPlanFormularyIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<HealthPlanFormularyImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<HealthPlanFormularyDescriptionFieldEnum>,
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
    pub subject_of: Vec<HealthPlanFormularySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<HealthPlanFormularyMainEntityOfPageFieldEnum>,
}
