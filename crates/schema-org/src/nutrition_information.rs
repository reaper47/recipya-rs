use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::{AtType, Energy, Mass};

///<https://schema.org/NutritionInformation>
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NutritionInformation {
    ///<https://schema.org/calories>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub calories: Vec<Energy>,
    ///<https://schema.org/carbohydrateContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub carbohydrate_content: Vec<Mass>,
    ///<https://schema.org/cholesterolContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cholesterol_content: Vec<Mass>,
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/fatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fat_content: Vec<Mass>,
    ///<https://schema.org/fiberContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fiber_content: Vec<Mass>,
    ///<https://schema.org/proteinContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub protein_content: Vec<Mass>,
    ///<https://schema.org/saturatedFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub saturated_fat_content: Vec<Mass>,
    ///<https://schema.org/servingSize>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub serving_size: Vec<String>,
    ///<https://schema.org/sodiumContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sodium_content: Vec<Mass>,
    ///<https://schema.org/sugarContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sugar_content: Vec<Mass>,
    #[serde(rename = "@type")]
    pub r#type: AtType,
    ///<https://schema.org/transFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trans_fat_content: Vec<Mass>,
    ///<https://schema.org/unsaturatedFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unsaturated_fat_content: Vec<Mass>,
}
