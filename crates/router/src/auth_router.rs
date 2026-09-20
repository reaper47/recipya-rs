use axum::routing::{delete, get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::auth::{
    change_password_post_handler, forgot_password_handler, forgot_password_post_handler,
    forgot_password_reset_handler, forgot_password_reset_post_handler, login_handler,
    login_post_handler, logout_post_handler, register_handler, register_post_handler,
    user_delete_handler, verify_email_handler,
};
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the authentication-related routes.
pub fn auth_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/change-password", post(change_password_post_handler))
        .route("/user", delete(user_delete_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ));

    let public = Router::new()
        .route(
            "/forgot-password",
            get(forgot_password_handler).post(forgot_password_post_handler),
        )
        .route(
            "/forgot-password/reset",
            get(forgot_password_reset_handler).post(forgot_password_reset_post_handler),
        )
        .route("/login", get(login_handler).post(login_post_handler))
        .route("/logout", post(logout_post_handler))
        .route(
            "/register",
            get(register_handler).post(register_post_handler),
        )
        .route("/verify-email", get(verify_email_handler));

    Router::new().merge(public).merge(protected)
}
