use axum::extract::State;
use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::{IntoResponse, Response},
};
use lazy_regex::regex_captures;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    ctx::Ctx,
    model::ModelManager,
    web::{Error, AUTH_TOKEN},
};

type CtxExtResult = Result<Ctx, CtxExtError>;

#[derive(Clone, Debug, Serialize)]
pub enum CtxExtError {
    TokenNotInCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}

pub async fn resolver(
    _mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = Ctx::new(100).map_err(|e| CtxExtError::CtxCreateFail(e.to_string()));

    if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    next.run(req).await.into_response()
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

fn parse_token(token: String) -> Result<(u64, String, String), Error> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailNoAuthTokenCookie)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
