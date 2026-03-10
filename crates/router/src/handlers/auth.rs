use std::collections::HashMap;

use axum::Form;
use axum::extract::{Query, State, ws::Message};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect};
use tower_cookies::Cookies;
use tracing::{debug, error, warn};
use uuid::Uuid;
use validator::Validate;

use app::state::AppState;
use auth::pwd::scheme::SchemeStatus;
use auth::pwd::{ContentToHash, validate_pwd};
use auth::token::generate_access_token;
use auth::token::http::{clear_auth_cookies, set_auth_cookies};
use email::{Data, Email, Template};
use models::tokens::{
    EmailVerificationToken, EmailVerificationTokenForCreate, PasswordResetToken,
    PasswordResetTokenForCreate, RefreshToken, RefreshTokenForCreate,
};
use models::user::User;

use crate::handlers::message::{IMessage, MessageHtmx, MessageWs, add_hx_message, broadcast_error};
use crate::middleware::mw_auth::{OptionalAuth, RequireAuth};
use crate::schemas::auth::{
    ChangePasswordForm, ForgotPasswordForm, ForgotPasswordResetForm, LoginForm, RegisterForm,
};
use crate::{Error, Result};

/// Handles a user's update password request.
pub async fn change_password_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<ChangePasswordForm>,
) -> impl IntoResponse {
    if state.config.read().await.is_autologin {
        return Error::ConfirmForbidden.into_response();
    }

    if form.password == form.new_password {
        broadcast_error(
            &state,
            user.id,
            "New password cannot be the same as the current.",
        )
        .await;
        return Error::Form.into_response();
    }

    if form.validate().is_err() {
        broadcast_error(&state, user.id, "Passwords do not match.").await;
        return Error::Form.into_response();
    }

    match User::update_password_by_user_id(&state.mm, user.id, &form.new_password).await {
        Ok(()) => {
            let toast = MessageWs::success("Your password has been updated.");

            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(Message::Text(json.into()), user.id).await;
            }

            (StatusCode::NO_CONTENT, "").into_response()
        }
        Err(err) => {
            broadcast_error(&state, user.id, "Failed to update password.").await;
            Error::Model(err).into_response()
        }
    }
}

/// Handles account confirmation once the user clicks their confirm button.
pub async fn verify_email_handler(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match get_token_from_query(&query) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    let Ok(Some(verification_token)) =
        EmailVerificationToken::find_by_token(&state.mm, &token).await
    else {
        return Error::Model(models::Error::EntityNotFound {
            entity: "email token",
            id: token,
        })
        .into_response();
    };

    if verification_token.is_expired() {
        if let Err(err) = EmailVerificationToken::delete(&state.mm, &token).await {
            error!("Failed to delete email verification token '{token}': {err}");
            return Error::Database.into_response();
        }
        return Error::Gone.into_response();
    }

    if let Err(err) =
        EmailVerificationToken::verify_user_email(&state.mm, verification_token.user_id).await
    {
        return Error::Model(err).into_response();
    }

    if let Err(err) = EmailVerificationToken::delete(&state.mm, &token).await {
        error!("Failed to delete email verification token '{token}': {err}");
        return Error::Database.into_response();
    }

    templates::general::simple("Success", "Your account has been verified.").into_response()
}

fn get_token_from_query(query: &HashMap<String, String>) -> Result<String> {
    query
        .get("token")
        .map_or_else(|| Err(Error::NoToken), |token| Ok(token.clone()))
}

/// Renders the forgot password request page.
pub async fn forgot_password_handler(OptionalAuth(user): OptionalAuth) -> impl IntoResponse {
    match user {
        Some(_) => Redirect::to("/recipes").into_response(),
        None => templates::auth::forgot_password().into_response(),
    }
}

/// Handles the forgot password request form.
pub async fn forgot_password_post_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordForm>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        if form.validate().is_err() {
            return Error::Form.into_response();
        }

        let user_email = form.email;

        if let Some(email) = state.email_service
            && let Ok(Some(user)) = User::get_user_by_email(&state.mm, &user_email).await
            && let Ok(password_token) =
                PasswordResetToken::new(&state.mm, PasswordResetTokenForCreate::new(user.id, 1))
                    .await
        {
            let payload = Email {
                to: user_email.clone(),
                subject: "Reset password".into(),
                body: String::new(),
                template: Some(Template::ForgotPassword),
                data: Some(Data {
                    token: password_token.token,
                    username: user_email,
                    url: state.config.read().await.base_url.clone(),
                }),
            };

            tokio::spawn(async move {
                if let Err(err) = email.send(&payload) {
                    error!("Could not send email 'Reset password': {:?}", err);
                }
            });
        }

        templates::general::simple(
             "Password Reset Requested",
             "An email with instructions on how to reset your password has been sent to you. Please check your inbox and follow the provided steps to regain access to your account.",
         ).into_response()
    }
}

