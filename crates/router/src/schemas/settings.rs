use serde::{Deserialize, Serialize};

/// Represents the payload for setting themes.
#[derive(Deserialize, Serialize)]
pub struct ThemePayload {
    pub theme: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Json,
    Pdf,
}

/// Represents the payload for exporting data.
#[derive(Debug, Deserialize)]
pub struct ExportDataPayload {
    pub r#type: ExportType,
    #[serde(rename = "recipe-ids", default)]
    pub recipe_ids: Vec<i64>,
}

/// Represents the payload for setting nutrition sources.
#[derive(Deserialize, Serialize)]
pub struct NutritionSourcePayload {
    #[serde(rename = "nutrition-source")]
    pub nutrition_source: String,
}
