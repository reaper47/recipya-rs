use crate::handlers::KEY_HX_TRIGGER;
use axum::{body::Body, http::HeaderValue, response::Response};
use serde_with::serde_derive::Serialize;

pub(crate) fn add_hx_message(res: &mut Response<Body>, message: MessageHtmx) {
    if let Ok(toast) = serde_json::to_string(&message) {
        if let Ok(value) = HeaderValue::from_str(&toast) {
            res.headers_mut().insert(KEY_HX_TRIGGER, value);
        }
    }
}

pub(crate) trait IMessage {
    fn builder(
        message_type: MessageType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> MessageBuilder;
    fn error(message: impl Into<String>) -> Self;
    fn success(message: impl Into<String>) -> Self;
}

#[derive(Serialize)]
pub(crate) struct MessageHtmx {
    #[serde(rename = "showMessageHtmx")]
    content: Content,
}

#[derive(Serialize)]
pub(crate) struct MessageWs {
    #[serde(rename = "showMessageWs")]
    content: Content,
}

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

#[derive(Default, Serialize)]
enum MessageType {
    #[serde(rename = "toast")]
    #[default]
    Toast,
    #[serde(rename = "file")]
    File,
}

#[derive(Default, Serialize)]
pub(crate) enum MessageStatus {
    #[serde(rename = "alert-error")]
    Error,
    #[default]
    #[serde(rename = "alert-info")]
    Success,
    #[serde(rename = "alert-warning")]
    Warning,
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
}

#[derive(Default)]
struct MessageBuilder {
    message_type: MessageType,
    action: Option<String>,
    message: String,
    status: MessageStatus,
    title: String,
}

impl MessageBuilder {
    fn action(mut self, action: Option<String>) -> Self {
        self.action = action;
        self
    }

    pub(crate) fn status(mut self, status: MessageStatus) -> Self {
        self.status = status;
        self
    }

    pub(crate) fn message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    pub(crate) fn build(self) -> MessageHtmx {
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