/// Renders the forgot password reset page.
pub async fn forgot_password_reset_handler(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match get_token_from_query(&query) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    match PasswordResetToken::find_by_token(&state.mm, &token).await {
        Ok(Some(v)) => {
            if v.is_expired() {
                if let Err(err) = PasswordResetToken::delete(&state.mm, &v.token).await {
                    error!("Failed to delete expired token '{token}': {err}");
                    return Error::Database.into_response();
                }

                let mut res = templates::general::simple(
                    "Token Expired",
                    "The token associated with the URL expired.",
                )
                .into_response();
                *res.status_mut() = StatusCode::BAD_REQUEST;
                return res;
            }

            templates::auth::forgot_password_reset(&v.token).into_response()
        }
        Ok(_) => {
            warn!("The token '{token}' was not found in the database.");
            Error::Database.into_response()
        }
        Err(err) => {
            error!("Failed to find the token '{token}' in the database: {err}");
            Error::Database.into_response()
        }
    }
}

/// Handles the submission of the password reset form.
pub async fn forgot_password_reset_post_handler(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordResetForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = Error::Form.into_response();
        add_hx_message(&mut res, &MessageHtmx::success("Password is invalid"));
        return res;
    }

    let entry = match PasswordResetToken::find_by_token(&state.mm, &form.token).await {
        Ok(Some(token)) => {
            if token.is_expired() {
                let mut res = templates::general::simple(
                    "Token Expired",
                    "The token associated with the URL expired.",
                )
                .into_response();
                *res.status_mut() = StatusCode::BAD_REQUEST;
                return res;
            }
            token
        }
        Ok(_) => {
            warn!("The token '{}' was not found in the database.", form.token);
            return Error::Database.into_response();
        }
        Err(err) => {
            error!(
                "Failed to find the token '{}' in the database: {err}",
                form.token
            );
            return Error::Database.into_response();
        }
    };

    let user_id = entry.user_id;

    if let Err(err) = User::update_password_by_user_id(&state.mm, user_id, &form.password).await {
        error!("Failed to update password for user '{user_id}': {err}",);
        let mut res = Error::Form.into_response();
        add_hx_message(&mut res, &MessageHtmx::error("Failed to update password."));
        return res;
    }

    if let Err(err) = PasswordResetToken::delete_all_for_user(&state.mm, user_id).await {
        error!("Failed to delete expired token '{user_id}': {err}");
        return Error::Database.into_response();
    }

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    add_hx_message(
        &mut res,
        &MessageHtmx::success("Your password has been updated."),
    );

    if let Ok(value) = HeaderValue::from_str("/auth/login") {
        res.headers_mut().insert(axum_htmx::HX_REDIRECT, value);
    }

    res
}

/// Renders the login page.
pub async fn login_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        let config = state.config.read().await;

        templates::auth::login(config.is_demo, config.is_no_signups).into_response()
    }
}

/// Handles user login requests.
pub async fn login_post_handler(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = Error::Form.into_response();
        add_hx_message(&mut res, &MessageHtmx::error("Credentials are invalid."));
        return res;
    }

    let user = match User::get_user_by_email(&state.mm, String::from(&form.email)).await {
        Ok(user) => match user {
            None => {
                let mut res = Error::LoginFailUsernameNotFound.into_response();
                add_hx_message(&mut res, &MessageHtmx::error("Credentials are invalid."));
                return res;
            }
            Some(user) => user,
        },
        Err(error) => return Error::Model(error).into_response(),
    };

    let scheme_status = match validate_pwd(
        ContentToHash {
            salt: user.password_salt,
            content: String::from(&form.password),
        },
        &user.password_hash,
    )
    .await
    {
        Ok(status) => status,
        Err(_err) => {
            let mut res = Error::PwdNotMatching { user_id: user.id }.into_response();
            add_hx_message(&mut res, &MessageHtmx::error("Credentials are invalid."));
            return res;
        }
    };

    // Update password scheme if needed
    if matches!(scheme_status, SchemeStatus::Outdated) {
        debug!("pwd encrypt scheme outdated, upgrading");
        if user
            .update_password(&state.mm, &form.password)
            .await
            .is_err()
        {
            let mut res = Error::UpdatePassword.into_response();
            add_hx_message(
                &mut res,
                &MessageHtmx::error("Failed to update password schema."),
            );
            return res;
        }
    }

    let access_token = match generate_access_token(&user.id) {
        Ok(token) => token,
        Err(err) => {
            let mut res = Error::GenerateToken.into_response();
            add_hx_message(
                &mut res,
                &MessageHtmx::error("Failed to generate access token."),
            );
            error!(
                "Failed to generate access token for user {}: {err}",
                user.id
            );
            return res;
        }
    };

    let refresh_token_entry = match RefreshToken::new(
        &state.mm,
        RefreshTokenForCreate::new(user.id, user.is_remember_me),
    )
    .await
    {
        Ok(entry) => entry,
        Err(err) => {
            let mut res = Error::GenerateToken.into_response();
            add_hx_message(
                &mut res,
                &MessageHtmx::error("Failed to generate refresh token."),
            );
            error!(
                "Failed to generate refresh token for user {}: {err}",
                user.id
            );
            return res;
        }
    };

    set_auth_cookies(
        &cookies,
        access_token,
        refresh_token_entry.token,
        form.is_remember_me(),
        state.config.read().await.is_production,
    );

    (
        StatusCode::SEE_OTHER,
        [("HX-Redirect", "/recipes"), ("Location", "/recipes")],
    )
        .into_response()
}

