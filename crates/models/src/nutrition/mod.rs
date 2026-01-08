mod data_sources;
mod testdata;

pub mod fdc;
pub mod states;
pub mod tables;

pub use data_sources::*;

/// Represents calculated nutritional information per 100g and per serving.
#[derive(Debug, Default, PartialEq)]
pub struct CalculatedNutrition {
    pub per_100g: NutritionComponents,
    pub per_serving: NutritionComponents,
}

/// Represents the nutritional components of a recipe.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NutritionComponents {
    pub calories_kcal: i16,
    pub total_carbohydrates: f64,
    pub sugars_g: f64,
    pub protein_g: f64,
    pub total_fat_g: f64,
    pub saturated_fat_g: f64,
    pub unsaturated_fat_g: f64,
    pub cholesterol_mg: f64,
    pub sodium_mg: f64,
    pub fiber_g: f64,
    pub trans_fat_g: f64,
}

impl std::ops::Mul<f64> for NutritionComponents {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self {
        self.calories_kcal = (self.calories_kcal as f64 * rhs).round() as i16;
        self.total_carbohydrates *= rhs;
        self.sugars_g *= rhs;
        self.protein_g *= rhs;
        self.total_fat_g *= rhs;
        self.saturated_fat_g *= rhs;
        self.unsaturated_fat_g *= rhs;
        self.cholesterol_mg *= rhs;
        self.sodium_mg *= rhs;
        self.fiber_g *= rhs;
        self.trans_fat_g *= rhs;
        self
    }
}

impl std::ops::Div<i16> for NutritionComponents {
    type Output = Self;

    fn div(mut self, rhs: i16) -> Self {
        self.calories_kcal /= rhs;
        self.total_carbohydrates /= rhs as f64;
        self.sugars_g /= rhs as f64;
        self.protein_g /= rhs as f64;
        self.total_fat_g /= rhs as f64;
        self.saturated_fat_g /= rhs as f64;
        self.unsaturated_fat_g /= rhs as f64;
        self.cholesterol_mg /= rhs as f64;
        self.sodium_mg /= rhs as f64;
        self.fiber_g /= rhs as f64;
        self.trans_fat_g /= rhs as f64;
        self
    }
}

impl std::ops::AddAssign for NutritionComponents {
    fn add_assign(&mut self, other: Self) {
        self.calories_kcal += other.calories_kcal;
        self.total_carbohydrates += other.total_carbohydrates;
        self.sugars_g += other.sugars_g;
        self.protein_g += other.protein_g;
        self.total_fat_g += other.total_fat_g;
        self.saturated_fat_g += other.saturated_fat_g;
        self.unsaturated_fat_g += other.unsaturated_fat_g;
        self.cholesterol_mg += other.cholesterol_mg;
        self.sodium_mg += other.sodium_mg;
        self.fiber_g += other.fiber_g;
        self.trans_fat_g += other.trans_fat_g;
    }
}
