use crate::{
    error::{Error, Result},
    utils::token::{set_token_cookie, AUTH_TOKEN},
    AppState,
};

use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{request::Parts, Request},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use lib_auth::token::{validate_web_token, Token};
use lib_core::{
    config,
    ctx::Ctx,
    model::user::{UserBmc, UserForAuth},
};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_ctx_require(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    if ctx.is_err() {
        let intended_url = req
            .uri()
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");
        let redirect_to = format!("/auth/login?redirect_to={}", intended_url);
        return Ok(Redirect::to(&redirect_to).into_response());
    }

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
    state: State<AppState>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let ctx_ext_result = _ctx_resolve(state, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_ext_result in the request extension (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(state: State<AppState>, cookies: &Cookies) -> CtxExtResult {
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    let ctx = Ctx::root_ctx();
    let user: UserForAuth = UserBmc::first_by_email_auth(&ctx, &state.mm, &token.ident)
        .await
        .map_err(|_| CtxExtError::UserNotFound)?;

    if let Err(err) =
        validate_web_token(&token, user.token_salt).map_err(|_| CtxExtError::FailValidate)
    {
        UserBmc::update_remember_me(&ctx, &state.mm, user.id, false)
            .await
            .map_err(|_| CtxExtError::ModelAccessError("Could not set remember me".to_string()))?;
        return Err(err);
    }

    set_token_cookie(cookies, &user.email, user.token_salt, user.is_remember_me)
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    Ctx::new(user.id)
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

pub async fn mw_redirect_if_authenticated(
    ctx: Result<CtxW>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    if ctx.is_ok() || config().IS_AUTOLOGIN {
        return Ok(Redirect::to("/").into_response());
    }

    Ok(next.run(req).await)
}

#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Debug, Serialize)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,

    CannotSetTokenCookie,
    FailValidate,
    ModelAccessError(String),
    UserNotFound,

    CtxNotInRequestExt,
    CtxCreateFail(String),
}
