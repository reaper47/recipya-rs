use derive_more::From;

use lib_auth::pwd;
use serde::Serialize;

use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Serialize)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },

    // Modules
    #[from]
    Pwd(pwd::Error),
    #[from]
    Store(store::Error),

    // Externals
    Diesel(String),
    FailConnection(String),
}

impl From<diesel_async::pooled_connection::bb8::RunError> for Error {
    fn from(value: diesel_async::pooled_connection::bb8::RunError) -> Self {
        Error::FailConnection(value.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::Diesel(value.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
