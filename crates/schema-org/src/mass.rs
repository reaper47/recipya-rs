use serde::{Deserialize, Serialize};

use crate::AtType;
use crate::field::{MassDescriptionFieldEnum, MassImageFieldEnum};
use crate::helpers::one_or_many;

///<https://schema.org/Mass>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Mass {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MassImageFieldEnum>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MassDescriptionFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
}

impl Mass {
    /// Creates a new Mass struct with the given value.
    pub fn new(mass: impl Into<String>) -> Self {
        Self {
            r#type: Some(AtType::NutritionInformation.to_string()),
            name: vec![mass.into()],
            ..Default::default()
        }
    }

    /// Converts the mass to a number.
    pub fn to_number(&self) -> i16 {
        self.name
            .first()
            .map(|s| {
                s.split(' ')
                    .map(|s| s.parse::<i16>().unwrap_or_default())
                    .sum()
            })
            .unwrap_or_default()
    }
}
