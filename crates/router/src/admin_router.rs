use axum::routing::{delete, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::admin::delete_user_handler;
use crate::middleware::mw_auth::{mw_ctx_require, mw_only_admin};

/// Defines the routes for endpoints related to the administrator module.
pub(super) fn admin_routes(state: AppState) -> Router<AppState> {
    Router::new()
        //.route("/user", post(add_user_handler))
        .route("/user/{:id}", delete(delete_user_handler))
        .layer(middleware::from_fn(mw_only_admin))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_ctx_require,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Method;
    use testing::utils::{
        TestDb, assert_must_be_logged_in, assert_ws_message, build_server_logged_in,
        build_server_ws, build_server_ws_other_user, create_app_state, insert_other_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_users {
        use super::*;
        use axum_test::http::StatusCode;
        use models::user::User;

        fn base_uri(id: u32) -> String {
            format!("/admin/user/{id}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::DELETE, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_must_be_admin_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _) = build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

            let res = server.delete(&base_uri(2)).await;

            res.assert_status_forbidden();
            Ok(())
        }

        #[tokio::test]
        async fn test_cannot_delete_admin_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server.delete(&base_uri(1)).await;

            res.assert_status_forbidden();
            assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Cannot delete an admin.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_user_not_exist_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.delete(&base_uri(2)).await;

            res.assert_status(StatusCode::NO_CONTENT);
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_user_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let user2 = insert_other_user(config.clone(), "slava@ukraini.ua").await?;

            let res = server.delete(&base_uri(user2.id.try_into().unwrap())).await;

            res.assert_status(StatusCode::NO_CONTENT);
            let state = create_app_state(config.clone()).await;
            let user = User::get_user_by_id(&state.mm, user2.id).await;
            if user.is_ok() && user.unwrap().is_some() {
                panic!("User should be deleted");
            }
            Ok(())
        }
    }
}
