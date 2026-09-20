use axum::routing::{delete, get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::admin::{
    add_user_handler, delete_user_handler, update_user_form_handler, update_user_handler,
    user_row_handler,
};
use crate::middleware::mw_auth::{mw_only_admin, mw_refresh_token};

/// Defines the routes for endpoints related to the administrator module.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn admin_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/user", post(add_user_handler))
        .route(
            "/user/{:id}",
            delete(delete_user_handler)
                .patch(update_user_handler)
                .get(update_user_form_handler),
        )
        .route("/user/{:id}/row", get(user_row_handler))
        .layer(middleware::from_fn_with_state(state.clone(), mw_only_admin))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ))
}
