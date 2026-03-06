use axum::body::Body;
use axum::extract::ws::Message;
use axum::http::HeaderValue;
use axum::response::Response;
use serde::Serialize;
use uuid::Uuid;

use app::state::AppState;

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

/// Represents an HTMX-compatible message.
#[derive(Serialize)]
pub struct MessageHtmx {
    #[serde(rename = "showMessageHtmx")]
    content: Content,
}

#[derive(Serialize)]
pub struct MessageWs {
    #[serde(rename = "showMessageWs")]
    content: Content,
}

/// Defines the structure of an HTMX message.
#[derive(Default, Serialize)]
struct Content {
    #[serde(rename = "type")]
    pub _type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub message: String,
    pub status: MessageStatus,
    pub title: String,
}

/// Enum representing different message types.
#[allow(dead_code)]
#[derive(Default, Serialize)]
pub enum MessageType {
    #[serde(rename = "toast")]
    #[default]
    Toast,
    #[serde(rename = "file")]
    File,
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
    pub fn action(mut self, action: Option<&str>) -> Self {
        self.action = action.map(String::from);
        self
    }

    /// Sets the message status.
    #[allow(dead_code)]
    pub const fn status(mut self, status: MessageStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the message type.
    #[allow(dead_code)]
    pub const fn message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    /// Builds and returns a `MessageHtmx` instance.
    pub fn build(self) -> MessageHtmx {
        MessageHtmx {
            content: Content {
                _type: self.message_type,
                action: self.action,
                message: self.message,
                status: self.status,
                title: self.title,
            },
        }
    }
}

impl IMessage for MessageHtmx {
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
                _type: MessageType::Toast,
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
                _type: MessageType::Toast,
                message: message.into(),
                title: "Operation Successful".into(),
                ..Default::default()
            },
        }
    }

    fn warning(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                _type: MessageType::Toast,
                message: message.into(),
                title: "Attention".into(),
                status: MessageStatus::Warning,
                ..Default::default()
            },
        }
    }
}

impl IMessage for MessageWs {
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
                message: message.into(),
                title: "Operation Failed".to_string(),
                ..Default::default()
            },
        }
    }

    fn success(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: "Operation Successful".to_string(),
                ..Default::default()
            },
        }
    }

    fn warning(message: impl Into<String>) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: "Attention".to_string(),
                ..Default::default()
            },
        }
    }
}

/// Adds an HTMX message to a response's headers.
pub fn add_hx_message(res: &mut Response<Body>, message: &MessageHtmx) {
    if let Ok(toast) = serde_json::to_string(&message)
        && let Ok(value) = HeaderValue::from_str(&toast)
    {
        res.headers_mut().insert(axum_htmx::HX_TRIGGER, value);
    }
}

/// Broadcasts a success toast to all active WebSocket subscribers of a given user.
pub async fn broadcast_success(state: &AppState, user_id: Uuid, message: &str) {
    let toast = MessageHtmx::success(message);
    if let Ok(json) = serde_json::to_string(&toast) {
        state.broadcast(Message::Text(json.into()), user_id).await;
    }
}

/// Broadcasts an error toast to all active WebSocket subscribers of a given user.
pub async fn broadcast_error(state: &AppState, user_id: Uuid, message: &str) {
    let toast = MessageHtmx::error(message);
    if let Ok(json) = serde_json::to_string(&toast) {
        state.broadcast(Message::Text(json.into()), user_id).await;
    }
}

/// Broadcasts a warning toast to all active WebSocket subscribers of a given user.
pub async fn broadcast_warning(state: &AppState, user_id: Uuid, message: &str) {
    let toast = MessageHtmx::warning(message);
    if let Ok(json) = serde_json::to_string(&toast) {
        state.broadcast(Message::Text(json.into()), user_id).await;
    }
}
