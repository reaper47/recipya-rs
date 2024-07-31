use axum::{
    {Form, middleware, Router},
    extract::State,
    http::{HeaderValue, StatusCode},
    Json,
    response::{IntoResponse, Redirect}, routing::get,
};
use axum::routing::post;
use maud::Markup;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;

use crate::{
    {crypt, model, web, web::KEY_HX_REDIRECT},
    ctx::Ctx,
    model::{ModelManager, payloads::collect_errors, user::UserBmc},
    web::{Error, remove_token_cookie},
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/auth/confirm", get(confirm))
        .route(
            "/auth/forgot-password",
            get(forgot_password).post(forgot_password_post),
        )
        .route(
            "/auth/forgot-password/reset",
            post(forgot_password_reset_post),
        )
        .route("/auth/logout", post(logout_post))
        .merge(routes_require_auth(mm.clone()))
        .merge(routes_redirect_if_logged_in(mm.clone()))
        .merge(routes_register(mm.clone()))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            web::middleware::ctx::require,
        ))
        .with_state(mm.clone())
}

fn routes_require_auth(_mm: ModelManager) -> Router<ModelManager> {
    Router::new().route("/auth/change-password", post(change_password))
    /*.layer(middleware::from_fn_with_state(
        mm,
        web::middleware::auth::require,
    ))*/
}

fn routes_redirect_if_logged_in(mm: ModelManager) -> Router<ModelManager> {
    Router::new()
        .route("/auth/login", get(login).post(login_post))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            web::middleware::auth::redirect_if_logged_in,
        ))
}

fn routes_register(mm: ModelManager) -> Router<ModelManager> {
    Router::new()
        .route("/auth/register", get(register).post(register_post))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            web::middleware::auth::redirect_if_logged_in,
        ))
        .layer(middleware::from_fn_with_state(
            mm,
            web::middleware::auth::redirect_if_no_signups,
        ))
}

async fn change_password() -> Markup {
    todo!()
}

async fn confirm() -> Markup {
    todo!()
}

async fn forgot_password() -> Markup {
    web::templates::auth::forgot_password().await
}

async fn forgot_password_post() -> Markup {
    todo!()
}

async fn forgot_password_reset_post() -> Markup {
    todo!()
}

async fn login() -> Markup {
    web::templates::auth::login().await
}

async fn login_post(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Form(form): Form<model::payloads::LoginForm>,
) -> impl IntoResponse {
    let errors = collect_errors(&form);
    if !errors.is_empty() {
        return (StatusCode::BAD_REQUEST, collect_errors(&form).join(", ")).into_response();
    }

    let root_ctx = Ctx::root_ctx();

    let user = match UserBmc::first_by_email(&root_ctx, &mm, &form.email).await {
        Ok(user) => match user {
            None => return Error::LoginFailUserNotFound.into_response(),
            Some(user) => user,
        },
        Err(error) => return Error::RepositoryError(error.to_string()).into_response(),
    };

    let res = crypt::password::validate(
        &crypt::EncryptContent {
            salt: user.password_salt.to_string(),
            content: form.password,
        },
        &user.password,
    );
    if res.is_err() {
        return Error::LoginFailPasswordNotMatching { user_id: user.id }.into_response();
    }

    match web::set_token_cookie(&cookies, &user.email, &user.token_salt.to_string()) {
        Ok(_) =>  (StatusCode::OK, "Login successful").into_response(),
        Err(_) =>  (StatusCode::BAD_REQUEST, "Login failed").into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}

async fn logout_post(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>, Error> {
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

async fn register() -> impl IntoResponse {
    web::templates::auth::register().await.into_response()
}

async fn register_post(
    State(_mm): State<ModelManager>,
    Form(_form): Form<model::payloads::RegisterForm>,
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
