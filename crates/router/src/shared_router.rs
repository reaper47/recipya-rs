use axum::Router;
use axum::routing::get;

use app::state::AppState;

use crate::handlers::shared as h;

/// Defines the routes for shared resources.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn shared_routes() -> Router<AppState> {
    Router::new()
        .route("/r/{:link}", get(h::share_recipe_handler))
        .route("/sl/{:list_id}", get(h::share_shopping_list_handler))
}
