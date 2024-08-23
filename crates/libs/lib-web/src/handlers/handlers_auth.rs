use std::collections::HashMap;

use axum::{
    extract::{ws::Message, Query, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Redirect},
    Form,
};
use maud::Markup;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use tracing::{debug, error};
use validator::Validate;

use crate::{
    error::{collect_errors, Error},
    middleware::mw_auth::CtxW,
    templates,
    utils::token::{remove_token_cookie, set_token_cookie},
    AppState,
};
use lib_auth::{
    pwd::scheme::SchemeStatus,
    token::{generate_web_token, validate_web_token, Token},
};
use lib_core::{
    config,
    ctx::Ctx,
    model::user::{self, UserBmc, UserForCreate},
};
use lib_email::{Data, Template};

use super::{add_hx_toast, Toast, ToastBuilder, ToastStatus, ToastType, KEY_HX_REDIRECT};

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct ChangePasswordForm {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordResetForm {
    pub user_id: i64,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct RegisterForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub password_confirm: String,
}

pub async fn change_password(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<ChangePasswordForm>,
) -> impl IntoResponse {
    if config().IS_AUTOLOGIN {
        return Error::ConfirmForbidden.into_response();
    }

    if form.password == form.new_password {
        let toast = ToastBuilder::new(
            "Request Error",
            "New password cannot be the same as the current.",
        )
        .status(ToastStatus::Error)
        .build();

        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(ctx.0.user_id(), Message::Text(json)).await;
        }

        return Error::Form.into_response();
    }

    if form.validate().is_err() {
        let toast = ToastBuilder::new("Request Error", "Passwords do not match.")
            .status(ToastStatus::Error)
            .build();

        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(ctx.0.user_id(), Message::Text(json)).await;
        }

        return Error::Form.into_response();
    }

    let ctx = ctx.0;
    let user_id = ctx.user_id();

    match UserBmc::update_password(&ctx, &state.mm, user_id, &form.new_password).await {
        Ok(_) => {
            let toast = Toast::success("Your password has been updated.");

            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json)).await;
            }

            (StatusCode::NO_CONTENT, "").into_response()
        }
        Err(err) => {
            let toast = ToastBuilder::new("Operation Failed", "Failed to update password.")
                .status(ToastStatus::Error)
                .build();

            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json)).await;
            }

            Error::Model(err).into_response()
        }
    }
}

pub async fn confirm(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token: Token = match query.get("token") {
        Some(token) => match token.parse() {
            Ok(token) => token,
            Err(err) => return Error::Token(err).into_response(),
        },
        None => return Error::NoToken.into_response(),
    };

    let user = match UserBmc::first_by_email(&Ctx::root_ctx(), &state.mm, &token.ident).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Error::Model(lib_core::model::Error::EntityNotFound {
                    entity: "user",
                    id: -1,
                })
                .into_response()
            }
        },
        Err(err) => return Error::Model(err).into_response(),
    };

    if validate_web_token(&token, user.token_salt).is_err() {
        return Error::ConfirmInvalidToken.into_response();
    }

    if let Err(err) = UserBmc::set_is_confirmed(&Ctx::root_ctx(), &state.mm, token.ident).await {
        return Error::Model(err).into_response();
    };

    templates::general::simple("Success", "Your account has been confirmed.").into_response()
}

pub async fn forgot_password() -> impl IntoResponse {
    templates::auth::forgot_password().into_response()
}

pub async fn forgot_password_post(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        return Error::Form.into_response();
    }

    tokio::spawn(async move {
        if let Ok(Some(user)) =
            UserBmc::first_by_email(&Ctx::root_ctx(), &state.mm, &form.email).await
        {
            if let Ok(token) = lib_auth::token::generate_web_token(&form.email, user.token_salt) {
                state
                    .send_email(
                        String::from(&form.email),
                        "Reset your password".to_string(),
                        Template::ForgotPassword,
                        Data {
                            token: token.to_string(),
                            username: String::from(&form.email),
                            url: config().ADDRESS_URL.clone(),
                        },
                    )
                    .await;
            }
        }
    });

    templates::general::simple(
        "Password Reset Requested", 
        "An email with instructions on how to reset your password has been sent to you. Please check your inbox and follow the provided steps to regain access to your account."
    ).into_response()
}

