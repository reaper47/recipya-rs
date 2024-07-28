use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    {middleware, Form, Router},
};
use maud::Markup;
use tower_cookies::{Cookie, Cookies};

use crate::model::payloads::collect_errors;
use crate::{
    model,
    web::KEY_HX_REDIRECT,
    {app, web},
};

pub fn routes(state: Arc<app::App>) -> Router {
    Router::new()
        /*.route("/auth/confirm", get(todo!()))
        .route("/auth/forgot-password", get(todo!()).post(todo!()))
        .route("/auth/forgot-password/reset", post(todo!()))
        .route("/auth/forgot-password", get(forgot_password))
        .route("/auth/logout", post(todo!()))*/
        .merge(routes_require_auth(Arc::clone(&state)))
        .merge(routes_redirect_if_logged_in(Arc::clone(&state)))
        .merge(routes_register(Arc::clone(&state)))
        .with_state(state)
}

fn routes_require_auth(state: Arc<app::App>) -> Router<Arc<app::App>> {
    Router::new()
        //.route("/auth/change-password", post(todo!()))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            web::middleware::auth::require,
        ))
}

fn routes_redirect_if_logged_in(state: Arc<app::App>) -> Router<Arc<app::App>> {
    Router::new()
        .route("/auth/login", get(login).post(login_post))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            web::middleware::auth::redirect_if_logged_in,
        ))
}

fn routes_register(state: Arc<app::App>) -> Router<Arc<app::App>> {
    Router::new()
        .route("/auth/register", get(register).post(register_post))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            web::middleware::auth::redirect_if_logged_in,
        ))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            web::middleware::auth::redirect_if_no_signups,
        ))
}

async fn forgot_password() -> Markup {
    web::templates::auth::forgot_password().await
}

async fn login() -> Markup {
    web::templates::auth::login().await
}

async fn login_post(
    cookies: Cookies,
    Form(form): Form<model::payloads::LoginForm>,
) -> impl IntoResponse {
    let errors = collect_errors(&form);
    if !errors.is_empty() {
        return (StatusCode::BAD_REQUEST, collect_errors(&form).join(", ")).into_response();
    }

    // TODO: Make secure cookie
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    (StatusCode::OK, "Login successful").into_response()
}

async fn register() -> impl IntoResponse {
    web::templates::auth::register().await.into_response()
}

async fn register_post(
    State(app): State<Arc<app::App>>,
    Form(form): Form<model::payloads::RegisterForm>,
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

    if let Some(email) = &app.email {
        todo!();
        /*email.send(
            String::from(&form.email),
            "Confirm Account".to_string(),
            Template::Intro,
            Data {
                token: "".to_string(),
                username: form.email,
                url: app.address(false),
            },
        );*/
    }

    let mut res = Redirect::to("/auth/login").into_response();
    res.headers_mut()
        .insert(KEY_HX_REDIRECT, HeaderValue::from_static("/auth/login"));
    res
}
