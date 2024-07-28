use crate::web;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    RegisterFail,
    UserExists,

    // Auth
    AuthFailCtxNotInRequestExt,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // Services
    RepositoryError(String),

    // CtxExtError
    CtxExt(web::middleware::ctx::CtxExtError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();
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
            LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            RegisterFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::REGISTER_FAIL,
            ),
            UserExists => (StatusCode::CONFLICT, ClientError::SERVICE_ERROR),

            // Login/Auth
            AuthFailNoAuthTokenCookie | AuthFailTokenWrongFormat | AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model

            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    REGISTER_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
}
