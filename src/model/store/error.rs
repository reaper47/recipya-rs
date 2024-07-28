use deadpool_postgres::{tokio_postgres, PoolError};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    FailToCreatePool(String),

    FailToGetConnection(#[serde_as(as = "DisplayFromStr")] PoolError),
    QueryFail(#[serde_as(as = "DisplayFromStr")] tokio_postgres::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Error {
        Error::QueryFail(err)
    }
}

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Error {
        Error::FailToGetConnection(err)
    }
}

impl std::error::Error for Error {} // To be able to use the question mark operator (?)
