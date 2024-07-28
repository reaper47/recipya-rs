pub mod error;
mod middleware;
mod routes;
pub mod templates;

pub use error::*;
pub use routes::*;

pub const AUTH_TOKEN: &str = "auth-token";

pub const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub const KEY_HX_TRIGGER: &str = "HX-Trigger";
