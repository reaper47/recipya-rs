use std::fmt;

use iso8601::DateTime;
use schema_org::{Energy, Mass, NutritionInformation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct RecipeList {
    pub next: Option<String>,
    pub results: Vec<RecipeOverview>,
}

#[derive(Deserialize)]
pub struct RecipeOverview {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct RecipeRetrieve {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub keywords: Option<Vec<Keyword>>,
    pub steps: Vec<Step>,
    pub working_time: Option<i32>,
    pub waiting_time: Option<i32>,
    pub created_by: CreatedBy,
    pub created_at: DateTime,
    pub source_url: Option<String>,
    pub nutrition: Option<Nutrition>,
    pub servings: Option<i32>,
    pub servings_text: Option<String>,
    pub rating: Option<f64>,
}

#[derive(Deserialize)]
pub struct Keyword {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Step {
    pub instruction: Option<String>,
    pub ingredients: Vec<Ingredient>,
    pub instructions_markdown: String,
}

#[derive(Deserialize)]
pub struct Ingredient {
    pub food: Option<Food>,
    pub unit: Option<Unit>,
    pub amount: f64,
    pub original_text: Option<String>,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.original_text.as_ref() {
            Some(s) => write!(f, "{s}"),
            None => {
                let res = [
                    self.amount.to_string(),
                    self.unit
                        .as_ref()
                        .map(|u| u.name.clone())
                        .unwrap_or_default(),
                    self.food
                        .as_ref()
                        .map(|f| f.name.clone())
                        .unwrap_or_default(),
                ]
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ");

                write!(f, "{}", res)
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Food {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Unit {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreatedBy {
    pub username: String,
}

#[derive(Deserialize)]
pub struct Nutrition {
    pub id: Option<u32>,
    pub carbohydrates: Option<f64>,
    pub fats: Option<f64>,
    pub proteins: Option<f64>,
    pub calories: Option<f64>,
    pub source: Option<String>,
}

impl From<Nutrition> for NutritionInformation {
    fn from(nutrition: Nutrition) -> Self {
        NutritionInformation {
            calories: match nutrition.calories {
                Some(c) => vec![Energy::new(c.to_string())],
                None => vec![],
            },
            carbohydrate_content: match nutrition.carbohydrates {
                Some(c) => vec![Mass::new(c.to_string())],
                None => vec![],
            },
            fat_content: match nutrition.fats {
                Some(f) => vec![Mass::new(f.to_string())],
                None => vec![],
            },
            protein_content: match nutrition.proteins {
                Some(p) => vec![Mass::new(p.to_string())],
                None => vec![],
            },
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}
