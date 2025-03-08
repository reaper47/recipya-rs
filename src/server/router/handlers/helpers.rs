use axum::http::HeaderMap;

/// Checks whether the request was sent by htmx.
pub fn is_hx_request(headers: HeaderMap) -> bool {
    headers
        .get(axum_htmx::headers::HX_REQUEST)
        .map(|v| v == "true")
        .unwrap_or(false)
}
