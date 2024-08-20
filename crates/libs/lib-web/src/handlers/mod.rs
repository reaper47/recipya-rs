use axum::{body::Body, http::HeaderValue, response::Response};
use serde::Serialize;

pub mod handlers_auth;
pub mod handlers_general;
pub mod handlers_rpc;

pub(crate) const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub const KEY_HX_TRIGGER: &str = "HX-Trigger";

pub(crate) fn add_toast(res: &mut Response<Body>, toast: Toast) {
    if let Ok(toast) = serde_json::to_string(&toast) {
        if let Ok(value) = HeaderValue::from_str(&toast) {
            res.headers_mut().insert(KEY_HX_TRIGGER, value);
        }
    }
}

#[derive(Default, Serialize)]
pub(crate) struct Toast {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub message: String,
    #[serde(rename = "background")]
    pub status: ToastStatus,
}

#[derive(Default, Serialize)]
pub(crate) enum ToastStatus {
    #[serde(rename = "alert-error")]
    Error,
    #[default]
    #[serde(rename = "alert-info")]
    Success,
    #[serde(rename = "alert-warning")]
    Warning,
}
