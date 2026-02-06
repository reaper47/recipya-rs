use std::collections::HashMap;
use std::iter::Sum;
use std::path::PathBuf;
use std::str::FromStr;

use axum::extract::multipart::{Field, InvalidBoundary, MultipartRejection};
use axum::extract::{FromRequest, Multipart, Request};
use tracing::error;
use uuid::Uuid;

use crate::recipe::helpers::text_trim;
use crate::recipe::save_media_field;
use crate::recipe::structs::nutrition::{
    NutritionDetailsForCreate, NutritionForCreate, NutritionPerServingDetailsForCreate,
};
use crate::recipe::structs::time::TimesForCreate;
use crate::recipe::structs::tool::ToolForCreate;

/// Represents a form used to create or update a recipe.
pub struct RecipeForm {
    pub category: Option<String>,
    pub cuisine: Option<String>,
    pub description: Option<String>,
    pub images: HashMap<String, PathBuf>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub keywords: Vec<String>,
    pub notes: Option<String>,
    pub nutrition: NutritionDetailsForCreate,
    pub rating: Option<i16>,
    pub source: Option<String>,
    pub times: Option<TimesForCreate>,
    pub title: String,
    pub tools: Vec<ToolForCreate>,
    pub videos: HashMap<String, PathBuf>,
    pub yield_: Option<i16>,
}

