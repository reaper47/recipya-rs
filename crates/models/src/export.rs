use std::{
    collections::HashMap,
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tokio::task::spawn_blocking;
use tracing::error;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

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

impl From<&[RecipeDetails]> for Manifest {
    fn from(recipes: &[RecipeDetails]) -> Self {
        let mut manifest: HashMap<i64, &RecipeDetails> = HashMap::new();
        for (idx, r) in recipes.iter().enumerate() {
            let order = i64::try_from(idx + 1)
                .inspect_err(|err| error!("Failed to convert manifest export index to i64: {err}"))
                .unwrap_or(0);

            manifest.insert(order, r);
        }

        Self {
            recipes: manifest
                .iter()
                .map(|(u, recipe)| ManifestRecipe {
                    directory: u.to_string(),
                    recipe_name: recipe.recipe.name.clone(),
                })
                .collect(),
            version: 1,
        }
    }
}

#[derive(Serialize)]
struct ManifestRecipe {
    directory: String,
    recipe_name: String,
}

/// Represents the data to be exported.
pub struct ExportData {
    r#type: ExportType,
    recipes: Vec<RecipeDetails>,
    images_dir: PathBuf,
}

impl ExportData {
    /// Creates a new `ExportData` instance.
    pub fn new(r#type: ExportType, recipes: Vec<RecipeDetails>, images_dir: &Path) -> Self {
        Self {
            r#type,
            recipes,
            images_dir: images_dir.to_path_buf(),
        }
    }

    /// Exports the data to the specified type.
    ///
    /// Returns the path to the file in the temp directory.
    pub async fn export(self) -> Result<PathBuf> {
        let std_file = spawn_blocking(move || {
            let mut file = NamedTempFile::new()?;
            let mut zip = ZipWriter::new(&mut file);
            let options = FileOptions::<()>::default()
                .compression_method(CompressionMethod::Deflated)
                .compression_level(Some(1));

            let manifest: Manifest = Manifest::from(self.recipes.as_slice());
            zip.start_file("manifest.json", options)?;
            serde_json::to_writer_pretty(&mut zip, &manifest)?;

            match self.r#type {
                ExportType::Json => {
                    for (idx, recipe) in self.recipes.into_iter().enumerate() {
                        let recipe_id = recipe.recipe.id;

                        for img in recipe.all_images() {
                            let fname = format!("{img}.webp");
                            let path = self.images_dir.join(&fname);
                            if path.exists() {
                                let file = std::fs::File::open(&path)?;
                                let mut buff = std::io::BufReader::new(file);
                                zip.start_file(format!("{}/{fname}", idx + 1), options)?;
                                std::io::copy(&mut buff, &mut zip)?;
                            }
                        }

                        let serialized =
                            serde_json::to_vec_pretty(&schema_org::Recipe::from(recipe))
                                .inspect_err(|err| {
                                    error!("Failed to serialize recipe '{recipe_id}': {err}");
                                })?;
                        zip.start_file(format!("{}/recipe.json", idx + 1), options)?;
                        zip.write_all(&serialized)?;
                    }
                }
                ExportType::Pdf => {
                    todo!()
                }
            }

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
