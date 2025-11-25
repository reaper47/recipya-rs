use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::helpers::{is_smallvec_empty, one_or_many};
use crate::{Energy, Mass};

///<https://schema.org/NutritionInformation>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct NutritionInformation {
    ///<https://schema.org/calories>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub calories: SmallVec<[Energy; 1]>,
    ///<https://schema.org/carbohydrateContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub carbohydrate_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/cholesterolContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub cholesterol_content: SmallVec<[Mass; 1]>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/fatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub fat_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/fiberContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub fiber_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/proteinContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub protein_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/saturatedFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub saturated_fat_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/servingSize>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub serving_size: SmallVec<[String; 1]>,
    ///<https://schema.org/sodiumContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub sodium_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/sugarContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub sugar_content: SmallVec<[Mass; 1]>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    ///<https://schema.org/transFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub trans_fat_content: SmallVec<[Mass; 1]>,
    ///<https://schema.org/unsaturatedFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub unsaturated_fat_content: SmallVec<[Mass; 1]>,
}

impl NutritionInformation {
    /// Checks whether all fields of the schema are `None`.
    pub fn is_empty(&self) -> bool {
        self.calories.is_empty()
            && self.carbohydrate_content.is_empty()
            && self.cholesterol_content.is_empty()
            && self.fat_content.is_empty()
            && self.fiber_content.is_empty()
            && self.protein_content.is_empty()
            && self.saturated_fat_content.is_empty()
            && self.serving_size.is_empty()
            && self.sodium_content.is_empty()
            && self.sugar_content.is_empty()
            && self.trans_fat_content.is_empty()
            && self.unsaturated_fat_content.is_empty()
    }
}
