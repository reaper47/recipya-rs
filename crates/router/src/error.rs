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
        use self::Error::{
            BadTimeFormat, ConfirmForbidden, ConfirmInvalidToken, DeleteForbidden, EntityExists,
            EntityNotFound, FailParse, FileExists, Form, Gone, InvalidClaims, InvalidPayload,
            InvalidQuery, LoginFailUsernameNotFound, LogoutFail, LogoutForbidden, Model, NoToken,
            NoUser, PwdNotMatching, UserNotAdmin,
        };

        match self {
            // Auth
            ConfirmForbidden => (StatusCode::FORBIDDEN, ClientError::ConfirmFail),
            ConfirmInvalidToken => (StatusCode::BAD_REQUEST, ClientError::ConfirmFail),
            LoginFailUsernameNotFound | PwdNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LoginFail)
            }
            LogoutFail => (StatusCode::BAD_REQUEST, ClientError::LogoutFail),
            LogoutForbidden => (StatusCode::FORBIDDEN, ClientError::LogoutFail),
            NoToken => (StatusCode::BAD_REQUEST, ClientError::MissingParams),

            BadTimeFormat => (StatusCode::BAD_REQUEST, ClientError::BadTimeFormat),
            DeleteForbidden => (StatusCode::FORBIDDEN, ClientError::DeleteForbidden),
            EntityExists { entity } => (
                StatusCode::CONFLICT,
                ClientError::EntityNotFound {
                    entity,
                    id: "-1".into(),
                },
            ),
            EntityNotFound { entity } => (
                StatusCode::NOT_FOUND,
                ClientError::EntityNotFound {
                    entity,
                    id: "-1".into(),
                },
            ),
            FailParse | InvalidPayload | InvalidQuery => {
                (StatusCode::BAD_REQUEST, ClientError::InvalidPayload)
            }
            FileExists => (
                StatusCode::CONFLICT,
                ClientError::EntityNotFound {
                    entity: "",
                    id: "-1".into(),
                },
            ),
            Form => (StatusCode::BAD_REQUEST, ClientError::FormError),
            Gone => (StatusCode::GONE, ClientError::Gone),
            InvalidClaims => (StatusCode::UNAUTHORIZED, ClientError::Unauthorized),
            NoUser => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::EntityNotFound {
                    entity: "user",
                    id: "-1".into(),
                },
            ),
            UserNotAdmin => (StatusCode::FORBIDDEN, ClientError::ForbiddenRequest),

            // Modules
            Model(models::Error::EntityNotFound { entity, id }) => (
                StatusCode::NOT_FOUND,
                ClientError::EntityNotFound {
                    entity,
                    id: id.clone(),
                },
            ),

            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServiceError),
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
    ConfirmFail,
    DeleteForbidden,
    EntityNotFound { entity: &'static str, id: String },
    BadTimeFormat,
    ForbiddenRequest,
    FormError,
    Gone,
    InvalidPayload,
    InvalidQuery,
    LoginFail,
    LogoutFail,
    MissingParams,
    ServiceError,
    Unauthorized,
}