impl<S> FromRequest<S> for RecipeForm
where
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    #[allow(clippy::too_many_lines)]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut category: Option<String> = None;
        let mut cuisine: Option<String> = None;
        let mut description: Option<String> = None;
        let mut images: HashMap<String, PathBuf> = HashMap::new();
        let mut ingredients: Vec<String> = Vec::new();
        let mut instructions: Vec<String> = Vec::new();
        let mut keywords: Vec<String> = Vec::new();
        let mut notes: Option<String> = None;
        let mut nutrition_per_100g = NutritionForCreate::default();
        let mut nutrition_per_serving = NutritionPerServingDetailsForCreate::default();
        let mut rating: Option<i16> = None;
        let mut source: Option<String> = None;
        let mut times: TimesForCreate = TimesForCreate {
            prep_seconds: 60 * 15,
            cook_seconds: 60 * 30,
        };
        let mut title: Option<String> = None;
        let mut tools: Vec<ToolForCreate> = Vec::new();
        let mut videos: HashMap<String, PathBuf> = HashMap::new();
        let mut yield_: Option<i16> = None;

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            error!("Failed to read multipart field in recipe: {err:?}");
            InvalidBoundary::default()
        })? {
            let name = field.name().unwrap_or("");

            match name {
                "category" => category = text_trim(field).await,
                "cuisine" => cuisine = text_trim(field).await,
                "description" => description = text_trim(field).await,
                "ingredient" => push_non_empty(field, &mut ingredients).await,
                "instruction" => push_non_empty(field, &mut instructions).await,
                "keyword" => push_non_empty(field, &mut keywords).await,
                "media" => {
                    if let Err(err) = save_media_field(field, &mut images, &mut videos).await {
                        error!("Saving media failed: {err:?}");
                    }
                }
                "media-existing-image" => {
                    let filename = text_trim(field)
                        .await
                        .ok_or_else(InvalidBoundary::default)?;
                    let path = PathBuf::from(filename);

                    let uuid = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .replace(".webp", "")
                        .parse::<Uuid>()
                        .unwrap_or_default();

                    images.insert(uuid.into(), path);
                }
                "media-existing-video" => {
                    let filename = text_trim(field)
                        .await
                        .ok_or_else(InvalidBoundary::default)?;
                    let path = PathBuf::from(filename);

                    let uuid = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .replace(".webm", "")
                        .parse::<Uuid>()
                        .unwrap_or_default();

                    videos.insert(uuid.into(), path);
                }
                "notes" => notes = text_trim(field).await,
                "rating" => rating = parse_i16(field).await,
                "source" => source = text_trim(field).await,
                "time-prep" => times.prep_seconds = calc_time_from_field(field).await,
                "time-cook" => times.cook_seconds = calc_time_from_field(field).await,
                "title" => {
                    title = text_trim(field)
                        .await
                        .ok_or_else(InvalidBoundary::default)?
                        .into();
                }
                "tool" => field.text().await.ok().iter().for_each(|tool| {
                    let quantity = tool
                        .split_whitespace()
                        .next()
                        .and_then(|n| n.parse::<i16>().ok())
                        .unwrap_or(1);

                    let name = tool.replace(&quantity.to_string(), "").trim().to_owned();
                    if !name.is_empty() {
                        tools.push(ToolForCreate { name, quantity });
                    }
                }),
                "yield" => yield_ = parse_i16(field).await,

                "calories-per-100g" => nutrition_per_100g.calories_kcal = parse_i16(field).await,
                "cholesterol-per-100g" => {
                    nutrition_per_100g.cholesterol_mg = parse_i16(field).await;
                }
                "fiber-per-100g" => nutrition_per_100g.fiber_g = parse_i16(field).await,
                "protein-per-100g" => nutrition_per_100g.protein_g = parse_i16(field).await,
                "total-carbohydrates-per-100g" => {
                    nutrition_per_100g.total_carbohydrates = parse_i16(field).await;
                }
                "total-fat-per-100g" => nutrition_per_100g.total_fat_g = parse_i16(field).await,
                "saturated-fat-per-100g" => {
                    nutrition_per_100g.saturated_fat_g = parse_i16(field).await;
                }
                "sodium-per-100g" => nutrition_per_100g.sodium_mg = parse_i16(field).await,
                "sugars-per-100g" => nutrition_per_100g.sugars_g = parse_i16(field).await,
                "trans-fat-per-100g" => nutrition_per_100g.trans_fat_g = parse_i16(field).await,
                "unsaturated-fat-per-100g" => {
                    nutrition_per_100g.unsaturated_fat_g = parse_i16(field).await;
                }

                "calories-per-serving" => {
                    nutrition_per_serving.nutrition.calories_kcal = parse_i16(field).await;
                }
                "cholesterol-per-serving" => {
                    nutrition_per_serving.nutrition.cholesterol_mg = parse_i16(field).await;
                }
                "fiber-per-serving" => {
                    nutrition_per_serving.nutrition.fiber_g = parse_i16(field).await;
                }
                "protein-per-serving" => {
                    nutrition_per_serving.nutrition.protein_g = parse_i16(field).await;
                }
                "total-carbohydrates-per-serving" => {
                    nutrition_per_serving.nutrition.total_carbohydrates = parse_i16(field).await;
                }
                "total-fat-per-serving" => {
                    nutrition_per_serving.nutrition.total_fat_g = parse_i16(field).await;
                }
                "saturated-fat-per-serving" => {
                    nutrition_per_serving.nutrition.saturated_fat_g = parse_i16(field).await;
                }
                "serving-size" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition_per_serving.serving_size = s.into()),
                "sodium-per-serving" => {
                    nutrition_per_serving.nutrition.sodium_mg = parse_i16(field).await;
                }
                "sugars-per-serving" => {
                    nutrition_per_serving.nutrition.sugars_g = parse_i16(field).await;
                }
                "trans-fat-per-serving" => {
                    nutrition_per_serving.nutrition.trans_fat_g = parse_i16(field).await;
                }
                "unsaturated-fat-per-serving" => {
                    nutrition_per_serving.nutrition.unsaturated_fat_g = parse_i16(field).await;
                }
                _ => {}
            }
        }

        if ingredients.is_empty() || instructions.is_empty() {
            return Err(InvalidBoundary::default())?;
        }

        Ok(Self {
            title: title.ok_or_else(InvalidBoundary::default)?,
            category,
            cuisine,
            description,
            images,
            ingredients,
            instructions,
            keywords,
            notes,
            nutrition: NutritionDetailsForCreate::new(nutrition_per_100g, nutrition_per_serving),
            rating,
            source,
            times: (times.prep_seconds > 0 && times.cook_seconds > 0).then_some(times),
            tools,
            videos,
            yield_,
        })
    }
}

async fn push_non_empty(field: Field<'_>, vec: &mut Vec<String>) {
    if let Some(s) = text_trim(field).await {
        vec.push(s);
    }
}

async fn parse_i16<T>(field: Field<'_>) -> Option<T>
where
    T: FromStr + Default + Sum,
{
    text_trim(field).await.and_then(|s| s.parse::<T>().ok())
}

async fn calc_time_from_field(field: Field<'_>) -> i32 {
    field.text().await.map_or(0, |text| {
        let parts: Vec<&str> = text.split(':').collect();
        if parts.len() == 3 {
            let [hours, minutes, seconds]: [i32; 3] = parts
                .iter()
                .map(|&part| part.parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or([0, 0, 0]);

            hours * 60 * 60 + minutes * 60 + seconds
        } else {
            0
        }
    })
}
