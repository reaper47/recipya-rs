use axum::{body::Body, http::HeaderValue, response::Response};
use serde::Serialize;

pub mod handlers_auth;
pub mod handlers_general;
pub mod handlers_rpc;

pub const KEY_HX_REDIRECT: &str = "HX-Redirect";
pub const KEY_HX_TRIGGER: &str = "HX-Trigger";

pub(crate) fn add_hx_toast(res: &mut Response<Body>, mut toast: Toast) {
    toast._type = ToastType::HX.to_string();
    if let Ok(toast) = serde_json::to_string(&toast) {
        if let Ok(value) = HeaderValue::from_str(&toast) {
            res.headers_mut().insert(KEY_HX_TRIGGER, value);
        }
    }
}

#[derive(Serialize)]
pub(crate) struct Toast {
    #[serde(rename = "type")]
    _type: String,
    data: ToastData,
}

impl Toast {
    pub(crate) fn success(message: impl Into<String>) -> Self {
        Self {
            _type: "toast".to_string(),
            data: ToastData {
                message: message.into(),
                title: "Operation Successful".to_string(),
                ..Default::default()
            },
        }
    }

    pub(crate) fn builder() -> ToastBuilder {
        ToastBuilder::default()
    }
}

#[derive(Default, Serialize)]
struct ToastData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub message: String,
    #[serde(rename = "background")]
    pub status: ToastStatus,
    pub title: String,
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

#[derive(Default)]
pub(crate) enum ToastType {
    HX,
    #[default]
    Toast,
}

impl ToString for ToastType {
    fn to_string(&self) -> String {
        match self {
            ToastType::HX => "hx-toast".to_string(),
            ToastType::Toast => "toast".to_string(),
        }
    }
}

#[derive(Default)]
pub(crate) struct ToastBuilder {
    _type: ToastType,
    action: Option<String>,
    message: String,
    status: ToastStatus,
    title: String,
}

impl ToastBuilder {
    pub(crate) fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    pub(crate) fn action(mut self, action: Option<String>) -> Self {
        self.action = action;
        self
    }

    pub(crate) fn status(mut self, status: ToastStatus) -> Self {
        self.status = status;
        self
    }

    pub(crate) fn build(self) -> Toast {
        Toast {
            _type: self._type.to_string(),
            data: ToastData {
                action: self.action,
                message: self.message,
                status: self.status,
                title: self.title,
            },
        }
    }
}
