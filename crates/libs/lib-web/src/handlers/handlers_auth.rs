use axum::{
    extract::State,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Redirect},
    Form, Json,
};
use lib_auth::pwd::scheme::SchemeStatus;
use lib_core::{
    ctx::Ctx,
    model::{user::UserBmc, ModelManager},
};
use maud::Markup;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;
use validator::Validate;

use crate::{
    error::{collect_errors, Error},
    templates,
    utils::token::{remove_token_cookie, set_token_cookie, KEY_HX_REDIRECT},
};

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

pub async fn confirm() -> Markup {
    todo!()
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

    match set_token_cookie(&cookies, &user.email, user.token_salt) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct LogoffPayload {
    logoff: bool,
}

pub async fn logout_post(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> std::result::Result<Json<Value>, Error> {
    let is_logoff = payload.logoff;
    if is_logoff {
        remove_token_cookie(&cookies)?;
    }

    let body = Json(json!({
        "result": {
            "logged_off": is_logoff,
        }
    }));

    Ok(body)
}

pub async fn register() -> impl IntoResponse {
    templates::auth::register().await.into_response()
}

pub async fn register_post(
    State(_mm): State<ModelManager>,
    Form(_form): Form<RegisterForm>,
) -> impl IntoResponse {
    /*if let Err(error) = app.repository.register(&form.email, &form.password).await {
        // TODO: Log error
        println!("{error:?}");

        let mut res = Error::RegisterFail.into_response();

        let toast = ToastBuilder::new("Registration failed", "Credentials are invalid.");
        if let Ok(toast) = serde_json::to_string(&toast) {
            if let Ok(value) = HeaderValue::from_str(&toast) {
                res.headers_mut().insert(KEY_HX_TRIGGER, value);
            }
        }

        return res;
    }*/

    /*if let Some(email) = &app.email {
        email.send(
            String::from(&form.email),
            "Confirm Account".to_string(),
            Template::Intro,
            Data {
                token: "".to_string(),
                username: form.email,
                url: app.address(false),
            },
        );
    }*/

    let mut res = Redirect::to("/auth/login").into_response();
    res.headers_mut()
        .insert(KEY_HX_REDIRECT, HeaderValue::from_static("/auth/login"));
    res
}
