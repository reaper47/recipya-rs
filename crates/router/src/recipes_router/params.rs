use axum::extract::multipart::{InvalidBoundary, MultipartRejection};
use axum::extract::{FromRequest, Multipart, Request};
use integrations::{App, FileFormat, parse_recipe};
use recipe_schema::RecipeSchema;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::str::FromStr;
use tracing::{error, info, warn};

/// Represents the content of the "Add Recipe -> Import from an app" form.
#[derive(Default)]
pub struct ImportFromAppForm {
    pub file_data: Vec<u8>,
    pub file_name: String,
    pub app: App,
    pub file_format: FileFormat,
}

impl<S> FromRequest<S> for ImportFromAppForm
where
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut form = ImportFromAppForm::default();

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            error!("Failed to read multipart field in import recipes from app: {err}");
            InvalidBoundary::default()
        })? {
            let name = field.name().unwrap_or("");
            match name {
                "app" => {
                    let app_name = field.text().await.map_err(|_| InvalidBoundary::default())?;
                    let app = App::from_str(&app_name.to_lowercase()).unwrap_or_default();
                    if matches!(app, App::Unknown) {
                        error!("Import recipes from app form field 'app' is invalid: {app_name}");
                        return Err(InvalidBoundary::default())?;
                    }
                    form.app = app;
                }
                "file" => {
                    let filename = field
                        .file_name()
                        .map(|s| s.to_string())
                        .ok_or(InvalidBoundary::default())?;
                    form.file_name = filename.clone();

                    form.file_data = field
                        .bytes()
                        .await
                        .map_err(|err| {
                            error!("Failed to read file bytes for '{filename}': {err}");
                            InvalidBoundary::default()
                        })?
                        .to_vec();

                    form.file_format = FileFormat::from_filename(&filename);
                }
                _ => {
                    warn!("Import recipes from app form field '{name}' is not processed")
                }
            }
        }

        info!(
            "Importing recipes from '{}' using {} parser (format: {})",
            form.file_name, form.app, form.file_format
        );
        Ok(form)
    }
}

impl ImportFromAppForm {
    /// Parses the recipe file contained in the form.
    pub fn parse_recipes(&mut self) -> crate::error::Result<Vec<RecipeSchema>> {
        let mut data = Cursor::new(&self.file_data);
        let app = &self.app;
        let file_name = &self.file_name;
        let file_format = &self.file_format;
        Ok(parse_recipe(&mut data, app, file_name, file_format)?)
    }
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

/// Represents the content of a JSON preview form.
#[derive(Deserialize, Serialize)]
pub struct PreviewForm {
    #[serde(rename = "json-input")]
    pub(crate) json_input: String,
}
