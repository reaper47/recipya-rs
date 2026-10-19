use std::collections::HashMap;

use axum::Form;
use axum::extract::{Query, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect};
use fluent_static::support::axum::RequestLanguage;
use tower_cookies::Cookies;
use tracing::{debug, error, warn};
use uuid::Uuid;
use validator::Validate;

use app::message::ToastCtx;
use app::state::AppState;
use auth::pwd::{ContentToHash, scheme::SchemeStatus, validate_pwd};
use auth::token::{
    generate_access_token,
    http::{clear_auth_cookies, set_auth_cookies},
};
use auth::types::Email as EmailType;
use config::{AutologinState, DemoState, ProductionState, SignupsState};
use email::{Data, Email, Template};
use l10n::Messages;
use models::tokens::{
    EmailVerificationToken, EmailVerificationTokenForCreate, PasswordResetToken,
    PasswordResetTokenForCreate, RefreshToken, RefreshTokenForCreate,
};
use models::user::User;
use repository::ModelManager;
use support::states::{AuthenticatedState, UnauthenticatedState};

use crate::Error;
use crate::extractors::{ServerError, ValidatedForm};
use crate::middleware::mw_auth::{OptionalAuth, RequireAuth};
use crate::schemas::auth::{
    ChangePasswordForm, ForgotPasswordForm, ForgotPasswordResetForm, LoginForm, RegisterForm,
};

/// Handles a user's update password request.
pub async fn change_password_post_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Form(form): Form<ChangePasswordForm>,
) -> impl IntoResponse {
    if state.config.read().await.states.autologin == AutologinState::On {
        return Error::ConfirmForbidden.into_response();
    }

    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    if form.password == form.new_password {
        toast
            .error(&messages.toast_auth_new_password_same_as_old())
            .await;
        return Error::Form.into_response();
    }

    if form.validate().is_err() {
        toast.error(&messages.toast_auth_password_no_match()).await;
        return Error::Form.into_response();
    }

    match User::update_password_by_user_id(&state.mm, user.id, form.new_password).await {
        Ok(()) => {
            toast.success(&messages.toast_auth_password_updated()).await;
            (StatusCode::NO_CONTENT, "").into_response()
        }
        Err(err) => {
            toast
                .error(&messages.toast_auth_password_update_failed())
                .await;
            Error::Model(err).into_response()
        }
    }
}

/// Handles account confirmation once the user clicks their confirm button.
pub async fn verify_email_handler(
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Query(mut query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
    let token = get_token_from_query(&mut query)?;

    let verification_token = EmailVerificationToken::find_by_token(&state.mm, &token)
        .await
        .ok()
        .flatten()
        .ok_or_else(|| {
            Error::Model(models::Error::EntityNotFound {
                entity: "email token",
                id: token.clone(),
            })
        })?;

    if verification_token.is_expired() {
        delete_token(&state.mm, &token).await?;
        return Err(Error::Gone);
    }

    EmailVerificationToken::verify_user_email(&state.mm, verification_token.user_id).await?;
    delete_token(&state.mm, &token).await?;

    Ok(templates::general::simple(
        &messages.toast_title_success(),
        &messages.auth_account_verified(),
        &messages,
    ))
}

fn get_token_from_query(query: &mut HashMap<String, String>) -> crate::Result<String> {
    query.remove("token").ok_or(Error::NoToken)
}

async fn delete_token(mm: &ModelManager, token: &str) -> Result<(), Error> {
    EmailVerificationToken::delete(mm, token)
        .await
        .map_err(|err| {
            error!(?token, ?err, "Failed to delete email verification token");
            Error::Database
        })
}

/// Renders the forgot password request page.
pub async fn forgot_password_handler(
    OptionalAuth(user): OptionalAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
) -> impl IntoResponse {
    match user {
        Some(_) => Redirect::to("/recipes").into_response(),
        None => templates::auth::forgot_password(&messages).into_response(),
    }
}

/// Handles the forgot password request form.
pub async fn forgot_password_post_handler(
    OptionalAuth(user): OptionalAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    form: Result<ValidatedForm<ForgotPasswordForm>, ServerError>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        let Ok(ValidatedForm(form)) = form else {
            return Error::Form.into_response();
        };

        let user_email = form.email;

        if let Some(email) = state.email_service
            && let Ok(Some(user)) = User::get_user_by_email(&state.mm, &user_email).await
            && let Ok(password_token) =
                PasswordResetToken::new(&state.mm, PasswordResetTokenForCreate::new(user.id, 1))
                    .await
        {
            let payload = Email {
                to: user_email.as_ref().to_string(),
                subject: messages.reset_password_form_title().to_string(),
                body: String::new(),
                template: Some(Template::ForgotPassword),
                data: Some(Data {
                    token: password_token.token,
                    username: user_email.as_ref().to_string(),
                    url: state.config.read().await.base_url.clone(),
                }),
            };

            tokio::spawn(async move {
                if let Err(err) = email.send(payload) {
                    error!("Could not send email 'Reset password': {:?}", err);
                }
            });
        }

        templates::general::simple(
            &messages.auth_password_reset_requested_title(),
            &messages.auth_password_reset_requested_text(),
            &messages,
        )
        .into_response()
    }
}

