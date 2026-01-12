use axum::Json;
use axum::body::Body;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::{HeaderMap, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use reqwest::{StatusCode, header};
use serde::Serialize;
use serde_json::json;

use app::state::AppState;
use auth::token::{AUTH_TOKEN, jwt::validate_token};
use models::user::User;
use uuid::Uuid;

use crate::handlers::context::Ctx;
use crate::{Error, Result};

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

/// Middleware to redirect authenticated users to the appropriate page.
pub async fn mw_only_admin(
    RequireAuth(user): RequireAuth,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    if user.is_admin {
        Ok(next.run(req).await)
    } else {
        Err(Error::UserNotAdmin)
    }
}

pub enum AuthRejection {
    Redirect(Redirect),
    Unauthorized(String),
}

impl AuthRejection {
    fn unauthorized(error: Error) -> Self {
        Self::Unauthorized(error.to_string())
    }

    fn redirect_to_login() -> Self {
        Self::Redirect(Redirect::to("/auth/login"))
    }
}

// For protected routes - requires valid JWT
pub struct RequireAuth(pub User);

// For optional auth - extracts user if token present
pub struct OptionalAuth(pub Option<User>);

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        match self {
            AuthRejection::Redirect(redirect) => redirect.into_response(),
            AuthRejection::Unauthorized(message) => {
                (StatusCode::UNAUTHORIZED, Json(json!({"error": message}))).into_response()
            }
        }
    }
}

impl<S> FromRequestParts<S> for RequireAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let is_api = is_api_request(parts);
        let app_state = AppState::from_ref(state);

        if app_state.config.read().await.is_autologin {
            let admin = User::get_first_admin(&app_state.mm)
                .await
                .map_err(|_| AuthRejection::unauthorized(Error::NoUser))?
                .expect("At least one admin user should be in the database");

            return Ok(RequireAuth(admin));
        }

        let token = if is_api {
            extract_token_from_headers(&parts.headers)
                .ok_or_else(|| AuthRejection::unauthorized(Error::NoToken))?
        } else {
            &extract_token_from_cookie(parts, state)
                .await?
                .ok_or_else(|| AuthRejection::redirect_to_login())?
        };

        authenticate_user(token, &app_state, is_api).await
    }
}

fn is_api_request(parts: &Parts) -> bool {
    parts.uri.path().starts_with("/api/")
        || parts
            .headers
            .get(header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.contains("application/json"))
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

async fn extract_token_from_cookie<S>(
    parts: &mut Parts,
    state: &S,
) -> std::result::Result<Option<String>, AuthRejection>
where
    S: Send + Sync,
{
    let cookies = CookieJar::from_request_parts(parts, state)
        .await
        .map_err(|_| AuthRejection::redirect_to_login())?;

    Ok(cookies.get(AUTH_TOKEN).map(|c| c.value().to_string()))
}

async fn authenticate_user(
    token: &str,
    app_state: &AppState,
    is_api: bool,
) -> std::result::Result<RequireAuth, AuthRejection> {
    let claims = validate_token(token).map_err(|_| rejection_for(Error::InvalidClaims, is_api))?;

    let user_id =
        Uuid::parse_str(&claims.sub).map_err(|_| rejection_for(Error::InvalidClaims, is_api))?;

    let user = User::get_user_by_id(&app_state.mm, user_id)
        .await
        .map_err(|e| rejection_for_error(e, is_api))?
        .ok_or_else(|| rejection_for(Error::NoUser, is_api))?;

    Ok(RequireAuth(user))
}

fn rejection_for(error: Error, is_api: bool) -> AuthRejection {
    if is_api {
        AuthRejection::unauthorized(error)
    } else {
        AuthRejection::redirect_to_login()
    }
}

fn rejection_for_error(error: impl std::fmt::Display, is_api: bool) -> AuthRejection {
    if is_api {
        AuthRejection::Unauthorized(error.to_string())
    } else {
        AuthRejection::redirect_to_login()
    }
}

impl<S> FromRequestParts<S> for OptionalAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let user = try_authenticate_optional(parts, state).await;

        Ok(OptionalAuth(user))
    }
}

async fn try_authenticate_optional<S>(parts: &mut Parts, state: &S) -> Option<User>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    let app_state = AppState::from_ref(state);

    if app_state.config.read().await.is_autologin {
        return User::get_first_admin(&app_state.mm)
            .await
            .map_err(|_| AuthRejection::unauthorized(Error::NoUser))
            .ok()?;
    }

    let is_api = is_api_request(parts);

    let token = if is_api {
        extract_token_from_headers(&parts.headers)?
    } else {
        &extract_token_from_cookie(parts, state).await.ok()??
    };

    let claims = validate_token(token).ok()?;
    let user_id = Uuid::parse_str(&claims.sub).ok()?;

    User::get_user_by_id(&app_state.mm, user_id).await.ok()?
}
