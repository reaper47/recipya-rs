use std::collections::HashMap;

use axum::{
    extract::{Query, State},
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
    templates,
    utils::token::{remove_token_cookie, set_token_cookie},
};
use lib_auth::{
    pwd::scheme::SchemeStatus,
    token::{generate_web_token, validate_web_token, Token},
};
use lib_core::{
    config,
    ctx::Ctx,
    model::{
        user::{UserBmc, UserForCreate},
        ModelManager,
    },
};
use lib_email::{Data, Template};

use super::{add_toast, Toast, ToastStatus};

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

pub async fn change_password() -> Markup {
    todo!()
}

pub async fn confirm(
    State(mm): State<ModelManager>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token: Token = match query.get("token") {
        Some(token) => match token.parse() {
            Ok(token) => token,
            Err(err) => return Error::Token(err).into_response(),
        },
        None => return Error::ConfirmNoToken.into_response(),
    };

    let user = match UserBmc::first_by_email(&Ctx::root_ctx(), &mm, &token.ident).await {
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

    if let Err(err) = UserBmc::set_is_confirmed(&Ctx::root_ctx(), &mm, token.ident).await {
        return Error::Model(err).into_response();
    };

    templates::general::simple("Success", "Your account has been confirmed.").into_response()
}

pub async fn forgot_password() -> Markup {
    templates::auth::forgot_password().await
}

pub async fn forgot_password_post() -> Markup {
    todo!()
}

pub async fn forgot_password_reset_post() -> Markup {
    todo!()
}

pub async fn login() -> Markup {
    templates::auth::login().await
}

pub async fn login_post(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Query(query): Query<HashMap<String, String>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let errors = collect_errors(&form);
    if !errors.is_empty() {
        return (StatusCode::BAD_REQUEST, collect_errors(&form).join(", ")).into_response();
    }

    let root_ctx = Ctx::root_ctx();

    let user = match UserBmc::first_by_email(&root_ctx, &mm, &form.email).await {
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
        if UserBmc::update_password(&root_ctx, &mm, user.id, &user.password)
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
    templates::auth::register().await.into_response()
}

pub async fn register_post(
    State(mm): State<ModelManager>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    if lib_core::config().IS_NO_SIGNUPS {
        return Redirect::to("/auth/login").into_response();
    }

    let ctx = Ctx::root_ctx();
    let user_c = UserForCreate {
        email: form.email.clone(),
        password_clear: form.password,
    };

    let id = match UserBmc::create(&ctx, &mm, user_c).await {
        Ok(id) => id,
        Err(_) => {
            let mut res = Error::RegisterFail.into_response();
            add_toast(
                &mut res,
                Toast {
                    action: None,
                    message: "Registration failed".to_string(),
                    status: ToastStatus::Error,
                },
            );
            return res;
        }
    };

    match UserBmc::get(&ctx, &mm, id)
        .await
        .map_err(|_| Error::RegisterFail)
    {
        Ok(user) => {
            let token = match generate_web_token(&form.email, user.token_salt) {
                Ok(token) => token,
                Err(_) => return Error::GenerateToken.into_response(),
            };

            tokio::spawn(async move {
                if let Err(err) = lib_email::Sendgrid::new()
                    .send(
                        String::from(&form.email),
                        "Confirm Account".to_string(),
                        Template::Intro,
                        Data {
                            token: token.to_string(),
                            username: form.email,
                            url: config().ADDRESS_URL.clone(),
                        },
                    )
                    .await
                {
                    let id = user.id;
                    error!(name: "Error sending email","error: {} - user id: {}", err, id);
                }
            });
        }
        Err(err) => return err.into_response(),
    }

    Redirect::to("/auth/login").into_response()
}
