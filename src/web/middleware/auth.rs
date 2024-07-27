use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::{app, Error};
use crate::ctx::Ctx;

pub async fn require(
    ctx: Result<Ctx, Error>,
    State(app): State<Arc<app::App>>,
    req: Request,
    next: Next,
) -> Response {
    if app.config.server.is_autologin {
        return Redirect::to("/recipes").into_response();
    }

    if ctx.is_err() {
        return Redirect::to("/auth/login").into_response();
    }

    next.run(req).await.into_response()
}

pub async fn redirect_if_no_signups(
    State(app): State<Arc<app::App>>,
    req: Request,
    next: Next,
) -> Response {
    if app.config.server.is_no_signups {
        return Redirect::to("/auth/login").into_response();
    }
    next.run(req).await
}

pub async fn redirect_if_logged_in(
    ctx: Result<Ctx, Error>,
    State(app): State<Arc<app::App>>,
    mut req: Request,
    next: Next,
) -> Response {
    if app.config.server.is_autologin {
        let ctx = Ctx::new(1);
        req.extensions_mut().insert(ctx);
        return Redirect::to("/").into_response();
    }

    if ctx.is_ok() {
        return Redirect::to("/").into_response();
    }

    next.run(req).await.into_response()
}
