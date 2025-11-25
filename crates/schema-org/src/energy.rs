use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

use crate::AtType;
use crate::field::EnergyDescriptionFieldEnum;
use crate::helpers::{is_smallvec_empty, one_or_many};

///<https://schema.org/Energy>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Energy {
    #[serde(rename = "@type", default = "set_type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[EnergyDescriptionFieldEnum; 1]>,
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

impl Energy {
    /// Creates a new Energy struct with the given value.
    pub fn new(energy: impl Into<String>) -> Self {
        Self {
            name: smallvec![energy.into()],
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

fn set_type() -> Option<String> {
    Some(AtType::Energy.to_string())
}
