use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::{
    config,
    ctx::Ctx,
    web::{middleware::ctx::CtxExtError, Error},
};

pub async fn require(ctx: Result<Ctx, Error>, req: Request, next: Next) -> Response {
    if config().IS_AUTOLOGIN {
        return Redirect::to("/recipes").into_response();
    }

    if ctx.is_err() {
        return Redirect::to("/auth/login").into_response();
    }

    next.run(req).await.into_response()
}

pub async fn redirect_if_no_signups(req: Request, next: Next) -> Response {
    if config().IS_NO_SIGNUPS {
        return Redirect::to("/auth/login").into_response();
    }
    next.run(req).await
}

pub async fn redirect_if_logged_in(
    ctx: Result<Ctx, Error>,
    mut req: Request,
    next: Next,
) -> Response {
    if config().IS_AUTOLOGIN {
        let ctx = Ctx::new(1).map_err(|_| CtxExtError::CtxNotInRequestExt);
        req.extensions_mut().insert(ctx);
        return Redirect::to("/").into_response();
    }

    if ctx.is_ok() {
        return Redirect::to("/").into_response();
    }

    next.run(req).await.into_response()
}
