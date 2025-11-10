use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EngineSpecificationAdditionalTypeFieldEnum = String;
///<https://schema.org/EngineSpecification>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct EngineSpecification {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/engineType>
    #[serde(rename = "engineType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub engine_type: Vec<EngineSpecificationEngineTypeFieldEnum>,
    ///<https://schema.org/engineDisplacement>
    #[serde(rename = "engineDisplacement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub engine_displacement: Vec<QuantitativeValue>,
    ///<https://schema.org/enginePower>
    #[serde(rename = "enginePower")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub engine_power: Vec<QuantitativeValue>,
    ///<https://schema.org/fuelType>
    #[serde(rename = "fuelType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub fuel_type: Vec<EngineSpecificationFuelTypeFieldEnum>,
    ///<https://schema.org/torque>
    #[serde(rename = "torque")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub torque: Vec<QuantitativeValue>,
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
    pub additional_type: Vec<EngineSpecificationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<EngineSpecificationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<EngineSpecificationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<EngineSpecificationDescriptionFieldEnum>,
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
    pub subject_of: Vec<EngineSpecificationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<EngineSpecificationMainEntityOfPageFieldEnum>,
}
