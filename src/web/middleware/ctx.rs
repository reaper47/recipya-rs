use axum::{
    {async_trait, RequestPartsExt},
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::{IntoResponse, Response},
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::{ctx::Ctx, Error, web::AUTH_TOKEN};

pub async fn resolver(/*_db */ cookies: Cookies, mut req: Request, next: Next) -> Response {
    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations (is expensive)
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(Error::from(e)),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)){
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    next.run(req).await.into_response()
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Result<Ctx, Error>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
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
