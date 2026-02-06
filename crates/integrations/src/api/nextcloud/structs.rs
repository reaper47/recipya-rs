use iso8601::{DateTime, Duration};
use schema_org::{AtType, Energy, Mass, NutritionInformation, at_context};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipes {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct Recipe {
    pub name: String,
    pub keywords: String,
    pub date_created: DateTime,
    pub date_modified: Option<DateTime>,
    pub image_url: String,
    pub id: String,
    pub prep_time: Duration,
    pub cook_time: Duration,
    pub total_time: Duration,
    pub description: String,
    pub url: String,
    pub image: Option<String>,
    pub recipe_yield: i16,
    pub recipe_category: String,
    pub tool: Vec<String>,
    pub recipe_ingredient: Vec<String>,
    pub recipe_instructions: Vec<String>,
    pub nutrition: Option<Nutrition>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrition {
    pub calories: Option<String>,
    pub carbohydrate_content: Option<String>,
    pub cholesterol_content: Option<String>,
    pub fat_content: Option<String>,
    pub fiber_content: Option<String>,
    pub protein_content: Option<String>,
    pub saturated_fat_content: Option<String>,
    pub serving_size: Option<String>,
    pub sodium_content: Option<String>,
    pub sugar_content: Option<String>,
    pub trans_fat_content: Option<String>,
    pub unsaturated_fat_content: Option<String>,
}

impl From<Nutrition> for NutritionInformation {
    fn from(n: Nutrition) -> Self {
        Self {
            r#type: AtType::NutritionInformation.to_opt(),
            calories: n.calories.map_or_else(Vec::new, |cal| {
                if cal.is_empty() {
                    vec![]
                } else {
                    vec![Energy::new(cal)]
                }
            }),
            carbohydrate_content: new_mass(n.carbohydrate_content),
            fat_content: new_mass(n.fat_content),
            fiber_content: new_mass(n.fiber_content),
            protein_content: new_mass(n.protein_content),
            serving_size: n.serving_size.map_or_else(Vec::new, |serving_size| {
                if serving_size.is_empty() {
                    vec![]
                } else {
                    vec![serving_size]
                }
            }),
            cholesterol_content: new_mass(n.cholesterol_content),
            context: at_context(),
            saturated_fat_content: new_mass(n.saturated_fat_content),
            sodium_content: new_mass(n.sodium_content),
            sugar_content: new_mass(n.sugar_content),
            trans_fat_content: new_mass(n.trans_fat_content),
            unsaturated_fat_content: new_mass(n.unsaturated_fat_content),
        }
    }
}

fn new_mass(s: Option<String>) -> Vec<Mass> {
    s.map_or_else(Vec::new, |s| {
        if s.is_empty() {
            vec![]
        } else {
            vec![Mass::new(s)]
        }
    })
}

impl Nutrition {
    /// Verifies whether the nutrition information contains no information.
    pub fn is_empty(&self) -> bool {
        is_opt_field_empty(self.calories.as_ref())
            && is_opt_field_empty(self.carbohydrate_content.as_ref())
            && is_opt_field_empty(self.cholesterol_content.as_ref())
            && is_opt_field_empty(self.fat_content.as_ref())
            && is_opt_field_empty(self.fiber_content.as_ref())
            && is_opt_field_empty(self.protein_content.as_ref())
            && is_opt_field_empty(self.saturated_fat_content.as_ref())
            && is_opt_field_empty(self.serving_size.as_ref())
            && is_opt_field_empty(self.sodium_content.as_ref())
            && is_opt_field_empty(self.sugar_content.as_ref())
            && is_opt_field_empty(self.trans_fat_content.as_ref())
            && is_opt_field_empty(self.unsaturated_fat_content.as_ref())
    }
}

fn is_opt_field_empty(s: Option<&String>) -> bool {
    s.is_none_or(String::is_empty)
}
