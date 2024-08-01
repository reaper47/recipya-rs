use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use lib_auth::token::generate_web_token;

pub use self::error::{ClientError, Error, Result};

mod error;
pub(crate) mod templates;

pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_auth;
pub mod routes_general;
pub mod routes_rpc;

pub const AUTH_TOKEN: &str = "auth-token";

pub const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub const KEY_HX_TRIGGER: &str = "HX-Trigger";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
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
