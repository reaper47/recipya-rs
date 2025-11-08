use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/unitCode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type QuantitativeValueUnitCodeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type QuantitativeValueAdditionalTypeFieldEnum = String;
///<https://schema.org/QuantitativeValue>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct QuantitativeValue {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/minValue>
    #[serde(rename = "minValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub min_value: Vec<f32>,
    ///<https://schema.org/maxValue>
    #[serde(rename = "maxValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub max_value: Vec<f32>,
    ///<https://schema.org/valueReference>
    #[serde(rename = "valueReference")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_reference: Vec<QuantitativeValueValueReferenceFieldEnum>,
    ///<https://schema.org/unitText>
    #[serde(rename = "unitText")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unit_text: Vec<String>,
    ///<https://schema.org/value>
    #[serde(rename = "value")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value: Vec<QuantitativeValueValueFieldEnum>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/unitCode>
    #[serde(rename = "unitCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unit_code: Vec<QuantitativeValueUnitCodeFieldEnum>,
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
    pub additional_type: Vec<QuantitativeValueAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<QuantitativeValueIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<QuantitativeValueImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<QuantitativeValueDescriptionFieldEnum>,
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
    pub subject_of: Vec<QuantitativeValueSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<QuantitativeValueMainEntityOfPageFieldEnum>,
}
