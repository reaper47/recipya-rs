use std::ops::Deref;

use axum::Json;
use axum::body::Body;
use axum::extract::{FromRef, FromRequestParts, Request, State};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use config::{AutologinState, ProductionState};
use email::{Email, Template};
use models::tokens::{RefreshToken, RefreshTokenForCreate};
use reqwest::{StatusCode, header};
use serde_json::json;

use app::state::AppState;
use auth::token::generate_access_token;
use auth::token::http::{AUTH_TOKEN, REFRESH_TOKEN, clear_auth_cookies, set_auth_cookies};
use auth::token::jwt::validate_token;
use models::user::User;
use tower_cookies::Cookies;
use tracing::error;
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Clone)]
struct UserId(Uuid);

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
    fn unauthorized(error: &Error) -> Self {
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
            Self::Redirect(redirect) => redirect.into_response(),
            Self::Unauthorized(message) => {
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

        if app_state.config.read().await.states.autologin == AutologinState::On {
            let admin = User::get_first_admin(&app_state.mm)
                .await
                .map_err(|_| AuthRejection::unauthorized(&Error::NoUser))?
                .expect("At least one admin user should be in the database");

            return Ok(Self(admin));
        }

        match parts.extensions.get::<UserId>().cloned() {
            Some(user_id) => {
                let user = User::get_user_by_id(&app_state.mm, *user_id)
                    .await
                    .map_err(|err| rejection_for_error(err, is_api))?
                    .ok_or_else(|| rejection_for(&Error::NoUser, is_api))?;

                Ok(Self(user))
            }
            None => Err(AuthRejection::unauthorized(&Error::NoUser)),
        }
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

fn rejection_for(error: &Error, is_api: bool) -> AuthRejection {
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

        Ok(Self(user))
    }
}

async fn try_authenticate_optional<S>(parts: &mut Parts, state: &S) -> Option<User>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    let app_state = AppState::from_ref(state);

    if app_state.config.read().await.states.autologin == AutologinState::On {
        return User::get_first_admin(&app_state.mm)
            .await
            .map_err(|_| AuthRejection::unauthorized(&Error::NoUser))
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

pub async fn mw_refresh_token(
    State(state): State<AppState>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    if state.config.read().await.states.autologin == AutologinState::On {
        return Ok(next.run(req).await);
    }

    if let Some(access_cookie) = cookies.get(AUTH_TOKEN)
        && let Ok(claims) = validate_token(access_cookie.value())
    {
        req.extensions_mut().insert(UserId(
            claims.sub.parse::<Uuid>().unwrap_or_else(|_| Uuid::nil()),
        ));
        return Ok(next.run(req).await);
    }

    if let Some(refresh_cookie) = cookies.get(REFRESH_TOKEN) {
        let token = refresh_cookie.value();

        let refresh_token_entry = match RefreshToken::find_by_token(&state.mm, token).await {
            Ok(Some(refresh_token)) => refresh_token,
            Ok(_) => {
                clear_auth_cookies(&cookies);
                return Ok(Redirect::to("/auth/login").into_response());
            }
            Err(err) => {
                error!(?err, "Database error finding refresh token");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        if refresh_token_entry.is_expired() {
            if let Err(err) = RefreshToken::delete(&state.mm, token).await {
                error!("Failed to delete expired refresh token: {err}");
            }

            clear_auth_cookies(&cookies);
            return Ok(Redirect::to("/auth/login").into_response());
        }

        let user_id = refresh_token_entry.user_id;

        if refresh_token_entry.is_used {
            error!("TOKEN REUSE DETECTED!");
            error!(?token, "Token");
            error!(?user_id, "User ID");
            error!(used_at = ?refresh_token_entry.used_at, "Originally used at");

            if let Err(err) = RefreshToken::delete_all_for_user(&state.mm, user_id).await {
                error!(?err, "Failed to delete all refresh tokens for user");
            }

            if let Some(service) = state.email_service
                && let Ok(Some(user)) = User::get_user_by_id(&state.mm, user_id).await
            {
                tokio::spawn(async move {
                    service.send(Email {
                        to: user.email,
                        subject: "Security Alert: Suspicious Activity Detected".into(),
                        body: String::new(),
                        template: Some(Template::SecurityAlert),
                        data: None,
                    })
                });
            }

            clear_auth_cookies(&cookies);
            return Ok(Redirect::to("/auth/login").into_response());
        }

        if let Err(err) = RefreshToken::mark_as_used(&state.mm, token).await {
            error!(?err, "Failed to mark refresh token as used");
            clear_auth_cookies(&cookies);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let new_refresh_token_entry = match RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate::new(user_id, refresh_token_entry.is_remember_me),
        )
        .await
        {
            Ok(entry) => entry,
            Err(err) => {
                error!(?err, "Failed to create new refresh token");
                clear_auth_cookies(&cookies);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let access_token = match generate_access_token(&refresh_token_entry.user_id) {
            Ok(token) => token,
            Err(err) => {
                error!(?err, "Failed to generate access token");
                clear_auth_cookies(&cookies);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        set_auth_cookies(
            &cookies,
            access_token,
            new_refresh_token_entry.token,
            refresh_token_entry.is_remember_me,
            state.config.read().await.states.production == ProductionState::On,
        );

        req.extensions_mut()
            .insert(UserId(refresh_token_entry.user_id));
        return Ok(next.run(req).await);
    }

    Ok(Redirect::to("/auth/login").into_response())
}
