use axum::routing::get;
use axum::Router;

use crate::server::AppState;
use crate::server::router::handlers::recipes::recipe_view_handler;

/// Defines the routes for endpoints related to recipes.
pub(super) fn recipes_routes(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/{:recipe_id}",
        get(recipe_view_handler),
    )/*.layer(middleware::from_fn_with_state(
        state.clone(),
        mw_auth::mw_must_be_logged_in,
    ))*/
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::config::Config;
    use crate::server::test_utils::{build_server_logged_in, TestDb};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_recipe {
        use axum::http::HeaderValue;
        use super::*;

        fn base_uri(recipe_id: i64) -> String {
            format!("/recipes/{recipe_id}")
        }

        #[tokio::test]
        async fn test_get_recipe_must_be_logged_in() -> Result<()> {
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_exists_from_htmx_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let mut server = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::headers::HX_REQUEST, HeaderValue::from_static("true"));

            let res = server.get(&base_uri(1)).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_not_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }
    }
}
