use std::sync::Arc;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    // Auth
    AuthFailCtxNotInRequestExt,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    HashError(argon2::password_hash::Error),
    LoginFail,
    RegisterFail,
    UserExists,

    // Services
    QueryFail(Arc<deadpool_postgres::tokio_postgres::Error>),
    PoolFail(Arc<deadpool::managed::PoolError<deadpool_postgres::tokio_postgres::Error>>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Error::HashError(value)
    }
}

impl From<deadpool::managed::PoolError<deadpool_postgres::tokio_postgres::Error>> for Error {
    fn from(value: deadpool::managed::PoolError<deadpool_postgres::tokio_postgres::Error>) -> Self {
        Error::PoolFail(Arc::new(value))
    }
}

impl From<deadpool_postgres::tokio_postgres::Error> for Error {
    fn from(value: deadpool_postgres::tokio_postgres::Error) -> Self {
        Error::QueryFail(Arc::new(value))
    }
}
