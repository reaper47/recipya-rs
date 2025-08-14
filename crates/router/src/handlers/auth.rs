use std::collections::HashMap;

use axum::Form;
use axum::extract::ws::Message;
use axum::extract::{Query, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect};
use tower_cookies::Cookies;
use tracing::{debug, error};
use validator::Validate;

use auth::pwd::scheme::SchemeStatus;
use auth::pwd::{ContentToHash, validate_pwd};
use auth::token::{
    Token, generate_web_token, remove_token_cookie, set_token_cookie, validate_web_token,
};
use email::{Data, Email, Template};
use models::Error::EntityNotFound;
use models::user::User;

use crate::auth_router::{
    ChangePasswordForm, ForgotPasswordForm, ForgotPasswordResetForm, LoginForm, RegisterForm,
};
use crate::handlers::message::{IMessage, MessageHtmx, MessageWs, add_hx_message, broadcast_error};
use crate::middleware::mw_auth::CtxW;
use crate::{Error, Result};
use app::state::AppState;

/// Handles a user's update password request.
pub async fn change_password_post_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<ChangePasswordForm>,
) -> impl IntoResponse {
    if state.config.read().await.is_autologin {
        return Error::ConfirmForbidden.into_response();
    }

    let user_id = ctx.0.user_id();

    if form.password == form.new_password {
        broadcast_error(
            &state,
            user_id,
            "New password cannot be the same as the current.",
        )
        .await;
        return Error::Form.into_response();
    }

    if form.validate().is_err() {
        broadcast_error(&state, user_id, "Passwords do not match.").await;
        return Error::Form.into_response();
    }

    let user_id = ctx.0.user_id();

    match User::update_password_by_user_id(&state.mm, user_id, &form.new_password).await {
        Ok(_) => {
            let toast = MessageWs::success("Your password has been updated.");

            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }

            (StatusCode::NO_CONTENT, "").into_response()
        }
        Err(err) => {
            broadcast_error(&state, user_id, "Failed to update password.").await;
            Error::Model(err).into_response()
        }
    }
}

/// Handles account confirmation once the user clicks their confirm button.
pub async fn confirm_handler(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match get_token_from_query(query) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    let user = match User::get_user_by_email(&state.mm, &token.ident).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Error::Model(EntityNotFound {
                    entity: "user",
                    id: -1,
                })
                .into_response();
            }
        },
        Err(err) => return Error::Model(err).into_response(),
    };

    if validate_web_token(&token, user.token_salt).is_err() {
        return Error::ConfirmInvalidToken.into_response();
    }

    if let Err(err) = user.set_is_confirmed(&state.mm).await {
        return Error::Model(err).into_response();
    };

    templates::general::simple("Success", "Your account has been confirmed.").into_response()
}

fn get_token_from_query(query: HashMap<String, String>) -> Result<Token> {
    let token: Token = match query.get("token") {
        Some(token) => token.parse()?,
        None => return Err(Error::NoToken),
    };
    Ok(token)
}

/// Renders the forgot password request page.
pub async fn forgot_password_handler() -> impl IntoResponse {
    templates::auth::forgot_password().into_response()
}

/// Handles the forgot password request form.
pub async fn forgot_password_post_handler(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        return Error::Form.into_response();
    }

    let user_email = form.email;

    if let Ok(Some(user)) = User::get_user_by_email(&state.mm, &user_email).await {
        if let Ok(token) = generate_web_token(&user_email, user.token_salt) {
            if let Some(email) = state.email_service {
                let payload = Email {
                    to: user_email.clone(),
                    subject: "Reset your password".into(),
                    body: "".to_string(),
                    template: Some(Template::ForgotPassword),
                    data: Some(Data {
                        token: token.to_string(),
                        username: user_email,
                        url: state.config.read().await.base_url.clone(),
                    }),
                };

                if let Err(err) = email.send(&payload) {
                    error!("Could not send email 'Reset your password': {:?}", err);
                }
            }
        }
    }

    templates::general::simple(
        "Password Reset Requested",
        "An email with instructions on how to reset your password has been sent to you. Please check your inbox and follow the provided steps to regain access to your account.",
    ).into_response()
}

/// Renders the forgot password reset page.
pub async fn forgot_password_reset_handler(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match get_token_from_query(query) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    let user = match User::get_user_by_email(&state.mm, &token.ident).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Error::NoUser.into_response(),
        },
        Err(err) => return Error::Model(err).into_response(),
    };

    if validate_web_token(&token, user.token_salt).is_err() {
        let mut res = templates::general::simple(
            "Token Expired", "The token associated with the URL expired. The problem has been forwarded to our team automatically. We will look into it and come back to you. We apologise for this inconvenience.",
        ).into_response();
        *res.status_mut() = StatusCode::BAD_REQUEST;
        return res;
    }

    templates::auth::forgot_password_reset(user.id).into_response()
}

/// Handles the submission of the password reset form.
pub async fn forgot_password_reset_post_handler(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordResetForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = Error::Form.into_response();
        add_hx_message(&mut res, MessageHtmx::success("Password is invalid"));
        return res;
    }

    let user_id = form.user_id;

    if let Err(err) = User::update_password_by_user_id(&state.mm, user_id, &form.password).await {
        error!("Failed to update password for user {user_id} - Error: {err}");
        let mut res = Error::Form.into_response();
        add_hx_message(&mut res, MessageHtmx::error("Failed to update password."));
        return res;
    }

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    add_hx_message(
        &mut res,
        MessageHtmx::success("Your password has been updated."),
    );
    if let Ok(value) = HeaderValue::from_str("/auth/login") {
        res.headers_mut()
            .insert(axum_htmx::headers::HX_REDIRECT, value);
    }
    res
}