/// Renders the forgot password reset page.
pub async fn forgot_password_reset_handler(
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Query(mut query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match get_token_from_query(&mut query) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    match PasswordResetToken::find_by_token(&state.mm, &token).await {
        Ok(Some(v)) => {
            if v.is_expired() {
                if let Err(err) = PasswordResetToken::delete(&state.mm, &v.token).await {
                    error!(?token, ?err, "Failed to delete expired token");
                    return Error::Database.into_response();
                }

                let mut res = templates::general::simple(
                    &messages.auth_token_expired_title(),
                    &messages.auth_token_expired_text(),
                    &messages,
                )
                .into_response();
                *res.status_mut() = StatusCode::BAD_REQUEST;
                return res;
            }

            templates::auth::forgot_password_reset(&v.token, &messages).into_response()
        }
        Ok(_) => {
            warn!("The token '{token}' was not found in the database.");
            Error::Database.into_response()
        }
        Err(err) => {
            error!(?token, ?err, "Failed to find the token in the database");
            Error::Database.into_response()
        }
    }
}

/// Handles the submission of the password reset form.
pub async fn forgot_password_reset_post_handler(
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    form: Result<ValidatedForm<ForgotPasswordResetForm>, ServerError>,
) -> impl IntoResponse {
    let toast = ToastCtx::<UnauthenticatedState>::new(&state, &messages);

    let Ok(ValidatedForm(form)) = form else {
        return toast.error(
            Error::Form.into_response(),
            &messages.toast_auth_password_invalid(),
        );
    };

    let entry = match PasswordResetToken::find_by_token(&state.mm, &form.token).await {
        Ok(Some(token)) => {
            if token.is_expired() {
                let mut res = templates::general::simple(
                    &messages.auth_token_expired_title(),
                    &messages.auth_token_expired_text(),
                    &messages,
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
                token = form.token,
                ?err,
                "Failed to find the token in the database",
            );
            return Error::Database.into_response();
        }
    };

    let user_id = entry.user_id;

    if let Err(err) = User::update_password_by_user_id(&state.mm, user_id, form.password).await {
        error!(?user_id, ?err, "Failed to update password for user");
        return toast.error(
            Error::Form.into_response(),
            &messages.toast_auth_password_update_failed(),
        );
    }

    if let Err(err) = PasswordResetToken::delete_all_for_user(&state.mm, user_id).await {
        error!(?user_id, ?err, "Failed to delete expired token");
        return Error::Database.into_response();
    }

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    res = toast.success(res, &messages.toast_auth_password_updated());
    if let Ok(value) = HeaderValue::from_str("/auth/login") {
        res.headers_mut().insert(axum_htmx::HX_REDIRECT, value);
    }
    res
}

/// Renders the login page.
pub async fn login_handler(
    OptionalAuth(user): OptionalAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        let config = state.config.read().await;

        templates::auth::login(&config.states.demo, &config.states.signups, &messages)
            .into_response()
    }
}

#[axum::debug_handler]
/// Handles user login requests.
pub async fn login_post_handler(
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    cookies: Cookies,
    form: Result<ValidatedForm<LoginForm>, ServerError>,
) -> impl IntoResponse {
    let toast = ToastCtx::<UnauthenticatedState>::new(&state, &messages);

    let Ok(ValidatedForm(form)) = form else {
        return toast.error(
            Error::Form.into_response(),
            &messages.toast_auth_credentials_invalid(),
        );
    };

    let user = match User::get_user_by_email(&state.mm, &form.email).await {
        Ok(user) => match user {
            None => {
                return toast.error(
                    Error::LoginFailUsernameNotFound.into_response(),
                    &messages.toast_auth_credentials_invalid(),
                );
            }
            Some(user) => user,
        },
        Err(_) => {
            return toast.error(
                Error::LoginFailUsernameNotFound.into_response(),
                &messages.toast_auth_credentials_invalid(),
            );
        }
    };

    let scheme_status = match validate_pwd(
        ContentToHash {
            salt: user.password_salt,
            content: form.password.clone().into_inner(),
        },
        &user.password_hash,
    )
    .await
    {
        Ok(status) => status,
        Err(err) => {
            error!(
                user = ?user.id,
                ?form,
                ?err,
                "Password validation failed"
            );
            return toast.error(
                Error::PwdNotMatching { user_id: user.id }.into_response(),
                &messages.toast_auth_credentials_invalid(),
            );
        }
    };

    // Update password scheme if needed
    if matches!(scheme_status, SchemeStatus::Outdated) {
        debug!("pwd encrypt scheme outdated, upgrading");
        if user
            .update_password(&state.mm, form.password.clone())
            .await
            .is_err()
        {
            return toast.error(
                Error::UpdatePassword.into_response(),
                &messages.toast_auth_password_schema_update_failed(),
            );
        }
    }

    let access_token = match generate_access_token(&user.id) {
        Ok(token) => token,
        Err(err) => {
            error!(user = ?user.id, ?err, "Failed to generate access token");
            return toast.error(
                Error::GenerateToken.into_response(),
                &messages.toast_auth_access_token_failed(),
            );
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
            error!(user = ?user.id, ?err, "Failed to generate refresh token");
            return toast.error(
                Error::GenerateToken.into_response(),
                &messages.toast_auth_refresh_token_failed(),
            );
        }
    };

    set_auth_cookies(
        &cookies,
        access_token,
        refresh_token_entry.token,
        form.is_remember_me(),
        state.config.read().await.states.production == ProductionState::On,
    );

    (StatusCode::SEE_OTHER, [("HX-Redirect", "/recipes")]).into_response()
}

/// Handles a user logging out.
pub async fn logout_post_handler(
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    if state.config.read().await.states.autologin == AutologinState::On {
        return Error::LogoutForbidden.into_response();
    }

    if let Some(user) = user
        && let Err(err) = User::update_remember_me(&state.mm, user.id, false).await
    {
        error!(user = ?user.id, ?err, "Could not update remember_me");
    }

    clear_auth_cookies(&cookies);
    Redirect::to("/").into_response()
}

/// Renders the user registration page.
pub async fn register_handler(
    RequestLanguage(messages): RequestLanguage<Messages>,
    OptionalAuth(user): OptionalAuth,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        if state.config.read().await.states.signups == SignupsState::Off {
            return Redirect::to("/auth/login").into_response();
        }

        templates::auth::register(&messages).into_response()
    }
}

