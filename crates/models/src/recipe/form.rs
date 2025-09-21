use std::collections::HashMap;
use std::env::temp_dir;
use std::ops::Not;
use std::path::PathBuf;

use axum::extract::multipart::{Field, InvalidBoundary, MultipartRejection};
use axum::extract::{FromRequest, Multipart, Request};
use tracing::error;
use uuid::Uuid;

use super::{NutritionForCreate, TimesForCreate, ToolForCreate};

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
    pub nutrition: Option<NutritionForCreate>,
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
        let mut nutrition = NutritionForCreate::default();
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
                    let filename = text_trim(field).await.ok_or(InvalidBoundary::default())?;
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
                    let filename = text_trim(field).await.ok_or(InvalidBoundary::default())?;
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
                        .ok_or(InvalidBoundary::default())?
                        .into()
                }
                "tool" => field.text().await.ok().iter().for_each(|tool| {
                    let quantity = tool
                        .split_whitespace()
                        .next()
                        .and_then(|n| n.parse::<i16>().ok())
                        .unwrap_or(1);

                    let name = tool.replace(&quantity.to_string(), "").trim().to_owned();
                    if !name.is_empty() {
                        tools.push(ToolForCreate { name, quantity })
                    }
                }),
                "yield" => yield_ = parse_i16(field).await,

                "calories" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.calories_kcal = Some(v);
                    }
                }
                "cholesterol" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.cholesterol_mg = Some(v);
                    }
                }
                "fiber" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.fiber_g = Some(v);
                    }
                }
                "protein" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.protein_g = Some(v);
                    }
                }
                "total-carbohydrates" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.total_carbohydrates = Some(v);
                    }
                }
                "total-fat" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.total_fat_g = Some(v);
                    }
                }
                "saturated-fat" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.saturated_fat_g = Some(v);
                    }
                }
                "serving-size" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.serving_size = s.parse().ok()),
                "sodium" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.sodium_mg = Some(v);
                    }
                }
                "sugars" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.sugars_g = Some(v);
                    }
                }
                "trans-fat" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.trans_fat_g = Some(v);
                    }
                }
                "unsaturated-fat" => {
                    if let Some(v) = parse_i16(field).await {
                        nutrition.unsaturated_fat_g = Some(v);
                    }
                }
                _ => {}
            }
        }

        if ingredients.is_empty() || instructions.is_empty() {
            return Err(InvalidBoundary::default())?;
        }

        Ok(RecipeForm {
            title: title.ok_or(InvalidBoundary::default())?,
            category,
            cuisine,
            description,
            images,
            ingredients,
            instructions,
            keywords,
            notes,
            nutrition: nutrition.is_empty().not().then_some(nutrition),
            rating,
            source,
            times: (times.prep_seconds > 0 && times.cook_seconds > 0).then_some(times),
            tools,
            videos,
            yield_,
        })
    }
}

async fn text_trim(field: Field<'_>) -> Option<String> {
    match field.text().await {
        Ok(s) => {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_owned())
            }
        }
        Err(_) => None,
    }
}

async fn push_non_empty(field: Field<'_>, vec: &mut Vec<String>) {
    if let Some(s) = text_trim(field).await {
        vec.push(s);
    }
}

async fn parse_i16(field: Field<'_>) -> Option<i16> {
    text_trim(field).await.and_then(|s| s.parse::<i16>().ok())
}

async fn calc_time_from_field(field: Field<'_>) -> i32 {
    if let Ok(text) = field.text().await {
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
    } else {
        0
    }
}

async fn save_media_field(
    field: Field<'_>,
    images: &mut HashMap<String, PathBuf>,
    videos: &mut HashMap<String, PathBuf>,
) -> Result<(), InvalidBoundary> {
    let filename = Uuid::new_v4();

    let bytes = field.bytes().await.unwrap_or_default();
    if bytes.is_empty() {
        return Ok(());
    }

    let mime = infer::get(&bytes)
        .map(|k| k.mime_type())
        .unwrap_or("application/octet-stream");

    let path = temp_dir().join(filename.to_string());
    tokio::fs::write(&path, &bytes)
        .await
        .map_err(|_| InvalidBoundary::default())?;

    if mime.starts_with("video/") {
        videos.insert(filename.to_string(), path.clone());
    } else {
        images.insert(filename.to_string(), path.clone());
    }
    Ok(())
}
