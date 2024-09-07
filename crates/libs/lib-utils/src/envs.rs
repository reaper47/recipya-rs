use std::{env, str::FromStr};

use crate::b64::b64u_decode;

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name)
        .map(|v| v.trim_matches('"').to_string())
        .map_err(|_| Error::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    get_env(name)?
        .parse::<T>()
        .map_err(|_| Error::WrongFormat(name))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
