use axum::extract::OriginalUri;
use axum::http::HeaderMap;
use axum::{extract::State, response::IntoResponse};
use tracing::error;

use app::state::AppState;
use models::data::{Data, ShoppingData};
use models::shopping::{ShoppingList, ShoppingListDetails};

use crate::handlers::get_settings;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::broadcast_error;
use crate::middleware::mw_auth::RequireAuth;
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
