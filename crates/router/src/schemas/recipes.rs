use std::{collections::HashMap, io::Cursor, path::PathBuf, str::FromStr};

use axum::extract::{
    FromRequest, Multipart, Request,
    multipart::{InvalidBoundary, MultipartRejection},
};
use schema_org::Recipe;
use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime, macros::format_description};
use tracing::{error, info};
use uuid::Uuid;

use integrations::{App, FileFormat, api::Api, parse_recipe};
use models::{
    recipe::{save_media_field, text_trim},
    types::Rating,
};

/// Represents the content of the "Add Recipe -> Import -> Software" form.
#[derive(Clone, Default)]
pub struct ImportFromAppForm {
    pub app: App,
    pub files: Vec<AppFile>,
}

/// The description and content of a file.
#[derive(Clone, Default)]
pub struct AppFile {
    pub data: Vec<u8>,
    pub name: String,
    pub format: FileFormat,
}

impl ImportFromAppForm {
    /// Parses the recipe files contained in the form.
    pub fn parse_recipes(&self) -> crate::error::Result<Vec<Recipe>> {
        let mut recipes = Vec::new();

        for file in &self.files {
            recipes.extend(parse_recipe(
                &mut Cursor::new(&file.data),
                &self.app,
                &file.name,
                &file.format,
            )?);
        }

        Ok(recipes)
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
                    let name = field.text().await.map_err(|_| InvalidBoundary::default())?;
                    let app = App::from_str(&name.to_lowercase()).unwrap_or_default();
                    if matches!(app, App::Unknown) {
                        error!(?name, "Import recipes from app form field 'app' is invalid");
                        Err(InvalidBoundary::default())?;
                    }
                    form.app = app;
                }
                "file" => {
                    let file_name = field
                        .file_name()
                        .map(ToString::to_string)
                        .ok_or_else(InvalidBoundary::default)?;

                    form.files.push(AppFile {
                        data: field
                            .bytes()
                            .await
                            .map_err(|err| {
                                error!(?file_name, ?err, "Failed to read file bytes");
                                InvalidBoundary::default()
                            })?
                            .to_vec(),
                        format: FileFormat::from_filename(&file_name),
                        name: file_name,
                    });
                }
                _ => {}
            }
        }

        for f in &form.files {
            info!(
                "Importing recipes from '{}' using {} parser (format: {})",
                f.name, form.app, f.format
            );
        }

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

impl ShareRecipeForm {
    pub const fn new(datetime: Option<String>) -> Self {
        Self { datetime }
    }
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
    pub json_input: String,
}

/// Represents the components of the form used to add a new event to the timeline.
#[derive(Default)]
pub struct TimelineEventForm {
    pub title: String,
    pub date: Option<PrimitiveDateTime>,
    pub image: HashMap<String, PathBuf>,
    pub original_image_filename: Option<Uuid>,
    pub comment: Option<String>,
    pub rating: Rating,

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
                "rating-event" => {
                    form.rating = Rating::from(text_trim(field).await.and_then(|s| s.parse().ok()));
                }
                "index" => form.index = text_trim(field).await.and_then(|s| s.parse().ok()),
                "max-index" => {
                    form.max_index = text_trim(field).await.and_then(|s| s.parse().ok());
                }
                _ => {}
            }
        }

        form.image = images;
        Ok(form)
    }
}

/// Content of the form used to delete multiple recipes at once.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct BulkDeleteForm {
    #[serde(default)]
    pub ids: Vec<i64>,
}

/// Content of the form used to update the category of multiple recipes at once.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct BulkCategoriseForm {
    #[serde(default)]
    pub ids: Vec<i64>,
    pub name: String,
}
