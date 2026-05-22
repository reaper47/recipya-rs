use serde::{Deserialize, Serialize};

use models::export::ExportType;

#[derive(Deserialize, Serialize)]
pub struct PaperSizeForm {
    #[serde(rename = "paper-size")]
    pub paper_size: i16,
}

/// Represents the payload for setting themes.
#[derive(Deserialize, Serialize)]
pub struct ThemePayload {
    pub theme: String,
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
