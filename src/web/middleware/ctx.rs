use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    crypt::token::{Token, validate_web_token},
    ctx::Ctx,
    model::{ModelManager, user::UserBmc},
    web::{AUTH_TOKEN, Error, Result, set_token_cookie},
};

type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

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

pub async fn require(
    ctx: crate::web::Result<Ctx>,
    req: Request,
    next: Next,
) -> crate::web::Result<Response> {
    ctx?;
    Ok(next.run(req).await)
}

pub async fn resolve(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    let ctx_ext_result = _resolve(mm, &cookies).await;
    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    req.extensions_mut().insert(ctx_ext_result);
    Ok(next.run(req).await)
}

async fn _resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    let user = UserBmc::first_by_email_auth(&Ctx::root_ctx(), &mm, &token.id)
        .await
        .map_err(|_| CtxExtError::UserNotFound)?;

    validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailValidate)?;

    set_token_cookie(cookies, &user.email, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailValidate)?;

    Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
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
