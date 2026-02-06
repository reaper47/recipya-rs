use serde::{Deserialize, Serialize};
use serde_json::Number;
use uuid::Uuid;

use schema_org::{
    AtType, Energy, Mass, NutritionInformation, at_context, field::RecipeKeywordsFieldEnum,
};

#[derive(Deserialize)]
pub struct MealieRecipes {
    pub items: Vec<MealieRecipe>,
    pub next: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieRecipe {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub slug: String,
    pub image: Option<String>,
    pub recipe_servings: Number,
    pub recipe_yield_quantity: Number,
    pub total_time: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub perform_time: Option<String>,
    pub description: Option<String>,
    pub recipe_category: Option<Vec<MealieItem>>,
    pub tags: Option<Vec<MealieItem>>,
    pub tools: Vec<MealieTool>,
    pub rating: Option<Number>,
    #[serde(rename = "orgURL")]
    pub org_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub recipe_ingredient: Vec<MealieIngredient>,
    pub recipe_instructions: Option<Vec<MealieInstruction>>,
    pub nutrition: Option<MealieNutrition>,
    pub notes: Option<Vec<MealieNote>>,
    pub comments: Option<Vec<MealieComment>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieTool {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct MealieUser {
    pub id: Uuid,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub auth_method: String,
    pub admin: bool,
    pub group: String,
    pub household: String,
    pub advanced: bool,
    pub can_invite: bool,
    pub can_manage: bool,
    pub can_manage_household: bool,
    pub can_organize: bool,
    pub group_id: Uuid,
    pub group_slug: String,
    pub household_id: Uuid,
    pub household_slug: String,
    pub tokens: Option<Vec<UserToken>>,
    pub cache_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub name: String,
    pub id: u32,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieIngredient {
    pub display: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieItem {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieInstruction {
    pub summary: Option<String>,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieNutrition {
    pub calories: Option<String>,
    pub carbohydrate_content: Option<String>,
    pub cholesterol_content: Option<String>,
    pub fat_content: Option<String>,
    pub fiber_content: Option<String>,
    pub protein_content: Option<String>,
    pub saturated_fat_content: Option<String>,
    pub sodium_content: Option<String>,
    pub sugar_content: Option<String>,
    pub trans_fat_content: Option<String>,
    pub unsaturated_fat_content: Option<String>,
}

impl From<MealieNutrition> for NutritionInformation {
    fn from(n: MealieNutrition) -> Self {
        Self {
            calories: n.calories.map(|s| vec![Energy::new(s)]).unwrap_or_default(),
            carbohydrate_content: n
                .carbohydrate_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            cholesterol_content: n
                .cholesterol_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            context: at_context(),
            fat_content: n
                .fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            fiber_content: n
                .fiber_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            protein_content: n
                .protein_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            saturated_fat_content: n
                .saturated_fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            serving_size: vec![],
            sodium_content: n
                .sodium_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            sugar_content: n
                .sugar_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            r#type: AtType::NutritionInformation.to_opt(),
            trans_fat_content: n
                .trans_fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            unsaturated_fat_content: n
                .unsaturated_fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieNote {
    pub title: String,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieComment {
    pub text: String,
    pub created_at: String,
    pub updated_at: String,
    pub user: MealieRecipeComment,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealieRecipeComment {
    pub username: Option<String>,
    pub full_name: Option<String>,
}

impl MealieRecipeComment {
    pub fn author(&self) -> Option<String> {
        self.full_name
            .clone()
            .map_or_else(|| self.username.clone(), Some)
    }
}

#[derive(Serialize)]
pub struct AuthPayload {
    username: String,
    password: String,
    remember_me: bool,
}

impl AuthPayload {
    pub const fn new(username: String, password: String, remember_me: bool) -> Self {
        Self {
            username,
            password,
            remember_me,
        }
    }
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

impl MealieRecipe {
    pub fn extract_category_and_keywords(&self) -> (Vec<String>, Vec<RecipeKeywordsFieldEnum>) {
        let mut categories = self
            .recipe_category
            .iter()
            .flatten()
            .map(|c| c.name.clone());

        let category = categories.next().map(|c| vec![c]).unwrap_or_default();

        let mut keywords = categories.collect::<Vec<_>>();
        keywords.extend(self.tags.iter().flatten().map(|tag| tag.name.clone()));

        (
            category,
            keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
        )
    }
}

impl MealieUser {
    pub fn author(&self) -> Option<String> {
        self.full_name.clone().map_or_else(
            || {
                self.username
                    .clone()
                    .map_or_else(|| self.email.clone(), Some)
            },
            Some,
        )
    }
}
