use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PropertyValueSpecificationAdditionalTypeFieldEnum = String;
///<https://schema.org/PropertyValueSpecification>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PropertyValueSpecification {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/valueName>
    #[serde(rename = "valueName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_name: Vec<String>,
    ///<https://schema.org/valueMinLength>
    #[serde(rename = "valueMinLength")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_min_length: Vec<f32>,
    ///<https://schema.org/minValue>
    #[serde(rename = "minValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub min_value: Vec<f32>,
    ///<https://schema.org/valueRequired>
    #[serde(rename = "valueRequired")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_required: Vec<String>,
    ///<https://schema.org/maxValue>
    #[serde(rename = "maxValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub max_value: Vec<f32>,
    ///<https://schema.org/defaultValue>
    #[serde(rename = "defaultValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub default_value: Vec<PropertyValueSpecificationDefaultValueFieldEnum>,
    ///<https://schema.org/readonlyValue>
    #[serde(rename = "readonlyValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub readonly_value: Vec<String>,
    ///<https://schema.org/valuePattern>
    #[serde(rename = "valuePattern")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_pattern: Vec<String>,
    ///<https://schema.org/multipleValues>
    #[serde(rename = "multipleValues")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub multiple_values: Vec<String>,
    ///<https://schema.org/valueMaxLength>
    #[serde(rename = "valueMaxLength")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_max_length: Vec<f32>,
    ///<https://schema.org/stepValue>
    #[serde(rename = "stepValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub step_value: Vec<f32>,
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
    pub additional_type: Vec<PropertyValueSpecificationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<PropertyValueSpecificationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<PropertyValueSpecificationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<PropertyValueSpecificationDescriptionFieldEnum>,
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
    pub subject_of: Vec<PropertyValueSpecificationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<PropertyValueSpecificationMainEntityOfPageFieldEnum>,
}
