use serde::{Deserialize, Serialize};

/// Represents the payload for setting themes.
#[derive(Deserialize, Serialize)]
pub struct ThemePayload {
    pub theme: String,
}

/// Represents the payload for setting nutrition sources.
#[derive(Deserialize, Serialize)]
pub struct NutritionSourcePayload {
    #[serde(rename = "nutrition-source")]
    pub nutrition_source: String,
}
