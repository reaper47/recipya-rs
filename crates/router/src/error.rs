use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::derive::From;
use serde::Serialize;
use uuid::Uuid;

use support::impl_display_as_debug;

/// Result type for errors related to the server.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the server.
#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    // Auth
    ConfirmForbidden,
    ConfirmInvalidToken,
    DeleteUser,
    GenerateToken,
    Gone,
    InvalidClaims,
    LoginFailUsernameNotFound,
    LogoutFail,
    LogoutForbidden,
    NoToken,
    PwdNotMatching {
        user_id: Uuid,
    },
    UpdatePassword,
    UserNotAdmin,

    // Files
    AssetCouldNotCopy,
    FileExists,
    Fs,
    Templates,

    BadTimeFormat,
    Database,
    DeleteForbidden,
    EntityExists {
        entity: &'static str,
    },
    EntityNotFound {
        entity: &'static str,
    },
    FailFetch,
    FailParse,
    Form,
    InvalidPayload,
    InvalidQuery,
    NoUser,
    NoRecipe,

    // Modules
    #[from]
    Config(config::Error),
    #[from]
    HumanTime(humantime::DurationError),
    #[from]
    Email(email::Error),
    #[from]
    Integration(integrations::Error),
    #[from]
    Io(std::io::Error),
    #[from]
    Model(models::Error),
    #[from]
    Repository(repository::Error),
    #[from]
    Scraper(recipya_scraper::Error),
    #[from]
    Token(auth::token::Error),
    #[from]
    Var(std::env::VarError),
}

impl Error {
    /// Determines the HTTP status code and corresponding client error based on the current error state.
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use self::Error::*;

        match self {
            // Auth
            ConfirmForbidden => (StatusCode::FORBIDDEN, ClientError::CONFIRM_FAIL),
            ConfirmInvalidToken => (StatusCode::BAD_REQUEST, ClientError::CONFIRM_FAIL),
            LoginFailUsernameNotFound | PwdNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }
            LogoutFail => (StatusCode::BAD_REQUEST, ClientError::LOGOUT_FAIL),
            LogoutForbidden => (StatusCode::FORBIDDEN, ClientError::LOGOUT_FAIL),
            NoToken => (StatusCode::BAD_REQUEST, ClientError::MISSING_PARAMS),

            BadTimeFormat => (StatusCode::BAD_REQUEST, ClientError::BAD_TIME_FORMAT),
            DeleteForbidden => (StatusCode::FORBIDDEN, ClientError::DELETE_FORBIDDEN),
            EntityExists { entity } => (
                StatusCode::CONFLICT,
                ClientError::ENTITY_NOT_FOUND {
                    entity,
                    id: "-1".into(),
                },
            ),
            EntityNotFound { entity } => (
                StatusCode::NOT_FOUND,
                ClientError::ENTITY_NOT_FOUND {
                    entity,
                    id: "-1".into(),
                },
            ),
            FailParse => (StatusCode::BAD_REQUEST, ClientError::INVALID_PAYLOAD),
            FileExists => (
                StatusCode::CONFLICT,
                ClientError::ENTITY_NOT_FOUND {
                    entity: "",
                    id: "-1".into(),
                },
            ),
            Form => (StatusCode::BAD_REQUEST, ClientError::FORM_ERROR),
            Gone => (StatusCode::GONE, ClientError::GONE),
            InvalidClaims => (StatusCode::UNAUTHORIZED, ClientError::UNAUTHORIZED),
            InvalidPayload => (StatusCode::BAD_REQUEST, ClientError::INVALID_PAYLOAD),
            InvalidQuery => (StatusCode::BAD_REQUEST, ClientError::INVALID_PAYLOAD),
            NoUser => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::ENTITY_NOT_FOUND {
                    entity: "user",
                    id: "-1".into(),
                },
            ),
            UserNotAdmin => (StatusCode::FORBIDDEN, ClientError::FORBIDDEN_REQUEST),

            // Modules
            Model(models::Error::EntityNotFound { entity, id }) => (
                StatusCode::NOT_FOUND,
                ClientError::ENTITY_NOT_FOUND {
                    entity,
                    id: id.to_string(),
                },
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
    ENTITY_NOT_FOUND { entity: &'static str, id: String },
    BAD_TIME_FORMAT,
    FORBIDDEN_REQUEST,
    FORM_ERROR,
    GONE,
    INVALID_PAYLOAD,
    INVALID_QUERY,
    LOGIN_FAIL,
    LOGOUT_FAIL,
    MISSING_PARAMS,
    SERVICE_ERROR,
    UNAUTHORIZED,
}
