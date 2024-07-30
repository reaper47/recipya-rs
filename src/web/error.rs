use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{model, web};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
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
    Model(model::Error),
}

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
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

            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    REGISTER_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
}
