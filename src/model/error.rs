use serde::Serialize;
use crate::crypt;
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    PoolConnection,
    EntityNotFound { entity: &'static str, id: i64 },
    Bind(String),

    Crypt(crypt::Error),
    Diesel(String),
    Pool(String),
    Store(store::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<crypt::Error> for Error {
    fn from(err: crypt::Error) -> Error {
        Error::Crypt(err)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Error {
        Error::Bind(err.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Error::Diesel(error.to_string())
    }
}

impl From<diesel_async::pooled_connection::PoolError> for Error {
    fn from(error: diesel_async::pooled_connection::PoolError) -> Self {
        Error::Pool(error.to_string())
    }
}

impl From<diesel_async::pooled_connection::bb8::RunError> for Error {
    fn from(_: diesel_async::pooled_connection::bb8::RunError) -> Self {
        Error::PoolConnection
    }
}

impl std::error::Error for Error {}

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}