/// Renders the login page.
pub async fn login_handler(State(state): State<AppState>) -> impl IntoResponse {
    let config = state.config.read().await;

    templates::auth::login(config.is_demo, config.is_no_signups).into_response()
}

/// Handles user login requests.
pub async fn login_post_handler(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = Error::Form.into_response();
        add_hx_message(&mut res, MessageHtmx::error("Credentials are invalid."));
        return res;
    }

    let user = match User::get_user_by_email(&state.mm, String::from(&form.email)).await {
        Ok(user) => match user {
            None => {
                let mut res = Error::LoginFailUsernameNotFound.into_response();
                add_hx_message(&mut res, MessageHtmx::error("Credentials are invalid."));
                return res;
            }
            Some(user) => user,
        },
        Err(error) => return Error::Model(error).into_response(),
    };

    // Validate password
    let scheme_status = match validate_pwd(
        ContentToHash {
            salt: user.password_salt,
            content: String::from(&form.password),
        },
        &user.password,
    )
    .await
    {
        Ok(status) => status,
        Err(_err) => {
            let mut res = Error::PwdNotMatching { user_id: user.id }.into_response();
            add_hx_message(&mut res, MessageHtmx::error("Credentials are invalid."));
            return res;
        }
    };

    // Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("pwd encrypt scheme outdated, upgrading");
        if user
            .update_password(&state.mm, &form.password)
            .await
            .is_err()
        {
            let mut res = Error::UpdatePassword.into_response();
            add_hx_message(
                &mut res,
                MessageHtmx::error("Failed to update password schema."),
            );
            return res;
        };
    }

    match set_token_cookie(
        &cookies,
        &user.email,
        user.token_salt,
        form.is_remember_me(),
    ) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(err) => {
            let mut res = (StatusCode::BAD_REQUEST, "Login failed").into_response();
            add_hx_message(&mut res, MessageHtmx::error("Failed to log you in."));
            error!("Failed to set cookie for user {} - Error: {err}", user.id);
            res
        }
    }
}

/// Handles a user logging out.
pub async fn logout_post_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    if state.config.read().await.is_autologin {
        return Error::LogoutForbidden.into_response();
    }

    match remove_token_cookie(&cookies) {
        Ok(_) => {
            if let Err(err) = User::update_remember_me(&state.mm, ctx.0.user_id(), false).await {
                error!("Could not logout user with id {}: {err}", ctx.0.user_id());
                return Error::LogoutFail.into_response();
            }

            let mut res = Redirect::to("/").into_response();
            res.headers_mut().insert(
                axum_htmx::headers::HX_REDIRECT,
                HeaderValue::from_static("/"),
            );
            res
        }
        Err(_) => Error::LogoutFail.into_response(),
    }
}

/// Renders the user registration page.
pub async fn register_handler(State(state): State<AppState>) -> impl IntoResponse {
    if state.config.read().await.is_no_signups {
        return Redirect::to("/auth/login").into_response();
    }

    templates::auth::register().into_response()
}

/// Handles user registration.
pub async fn register_post_handler(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let config = state.config.read().await;

    if config.is_no_signups {
        return Redirect::to("/auth/login").into_response();
    }

    if form.validate().is_err() {
        let mut res = Error::PwdNotMatching { user_id: -1 }.into_response();
        add_hx_message(&mut res, MessageHtmx::error("Passwords do not match."));
        return res;
    }

    let user = match User::new(&state.mm, form.to_user()).await {
        Ok(id) => id,
        Err(err) => {
            let mut res = Error::Model(err).into_response();
            add_hx_message(
                &mut res,
                MessageHtmx::error("An error occurred during registration."),
            );
            return res;
        }
    };

    let token = match generate_web_token(&user.email, user.token_salt) {
        Ok(token) => token,
        Err(_) => {
            let mut res = Error::GenerateToken.into_response();
            add_hx_message(
                &mut res,
                MessageHtmx::error("Could not generate web token for authentication."),
            );
            return res;
        }
    };

    if let Some(service) = state.email_service {
        let base_url = config.base_url.clone();

        tokio::spawn(async move {
            service.send(&Email {
                to: user.email,
                subject: "Confirm Account".into(),
                body: "".into(),
                template: Some(Template::Intro),
                data: Some(Data {
                    token: token.to_string(),
                    username: form.email,
                    url: base_url,
                }),
            })
        });
    }

    Redirect::to("/auth/login").into_response()
}

/// Handles user deletion.
pub async fn user_delete_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    let config = state.config.read().await;
    let user_id = ctx.0.user_id();

    if config.is_autologin {
        broadcast_error(&state, user_id, "This account cannot be deleted.").await;
        return Error::DeleteForbidden.into_response();
    }

    if config.is_demo && is_demo_user(&state, user_id).await {
        broadcast_error(
            &state,
            user_id,
            "Trump is Putin's lap dog. Remove him from office!",
        )
        .await;
        return Error::DeleteForbidden.into_response();
    }

    match User::delete(&state.mm, user_id).await {
        Ok(_) => {
            drop(config);
            logout_post_handler(ctx, State(state), cookies)
                .await
                .into_response()
        }
        Err(err) => {
            error!("Could not delete user with id {user_id}: {err}");
            Error::DeleteUser.into_response()
        }
    }
}

async fn is_demo_user(state: &AppState, user_id: i64) -> bool {
    match User::get_user_by_email(&state.mm, "demo@demo.com").await {
        Ok(Some(user)) => user.id == user_id,
        _ => false,
    }
}
