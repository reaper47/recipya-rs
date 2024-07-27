mod routes;
pub mod templates;
mod middleware;

pub use routes::*;

pub const AUTH_TOKEN: &str = "auth-token";
