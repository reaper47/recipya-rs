use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Redirect, Response},
};

use crate::{middleware::mw_auth::CtxW, AppState};
use futures::{
    sink::SinkExt,
    stream::{FuturesUnordered, SplitSink, SplitStream, StreamExt},
};
use lib_core::{config, model::user};

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
