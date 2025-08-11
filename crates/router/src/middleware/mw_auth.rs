use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

use auth::token::{AUTH_TOKEN, Token, set_token_cookie, validate_web_token};
use models::user::{User, UserForAuth};

use crate::handlers::context::Ctx;
use crate::{Error, Result};
use app::state::AppState;

/// A wrapper around the `Ctx` type for use in request extraction.
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    /// Extracts `CtxW` from the request parts by looking for the `CtxExtResult`
    /// stored in the request extensions.
    ///
    /// If the context is not available in the request extensions, it returns a `CtxExtError::CtxNotInRequestExt` rejection.
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

/// Enumeration of errors related to the authentication middleware.
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

/// Middleware that ensures a valid authentication context.
pub async fn mw_ctx_require(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    if ctx.is_err() {
        return Ok(Redirect::to("/auth/login").into_response());
    }

    Ok(next.run(req).await)
}

/// Middleware for resolving the context (user authentication) from cookies.
pub async fn mw_ctx_resolver(
    state: State<AppState>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let ctx_ext_result = match ctx_resolve(state, &cookies).await {
        Err(_) if req.uri().path().starts_with("/shared") => Ok(CtxW(Ctx::root_ctx())),
        Err(CtxExtError::TokenNotInCookie) => {
            cookies.remove(Cookie::from(AUTH_TOKEN));
            Err(CtxExtError::TokenNotInCookie)
        }
        ok => ok,
    };

    // Store the ctx_ext_result in the request extension (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

/// Resolves the context (user authentication) from cookies and the application state.
async fn ctx_resolve(state: State<AppState>, cookies: &Cookies) -> CtxExtResult {
    if state.config.read().await.is_autologin {
        return Ctx::new(1)
            .map(CtxW)
            .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));
    }

    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    let user: UserForAuth = User::get_user_auth_by_email(&state.mm, &token.ident)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    if let Err(err) =
        validate_web_token(&token, user.token_salt).map_err(|_| CtxExtError::FailValidate)
    {
        User::update_remember_me(&state.mm, user.id, false)
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

/// Middleware to redirect authenticated users to the appropriate page.
pub async fn mw_redirect_if_authenticated(
    ctx: Result<CtxW>,
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let is_ok = ctx.is_ok();

    if is_ok && (state.config.read().await.is_autologin || is_path_to_redirect(req.uri().path())) {
        return Ok(Redirect::to("/recipes").into_response());
    }

    if is_ok && state.config.read().await.is_no_signups {
        return Ok(Redirect::to("/auth/login").into_response());
    }

    Ok(next.run(req).await)
}

fn is_path_to_redirect(path: &str) -> bool {
    path.eq("/") || path.eq("/login") || path.eq("/forgot-password") || path.eq("/register")
}

/// Middleware to redirect authenticated users to the appropriate page.
pub async fn mw_only_admin(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    match ctx {
        Ok(ctx) => {
            let user_id = ctx.0.user_id();

            if user_id == 1 {
                Ok(next.run(req).await)
            } else {
                Err(Error::UserNotAdmin)
            }
        }
        Err(err) => Err(Error::CtxExt(CtxExtError::CtxCreateFail(err.to_string()))),
    }
}
