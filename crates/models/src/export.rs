use std::{
    collections::HashMap,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tokio::task::spawn_blocking;
use uuid::Uuid;
use zip::{ZipWriter, write::FileOptions};

use crate::{Error, RecipeDetails, Result};

/// Represents the type of export to perform.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Json,
    Pdf,
}

#[derive(Serialize)]
struct Manifest {
    recipes: Vec<ManifestRecipe>,
    version: u8,
}

#[derive(Serialize)]
struct ManifestRecipe {
    path: String,
    recipe_name: String,
}

/// Represents the data to be exported.
pub struct ExportData {
    pub r#type: ExportType,
    pub recipes: Vec<RecipeDetails>,
}

impl ExportData {
    /// Creates a new `ExportData` instance.
    pub fn new(r#type: ExportType, recipes: Vec<RecipeDetails>) -> Self {
        Self { r#type, recipes }
    }

    /// Exports the data to the specified type.
    ///
    /// Returns the path to the file in the temp directory.
    pub async fn export(self) -> Result<PathBuf> {
        let std_file = spawn_blocking(move || {
            let mut file = NamedTempFile::new()?;
            let mut zip = ZipWriter::new(&mut file);
            let options = FileOptions::<()>::default();

            let mut manifest: HashMap<Uuid, RecipeDetails> = HashMap::new();
            self.recipes.into_iter().for_each(|r| {
                manifest.insert(Uuid::new_v4(), r);
            });

            let manifest = Manifest {
                recipes: manifest
                    .iter()
                    .map(|(u, recipe)| ManifestRecipe {
                        path: format!("{u}.json"),
                        recipe_name: recipe.recipe.name.clone(),
                    })
                    .collect(),
                version: 1,
            };

            zip.start_file("manifest.json", options)?;
            serde_json::to_writer_pretty(&mut zip, &manifest)?;

            // match self.r#type {
            //     ExportType::Json => {
            //         for recipe in &self.recipes {
            //             zip.start_file(format!("{}.json", recipe.recipe.id), options)?;
            //             let json = serde_json::to_string(&schema_org::Recipe::from(recipe))?;
            //             serde_json::to_writer_pretty(&mut zip, &json)?;
            //         }
            //     }
            //     ExportType::Pdf => {}
            // }

            zip.finish()?;
            file.seek(SeekFrom::Start(0))?;
            let (_, path) = file.keep()?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(path)
        })
        .await
        .map_err(|err| Error::File(err.to_string()))?
        .map_err(|err| Error::File(err.to_string()))?;

        Ok(std_file)
    }
}
