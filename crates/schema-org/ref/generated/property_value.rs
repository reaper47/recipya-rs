use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/propertyID>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PropertyValuePropertyIDFieldEnum = String;
///<https://schema.org/unitCode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PropertyValueUnitCodeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PropertyValueAdditionalTypeFieldEnum = String;
///<https://schema.org/PropertyValue>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PropertyValue {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/measurementMethod>
    #[serde(rename = "measurementMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measurement_method: Vec<PropertyValueMeasurementMethodFieldEnum>,
    ///<https://schema.org/minValue>
    #[serde(rename = "minValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub min_value: Vec<f32>,
    ///<https://schema.org/propertyID>
    #[serde(rename = "propertyID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub property_id: Vec<PropertyValuePropertyIDFieldEnum>,
    ///<https://schema.org/maxValue>
    #[serde(rename = "maxValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub max_value: Vec<f32>,
    ///<https://schema.org/valueReference>
    #[serde(rename = "valueReference")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_reference: Vec<PropertyValueValueReferenceFieldEnum>,
    ///<https://schema.org/measurementTechnique>
    #[serde(rename = "measurementTechnique")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measurement_technique: Vec<PropertyValueMeasurementTechniqueFieldEnum>,
    ///<https://schema.org/unitText>
    #[serde(rename = "unitText")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unit_text: Vec<String>,
    ///<https://schema.org/value>
    #[serde(rename = "value")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value: Vec<PropertyValueValueFieldEnum>,
    ///<https://schema.org/unitCode>
    #[serde(rename = "unitCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unit_code: Vec<PropertyValueUnitCodeFieldEnum>,
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
    pub additional_type: Vec<PropertyValueAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<PropertyValueIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<PropertyValueImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<PropertyValueDescriptionFieldEnum>,
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
    pub subject_of: Vec<PropertyValueSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<PropertyValueMainEntityOfPageFieldEnum>,
}
