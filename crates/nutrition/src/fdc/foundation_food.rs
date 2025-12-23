use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundationFoodRoot {
    #[serde(rename = "FoundationFoods")]
    pub foundation_foods: Vec<FoundationFood>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundationFood {
    pub food_class: String,
    pub description: String,
    pub food_nutrients: Vec<FoodNutrient>,
    pub food_attributes: Vec<Value>,
    #[serde(default)]
    pub nutrient_conversion_factors: Vec<NutrientConversionFactor>,
    pub is_historical_reference: bool,
    pub ndb_number: i64,
    pub data_type: String,
    pub food_category: FoodCategory,
    pub fdc_id: i64,
    #[serde(default)]
    pub food_portions: Vec<FoodPortion>,
    pub publication_date: String,
    #[serde(default)]
    pub input_foods: Vec<InputFood>,
    pub scientific_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodNutrient {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub nutrient: Nutrient,
    pub data_points: Option<i64>,
    pub food_nutrient_derivation: FoodNutrientDerivation,
    pub median: Option<f64>,
    pub amount: Option<f64>,
    pub max: Option<f64>,
    pub min: Option<f64>,
    pub footnote: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrient {
    pub id: i64,
    pub number: String,
    pub name: String,
    pub rank: i64,
    pub unit_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodNutrientDerivation {
    pub code: Option<String>,
    pub description: Option<String>,
    pub food_nutrient_source: FoodNutrientSource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodNutrientSource {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutrientConversionFactor {
    #[serde(rename = "type")]
    pub type_field: String,
    pub protein_value: Option<f64>,
    pub fat_value: Option<f64>,
    pub carbohydrate_value: Option<f64>,
    pub value: Option<f64>,
    pub nitrogen_value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodCategory {
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodPortion {
    pub id: i64,
    pub value: f64,
    pub measure_unit: MeasureUnit,
    pub modifier: Option<String>,
    pub gram_weight: f64,
    pub sequence_number: i64,
    pub amount: f64,
    pub min_year_acquired: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureUnit {
    pub id: i64,
    pub name: String,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputFood {
    pub id: i64,
    pub food_description: String,
    pub input_food: InputFood2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputFood2 {
    pub food_class: String,
    pub description: String,
    pub data_type: String,
    pub food_category: FoodCategory2,
    pub fdc_id: i64,
    pub publication_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodCategory2 {
    pub id: i64,
    pub code: String,
    pub description: String,
}
