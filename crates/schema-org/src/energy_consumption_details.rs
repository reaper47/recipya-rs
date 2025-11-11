use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::Action;
use crate::energy_efficiency_enumeration::EnergyEfficiencyEnumeration;
use crate::enums::EUEnergyEfficiencyEnumerationEnum;
use crate::helpers::one_or_many;
use crate::field::*;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EnergyConsumptionDetailsAdditionalTypeFieldEnum = String;

///<https://schema.org/EnergyConsumptionDetails>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EnergyConsumptionDetails {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/hasEnergyEfficiencyCategory>
    #[serde(rename = "hasEnergyEfficiencyCategory")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_energy_efficiency_category: Vec<EnergyEfficiencyEnumeration>,
    ///<https://schema.org/energyEfficiencyScaleMax>
    #[serde(rename = "energyEfficiencyScaleMax")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub energy_efficiency_scale_max: Vec<EUEnergyEfficiencyEnumerationEnum>,
    ///<https://schema.org/energyEfficiencyScaleMin>
    #[serde(rename = "energyEfficiencyScaleMin")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub energy_efficiency_scale_min: Vec<EUEnergyEfficiencyEnumerationEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<EnergyConsumptionDetailsAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<EnergyConsumptionDetailsIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<EnergyConsumptionDetailsImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<EnergyConsumptionDetailsDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<EnergyConsumptionDetailsSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<EnergyConsumptionDetailsMainEntityOfPageFieldEnum>,
}
