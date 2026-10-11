use axum::routing::{delete, get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::auth as h;
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the authentication-related routes.
pub fn auth_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/change-password", post(h::change_password_post_handler))
        .route("/user", delete(h::user_delete_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ));

    let public = Router::new()
        .route(
            "/forgot-password",
            get(h::forgot_password_handler).post(h::forgot_password_post_handler),
        )
        .route(
            "/forgot-password/reset",
            get(h::forgot_password_reset_handler).post(h::forgot_password_reset_post_handler),
        )
        .route("/login", get(h::login_handler).post(h::login_post_handler))
        .route("/logout", post(h::logout_post_handler))
        .route(
            "/register",
            get(h::register_handler).post(h::register_post_handler),
        )
        .route("/verify-email", get(h::verify_email_handler));

    Router::new().merge(public).merge(protected)
}
