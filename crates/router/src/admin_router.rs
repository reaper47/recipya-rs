use axum::routing::{delete, get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::admin as h;
use crate::middleware::mw_auth::{mw_only_admin, mw_refresh_token};

/// Defines the routes for endpoints related to the administrator module.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn admin_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/user", post(h::add_user_handler))
        .route(
            "/user/{:id}",
            delete(h::delete_user_handler)
                .patch(h::update_user_handler)
                .get(h::update_user_form_handler),
        )
        .route("/user/{:id}/row", get(h::user_row_handler))
        .layer(middleware::from_fn_with_state(state.clone(), mw_only_admin))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ))
}
