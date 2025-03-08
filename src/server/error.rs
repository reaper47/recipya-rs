use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::derive::From;
use serde::Serialize;
use validator::Validate;

use crate::impl_display_as_debug;

/// Result type for errors related to the server.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the server.
#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    ConfirmForbidden,
    ConfirmInvalidToken,
    GenerateToken,
    NoToken,

    Database,
    DeleteForbidden,
    Form,
    BadTimeFormat,
    NoUser,
    NoRecipe,

    DeleteUser,
    LoginFailUsernameNotFound,
    LogoutFail,
    LogoutForbidden,

    PwdNotMatching {
        user_id: i64,
    },
    UpdatePassword,

    // Modules
    #[from]
    CtxExt(crate::server::router::middleware::mw_auth::CtxExtError),
    #[from]
    Config(crate::core::config::Error),
    #[from]
    HumanTime(humantime::DurationError),
    #[from]
    Email(crate::core::email::Error),
    #[from]
    Model(crate::core::model::Error),
    #[from]
    Repository(crate::core::repository::Error),
    #[from]
    Token(crate::core::auth::token::Error),
    #[from]
    Var(std::env::VarError),
}

impl Error {
    /// Determines the HTTP status code and corresponding client error based on the current error state.
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use self::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            BadTimeFormat => (StatusCode::BAD_REQUEST, ClientError::BAD_TIME_FORMAT),
            DeleteForbidden => (StatusCode::FORBIDDEN, ClientError::DELETE_FORBIDDEN),
            Form => (StatusCode::BAD_REQUEST, ClientError::FORM_ERROR),
            NoUser => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::ENTITY_NOT_FOUND {
                    entity: "user",
                    id: -1,
                },
            ),

            LoginFailUsernameNotFound { .. } | PwdNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }
            LogoutFail => (StatusCode::BAD_REQUEST, ClientError::LOGOUT_FAIL),
            LogoutForbidden => (StatusCode::FORBIDDEN, ClientError::LOGOUT_FAIL),

            ConfirmForbidden => (StatusCode::FORBIDDEN, ClientError::CONFIRM_FAIL),
            ConfirmInvalidToken => (StatusCode::BAD_REQUEST, ClientError::CONFIRM_FAIL),
            NoToken => (StatusCode::BAD_REQUEST, ClientError::MISSING_PARAMS),

            Model(crate::core::model::Error::EntityNotFound { entity, id }) => (
                StatusCode::NOT_FOUND,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, _) = self.client_status_and_error();
        let mut response = status.into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

/// Enumeration of errors related to errors meant for clients.
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    CONFIRM_FAIL,
    DELETE_FORBIDDEN,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
    BAD_TIME_FORMAT,
    FORM_ERROR,
    LOGIN_FAIL,
    LOGOUT_FAIL,
    MISSING_PARAMS,
    SERVICE_ERROR,
}

/// Collects validation errors from a form and formats them as a vector of error messages.
pub fn collect_errors<T: Validate>(form: &T) -> Vec<String> {
    match form.validate() {
        Ok(_) => Vec::new(),
        Err(errors) => errors
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |err| {
                    format!(
                        "Field '{}': {}",
                        field,
                        err.message.as_deref().unwrap_or("Unknown error")
                    )
                })
            })
            .collect(),
    }
}
