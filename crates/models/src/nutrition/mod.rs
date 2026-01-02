mod data_sources;

pub mod fdc;
pub mod states;
pub mod tables;

pub use data_sources::*;

/// Represents the nutritional components of a recipe.
#[derive(Default)]
pub struct NutritionComponents {
    pub calories_kcal: i16,
    pub total_carbohydrates: i16,
    pub sugars_g: i16,
    pub protein_g: i16,
    pub total_fat_g: i16,
    pub saturated_fat_g: i16,
    pub unsaturated_fat_g: i16,
    pub cholesterol_mg: i16,
    pub sodium_mg: i16,
    pub fiber_g: i16,
    pub trans_fat_g: i16,
}
