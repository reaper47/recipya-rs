use super::b64::b64u_decode;
use crate::impl_display_as_debug;
use std::{env, str::FromStr};

/// Retrieves the value of an environment variable as a `String`.
pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name)
        .map(|v| v.trim_matches('"').to_string())
        .map_err(|_| Error::MissingEnv(name))
}

/// Retrieves and parses the value of an environment variable into a specified type.
pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    get_env(name)?
        .parse::<T>()
        .map_err(|_| Error::WrongFormat(name))
}

/// Retrieves the value of an environment variable and decodes it from base64url into a byte vector.
pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

/// Result type for errors related to environment variables.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to environment variables.
#[derive(Debug)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
