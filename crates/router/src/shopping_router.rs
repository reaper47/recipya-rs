use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};

use app::state::AppState;

use crate::{handlers::shopping as h, middleware::mw_auth::mw_refresh_token};

/// Defines the shopping routes for the application.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn shopping_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route(
            "/lists",
            get(h::shopping_lists_handler).post(h::shopping_lists_post_handler),
        )
        .route(
            "/lists/{:list_id}",
            get(h::shopping_list_handler)
                .delete(h::shopping_list_delete_handler)
                .put(h::shopping_list_put_handler),
        )
        .route("/lists/{:list_id}/copy", get(h::shopping_list_copy_handler))
        .route(
            "/lists/{:list_id}/export",
            get(h::shopping_list_export_handler),
        )
        .route(
            "/lists/{:list_id}/print",
            get(h::shopping_list_print_handler),
        )
        .route(
            "/lists/{:list_id}/share",
            post(h::shopping_list_share_post_handler),
        )
        .route("/lists/{:list_id}/view", get(h::shopping_list_view_handler))
        .route(
            "/lists/{:list_id}/labels",
            post(h::shopping_list_labels_post_handler),
        )
        .route(
            "/lists/{:list_id}/labels/new",
            get(h::shopping_list_labels_new_handler),
        )
        .route(
            "/lists/{:list_id}/labels/{:label_id}",
            put(h::shopping_list_label_put_handler),
        )
        .route(
            "/lists/{:list_id}/items",
            post(h::shopping_list_item_post_handler),
        )
        .route(
            "/lists/{:list_id}/items/positions",
            put(h::shopping_list_items_positions_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}",
            delete(h::shopping_list_item_delete_handler).put(h::shopping_list_item_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}/edit",
            get(h::shopping_list_item_edit_handler),
        )
        .route(
            "/recipes/{:recipe_id}/ingredients",
            get(h::shopping_recipe_ingredients_handler)
                .post(h::shopping_recipe_ingredients_post_handler),
        )
        .layer(from_fn_with_state(state.clone(), mw_refresh_token));

    let public = Router::new().route(
        "/lists/items/{:item_id}/toggle",
        post(h::shopping_list_item_toggle_handler),
    );

    Router::new().merge(public).merge(protected)
}
