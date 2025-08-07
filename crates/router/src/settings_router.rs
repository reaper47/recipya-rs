use axum::routing::get;
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::settings::settings_handler;
use crate::middleware::mw_auth;

/// Defines the routes for endpoints related to the settings module.
pub(super) fn settings_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(settings_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_auth::mw_ctx_require,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_ws_message, build_server_logged_in,
        build_server_ws, create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_settings {
        use super::*;
        use axum::http::Method;

        const BASE_URI: &str = "/settings";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }
    }
}
