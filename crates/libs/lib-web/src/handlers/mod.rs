use axum::{body::Body, http::HeaderValue, response::Response};
use serde::Serialize;

pub mod handlers_auth;
pub mod handlers_general;
pub mod handlers_rpc;
pub mod message;

pub const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub const KEY_HX_TRIGGER: &str = "HX-Trigger";
