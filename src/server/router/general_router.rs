use axum::routing::get;
use axum::{middleware, Router};

use crate::server::router::handlers::general::{index_handler, ws_handler};
use crate::server::router::middleware::mw_auth;
use crate::server::router::middleware::mw_auth::mw_redirect_if_authenticated;
use crate::server::AppState;

/// Defines the routes for general endpoints of the web application.
pub(super) fn general_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(index_handler).layer(middleware::from_fn_with_state(
                state.clone(),
                mw_redirect_if_authenticated,
            )),
        )
        .route(
            "/ws",
            get(ws_handler).layer(middleware::from_fn(mw_auth::mw_ctx_require)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod index {
        use super::*;
        use crate::core::config::Config;
        use crate::server::test_utils::{build_server_logged_in, TestDb};

        const BASE_URI: &str = "/";

        #[tokio::test]
        async fn test_get_index_redirect_to_recipes_when_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }
    }
}
