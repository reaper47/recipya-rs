use std::{thread, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{IntoResponse, Redirect},
};
use lib_core::config;

use crate::{middleware::mw_auth::CtxW, AppState};

use super::{Toast, ToastData};

pub async fn index() -> Redirect {
    let mut redirect_url = "/guide";
    if config().IS_BYPASS_GUIDE {
        redirect_url = "/auth/login";
    }

    Redirect::to(redirect_url)
}

pub async fn redirect_to_login() -> Redirect {
    Redirect::permanent("/auth/login")
}

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
