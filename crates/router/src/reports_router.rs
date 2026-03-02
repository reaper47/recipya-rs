use axum::{Router, middleware::from_fn_with_state, routing::get};

use app::state::AppState;

use crate::{handlers::reports::reports_handler, middleware::mw_auth::mw_refresh_token};

/// Defines the routes for endpoints related to the reports module.
pub fn reports_router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(reports_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token))
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use testing::utils::{TestDb, assert_html, assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/reports";

    mod tests_get {
        use super::*;

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_no_reports_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<ul class="menu bg-base-100 w-full"></ul>"#,
                    r#"<p class="p-4">No reports found.</p>"#,
                ],
            );
            Ok(())
        }
    }
}