/// Handles user registration.
pub async fn register_post_handler(
    OptionalAuth(user): OptionalAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    form: Result<ValidatedForm<RegisterForm>, ServerError>,
) -> impl IntoResponse {
    if user.is_some() {
        Redirect::to("/recipes").into_response()
    } else {
        let toast = ToastCtx::<UnauthenticatedState>::new(&state, &messages);
        let config = state.config.read().await;

        if config.states.signups == SignupsState::Off {
            return Redirect::to("/auth/login").into_response();
        }

        let ValidatedForm(form) = match form {
            Ok(form) => form,
            Err(ServerError::ValidationError(err)) => {
                let message = err
                    .field_errors()
                    .values()
                    .flat_map(|errors| errors.iter())
                    .filter_map(|err| err.message.as_ref())
                    .map(ToString::to_string)
                    .next()
                    .unwrap_or_else(|| "Validation error".to_string());

                return toast.error(StatusCode::UNPROCESSABLE_ENTITY.into_response(), &message);
            }
            Err(ServerError::AxumFormRejection(_)) => {
                return toast.error(
                    StatusCode::BAD_REQUEST.into_response(),
                    &messages.toast_auth_credentials_invalid(),
                );
            }
        };

        if let Err(err) = form.validate() {
            let message = err
                .field_errors()
                .values()
                .flat_map(|errors| errors.iter())
                .filter_map(|err| err.message.as_ref())
                .map(ToString::to_string)
                .next()
                .unwrap_or_else(|| "Validation error".to_string());

            return toast.error(StatusCode::UNPROCESSABLE_ENTITY.into_response(), &message);
        }

        match User::get_user_by_email(&state.mm, &form.email).await {
            Ok(Some(_)) => Redirect::to("/recipes").into_response(),
            Ok(_) => {
                let username = form.email.clone();

                let user = match User::new(&state.mm, form.to_user()).await {
                    Ok(id) => id,
                    Err(err) => {
                        error!(?err, "Error creating user");
                        return toast.error(
                            Error::Model(err).into_response(),
                            &messages.toast_auth_registration_error(),
                        );
                    }
                };

                if let Some(service) = state.email_service.clone() {
                    let token_entry = match EmailVerificationToken::new(
                        &state.mm,
                        EmailVerificationTokenForCreate::new(user.id, 24),
                    )
                    .await
                    {
                        Ok(entry) => entry,
                        Err(err) => {
                            error!(?err, "Error creating email verification token");
                            return toast.error(
                                Error::Model(err).into_response(),
                                &messages.toast_auth_registration_error(),
                            );
                        }
                    };

                    let base_url = config.base_url.clone();
                    drop(config);

                    tokio::spawn(async move {
                        service.send(Email {
                            to: user.email,
                            subject: messages.toast_auth_verify_email().to_string(),
                            body: String::new(),
                            template: Some(Template::Intro),
                            data: Some(Data {
                                token: token_entry.token,
                                username: username.into_inner(),
                                url: base_url,
                            }),
                        })
                    });
                }

                (StatusCode::SEE_OTHER, [("HX-Redirect", "/auth/login")]).into_response()
            }
            Err(err) => {
                error!(?err, "Failed to fetch user from database");
                toast.error(
                    Error::FailFetch.into_response(),
                    &messages.toast_users_fetch_users_failed(),
                )
            }
        }
    }
}

/// Handles user deletion.
pub async fn user_delete_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
    let config = state.config.read().await;

    if config.states.autologin == AutologinState::On {
        toast.error(&messages.toast_auth_account_no_delete()).await;
        return Error::DeleteForbidden.into_response();
    }

    if config.states.demo == DemoState::On && is_demo_user(&state, user.id).await {
        toast
            .error("Trump is Putin's lap dog. Remove him from office!")
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
            error!(user = ?user.id, ?err, "Could not delete user with id");
            Error::DeleteUser.into_response()
        }
    }
}

async fn is_demo_user(state: &AppState, user_id: Uuid) -> bool {
    match User::get_user_by_email(&state.mm, &EmailType::demo()).await {
        Ok(Some(user)) => user.id == user_id,
        _ => false,
    }
}
