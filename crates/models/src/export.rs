use std::{
    collections::HashMap,
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tokio::task::spawn_blocking;
use tracing::{error, warn};
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{Error, RecipeDetails, Result};

/// Represents the type of export to perform.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Json,
    Markdown,
    Pdf,
    Text,
}

impl ExportType {
    /// Returns the file extension for the export type.
    pub const fn extension(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Markdown => "md",
            Self::Pdf => "pdf",
            Self::Text => "txt",
        }
    }
}

/// Represents export options.
pub struct ExportOptions {
    pub paper_size: (f32, f32),
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

            let base_url = std::env::var("RECIPYA_BASE_URL").unwrap_or_default();

            let Self {
                recipes,
                images_dir,
                r#type,
                ..
            } = self;

            for (idx, recipe) in recipes.into_iter().enumerate() {
                let recipe_id = recipe.recipe.id;

                if let Err(err) =
                    ExportData::add_images(&mut zip, &recipe, idx, options, &images_dir)
                {
                    error!("Failed to add images for recipe '{recipe_id}': {err}");
                }

                let serialized: Vec<u8> = match r#type {
                    ExportType::Json => serde_json::to_vec_pretty(&schema_org::Recipe::from(
                        recipe,
                    ))
                    .inspect_err(|err| {
                        error!("Failed to serialize recipe '{recipe_id}': {err}");
                    })?,
                    ExportType::Markdown => match recipe.to_markdown(&base_url) {
                        Ok(serialized) => serialized.into_bytes(),
                        Err(err) => {
                            warn!("Failed to write recipe '{recipe_id}' as markdown: {err}");
                            continue;
                        }
                    },
                    ExportType::Pdf => todo!(),
                    ExportType::Text => match recipe.to_text(&base_url) {
                        Ok(serialized) => serialized.into_bytes(),
                        Err(err) => {
                            warn!("Failed to write recipe '{recipe_id}' as markdown: {err}");
                            continue;
                        }
                    },
                };

                zip.start_file(format!("{}/recipe.md", idx + 1), options)?;
                zip.write_all(&serialized.as_slice())?;
            }

            zip.finish()?;
            file.seek(SeekFrom::Start(0))?;

            let named = std::env::temp_dir().join("recipya-data-export.zip");
            file.persist(&named)
                .map_err(|err| Error::File(err.to_string()))?;

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(named)
        })
        .await
        .map_err(|err| Error::File(err.to_string()))?
        .map_err(|err| Error::File(err.to_string()))?;

        Ok(std_file)
    }

    fn add_images(
        zip: &mut zip::ZipWriter<&mut NamedTempFile>,
        recipe: &RecipeDetails,
        idx: usize,
        options: FileOptions<'_, ()>,
        images_dir: &Path,
    ) -> Result<()> {
        for img in recipe.all_images() {
            let fname = format!("{img}.webp");
            let path = images_dir.join(&fname);
            if path.exists() {
                let file = std::fs::File::open(&path)?;
                let mut buf = std::io::BufReader::new(file);
                zip.start_file(format!("{}/{fname}", idx + 1), options)?;
                std::io::copy(&mut buf, zip)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_export {
        use super::*;

        #[test]
        fn test_export() {
            for (export, expected) in [
                (ExportType::Json, "json"),
                (ExportType::Text, "txt"),
                (ExportType::Markdown, "md"),
                (ExportType::Pdf, "pdf"),
            ] {
                let got = export.extension();

                pretty_assertions::assert_eq!(got, expected);
            }
        }
    }
}
