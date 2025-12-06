use serde::{Deserialize, Serialize};

use crate::field::EnergyDescriptionFieldEnum;
use crate::helpers::one_or_many;
use crate::{AtType, at_context};

///<https://schema.org/Energy>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Energy {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<EnergyDescriptionFieldEnum>,
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

impl Energy {
    /// Creates a new Energy struct with the given value.
    pub fn new(energy: impl Into<String>) -> Self {
        Self {
            r#type: AtType::Energy.to_opt(),
            context: at_context(),
            name: vec![energy.into()],
            ..Default::default()
        }
    }

    /// Converts the energy to a number.
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
        let energy = Energy::new("100 kcal");

        assert_eq!(energy.r#type, AtType::Energy.to_opt());
        assert_eq!(energy.name, vec!["100 kcal"]);
    }

    #[test]
    fn test_to_number() {
        let energy = Energy::new("100 kcal");

        assert_eq!(energy.to_number(), 100);
    }
}
