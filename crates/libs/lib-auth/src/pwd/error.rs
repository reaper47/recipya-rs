use derive_more::derive::From;
use serde::Serialize;

use super::scheme;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Serialize)]
pub enum Error {
    PwdWithSchemeFailedParse,

    FailSpawnBlockForHash,
    FailSpawnBlockForValidate,

    // Modules
    #[from]
    Scheme(scheme::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
