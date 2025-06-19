use std::collections::HashMap;
use std::env::temp_dir;
use std::ops::Not;
use std::path::PathBuf;

use axum::extract::multipart::{Field, InvalidBoundary, MultipartRejection};
use axum::extract::{FromRequest, Multipart, Request};
use tracing::{error, warn};
use uuid::Uuid;

use support::fs::new_fs_support;

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
    pub nutrition: Option<NutritionForCreate>,
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
        let mut nutrition = NutritionForCreate::default();
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
            error!("Failed to read multipart field in recipe: {err}");
            InvalidBoundary::default()
        })? {
            let name = field.name().unwrap_or("");
            match name {
                "calories" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.calories_kcal = s.parse().ok()),
                "category" => category = field.text().await.ok().filter(|s| !s.trim().is_empty()),
                "cholesterol" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.cholesterol_mg = s.parse().ok()),
                "cuisine" => cuisine = field.text().await.ok().filter(|s| !s.trim().is_empty()),
                "description" => {
                    description = field.text().await.ok().filter(|s| !s.trim().is_empty())
                }
                "fiber" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.fiber_g = s.parse().ok()),
                "ingredient" => field.text().await.ok().map_or_else(
                    || (),
                    |text| {
                        if !text.trim().is_empty() {
                            ingredients.push(text);
                        }
                    },
                ),
                "instruction" => field.text().await.ok().map_or_else(
                    || (),
                    |text| {
                        if !text.trim().is_empty() {
                            instructions.push(text);
                        }
                    },
                ),
                "keyword" => field.text().await.ok().map_or_else(
                    || (),
                    |text| {
                        if !text.trim().is_empty() {
                            keywords.push(text);
                        }
                    },
                ),
                "media" => {
                    if let Some(file_name) = field.file_name() {
                        let (stem, ext) = file_name
                            .split('.')
                            .collect::<Vec<_>>()
                            .split_first()
                            .map_or(("".to_string(), ""), |(f, rest)| {
                                ((*f).to_string(), rest.last().unwrap_or(f))
                            });
                        let file_uuid = Uuid::new_v4();
                        let new_filename = format!("{file_uuid}.{ext}");
                        let temp = temp_dir().join(&new_filename);

                        if let Ok(bytes) = field.bytes().await {
                            let mime_type = if let Some(kind) = infer::get(&bytes) {
                                kind.mime_type()
                            } else {
                                "application/octet-stream"
                            };

                            let fs_support = new_fs_support();
                            fs_support
                                .upload_to_temp(bytes)
                                .await
                                .map_err(|_| InvalidBoundary::default())?;

                            if mime_type.starts_with("video/") {
                                videos.insert(stem, temp.clone());
                            } else {
                                images.insert(stem, temp.clone());
                            }
                        }
                    }
                }
                "protein" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.protein_g = s.parse().ok()),
                "source" => source = field.text().await.ok().filter(|s| !s.trim().is_empty()),
                "sugars" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.sugars_g = s.parse().ok()),
                "time-prep" => times.prep_seconds = calc_time_from_field(field).await,
                "time-cook" => times.cook_seconds = calc_time_from_field(field).await,
                "total-carbohydrates" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.total_carbohydrates = s.parse().ok()),
                "total-fat" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.total_fat_g = s.parse().ok()),
                "saturated-fat" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.saturated_fat_g = s.parse().ok()),
                "serving-size" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.serving_size = s.parse().ok()),
                "sodium" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.sodium_mg = s.parse().ok()),
                "title" => {
                    title = Some(
                        field
                            .text()
                            .await
                            .ok()
                            .filter(|s| !s.trim().is_empty())
                            .ok_or(InvalidBoundary::default())?,
                    )
                }
                "tool" => field.text().await.ok().iter().for_each(|tool| {
                    let quantity = tool
                        .split_whitespace()
                        .next()
                        .and_then(|n| n.parse::<i16>().ok())
                        .unwrap_or(1);

                    tools.push(ToolForCreate {
                        name: tool.replace(&quantity.to_string(), "").trim().to_owned(),
                        quantity,
                    })
                }),
                "trans-fat" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.trans_fat_g = s.parse().ok()),
                "unsaturated-fat" => field
                    .text()
                    .await
                    .ok()
                    .iter()
                    .for_each(|s| nutrition.unsaturated_fat_g = s.parse().ok()),
                "yield" => {
                    yield_ = field
                        .text()
                        .await
                        .ok()
                        .map(|text| text.trim().parse::<i16>().ok().unwrap_or(4))
                }
                _ => {
                    warn!("Field '{name}' is not processed")
                }
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
            nutrition: nutrition.is_empty().not().then_some(nutrition),
            source,
            times: (times.prep_seconds > 0 && times.cook_seconds > 0).then_some(times),
            tools,
            videos,
            yield_,
        })
    }
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
