use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;

use axum::extract::multipart::{InvalidBoundary, MultipartRejection};
use axum::extract::{FromRequest, Multipart, Request};
use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::{Date, PrimitiveDateTime};
use tracing::{error, info};
use uuid::Uuid;

use integrations::api::Api;
use integrations::{App, FileFormat, parse_recipe};
use models::recipe::{save_media_field, text_trim};
use schema_org::Recipe;

/// Represents the content of the "Add Recipe -> Import -> Software" form.
#[derive(Clone, Default)]
pub struct ImportFromAppForm {
    pub file_data: Vec<u8>,
    pub file_name: String,
    pub app: App,
    pub file_format: FileFormat,
}

impl ImportFromAppForm {
    /// Parses the recipe file contained in the form.
    pub fn parse_recipes(&self) -> crate::error::Result<Vec<Recipe>> {
        let mut data = Cursor::new(&self.file_data);
        let app = &self.app;
        let file_name = &self.file_name;
        let file_format = &self.file_format;
        Ok(parse_recipe(&mut data, app, file_name, file_format)?)
    }
}

impl<S> FromRequest<S> for ImportFromAppForm
where
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut form = Self::default();

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            error!(
                ?err,
                "Failed to read multipart field in import recipes from app"
            );
            InvalidBoundary::default()
        })? {
            let name = field.name().unwrap_or("");
            match name {
                "app" => {
                    let app_name = field.text().await.map_err(|_| InvalidBoundary::default())?;
                    let app = App::from_str(&app_name.to_lowercase()).unwrap_or_default();
                    if matches!(app, App::Unknown) {
                        error!(
                            ?app_name,
                            "Import recipes from app form field 'app' is invalid"
                        );
                        Err(InvalidBoundary::default())?;
                    }
                    form.app = app;
                }
                "file" => {
                    let filename = field
                        .file_name()
                        .map(ToString::to_string)
                        .ok_or_else(InvalidBoundary::default)?;
                    form.file_name.clone_from(&filename);

                    form.file_data = field
                        .bytes()
                        .await
                        .map_err(|err| {
                            error!(?filename, ?err, "Failed to read file bytes");
                            InvalidBoundary::default()
                        })?
                        .to_vec();

                    form.file_format = FileFormat::from_filename(&filename);
                }
                _ => {}
            }
        }

        info!(
            "Importing recipes from '{}' using {} parser (format: {})",
            form.file_name, form.app, form.file_format
        );
        Ok(form)
    }
}

/// Represents the content of the "Add Recipe -> Import -> API" form.
#[derive(Deserialize)]
pub struct ImportFromApiForm {
    pub api: Api,
    pub url: String,
    pub username: String,
    pub password: String,
}

/// Represents the content of the share recipe form.
#[derive(Deserialize, Serialize)]
pub struct ShareRecipeForm {
    pub datetime: Option<String>,
}

/// Represents the content of a recipe category form.
#[derive(Deserialize, Serialize)]
pub struct RecipeCategoryForm {
    pub category: String,
}

/// Represents the content of a fetch recipe from URLs form.
#[derive(Deserialize, Serialize)]
pub struct RecipeScrapeForm {
    pub urls: String,
}

/// Represents parameters to pass when marking or unmarking a recipe as favourite.
#[derive(Deserialize, Serialize)]
pub struct FavouriteParams {
    #[serde(rename = "view-recipe")]
    pub is_view_recipe: Option<bool>,
}

/// Stores parameters for order.
#[derive(Deserialize, Serialize)]
pub struct OrderParams {
    pub index: usize,
    #[serde(rename = "max-index")]
    pub max_index: usize,
}

/// Represents the content of a JSON preview form.
#[derive(Deserialize, Serialize)]
pub struct PreviewForm {
    #[serde(rename = "json-input")]
    pub(crate) json_input: String,
}

/// Represents the components of the form used to add a new event to the timeline.
#[derive(Default)]
pub struct TimelineEventForm {
    pub title: String,
    pub date: Option<PrimitiveDateTime>,
    pub image: HashMap<String, PathBuf>,
    pub original_image_filename: Option<Uuid>,
    pub comment: Option<String>,
    pub rating: Option<i16>,

    pub index: Option<usize>,
    pub max_index: Option<usize>,
}

impl<S> FromRequest<S> for TimelineEventForm
where
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut form = Self::default();

        let mut images: HashMap<String, PathBuf> = HashMap::new();
        let mut videos: HashMap<String, PathBuf> = HashMap::new();

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            error!(
                ?err,
                "Failed to read multipart field in timeline event form"
            );
            InvalidBoundary::default()
        })? {
            let name = field.name().unwrap_or("");
            match name {
                "title" => form.title = text_trim(field).await.unwrap_or_default(),
                "date" => {
                    form.date = text_trim(field).await.and_then(|s| {
                        Date::parse(&s, format_description!("[year]-[month]-[day]"))
                            .ok()
                            .map(|d| PrimitiveDateTime::new(d, time::Time::MIDNIGHT))
                    });
                }
                "image" => {
                    form.original_image_filename = field.file_name().map(|s| {
                        let stem = PathBuf::from(s)
                            .file_stem()
                            .map(|s| s.to_string_lossy().into_owned())
                            .unwrap_or_default();

                        Uuid::parse_str(&stem).unwrap_or_default()
                    });

                    if let Err(err) = save_media_field(field, &mut images, &mut videos).await {
                        error!(?err, "Saving media failed");
                    }
                }
                "comment" => form.comment = text_trim(field).await,
                "rating-event" => form.rating = text_trim(field).await.and_then(|s| s.parse().ok()),
                "index" => form.index = text_trim(field).await.and_then(|s| s.parse().ok()),
                "max-index" => form.max_index = text_trim(field).await.and_then(|s| s.parse().ok()),
                _ => {}
            }
        }

        form.image = images;
        Ok(form)
    }
}
