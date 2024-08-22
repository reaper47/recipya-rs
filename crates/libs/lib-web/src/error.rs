use crate::middleware;
use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::From;
use serde::Serialize;
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};
use tracing::warn;
use validator::Validate;

use lib_auth::{pwd, token};
use lib_core::model;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: i64,
    },
    LoginFailPwdNotMatching {
        user_id: i64,
    },

    // Logout
    LogoutFail,
    LogoutForbidden,

    // Register
    RegisterFail,

    // Auth
    ConfirmInvalidToken,
    ConfirmNoToken,
    GenerateToken,
    UpdatePassword,
    ValidateToken,

    // General
    Form,

    // CtxExtError
    #[from]
    CtxExt(middleware::mw_auth::CtxExtError),

    // Modules
    #[from]
    Model(model::Error),
    #[from]
    Pwd(pwd::Error),
    #[from]
    Rpc(lib_rpc_core::Error),
    RpcHandlerErrorUnhandled(&'static str),
    RpcLibRpc(lib_rpc_core::Error),
    #[from]
    RpcRequestParsing(rpc_router::RequestParsingError),
    RpcRouter {
        id: Value,
        method: String,
        error: rpc_router::Error,
    },
    #[from]
    Token(token::Error),

    // External Modules
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, _) = self.client_status_and_error();
        let mut response = status.into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// The purpose of this `From` implementation is to extract the error types we recognize
/// from the `rpc_router`'s `RpcHandlerError` within the `rpc_router::Error::Handler`
/// and place them into the appropriate variant of our application error enum.
///
/// - The `rpc-router` provides an `RpcHandlerError` scheme to allow application RPC handlers
///   to return the errors they wish with minimal constraints.
/// - This approach requires us to "unpack" those types in our code and assign them to the correct
///   "concrete/direct" variant (not `Box<dyn Any>`...).
/// - If it's not an `rpc_router::Error::Handler` variant, then we can capture the `rpc_router::Error`
///   as it is, treating all other variants as "concrete/direct" types.
impl From<rpc_router::CallError> for Error {
    fn from(call_error: rpc_router::CallError) -> Self {
        let rpc_router::CallError { id, method, error } = call_error;
        match error {
            rpc_router::Error::Handler(mut rpc_handler_error) => {
                if let Some(lib_rpc_error) = rpc_handler_error.remove::<lib_rpc_core::Error>() {
                    Error::RpcLibRpc(lib_rpc_error)
                }
                // report the unhandled error for debugging and completing code.
                else {
                    let type_name = rpc_handler_error.type_name();
                    warn!("Unhandled RpcHandlerError type: {type_name}");
                    Error::RpcHandlerErrorUnhandled(type_name)
                }
            }
            error => Error::RpcRouter { id, method, error },
        }
    }
}

/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use self::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            // Login
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Logout
            LogoutFail => (StatusCode::BAD_REQUEST, ClientError::LOGOUT_FAIL),
            LogoutForbidden => (StatusCode::FORBIDDEN, ClientError::LOGOUT_FAIL),

            // Register
            RegisterFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::REGISTER_FAIL,
            ),

            // Auth
            ConfirmInvalidToken | ConfirmNoToken => {
                (StatusCode::BAD_REQUEST, ClientError::CONFIRM_FAIL)
            }
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // General
            Form => (StatusCode::BAD_REQUEST, ClientError::FORM_ERROR),

            // Model
            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),

            // -- Rpc
            RpcRequestParsing(req_parsing_err) => (
                StatusCode::BAD_REQUEST,
                ClientError::RPC_REQUEST_INVALID(req_parsing_err.to_string()),
            ),
            RpcRouter {
                error: rpc_router::Error::MethodUnknown,
                method,
                ..
            } => (
                StatusCode::BAD_REQUEST,
                ClientError::RPC_REQUEST_METHOD_UNKNOWN(format!("rpc method '{method}' unknown")),
            ),
            RpcRouter {
                error: rpc_router::Error::ParamsParsing(params_parsing_err),
                ..
            } => (
                StatusCode::BAD_REQUEST,
                ClientError::RPC_PARAMS_INVALID(params_parsing_err.to_string()),
            ),
            RpcRouter {
                error: rpc_router::Error::ParamsMissingButRequested,
                method,
                ..
            } => (
                StatusCode::BAD_REQUEST,
                ClientError::RPC_PARAMS_INVALID(format!(
                    "Params missing. Method '{method}' requires params"
                )),
            ),

            // Fallback.
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
    CONFIRM_FAIL,
    FORM_ERROR,
    LOGIN_FAIL,
    LOGOUT_FAIL,
    REGISTER_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

    RPC_REQUEST_INVALID(String),
    RPC_REQUEST_METHOD_UNKNOWN(String),
    RPC_PARAMS_INVALID(String),

    SERVICE_ERROR,
}

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
