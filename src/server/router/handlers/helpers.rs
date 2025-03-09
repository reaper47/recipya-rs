use axum::http::HeaderMap;

/// Checks whether the request was sent by htmx.
pub fn is_hx_request(header_map: &HeaderMap) -> bool {
    header_map
        .get(axum_htmx::headers::HX_REQUEST)
        .map(|v| v == "true")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::http::header::HeaderValue;
    use axum::http::HeaderMap;

    #[test]
    fn test_hx_request_present_and_true() {
        let mut headers = HeaderMap::new();
        headers.insert(axum_htmx::headers::HX_REQUEST, HeaderValue::from_static("true"));

        assert!(is_hx_request(&headers));
    }

    #[test]
    fn test_hx_request_present_but_false() {
        let mut headers = HeaderMap::new();
        headers.insert(axum_htmx::headers::HX_REQUEST, HeaderValue::from_static("false"));

        assert!(!is_hx_request(&headers));
    }

    #[test]
    fn test_hx_request_missing() {
        let headers = HeaderMap::new();

        assert!(!is_hx_request(&headers));
    }

    #[test]
    fn test_hx_request_case_sensitivity() {
        let mut headers = HeaderMap::new();
        headers.insert(axum_htmx::headers::HX_REQUEST, HeaderValue::from_static("TRUE"));

        assert!(!is_hx_request(&headers)); // Should be case-sensitive
    }

    #[test]
    fn test_hx_request_extra_whitespace() {
        let mut headers = HeaderMap::new();
        headers.insert(axum_htmx::headers::HX_REQUEST, HeaderValue::from_static(" true "));

        assert!(!is_hx_request(&headers)); // Whitespace-sensitive
    }
}
