use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};

use repository::schema;
use schema_org::NutritionInformation;

use crate::recipe::structs::recipe::Recipe;

/// Represents a nutrition entity stored in the database.
#[derive(
    AsChangeset, Associations, Clone, Debug, Default, Queryable, Identifiable, PartialEq, Selectable,
)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::nutrition)]
#[diesel(treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nutrition {
    /// Unique identifier for the nutrition entry.
    pub id: i64,
    /// Identifier of the associated recipe.
    pub recipe_id: i64,
    /// Total calories in kilocalories (kcal) per serving.
    pub calories_kcal: Option<i16>,
    /// Total carbohydrates in grams (g) per serving.
    pub total_carbohydrates: Option<i16>,
    /// Total sugar content in grams (g) per serving.
    pub sugars_g: Option<i16>,
    /// Total protein content in grams (g) per serving.
    pub protein_g: Option<i16>,
    /// Total fat content in grams (g) per serving.
    pub total_fat_g: Option<i16>,
    /// Saturated fat content in grams (g) per serving.
    pub saturated_fat_g: Option<i16>,
    /// Unsaturated fat content in grams (g) per serving.
    pub unsaturated_fat_g: Option<i16>,
    /// Cholesterol content in milligrams (mg) per serving.
    pub cholesterol_mg: Option<i16>,
    /// Sodium content in milligrams (mg) per serving.
    pub sodium_mg: Option<i16>,
    /// Dietary fiber content in grams (g) per serving.
    pub fiber_g: Option<i16>,
    /// Trans fat content in grams (g) per serving.
    pub trans_fat_g: Option<i16>,
    /// Description of the serving size (e.g., "100g", "1 cup").
    pub serving_size: Option<String>,
}

impl Nutrition {
    /// Formats the nutrition as a sentence.
    pub fn to_line(&self) -> String {
        if self == &Nutrition::default() {
            return "No nutrition available.".into();
        }

        let mut result: String = if self.serving_size.as_deref() == Some("100g") {
            "Per 100g: "
        } else {
            "Per serving: "
        }
        .into();

        let parts: Vec<String> = [
            self.calories_kcal
                .as_ref()
                .map(|v| format!("calories {v} kcal")),
            self.total_carbohydrates
                .as_ref()
                .map(|v| format!("total carbohydrates {v}g")),
            self.sugars_g.as_ref().map(|v| format!("sugar {v}g")),
            self.protein_g.as_ref().map(|v| format!("protein {v}g")),
            self.total_fat_g.as_ref().map(|v| format!("total fat {v}g")),
            self.saturated_fat_g
                .as_ref()
                .map(|v| format!("saturated fat {v}g")),
            self.unsaturated_fat_g
                .as_ref()
                .map(|v| format!("unsaturated fat {v}g")),
            self.trans_fat_g.as_ref().map(|v| format!("trans fat {v}g")),
            self.cholesterol_mg
                .as_ref()
                .map(|v| format!("cholesterol {v}mg")),
            self.sodium_mg.as_ref().map(|v| format!("sodium {v}mg")),
            self.fiber_g.as_ref().map(|v| format!("fiber {v}g")),
        ]
        .into_iter()
        .flatten()
        .collect();

        result.push_str(&parts.join("; "));
        result
    }
}

// TODO: Test
impl From<&NutritionForCreate> for Nutrition {
    fn from(n: &NutritionForCreate) -> Self {
        Self {
            calories_kcal: n.calories_kcal,
            total_carbohydrates: n.total_carbohydrates,
            sugars_g: n.sugars_g,
            protein_g: n.protein_g,
            total_fat_g: n.total_fat_g,
            saturated_fat_g: n.saturated_fat_g,
            unsaturated_fat_g: n.unsaturated_fat_g,
            cholesterol_mg: n.cholesterol_mg,
            sodium_mg: n.sodium_mg,
            fiber_g: n.fiber_g,
            trans_fat_g: n.trans_fat_g,
            serving_size: n.serving_size.clone(),
            ..Default::default()
        }
    }
}

/// Represents the nutritional information provided when creating a new recipe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NutritionForCreate {
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

impl NutritionForCreate {
    /// Verifies whether all fields are blank.
    pub fn is_empty(&self) -> bool {
        self.calories_kcal.is_none()
            && self.total_carbohydrates.is_none()
            && self.sugars_g.is_none()
            && self.protein_g.is_none()
            && self.total_fat_g.is_none()
            && self.saturated_fat_g.is_none()
            && self.unsaturated_fat_g.is_none()
            && self.cholesterol_mg.is_none()
            && self.sodium_mg.is_none()
            && self.fiber_g.is_none()
            && self.trans_fat_g.is_none()
            && self.serving_size.is_none()
    }
}

impl From<NutritionInformation> for NutritionForCreate {
    fn from(schema: NutritionInformation) -> Self {
        Self::from(&schema)
    }
}

impl From<&NutritionInformation> for NutritionForCreate {
    fn from(schema: &NutritionInformation) -> Self {
        Self {
            calories_kcal: schema.calories.first().map(|v| v.to_number()),
            total_carbohydrates: schema.carbohydrate_content.first().map(|v| v.to_number()),
            sugars_g: schema.sugar_content.first().map(|v| v.to_number()),
            protein_g: schema.protein_content.first().map(|v| v.to_number()),
            total_fat_g: schema.fat_content.first().map(|v| v.to_number()),
            saturated_fat_g: schema.saturated_fat_content.first().map(|v| v.to_number()),
            unsaturated_fat_g: schema
                .unsaturated_fat_content
                .first()
                .map(|v| v.to_number()),
            cholesterol_mg: schema.cholesterol_content.first().map(|v| v.to_number()),
            sodium_mg: schema.sodium_content.first().map(|v| v.to_number()),
            fiber_g: schema.fiber_content.first().map(|v| v.to_number()),
            trans_fat_g: schema.trans_fat_content.first().map(|v| v.to_number()),
            serving_size: schema.serving_size.first().cloned(),
        }
    }
}

/// Represents the nutritional information associated with a recipe in the database.
#[derive(Associations, Insertable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::nutrition)]
pub(crate) struct NutritionForInsert {
    pub recipe_id: i64,
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
