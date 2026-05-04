use axum::Form;
use axum::extract::{OriginalUri, Path};
use axum::http::HeaderMap;
use axum::{extract::State, response::IntoResponse};
use axum_htmx::HX_PROMPT;
use chrono::NaiveDateTime;
use reqwest::StatusCode;
use tracing::error;

use app::state::AppState;
use models::data::{Data, ShoppingData};
use models::shopping::{
    ShareShoppingList, ShoppingList, ShoppingListDetails, ShoppingListItemForCreate,
    ShoppingListItemForUpdate,
};
use uuid::Uuid;

use crate::handlers::get_settings;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::{broadcast_error, broadcast_warning};
use crate::middleware::mw_auth::RequireAuth;
use crate::recipes_router::params::ShareRecipeForm;
use crate::schemas::shopping::{ListItemPayload, ListPayload};
use crate::{Error, Result};

/// Handles fetching the user's shopping lists.
pub async fn shopping_lists_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    let shopping_lists = match ShoppingList::get_all(&state.mm, user.id).await {
        Ok(lists) => lists,
        Err(err) => {
            error!(
                "Failed to get shopping lists for user '{}': {err:?}",
                user.id
            );
            broadcast_error(&state, user.id, "Failed to get shopping lists.").await;
            return Err(Error::Database);
        }
    };

    let selected_shopping_list = if let Some(list) = shopping_lists.first() {
        match ShoppingListDetails::get(&state.mm, list.id, user.id).await {
            Ok(details) => Some(details),
            Err(err) => {
                error!(
                    "Failed to get shopping list details of list '{}' for user '{}': {err:?}",
                    list.id, user.id
                );
                broadcast_error(&state, user.id, "Failed to get shopping list details.").await;
                None
            }
        }
    } else {
        None
    };

    Ok(templates::shopping::lists_index(
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            shopping: Some(ShoppingData {
                shopping_lists,
                selected_shopping_list,
            }),
            ..Default::default()
        },
        &settings,
    )
    .into_response())
}

pub async fn shopping_list_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    match ShoppingListDetails::get(&state.mm, list_id, user.id).await {
        Ok(list) => templates::shopping::render_shopping_list(&list).into_response(),
        Err(err) => {
            error!("Failed to get shopping list: {err}");
            Error::Database.into_response()
        }
    }
}

pub async fn shopping_list_delete_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(err) = ShoppingList::delete(&state.mm, list_id, user.id).await {
        error!("Failed to delete shopping list: {err}");
        broadcast_error(&state, user.id, "Failed to delete shopping list.").await;
        return Error::Database.into_response();
    }

    (StatusCode::SEE_OTHER, [("HX-Redirect", "/shopping/lists")]).into_response()
}

pub async fn shopping_list_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Form(payload): Form<ListPayload>,
) -> impl IntoResponse {
    if payload.name.is_empty() {
        broadcast_error(&state, user.id, "Title cannot be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    match ShoppingList::update_title(&state.mm, list_id, &payload.name, user.id).await {
        Ok(()) => {
            let num_items = ShoppingList::get(&state.mm, list_id, user.id)
                .await
                .map_or(0, |l| l.num_items);

            templates::shopping::render_shopping_list_title(list_id, &payload.name, num_items)
                .into_response()
        }
        Err(err) if err.to_string().contains("duplicate key") => {
            broadcast_warning(&state, user.id, "Title already exists.").await;
            Error::InvalidPayload.into_response()
        }
        Err(err) => {
            error!("Failed to update shopping list title: {err}");
            broadcast_error(&state, user.id, "Failed to update shopping list title.").await;
            Error::Database.into_response()
        }
    }
}

pub async fn shopping_list_print_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    let list = match ShoppingListDetails::get(&state.mm, list_id, user.id).await {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to get shopping list: {err}");
            return Error::Database.into_response();
        }
    };

    let content = templates::shopping::render_shopping_list_print_mode(&list);

    templates::general::render_print_view(content).into_response()
}

pub async fn shopping_list_share_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Form(form): Form<ShareRecipeForm>,
) -> impl IntoResponse {
    let expires_at: Option<NaiveDateTime> = form.datetime.and_then(|dt| {
        NaiveDateTime::parse_from_str(dt.as_str(), "%Y-%m-%dT%H:%M")
            .or_else(|_| NaiveDateTime::parse_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S%.f"))
            .or_else(|_| NaiveDateTime::parse_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S"))
            .inspect_err(|err| {
                error!("Invalid datetime '{}': {}", dt, err);
            })
            .ok()
    });

    match ShareShoppingList::new(&state.mm, list_id, user.id, expires_at).await {
        Ok(share) => {
            let url = format!(
                "{}/shared/sl/{}",
                state.config.read().await.base_url,
                share.link
            );
            templates::general::share_link(&url).into_response()
        }
        Err(err) => {
            error!(
                "Error generating shared recipe link for recipe '{list_id}' and user '{}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Error creating shared shopping list link.").await;
            Error::BadTimeFormat.into_response()
        }
    }
}

pub async fn shopping_list_labels_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Form(form): Form<ListPayload>,
) -> impl IntoResponse {
    if form.name.is_empty() {
        broadcast_warning(&state, user.id, "Label name cannot be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = ShoppingList::new_label(&state.mm, &form.name, user.id).await {
        broadcast_error(&state, user.id, "Error creating shopping list label.").await;
        error!("Failed to create shopping list label: {err}");
        return Error::Database.into_response();
    }

    templates::shopping::render_shopping_list_items(form.name, list_id, &[], true).into_response()
}

pub async fn shopping_list_labels_new_handler(
    RequireAuth(_): RequireAuth,
    Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
    templates::shopping::render_label_new(list_id).into_response()
}

pub async fn shopping_list_label_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, label_id)): Path<(Uuid, i64)>,
    Form(payload): Form<ListPayload>,
) -> impl IntoResponse {
    let new_label_id = match ShoppingList::get_or_insert_label(&state.mm, &payload.name).await {
        Ok(id) => id,
        Err(err) => {
            error!("Failed to update shopping list label: {err}");
            broadcast_error(&state, user.id, "Failed to update shopping list label.").await;
            return Error::Database.into_response();
        }
    };

    if let Err(err) =
        ShoppingList::update_item_labels(&state.mm, list_id, label_id, new_label_id, user.id).await
    {
        error!("Failed to update shopping list label: {err}");
        broadcast_error(&state, user.id, "Failed to update shopping list label.").await;
        return Error::Database.into_response();
    }

    templates::shopping::render_label(&payload.name, list_id, new_label_id).into_response()
}

