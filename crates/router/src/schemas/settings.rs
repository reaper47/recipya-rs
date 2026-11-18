use jiff::tz::TimeZone;
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
    #[serde(with = "tz_name")]
    pub tz: TimeZone,
}

// AI usage:
// I asked Claude Sonnet 5.5 to implement a serialize and deserialize function for TimeZone.
mod tz_name {
    use jiff::tz::TimeZone;
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::Error as _};

    pub fn serialize<S: Serializer>(tz: &TimeZone, serializer: S) -> Result<S::Ok, S::Error> {
        let name = tz
            .iana_name()
            .ok_or_else(|| S::Error::custom("time zone has no IANA name"))?;
        serializer.serialize_str(name)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TimeZone, D::Error> {
        let name = String::deserialize(deserializer)?;
        TimeZone::get(&name).map_err(D::Error::custom)
    }
}

/// Represents the payload for exporting data.
#[derive(Debug, Deserialize)]
pub struct ExportDataPayload {
    pub r#type: ExportType,
    #[serde(rename = "recipe-ids", default)]
    pub recipe_ids: Vec<i64>,
}

/// Represents the payload
#[derive(Debug, Deserialize, Serialize)]
pub struct LanguagePayload {
    pub locale: i64,
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
