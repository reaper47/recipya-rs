use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use lib_web::{handlers::handlers_auth, middleware::mw_auth, AppState};

#[allow(unused)]
pub fn routes_auth(state: AppState) -> Router {
    Router::new()
        .route("/change-password", post(handlers_auth::change_password))
        .route("/confirm", get(handlers_auth::confirm))
        .route(
            "/forgot-password",
            get(handlers_auth::forgot_password).post(handlers_auth::forgot_password_post),
        )
        .route(
            "/forgot-password/reset",
            post(handlers_auth::forgot_password_reset_post),
        )
        .route(
            "/login",
            get(handlers_auth::login)
                .post(handlers_auth::login_post)
                .layer(middleware::from_fn(mw_auth::mw_redirect_if_authenticated)),
        )
        .route("/logout", post(handlers_auth::logout_post))
        .route(
            "/register",
            get(handlers_auth::register)
                .post(handlers_auth::register_post)
                .layer(middleware::from_fn(mw_auth::mw_redirect_if_authenticated)),
        )
        .with_state(state)
}
