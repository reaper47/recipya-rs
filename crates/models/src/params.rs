use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{export::ExportType, view::ViewMode};

/// Holds the query parameters for exporting a shopping list.
#[derive(Debug, Deserialize)]
pub struct ShoppingListExportParams {
    pub format: ExportType,
}

/// Holds the download query parameters of the URL.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloadParams {
    pub token: Uuid,
}

/// Holds the fetch query parameters of the URL.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FetchParams {
    pub url: String,
}

/// Holds the search query parameters of the URL.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchParams {
    #[serde(rename = "fav")]
    pub is_favourites: Option<bool>,
    pub q: Option<String>,
    pub sort: Option<String>,
    pub page: Option<u64>,
}

/// Holds the view query parameters of the URL.
#[derive(Debug, Default, Deserialize)]
pub struct ViewParams {
    pub mode: ViewMode,
}
