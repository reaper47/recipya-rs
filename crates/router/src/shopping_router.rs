use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};

use app::state::AppState;

use crate::{
    handlers::shopping::{
        shopping_list_copy_handler, shopping_list_delete_handler, shopping_list_export_handler,
        shopping_list_handler, shopping_list_item_delete_handler, shopping_list_item_edit_handler,
        shopping_list_item_post_handler, shopping_list_item_put_handler,
        shopping_list_item_toggle_handler, shopping_list_items_positions_put_handler,
        shopping_list_label_put_handler, shopping_list_labels_new_handler,
        shopping_list_labels_post_handler, shopping_list_print_handler, shopping_list_put_handler,
        shopping_list_share_post_handler, shopping_list_view_handler, shopping_lists_handler,
        shopping_lists_post_handler, shopping_recipe_ingredients_handler,
        shopping_recipe_ingredients_post_handler,
    },
    middleware::mw_auth::mw_refresh_token,
};

/// Defines the shopping routes for the application.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn shopping_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route(
            "/lists",
            get(shopping_lists_handler).post(shopping_lists_post_handler),
        )
        .route(
            "/lists/{:list_id}",
            get(shopping_list_handler)
                .delete(shopping_list_delete_handler)
                .put(shopping_list_put_handler),
        )
        .route("/lists/{:list_id}/copy", get(shopping_list_copy_handler))
        .route(
            "/lists/{:list_id}/export",
            get(shopping_list_export_handler),
        )
        .route("/lists/{:list_id}/print", get(shopping_list_print_handler))
        .route(
            "/lists/{:list_id}/share",
            post(shopping_list_share_post_handler),
        )
        .route("/lists/{:list_id}/view", get(shopping_list_view_handler))
        .route(
            "/lists/{:list_id}/labels",
            post(shopping_list_labels_post_handler),
        )
        .route(
            "/lists/{:list_id}/labels/new",
            get(shopping_list_labels_new_handler),
        )
        .route(
            "/lists/{:list_id}/labels/{:label_id}",
            put(shopping_list_label_put_handler),
        )
        .route(
            "/lists/{:list_id}/items",
            post(shopping_list_item_post_handler),
        )
        .route(
            "/lists/{:list_id}/items/positions",
            put(shopping_list_items_positions_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}",
            delete(shopping_list_item_delete_handler).put(shopping_list_item_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}/edit",
            get(shopping_list_item_edit_handler),
        )
        .route(
            "/recipes/{:recipe_id}/ingredients",
            get(shopping_recipe_ingredients_handler).post(shopping_recipe_ingredients_post_handler),
        )
        .layer(from_fn_with_state(state.clone(), mw_refresh_token));

    let public = Router::new().route(
        "/lists/items/{:item_id}/toggle",
        post(shopping_list_item_toggle_handler),
    );

    Router::new().merge(public).merge(protected)
}
