use serde::{Deserialize, Serialize};

use crate::field::{
    FieldEnum64, QuantitativeValueDescriptionFieldEnum, QuantitativeValueValueFieldEnum,
    QuantitativeValueValueReferenceFieldEnum, float_to_i16_safe,
};
use crate::helpers::one_or_many;
use crate::{AtType, at_context};

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
    pub r#type: String,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/minValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub min_value: Vec<f32>,
    ///<https://schema.org/maxValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub max_value: Vec<f32>,
    ///<https://schema.org/valueReference>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value_reference: Vec<QuantitativeValueValueReferenceFieldEnum>,
    ///<https://schema.org/unitText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unit_text: Vec<String>,
    ///<https://schema.org/value>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuantitativeValueValueFieldEnum>,
    ///<https://schema.org/unitCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unit_code: Vec<QuantitativeValueUnitCodeFieldEnum>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<QuantitativeValueDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
}

impl QuantitativeValue {
    /// Creates a new `QuantitativeValue` for the given value.
    pub fn new(value: f64) -> Self {
        Self {
            r#type: AtType::QuantitativeValue.to_string(),
            context: at_context(),
            value: vec![QuantitativeValueValueFieldEnum::Number(value)],
            ..Default::default()
        }
    }

    /// Gets the numerical value.
    pub fn to_number(&self) -> i16 {
        self.value
            .first()
            .map(|v| match v {
                FieldEnum64::BooleanEnumOrText(s) => s.parse().ok().unwrap_or_default(),
                FieldEnum64::Number(n) => float_to_i16_safe(*n),
                FieldEnum64::StructuredValue(v) => v
                    .name
                    .first()
                    .map(|v| v.parse::<i16>().unwrap_or_default())
                    .unwrap_or_default(),
                FieldEnum64::QuantitativeValue(_) => 0,
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let value = 42.0;

        let got = QuantitativeValue::new(value);

        assert_eq!(got.r#type, AtType::QuantitativeValue.to_string());
        assert_eq!(got.context, at_context());
        assert_eq!(
            got.value,
            vec![QuantitativeValueValueFieldEnum::Number(value)]
        );
    }

    #[test]
    fn test_to_number() {
        let value = 42.0;

        let got = QuantitativeValue::new(value).to_number();

        assert_eq!(got, 42);
    }
}
