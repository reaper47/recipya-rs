use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{crypt, model, web};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    RpcMethodUnknown(String),
    RpcMissingParams { rpc_method: String },
    RpcFailJsonParams { rpc_method: String },

    LoginFailUserNotFound,
    LoginFailPasswordNotMatching { user_id: i64 },

    RegisterFail,
    UserExists,

    // Auth
    AuthFailCtxNotInRequestExt,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    CtxExt(web::middleware::ctx::CtxExtError),

    // Services
    RepositoryError(String),

    // Modules
    Crypt(crypt::Error),
    Model(model::Error),

    // External modules
    SerdeJson(String),
}

impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::SerdeJson(val.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, _client_error) = self.client_status_and_error();
        let mut res = status_code.into_response();
        res.extensions_mut().insert(self);
        res
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::error::Error::*;

        match self {
            LoginFailUserNotFound | LoginFailPasswordNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }
            RegisterFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::REGISTER_FAIL,
            ),
            UserExists => (StatusCode::CONFLICT, ClientError::SERVICE_ERROR),

            // Auth
            AuthFailNoAuthTokenCookie | AuthFailTokenWrongFormat | AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model
            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),

            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
    LOGIN_FAIL,
    REGISTER_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
}