pub async fn forgot_password_reset(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token: Token = match query.get("token") {
        Some(token) => match token.parse() {
            Ok(token) => token,
            Err(err) => return Error::Token(err).into_response(),
        },
        None => return Error::NoToken.into_response(),
    };

    let user = match UserBmc::first_by_email(&Ctx::root_ctx(), &state.mm, &token.ident).await {
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

pub async fn forgot_password_reset_post(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordResetForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = Error::Form.into_response();
        add_hx_toast(
            &mut res,
            ToastBuilder::new("Request Failed", "Password is invalid")
                .status(ToastStatus::Error)
                .build(),
        );
        return res;
    }

    if let Err(err) =
        UserBmc::update_password(&Ctx::root_ctx(), &state.mm, form.user_id, &form.password).await
    {
        error!(
            "Failed to update password for user {} - Error: {err}",
            form.user_id
        );

        let mut res = Error::Form.into_response();
        add_hx_toast(
            &mut res,
            ToastBuilder::new("Operation Failed", "Failed to update password.")
                .status(ToastStatus::Error)
                .build(),
        );
        return res;
    }

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    add_hx_toast(&mut res, Toast::success("Your password has been updated."));
    if let Ok(value) = HeaderValue::from_str("/auth/login") {
        res.headers_mut().insert(KEY_HX_REDIRECT, value);
    }
    res
}

pub async fn login() -> impl IntoResponse {
    templates::auth::login(false)
}

pub async fn login_post(
    State(state): State<AppState>,
    cookies: Cookies,
    Query(query): Query<HashMap<String, String>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = templates::auth::login(true).into_response();
        *res.status_mut() = StatusCode::BAD_REQUEST;
        return res;
    }

    let root_ctx = Ctx::root_ctx();

    let user = match UserBmc::first_by_email(&root_ctx, &state.mm, &form.email).await {
        Ok(user) => match user {
            None => return Error::LoginFailUsernameNotFound.into_response(),
            Some(user) => user,
        },
        Err(error) => return Error::Model(error).into_response(),
    };

    // Validate password
    let scheme_status = match lib_auth::pwd::validate_pwd(
        lib_auth::pwd::ContentToHash {
            salt: user.password_salt,
            content: form.password,
        },
        &user.password,
    )
    .await
    {
        Ok(status) => status,
        Err(_) => return Error::LoginFailPwdNotMatching { user_id: user.id }.into_response(),
    };

    // Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("pwd encrypt scheme outdated, upgrading");
        if UserBmc::update_password(&root_ctx, &state.mm, user.id, &user.password)
            .await
            .is_err()
        {
            return Error::UpdatePassword.into_response();
        }
    }

    match set_token_cookie(
        &cookies,
        &user.email,
        user.token_salt,
        form.remember_me.unwrap_or(false),
    ) {
        Ok(_) => {
            let redirect_to = query
                .get("redirect_to")
                .cloned()
                .unwrap_or_else(|| "/".to_string());

            // TODO: Test this for endpoints that require auth
            Redirect::to(&redirect_to).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
    }
}

pub async fn logout_post(cookies: Cookies) -> impl IntoResponse {
    if config().IS_AUTOLOGIN {
        return Error::LogoutForbidden.into_response();
    }

    match remove_token_cookie(&cookies) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => Error::LogoutFail.into_response(),
    }
}

pub async fn register() -> impl IntoResponse {
    if config().IS_NO_SIGNUPS {
        return Redirect::to("/auth/login").into_response();
    }
    templates::auth::register(false).into_response()
}

pub async fn register_post(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        let mut res = templates::auth::register(true).into_response();
        *res.status_mut() = StatusCode::BAD_REQUEST;
        return res;
    }

    if lib_core::config().IS_NO_SIGNUPS {
        return Redirect::to("/auth/login").into_response();
    }

    let ctx = Ctx::root_ctx();
    let user_c = UserForCreate {
        email: form.email.clone(),
        password_clear: form.password,
    };

    let id = match UserBmc::create(&ctx, &state.mm, user_c).await {
        Ok(id) => id,
        Err(_) => {
            let mut res = templates::auth::register(true).into_response();
            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return res;
        }
    };

    match UserBmc::get(&ctx, &state.mm, id)
        .await
        .map_err(|_| Error::RegisterFail)
    {
        Ok(user) => {
            let email: String = form.email;

            let token = match generate_web_token(&email, user.token_salt) {
                Ok(token) => token,
                Err(_) => return Error::GenerateToken.into_response(),
            };

            tokio::spawn(async move {
                state
                    .send_email(
                        String::from(&email),
                        "Confirm Account".to_string(),
                        Template::Intro,
                        Data {
                            token: token.to_string(),
                            username: String::from(&email),
                            url: config().ADDRESS_URL.clone(),
                        },
                    )
                    .await;
            });
        }
        Err(err) => return err.into_response(),
    }

    Redirect::to("/auth/login").into_response()
}
