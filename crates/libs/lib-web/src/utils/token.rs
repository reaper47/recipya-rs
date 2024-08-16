pub use crate::error::{ClientError, Error, Result};
use lib_auth::token::generate_web_token;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub const AUTH_TOKEN: &str = "auth-token";

pub(crate) const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub(crate) const KEY_HX_TRIGGER: &str = "HX-Trigger";

pub(crate) fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

pub(crate) fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
