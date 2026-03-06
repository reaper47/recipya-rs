use serde::{Deserialize, Serialize};

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
