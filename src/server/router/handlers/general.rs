use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::{IntoResponse, Redirect};
use tracing::error;

use crate::core::model::user::User;
use crate::server::AppState;
use crate::server::router::middleware::mw_auth::CtxW;

/// Handles the index page.
pub async fn index_handler() -> Redirect {
    Redirect::to("/auth/login")
}

/// Handlers the handler to retrieve the user's initials.
pub async fn user_initials_handler(ctx: CtxW, State(state): State<AppState>) -> impl IntoResponse {
    let user_id = ctx.0.user_id();
    match User::get_user_by_id(&state.mm, user_id).await {
        Ok(Some(user)) => {
            if let Some(first) = user.email.to_uppercase().chars().next() {
                first.to_string()
            } else {
                "A".into()
            }
        }
        Ok(None) => {
            error!("User {user_id} does not exist");
            "A".into()
        }
        Err(err) => {
            error!("Error getting user: {err}");
            "A".into()
        }
    }
}

/// WebSocket connection handler.
pub async fn ws_handler(
    ctx: CtxW,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(ctx.0.user_id(), socket, state))
}

async fn handle_socket(user_id: i64, socket: WebSocket, state: AppState) {
    state
        .subscribers
        .lock()
        .await
        .entry(user_id)
        .or_insert_with(Vec::new)
        .push(socket);
}
