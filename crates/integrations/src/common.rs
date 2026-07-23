/// Represents the preparation and cooking times of a recipe.
#[derive(Default)]
pub struct Times {
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}

/// Represents a tool used in a recipe.
pub struct Tool {
    pub name: String,
    pub quantity: i16,
}

/// Represents the nutrition information of a recipe.
#[derive(Default)]
pub struct Nutrition {
    pub calories_kcal: Option<i16>,
    pub total_carbohydrates: Option<i16>,
    pub sugars_g: Option<i16>,
    pub protein_g: Option<i16>,
    pub total_fat_g: Option<i16>,
    pub saturated_fat_g: Option<i16>,
    pub unsaturated_fat_g: Option<i16>,
    pub cholesterol_mg: Option<i16>,
    pub sodium_mg: Option<i16>,
    pub fiber_g: Option<i16>,
    pub trans_fat_g: Option<i16>,
    pub serving_size: Option<String>,
}
