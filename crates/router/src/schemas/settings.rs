use serde::{Deserialize, Serialize};

use models::export::ExportType;

/// Represents the payload for setting paper sizes.
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

/// Represents the payload for setting a timezone.
#[derive(Deserialize, Serialize)]
pub struct TzPayload {
    pub tz: String,
}

/// Represents the payload for exporting data.
#[derive(Debug, Deserialize)]
pub struct ExportDataPayload {
    pub r#type: ExportType,
    #[serde(rename = "recipe-ids", default)]
    pub recipe_ids: Vec<i64>,
}

/// Represents the payload for the bold ingredients setting.
#[derive(Deserialize, Serialize)]
pub struct BoldIngredientsPayload {
    #[serde(rename = "bold-ingredients")]
    pub is_bold_ingredients: Option<String>,
}

/// Represents the payload for setting nutrition sources.
#[derive(Deserialize, Serialize)]
pub struct NutritionSourcePayload {
    #[serde(rename = "nutrition-source")]
    pub nutrition_source: String,
}
