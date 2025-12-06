use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Number;
use tracing::error;
use uuid::Uuid;

use schema_org::{
    AtType, Energy, Mass, NutritionInformation, at_context, field::RecipeKeywordsFieldEnum,
};

#[derive(Deserialize)]
#[allow(unused)]
pub struct MealieRecipes {
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub total_pages: u32,
    pub items: Vec<MealieRecipe>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MealieRecipe {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub household_id: Uuid,
    pub group_id: Uuid,
    pub name: Option<String>,
    pub slug: String,
    pub image: Option<String>,
    pub recipe_servings: Number,
    pub recipe_yield_quantity: Number,
    pub recipe_yield: Option<String>,
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
    pub date_added: Option<String>,
    pub date_updated: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_made: Option<String>,
    #[serde(default)]
    pub recipe_ingredient: Vec<MealieIngredient>,
    pub recipe_instructions: Option<Vec<MealieInstruction>>,
    pub nutrition: Option<MealieNutrition>,
    pub settings: Option<MealieSettings>,
    pub assets: Option<Vec<MealieAsset>>,
    pub notes: Option<Vec<MealieNote>>,
    pub comments: Option<Vec<MealieComment>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct MealieTool {
    pub id: Option<String>,
    pub group_id: Option<String>,
    pub name: String,
    pub slug: String,
    pub households_with_tool: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
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
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct UserToken {
    pub name: String,
    pub id: u32,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MealieIngredient {
    pub quantity: Option<Number>,
    pub unit: Option<MealieIngredientUnit>,
    #[serde(default, deserialize_with = "deserialize_single_or_vec")]
    pub food: Option<Vec<MealieIngredientFood>>,
    pub note: Option<String>,
    pub display: String,
    pub title: Option<String>,
    pub original_text: Option<String>,
    pub reference_id: Uuid,
}

fn deserialize_single_or_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<MealieIngredientFood>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value: Option<Value> = Option::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(Value::Null) => Ok(None),
        Some(Value::Object(obj)) => {
            match serde_json::from_value::<MealieIngredientFood>(Value::from(obj)) {
                Ok(food) => Ok(Some(vec![food])),
                Err(err) => {
                    error!("Mealie API - Failed to deserialize MealieIngredientFood object: {err}");
                    Err(Error::custom(format!("Invalid food object: {err}")))
                }
            }
        }
        Some(Value::Array(arr)) => {
            match serde_json::from_value::<Vec<MealieIngredientFood>>(Value::from(arr)) {
                Ok(foods) => Ok(Some(foods)),
                Err(err) => {
                    error!("Mealie API - Failed to deserialize MealieIngredientFood array: {err}");
                    Err(Error::custom(format!("Invalid food array: {err}")))
                }
            }
        }
        Some(other) => {
            error!("Mealie API - Unexpected MealieIngredientFood value type: {other:?}");
            Err(Error::custom(format!(
                "Expected object or array for food, got: {other:?}"
            )))
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MealieIngredientUnit {
    pub id: Uuid,
    pub name: String,
    pub plural_name: Option<String>,
    pub description: String,
    pub fraction: bool,
    pub abbreviation: String,
    pub plural_abbreviation: Option<String>,
    pub use_abbreviation: bool,
    pub aliases: Vec<MealieItem>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MealieIngredientFood {
    pub id: Uuid,
    pub name: String,
    pub plural_name: Option<String>,
    pub description: String,
    pub label_id: Option<String>,
    pub aliases: Vec<MealieItem>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct MealieItem {
    pub id: Option<String>,
    pub group_id: Option<String>,
    pub name: String,
    pub slug: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MealieInstruction {
    pub id: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct MealieSettings {
    pub public: bool,
    pub show_nutrition: bool,
    pub show_assets: bool,
    pub landscape_view: bool,
    pub disable_comments: bool,
    pub locked: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct MealieAsset {
    pub name: String,
    pub icon: String,
    pub file_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct MealieNote {
    pub title: String,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
pub struct MealieComment {
    pub recipe_id: Uuid,
    pub text: String,
    pub id: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: Uuid,
    pub user: MealieRecipeComment,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MealieRecipeComment {
    pub id: Uuid,
    pub username: Option<String>,
    pub admin: bool,
    pub full_name: Option<String>,
}

impl MealieRecipeComment {
    pub fn author(&self) -> Option<String> {
        if let Some(u) = self.full_name.clone() {
            Some(u)
        } else {
            self.username.clone()
        }
    }
}

#[derive(Serialize)]
pub struct AuthPayload {
    username: String,
    password: String,
    remember_me: bool,
}

impl AuthPayload {
    pub fn new(username: String, password: String, remember_me: bool) -> Self {
        AuthPayload {
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
        if let Some(u) = self.full_name.clone() {
            Some(u)
        } else if let Some(u) = self.username.clone() {
            Some(u)
        } else {
            self.email.clone()
        }
    }
}
