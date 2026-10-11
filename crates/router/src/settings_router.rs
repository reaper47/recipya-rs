use axum::routing::{get, post};
use axum::{Router, middleware};

use app::state::AppState;
use tower_cookies::cookie::{self, SameSite};
use tower_cookies::{Cookie, Cookies};

use crate::handlers::settings as h;
use crate::middleware::mw_auth::{mw_only_admin, mw_refresh_token};

/// Name of the cookie that stores the selected language.
pub const LANGUAGE_COOKIE_NAME: &str = "lang";

/// Defines the routes for endpoints related to the settings module.
pub fn settings_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(h::settings_handler))
        .route("/bold-ingredients", post(h::set_bold_ingredients_handler))
        .route(
            "/export-data",
            get(h::export_data_handler).post(h::export_data_post_handler),
        )
        .route("/language", post(h::language_post_handler))
        .route("/nutrition/source", post(h::set_nutrition_source_handler))
        .route("/paper-size", post(h::set_paper_size_handler))
        .route(
            "/theme-default",
            post(h::set_default_theme_handler)
                .layer(middleware::from_fn_with_state(state.clone(), mw_only_admin)),
        )
        .route("/theme-selected", post(h::set_selected_theme_handler))
        .route("/tz", post(h::set_selected_timezone_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ))
}

pub fn set_language_cookie(cookies: &Cookies, token_value: &str, is_production: bool) {
    let mut cookie = Cookie::new(LANGUAGE_COOKIE_NAME, token_value.to_string());
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");
    cookie.set_max_age(cookie::time::Duration::weeks(52));

    if is_production {
        cookie.set_secure(true);
    }

    cookies.add(cookie);
}
