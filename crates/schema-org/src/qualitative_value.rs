use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::PropertyValue;
use crate::field::{
    QualitativeValueDescriptionFieldEnum, QualitativeValueSubjectOfFieldEnum,
    QualitativeValueValueReferenceFieldEnum,
};
use crate::helpers::{is_smallvec_empty, one_or_many};

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type QualitativeValueAdditionalTypeFieldEnum = String;
///<https://schema.org/QualitativeValue>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct QualitativeValue {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/lesser>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub lesser: SmallVec<[Box<QualitativeValue>; 1]>,
    ///<https://schema.org/greater>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub greater: SmallVec<[Box<QualitativeValue>; 1]>,
    ///<https://schema.org/equal>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub equal: SmallVec<[Box<QualitativeValue>; 1]>,
    ///<https://schema.org/valueReference>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub value_reference: SmallVec<[Box<QualitativeValueValueReferenceFieldEnum>; 1]>,
    ///<https://schema.org/greaterOrEqual>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub greater_or_equal: SmallVec<[Box<QualitativeValue>; 1]>,
    ///<https://schema.org/nonEqual>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub non_equal: SmallVec<[Box<QualitativeValue>; 1]>,
    ///<https://schema.org/lesserOrEqual>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub lesser_or_equal: SmallVec<[Box<QualitativeValue>; 1]>,
    ///<https://schema.org/additionalProperty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub additional_property: SmallVec<[PropertyValue; 1]>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub disambiguating_description: SmallVec<[String; 1]>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[QualitativeValueDescriptionFieldEnum; 1]>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub url: SmallVec<[String; 1]>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub subject_of: SmallVec<[QualitativeValueSubjectOfFieldEnum; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
}
