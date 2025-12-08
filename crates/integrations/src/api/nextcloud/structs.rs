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
            calories: if let Some(cal) = n.calories {
                if cal.is_empty() {
                    vec![]
                } else {
                    vec![Energy::new(cal)]
                }
            } else {
                vec![]
            },
            carbohydrate_content: new_mass(n.carbohydrate_content),
            fat_content: new_mass(n.fat_content),
            fiber_content: new_mass(n.fiber_content),
            protein_content: new_mass(n.protein_content),
            serving_size: if let Some(serving_size) = n.serving_size {
                if serving_size.is_empty() {
                    vec![]
                } else {
                    vec![serving_size]
                }
            } else {
                vec![]
            },
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
    match s {
        Some(s) => {
            if s.is_empty() {
                vec![]
            } else {
                vec![Mass::new(s)]
            }
        }
        None => vec![],
    }
}

impl Nutrition {
    /// Verifies whether the nutrition information contains no information.
    pub fn is_empty(&self) -> bool {
        is_opt_field_empty(&self.calories)
            && is_opt_field_empty(&self.carbohydrate_content)
            && is_opt_field_empty(&self.cholesterol_content)
            && is_opt_field_empty(&self.fat_content)
            && is_opt_field_empty(&self.fiber_content)
            && is_opt_field_empty(&self.protein_content)
            && is_opt_field_empty(&self.saturated_fat_content)
            && is_opt_field_empty(&self.serving_size)
            && is_opt_field_empty(&self.sodium_content)
            && is_opt_field_empty(&self.sugar_content)
            && is_opt_field_empty(&self.trans_fat_content)
            && is_opt_field_empty(&self.unsaturated_fat_content)
    }
}

fn is_opt_field_empty(s: &Option<String>) -> bool {
    match s {
        Some(s) => s.is_empty(),
        None => true,
    }
}
