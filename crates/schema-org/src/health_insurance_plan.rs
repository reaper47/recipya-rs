use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::contact_point::ContactPoint;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::health_plan_formulary::HealthPlanFormulary;
use crate::health_plan_network::HealthPlanNetwork;

///<https://schema.org/usesHealthPlanIdStandard>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type HealthInsurancePlanUsesHealthPlanIdStandardFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type HealthInsurancePlanAdditionalTypeFieldEnum = String;
///<https://schema.org/HealthInsurancePlan>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HealthInsurancePlan {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/healthPlanDrugTier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub health_plan_drug_tier: Vec<String>,
    ///<https://schema.org/contactPoint>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_point: Vec<ContactPoint>,
    ///<https://schema.org/includesHealthPlanFormulary>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes_health_plan_formulary: Vec<HealthPlanFormulary>,
    ///<https://schema.org/usesHealthPlanIdStandard>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub uses_health_plan_id_standard: Vec<
        HealthInsurancePlanUsesHealthPlanIdStandardFieldEnum,
    >,
    ///<https://schema.org/healthPlanDrugOption>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub health_plan_drug_option: Vec<String>,
    ///<https://schema.org/includesHealthPlanNetwork>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes_health_plan_network: Vec<HealthPlanNetwork>,
    ///<https://schema.org/healthPlanMarketingUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub health_plan_marketing_url: Vec<String>,
    ///<https://schema.org/benefitsSummaryUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub benefits_summary_url: Vec<String>,
    ///<https://schema.org/healthPlanId>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub health_plan_id: Vec<String>,
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
    pub additional_type: Vec<HealthInsurancePlanAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<HealthInsurancePlanIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<HealthInsurancePlanImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<HealthInsurancePlanDescriptionFieldEnum>,
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
    pub subject_of: Vec<HealthInsurancePlanSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<HealthInsurancePlanMainEntityOfPageFieldEnum>,
}
