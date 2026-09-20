use std::future::Future;

use axum::body::Body;
use axum::http::HeaderValue;
use axum::response::Response;
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

/// A trait defining message toasts broadcasting methods for structured responses.
pub trait Broadcaster {
    /// Broadcasts a success toast to all active SSE connections of a given user.
    fn broadcast_success(
        state: &AppState,
        user_id: Uuid,
        message: &str,
    ) -> impl Future<Output = ()> + Send;

    /// Broadcasts a warning toast to all active SSE connections of a given user.
    fn broadcast_warning(
        state: &AppState,
        user_id: Uuid,
        message: &str,
    ) -> impl Future<Output = ()> + Send;

    /// Broadcasts an error toast to all active SSE connections of a given user.
    fn broadcast_error(
        state: &AppState,
        user_id: Uuid,
        message: &str,
    ) -> impl Future<Output = ()> + Send;
}

/// A trait defining message creation methods for structured responses.
pub trait IMessage {
    fn builder(
        message_type: MessageType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> MessageBuilder;
    fn error(message: impl Into<String>) -> Self;
    fn success(message: impl Into<String>) -> Self;
    fn warning(message: impl Into<String>) -> Self;
}

/// Represents an HTMX-compatible message usually displayed in the top-right
/// corner of the screen.
#[derive(Serialize)]
pub struct Toast {
    #[serde(rename = "notification")]
    content: Content,
}

/// Represents an HTMX-compatible message usually displayed as live status
/// updates in the bottom right corner of the screen.
#[derive(Serialize)]
pub struct Snack {
    #[serde(rename = "notification")]
    content: Content,
}

/// Defines the structure of an HTMX message.
#[derive(Default, Serialize)]
struct Content {
    #[serde(rename = "type")]
    pub r#type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub message: String,
    pub status: MessageStatus,
    pub title: String,
}

/// Enum representing different message types.
#[allow(dead_code)]
#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    #[default]
    Toast,
    File,
    Snack,
}

/// Enum representing different message statuses.
#[derive(Default, Serialize)]
pub enum MessageStatus {
    #[serde(rename = "alert-error")]
    Error,
    #[default]
    #[serde(rename = "alert-info")]
    Success,
    #[serde(rename = "alert-warning")]
    Warning,
}

/// Builder for constructing `MessageHtmx` instances.
#[derive(Default)]
pub struct MessageBuilder {
    message_type: MessageType,
    action: Option<String>,
    message: String,
    status: MessageStatus,
    title: String,
}

impl MessageBuilder {
    /// Sets the action item.
    #[must_use]
    pub fn action(mut self, action: Option<&str>) -> Self {
        self.action = action.map(String::from);
        self
    }

    /// Sets the message status.
    #[must_use]
    pub const fn status(mut self, status: MessageStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the message type.
    #[must_use]
    pub const fn message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    /// Builds and returns a `MessageHtmx` instance.
    #[must_use]
    pub fn build(self) -> Toast {
        Toast {
            content: Content {
                r#type: self.message_type,
                action: self.action,
                message: self.message,
                status: self.status,
                title: self.title,
            },
        }
    }
}

impl IMessage for Toast {
    fn builder(
        message_type: MessageType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> MessageBuilder {
        MessageBuilder {
            title: title.into(),
            message: message.into(),
            message_type,
            ..Default::default()
        }
    }

    fn error(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: "Operation Failed".into(),
                status: MessageStatus::Error,
                ..Default::default()
            },
        }
    }

    fn success(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: "Success".into(),
                ..Default::default()
            },
        }
    }

    fn warning(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: "Attention".into(),
                status: MessageStatus::Warning,
                ..Default::default()
            },
        }
    }
}

impl IMessage for Snack {
    fn builder(
        message_type: MessageType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> MessageBuilder {
        MessageBuilder {
            message_type,
            title: title.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    fn error(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                r#type: MessageType::Snack,
                message: message.into(),
                title: "Operation Failed".to_string(),
                ..Default::default()
            },
        }
    }

    fn success(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                r#type: MessageType::Snack,
                message: message.into(),
                title: "Success".to_string(),
                ..Default::default()
            },
        }
    }

    fn warning(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                r#type: MessageType::Snack,
                message: message.into(),
                title: "Attention".to_string(),
                ..Default::default()
            },
        }
    }
}

impl Broadcaster for Toast {
    async fn broadcast_success(state: &AppState, user_id: Uuid, message: &str) {
        let toast = Self::success(message);
        if let Ok(json) = serde_json::to_string(&toast) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_warning(state: &AppState, user_id: Uuid, message: &str) {
        let toast = Self::warning(message);
        if let Ok(json) = serde_json::to_string(&toast) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_error(state: &AppState, user_id: Uuid, message: &str) {
        let toast = Self::error(message);
        if let Ok(json) = serde_json::to_string(&toast) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }
}

impl Broadcaster for Snack {
    async fn broadcast_success(state: &AppState, user_id: Uuid, message: &str) {
        let snack = Self::success(message);
        if let Ok(json) = serde_json::to_string(&snack) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_warning(state: &AppState, user_id: Uuid, message: &str) {
        let snack = Self::warning(message);
        if let Ok(json) = serde_json::to_string(&snack) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_error(state: &AppState, user_id: Uuid, message: &str) {
        let snack = Self::error(message);
        if let Ok(json) = serde_json::to_string(&snack) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }
}

/// Adds an HTMX message to a response's headers.
pub fn add_hx_message(res: &mut Response<Body>, message: &Toast) {
    if let Ok(toast) = serde_json::to_string(&message)
        && let Ok(value) = HeaderValue::from_str(&toast)
    {
        res.headers_mut().insert(axum_htmx::HX_TRIGGER, value);
    }
}
