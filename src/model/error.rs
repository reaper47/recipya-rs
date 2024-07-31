use serde::Serialize;
use crate::crypt;
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },

    // Modules
    Crypt(crypt::Error),
    Store(store::Error),

    // Externals
    Diesel(String),
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

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Error::Diesel(error.to_string())
    }
}

impl From<diesel_async::pooled_connection::PoolError> for Error {
    fn from(error: diesel_async::pooled_connection::PoolError) -> Self {
        Error::Diesel(error.to_string())
    }
}

impl From<diesel_async::pooled_connection::bb8::RunError> for Error {
    fn from(value: diesel_async::pooled_connection::bb8::RunError) -> Self {
        Error::Diesel(value.to_string())
    }
}

impl std::error::Error for Error {}

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}