/// Handles a user logging out.
pub async fn logout_post_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    if state.config.read().await.is_autologin {
        return Error::LogoutForbidden.into_response();
    }

    if let Some(user) = user
        && let Err(err) = User::update_remember_me(&state.mm, user.id, false).await
    {
        error!("Could not update remember_me for user {}: {err}", user.id);
    }

    clear_auth_cookies(&cookies);
    Redirect::to("/").into_response()
}

/// Renders the user registration page.
pub async fn register_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        if state.config.read().await.is_no_signups {
            return Redirect::to("/auth/login").into_response();
        }

        templates::auth::register().into_response()
    }
}

/// Handles user registration.
pub async fn register_post_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        let config = state.config.read().await;

        if config.is_no_signups {
            return Redirect::to("/auth/login").into_response();
        }

        if let Err(err) = form.validate() {
            let message = err
                .field_errors()
                .values()
                .flat_map(|errors| errors.iter())
                .filter_map(|err| err.message.as_ref())
                .map(ToString::to_string)
                .next()
                .unwrap_or_else(|| "Validation error".to_string());

            let mut res = StatusCode::UNPROCESSABLE_ENTITY.into_response();
            add_hx_message(&mut res, &MessageHtmx::error(&message));
            return res;
        }

        match User::get_user_by_email(&state.mm, form.email.clone()).await {
            Ok(Some(_)) => Redirect::to("/recipes").into_response(),
            Ok(_) => {
                let user = match User::new(&state.mm, form.to_user()).await {
                    Ok(id) => id,
                    Err(err) => {
                        error!("Error creating user: {}", err);
                        let mut res = Error::Model(err).into_response();
                        add_hx_message(
                            &mut res,
                            &MessageHtmx::error("An error occurred during registration."),
                        );
                        return res;
                    }
                };

                if let Some(service) = state.email_service {
                    let token_entry = match EmailVerificationToken::new(
                        &state.mm,
                        EmailVerificationTokenForCreate::new(user.id, 24),
                    )
                    .await
                    {
                        Ok(entry) => entry,
                        Err(err) => {
                            error!("Error creating email verification token: {err}");
                            let mut res = Error::Model(err).into_response();
                            add_hx_message(
                                &mut res,
                                &MessageHtmx::error("An error occurred during registration."),
                            );
                            return res;
                        }
                    };

                    let base_url = config.base_url.clone();
                    drop(config);

                    tokio::spawn(async move {
                        service.send(&Email {
                            to: user.email,
                            subject: "Verify your email address".into(),
                            body: String::new(),
                            template: Some(Template::Intro),
                            data: Some(Data {
                                token: token_entry.token,
                                username: form.email,
                                url: base_url,
                            }),
                        })
                    });
                }

                Redirect::to("/auth/login").into_response()
            }
            Err(err) => {
                error!("Failed to fetch user from database: {err}");

                let mut res = Error::FailFetch.into_response();
                add_hx_message(
                    &mut res,
                    &MessageHtmx::error("Failed to fetch user from database."),
                );
                res
            }
        }
    }
}

/// Handles user deletion.
pub async fn user_delete_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    let config = state.config.read().await;

    if config.is_autologin {
        broadcast_error(&state, user.id, "This account cannot be deleted.").await;
        return Error::DeleteForbidden.into_response();
    }

    if config.is_demo && is_demo_user(&state, user.id).await {
        broadcast_error(
            &state,
            user.id,
            "Trump is Putin's lap dog. Remove him from office!",
        )
        .await;
        return Error::DeleteForbidden.into_response();
    }

    match User::delete(&state.mm, user.id).await {
        Ok(()) => {
            drop(config);
            logout_post_handler(OptionalAuth(Some(user)), State(state), cookies)
                .await
                .into_response()
        }
        Err(err) => {
            error!("Could not delete user with id {}: {err}", user.id);
            Error::DeleteUser.into_response()
        }
    }
}

async fn is_demo_user(state: &AppState, user_id: Uuid) -> bool {
    match User::get_user_by_email(&state.mm, "demo@demo.com").await {
        Ok(Some(user)) => user.id == user_id,
        _ => false,
    }
}
