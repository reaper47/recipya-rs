use tower_cookies::{Cookie, Cookies};

pub use error::{Error, Result};
pub use routes::*;

use crate::crypt::token::generate_web_token;

mod middleware;
mod routes;

pub mod error;
pub mod templates;

pub const AUTH_TOKEN: &str = "auth-token";

pub const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub const KEY_HX_TRIGGER: &str = "HX-Trigger";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");
    cookies.remove(cookie);
    Ok(())
}
