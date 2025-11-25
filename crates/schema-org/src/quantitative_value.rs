use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

use crate::AtType;
use crate::field::{
    QuantitativeValueDescriptionFieldEnum, QuantitativeValueValueFieldEnum,
    QuantitativeValueValueReferenceFieldEnum,
};
use crate::helpers::{is_smallvec_empty, one_or_many};

///<https://schema.org/unitCode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type QuantitativeValueUnitCodeFieldEnum = String;

///<https://schema.org/QuantitativeValue>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct QuantitativeValue {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/minValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub min_value: SmallVec<[f32; 1]>,
    ///<https://schema.org/maxValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub max_value: SmallVec<[f32; 1]>,
    ///<https://schema.org/valueReference>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub value_reference: SmallVec<[Box<QuantitativeValueValueReferenceFieldEnum>; 1]>,
    ///<https://schema.org/unitText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub unit_text: SmallVec<[String; 1]>,
    ///<https://schema.org/value>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub value: SmallVec<[QuantitativeValueValueFieldEnum; 1]>,
    ///<https://schema.org/unitCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub unit_code: SmallVec<[QuantitativeValueUnitCodeFieldEnum; 1]>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[QuantitativeValueDescriptionFieldEnum; 1]>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub url: SmallVec<[String; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
}

impl QuantitativeValue {
    /// Creates a new `QuantitativeValue` for the given value.
    pub fn new(value: f32) -> Self {
        Self {
            r#type: Some(AtType::QuantitativeValue.to_string()),
            value: smallvec![QuantitativeValueValueFieldEnum::Number(value)],
            ..Default::default()
        }
    }
}
