use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::core::auth::token::{generate_long_lasting_web_token, generate_web_token};

pub use crate::error::Result;

/// The name of the authentication token cookie.
///
/// This constant defines the key for the authentication token cookie, which is used to identify
/// and authenticate the user.
pub const AUTH_TOKEN: &str = "auth-token";

/// Sets an authentication token cookie for the user.
pub(crate) fn set_token_cookie(
    cookies: &Cookies,
    user: &str,
    salt: Uuid,
    is_lasts_long: bool,
) -> Result<()> {
    let token = if is_lasts_long {
        generate_long_lasting_web_token(user, salt)?
    } else {
        generate_web_token(user, salt)?
    };

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);
    Ok(())
}

/// Removes the authentication token cookie from the user's browser.
pub(crate) fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);
    Ok(())
}
