use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::export::ExportType;

/// Holds the query parameters for exporting a shopping list.
#[derive(Debug, Deserialize)]
pub struct ShoppingListExportParams {
    pub format: ExportType,
}

/// Download params holds the download query parameters of the URL.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloadParams {
    pub token: Uuid,
}

/// Fetch params holds the fetch query parameters of the URL.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FetchParams {
    pub url: String,
}

/// Search params holds the search query parameters of the URL.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchParams {
    #[serde(rename = "fav")]
    pub is_favourites: Option<bool>,
    pub q: Option<String>,
    pub sort: Option<String>,
    pub page: Option<u64>,
}
