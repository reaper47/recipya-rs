use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::{IntoResponse, Redirect};

use crate::server::AppState;
use crate::server::router::middleware::mw_auth::CtxW;

/// Handles the index page.
pub async fn index_handler() -> Redirect {
    Redirect::to("/auth/login")
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
