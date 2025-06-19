use axum::routing::get;
use axum::{Router, middleware};

use crate::handlers::general::{index_handler, user_initials_handler, ws_handler};
use crate::middleware::mw_auth;
use crate::middleware::mw_auth::mw_redirect_if_authenticated;
use app::state::AppState;

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
            "/user-initials",
            get(user_initials_handler).layer(middleware::from_fn(mw_auth::mw_ctx_require)),
        )
        .route(
            "/ws",
            get(ws_handler).layer(middleware::from_fn(mw_auth::mw_ctx_require)),
        )
}

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_index {
        use super::*;

        use config::Config;

        use testing::utils::{TestDb, build_server_logged_in};

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

    mod tests_user_initials {
        use super::*;

        use axum::http::Method;

        use testing::utils::{TestDb, assert_must_be_logged_in, build_server_logged_in};

        const BASE_URI: &str = "/user-initials";

        #[tokio::test]
        async fn test_get_user_initials_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_get_user_initials_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(res.text(), "T");
            Ok(())
        }
    }
}
