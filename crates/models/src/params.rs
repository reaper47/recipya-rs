use serde::Deserialize;

/// Search params holds the search query parameters of the URL.
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub sort: Option<String>,
    pub page: Option<u64>,
}
