use serde::{Deserialize, Serialize};

use crate::field::{MassDescriptionFieldEnum, MassImageFieldEnum};
use crate::helpers::one_or_many;
use crate::{AtType, at_context};

///<https://schema.org/Mass>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Mass {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
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
            r#type: AtType::NutritionInformation.to_opt(),
            context: at_context(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mass = Mass::new("100 grams");

        assert_eq!(mass.r#type, AtType::NutritionInformation.to_opt());
        assert_eq!(mass.context, at_context());
        assert_eq!(mass.name, vec!["100 grams"]);
    }

    #[test]
    fn test_to_number() {
        let mass = Mass::new("100 grams");

        assert_eq!(mass.to_number(), 100);
    }
}
