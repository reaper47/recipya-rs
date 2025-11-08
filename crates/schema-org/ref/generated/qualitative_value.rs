use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type QualitativeValueAdditionalTypeFieldEnum = String;
///<https://schema.org/QualitativeValue>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct QualitativeValue {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/lesser>
    #[serde(rename = "lesser")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub lesser: Vec<QualitativeValue>,
    ///<https://schema.org/greater>
    #[serde(rename = "greater")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub greater: Vec<QualitativeValue>,
    ///<https://schema.org/equal>
    #[serde(rename = "equal")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub equal: Vec<QualitativeValue>,
    ///<https://schema.org/valueReference>
    #[serde(rename = "valueReference")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_reference: Vec<QualitativeValueValueReferenceFieldEnum>,
    ///<https://schema.org/greaterOrEqual>
    #[serde(rename = "greaterOrEqual")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub greater_or_equal: Vec<QualitativeValue>,
    ///<https://schema.org/nonEqual>
    #[serde(rename = "nonEqual")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub non_equal: Vec<QualitativeValue>,
    ///<https://schema.org/lesserOrEqual>
    #[serde(rename = "lesserOrEqual")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub lesser_or_equal: Vec<QualitativeValue>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/supersededBy>
    #[serde(rename = "supersededBy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub superseded_by: Vec<QualitativeValueSupersededByFieldEnum>,
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
    pub additional_type: Vec<QualitativeValueAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<QualitativeValueIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<QualitativeValueImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<QualitativeValueDescriptionFieldEnum>,
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
    pub subject_of: Vec<QualitativeValueSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<QualitativeValueMainEntityOfPageFieldEnum>,
}
