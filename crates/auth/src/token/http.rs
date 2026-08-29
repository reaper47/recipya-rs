use time::Duration;
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

pub use super::Result;

/// Name of the authentication token cookie.
pub const AUTH_TOKEN: &str = "auth-token";

/// Name of the refresh token cookie.
pub const REFRESH_TOKEN: &str = "refresh-token";

/// Sets both the access and refresh tokens at once.
pub fn set_auth_cookies(
    cookies: &Cookies,
    access_token: String,
    refresh_token: String,
    remember_me: bool,
    is_production: bool,
) {
    set_access_token_cookie(cookies, access_token, is_production);
    set_refresh_token_cookie(cookies, refresh_token, remember_me, is_production);
}

fn set_access_token_cookie(cookies: &Cookies, token_value: String, is_production: bool) {
    let mut cookie = Cookie::new(AUTH_TOKEN, token_value);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");
    cookie.set_max_age(Duration::minutes(15));

    if is_production {
        cookie.set_secure(true);
    }

    cookies.add(cookie);
}

fn set_refresh_token_cookie(
    cookies: &Cookies,
    token_value: String,
    remember_me: bool,
    is_production: bool,
) {
    let mut cookie = Cookie::new(REFRESH_TOKEN, token_value);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");

    if remember_me {
        cookie.set_max_age(Duration::days(30));
    }

    if is_production {
        cookie.set_secure(true);
    }

    cookies.add(cookie);
}

/// Removes the auth tokens from the user's browser storage.
pub fn clear_auth_cookies(cookies: &Cookies) {
    remove_token_cookie(cookies, AUTH_TOKEN.into());
    remove_token_cookie(cookies, REFRESH_TOKEN.into());
}

fn remove_token_cookie(cookies: &Cookies, name: String) {
    let mut access_cookie = Cookie::from(name);
    access_cookie.set_path("/");
    cookies.remove(access_cookie);
}