pub async fn shopping_lists_post_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let Some(title) = header_map
        .get(HX_PROMPT)
        .map(|h| h.to_str().ok().unwrap_or_default().trim())
        .filter(|s| !s.is_empty())
    else {
        broadcast_warning(&state, user.id, "List title must not be empty.").await;
        return Error::InvalidPayload.into_response();
    };

    let list_id = match ShoppingList::create(&state.mm, title, user.id).await {
        Ok(id) => id,
        Err(err) => {
            error!("Failed to create shopping list: {err:?}");
            broadcast_error(&state, user.id, "Failed to create shopping list.").await;
            return Error::Database.into_response();
        }
    };

    templates::shopping::render_new_shopping_list(list_id, title).into_response()
}

pub async fn shopping_list_item_post_handler(
    RequireAuth(user): RequireAuth,
    Path(list_id): Path<Uuid>,
    State(state): State<AppState>,
    Form(payload): Form<ListItemPayload>,
) -> impl IntoResponse {
    let item = payload.item.trim();
    if item.is_empty() {
        broadcast_warning(&state, user.id, "Item name must not be empty.").await;
        return Error::InvalidPayload.into_response();
    }

    let item = match ShoppingList::add_item(
        &state.mm,
        list_id,
        ShoppingListItemForCreate::new(
            payload.quantity.filter(|q| !q.is_empty()),
            item,
            payload.label,
            None,
        ),
        user.id,
    )
    .await
    {
        Ok(id) => id,
        Err(err) if err.to_string().contains("duplicate") => {
            broadcast_warning(&state, user.id, "Item already exists in the label.").await;
            return Error::EntityExists {
                entity: "shopping list item",
            }
            .into_response();
        }
        Err(err) => {
            error!("Failed to add shopping list item: {err:?}");
            broadcast_error(&state, user.id, "Failed to add shopping list item.").await;
            return Error::Database.into_response();
        }
    };

    let num_items = ShoppingList::items_count(&state.mm, list_id)
        .await
        .unwrap_or_default();

    templates::shopping::render_shopping_list_item_with_count(list_id, &item, num_items, false)
        .into_response()
}

pub async fn shopping_list_item_delete_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, item_id)): Path<(Uuid, i64)>,
) -> impl IntoResponse {
    if let Err(err) = ShoppingList::delete_item(&state.mm, list_id, item_id, user.id).await {
        error!("Failed to delete shopping list item: {err}");
        broadcast_error(&state, user.id, "Failed to delete shopping list item.").await;
        return Error::Database.into_response();
    }

    let num_items = ShoppingList::items_count(&state.mm, list_id)
        .await
        .unwrap_or_default();

    templates::shopping::render_shopping_list_item_count(list_id, num_items, true).into_response()
}

pub async fn shopping_list_item_put_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, item_id)): Path<(Uuid, i64)>,
    Form(payload): Form<ListItemPayload>,
) -> impl IntoResponse {
    let item_u = ShoppingListItemForUpdate {
        ingredient: Some(payload.item),
        quantity: payload.quantity,
        label: None,
        position: None,
        is_checked: None,
    };

    if let Err(err) = ShoppingList::update_item(&state.mm, list_id, item_id, item_u, user.id).await
        && !err
            .to_string()
            .contains("shopping_list_items_quantity_check")
    {
        error!("Failed to update shopping list item: {err:?}");
        broadcast_error(&state, user.id, "Failed to update shopping list item.").await;
        return Error::Database.into_response();
    }

    match ShoppingList::get_item(&state.mm, list_id, item_id, user.id).await {
        Ok(item) => {
            let num_items = ShoppingList::items_count(&state.mm, list_id)
                .await
                .unwrap_or_default();

            templates::shopping::render_shopping_list_item_with_count(
                list_id, &item, num_items, false,
            )
            .into_response()
        }
        Err(err) => {
            error!("Failed to get shopping list item: {err:?}");
            broadcast_error(&state, user.id, "Failed to get shopping list item.").await;
            Error::Database.into_response()
        }
    }
}

pub async fn shopping_list_item_edit_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path((list_id, item_id)): Path<(Uuid, i64)>,
) -> impl IntoResponse {
    match ShoppingList::get_item(&state.mm, list_id, item_id, user.id).await {
        Ok(item) => {
            templates::shopping::shopping_list_item(list_id, Some(&item.label), Some(&item))
                .into_response()
        }
        Err(err) => {
            error!("Failed to get shopping list item: {err:?}");
            broadcast_error(&state, user.id, "Failed to get shopping list item.").await;
            Error::EntityNotFound {
                entity: "shopping_list_item",
            }
            .into_response()
        }
    }
}
