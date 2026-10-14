use std::future::Future;
use std::marker::PhantomData;

use axum::body::Body;
use axum::http::HeaderValue;
use axum::response::Response;
use serde::Serialize;
use support::states::{AuthenticatedState, UnauthenticatedState};
use uuid::Uuid;

use l10n::Messages;

use crate::state::AppState;

/// Bundles the context needed to send a toast to a user.
///
/// # Example
///
/// ```ignore
/// let toast = ToastCtx::new(&state, user_id, &messages);
///
/// toast.error(&messages.toast_auth_email_or_password_invalid()).await;
/// ```
#[derive(Copy, Clone)]
pub struct ToastCtx<'a, State> {
    state: &'a AppState,
    user_id: Uuid,
    messages: &'a Messages,
    _state: PhantomData<State>,
}

impl<'a> ToastCtx<'a, AuthenticatedState> {
    /// Creates a toast context for sending toast to a user.
    pub const fn new(state: &'a AppState, user_id: Uuid, messages: &'a Messages) -> Self {
        Self {
            state,
            user_id,
            messages,
            _state: PhantomData,
        }
    }

    /// Sends a toast with a customised title, body and action to the context's user.
    pub async fn custom(
        &self,
        title: &str,
        body: &str,
        status: Option<MessageStatus>,
        action: Option<&str>,
    ) {
        let mut toast = Toast::builder(MessageType::Toast, title, body).action(action);

        if let Some(status) = status {
            toast = toast.status(status);
        }

        if let Ok(json) = serde_json::to_string(&toast.build()) {
            self.state
                .channels
                .broadcast_to_client(&json, self.user_id)
                .await;
        }
    }

    /// Broadcasts an error toast with `message` to the context's user.
    pub async fn error(&self, message: &str) {
        Toast::broadcast_error(self.state, self.user_id, message, self.messages).await;
    }

    /// Broadcasts a success toast with `message` to the context's user.
    pub async fn success(&self, message: &str) {
        Toast::broadcast_success(self.state, self.user_id, message, self.messages).await;
    }

    /// Broadcasts a warning toast with `message` to the context's user.
    pub async fn warning(&self, message: &str) {
        Toast::broadcast_warning(self.state, self.user_id, message, self.messages).await;
    }
}

impl<'a> ToastCtx<'a, UnauthenticatedState> {
    /// Creates a toast context for
    pub const fn new(state: &'a AppState, messages: &'a Messages) -> Self {
        Self {
            state,
            user_id: Uuid::nil(),
            messages,
            _state: PhantomData,
        }
    }

    /// Adds an HTMX success message to an [`axum::Response`].
    pub fn success(&self, mut res: Response, message: &str) -> Response {
        add_hx_message(&mut res, &Toast::success(message, self.messages));
        res
    }

    /// Adds an HTMX error message to an [`axum::Response`].
    pub fn error(&self, mut res: Response, message: &str) -> Response {
        add_hx_message(&mut res, &Toast::error(message, self.messages));
        res
    }
}

/// Adds an HTMX message to a response's headers.
fn add_hx_message(res: &mut Response<Body>, message: &Toast) {
    if let Ok(toast) = serde_json::to_string(&message)
        && let Ok(value) = HeaderValue::from_str(&toast)
    {
        res.headers_mut().insert(axum_htmx::HX_TRIGGER, value);
    }
}

/// A trait defining message toasts broadcasting methods for structured responses.
pub(crate) trait Broadcaster {
    /// Broadcasts a success toast to all active SSE connections of a given user.
    fn broadcast_success(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) -> impl Future<Output = ()> + Send;

    /// Broadcasts a warning toast to all active SSE connections of a given user.
    fn broadcast_warning(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) -> impl Future<Output = ()> + Send;

    /// Broadcasts an error toast to all active SSE connections of a given user.
    fn broadcast_error(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) -> impl Future<Output = ()> + Send;
}

/// A trait defining message creation methods for structured responses.
pub trait IMessage {
    fn builder(message_type: MessageType, title: &str, message: &str) -> MessageBuilder;
    fn error(message: &str, messages: &Messages) -> Self;
    fn success(message: &str, messages: &Messages) -> Self;
    fn warning(message: &str, messages: &Messages) -> Self;
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
    fn builder(message_type: MessageType, title: &str, message: &str) -> MessageBuilder {
        MessageBuilder {
            title: title.into(),
            message: message.into(),
            message_type,
            ..Default::default()
        }
    }

    fn error(message: &str, messages: &Messages) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: messages.toast_title_error().to_string(),
                status: MessageStatus::Error,
                ..Default::default()
            },
        }
    }

    fn success(message: &str, messages: &Messages) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: messages.toast_title_success().to_string(),
                ..Default::default()
            },
        }
    }

    fn warning(message: &str, messages: &Messages) -> Self {
        Self {
            content: Content {
                message: message.into(),
                title: messages.toast_title_warning().to_string(),
                status: MessageStatus::Warning,
                ..Default::default()
            },
        }
    }
}

impl IMessage for Snack {
    fn builder(message_type: MessageType, title: &str, message: &str) -> MessageBuilder {
        MessageBuilder {
            message_type,
            title: title.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    fn error(message: &str, messages: &Messages) -> Self {
        Self {
            content: Content {
                r#type: MessageType::Snack,
                message: message.into(),
                title: messages.toast_title_error().to_string(),
                ..Default::default()
            },
        }
    }

    fn success(message: &str, messages: &Messages) -> Self {
        Self {
            content: Content {
                r#type: MessageType::Snack,
                message: message.into(),
                title: messages.toast_title_success().to_string(),
                ..Default::default()
            },
        }
    }

    fn warning(message: &str, messages: &Messages) -> Self {
        Self {
            content: Content {
                r#type: MessageType::Snack,
                message: message.into(),
                title: messages.toast_title_warning().to_string(),
                ..Default::default()
            },
        }
    }
}

impl Broadcaster for Toast {
    async fn broadcast_success(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) {
        let toast = Self::success(message, messages);
        if let Ok(json) = serde_json::to_string(&toast) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_warning(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) {
        let toast = Self::warning(message, messages);
        if let Ok(json) = serde_json::to_string(&toast) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_error(state: &AppState, user_id: Uuid, message: &str, messages: &Messages) {
        let toast = Self::error(message, messages);
        if let Ok(json) = serde_json::to_string(&toast) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }
}

impl Broadcaster for Snack {
    async fn broadcast_success(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) {
        let snack = Self::success(message, messages);
        if let Ok(json) = serde_json::to_string(&snack) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_warning(
        state: &AppState,
        user_id: Uuid,
        message: &str,
        messages: &Messages,
    ) {
        let snack = Self::warning(message, messages);
        if let Ok(json) = serde_json::to_string(&snack) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }

    async fn broadcast_error(state: &AppState, user_id: Uuid, message: &str, messages: &Messages) {
        let snack = Self::error(message, messages);
        if let Ok(json) = serde_json::to_string(&snack) {
            state.channels.broadcast_to_client(&json, user_id).await;
        }
    }
}
