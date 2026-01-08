use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SrLegacyRoot<'a> {
    #[serde(rename = "SRLegacyFoods")]
    pub srlegacy_foods: Vec<SrlegacyFood<'a>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SrlegacyFood<'a> {
    pub food_class: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub food_nutrients: Vec<FoodNutrient<'a>>,
    pub food_attributes: Vec<Value>,
    pub nutrient_conversion_factors: Vec<NutrientConversionFactor<'a>>,
    pub is_historical_reference: bool,
    pub ndb_number: i64,
    pub fdc_id: i64,
    pub data_type: Cow<'a, str>,
    pub food_category: FoodCategory<'a>,
    pub food_portions: Vec<FoodPortion<'a>>,
    pub publication_date: Cow<'a, str>,
    pub input_foods: Vec<Value>,
    pub scientific_name: Option<Cow<'a, str>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodNutrient<'a> {
    #[serde(rename = "type")]
    pub type_field: Cow<'a, str>,
    pub id: i64,
    pub nutrient: Nutrient<'a>,
    pub data_points: i64,
    pub food_nutrient_derivation: FoodNutrientDerivation<'a>,
    pub amount: f64,
    pub max: Option<f64>,
    pub min: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrient<'a> {
    pub id: i64,
    pub number: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub rank: i64,
    pub unit_name: Cow<'a, str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodNutrientDerivation<'a> {
    pub code: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub food_nutrient_source: FoodNutrientSource<'a>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodNutrientSource<'a> {
    pub id: Option<i64>,
    pub code: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutrientConversionFactor<'a> {
    #[serde(rename = "type")]
    pub type_field: Cow<'a, str>,
    pub value: Option<f64>,
    pub protein_value: Option<f64>,
    pub fat_value: Option<f64>,
    pub carbohydrate_value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodCategory<'a> {
    pub description: Cow<'a, str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodPortion<'a> {
    pub id: i64,
    pub value: f64,
    pub measure_unit: MeasureUnit<'a>,
    pub modifier: Cow<'a, str>,
    pub gram_weight: f64,
    pub sequence_number: i64,
    pub amount: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureUnit<'a> {
    pub id: i64,
    pub name: Cow<'a, str>,
    pub abbreviation: Cow<'a, str>,
}
