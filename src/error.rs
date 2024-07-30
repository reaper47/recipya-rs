pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // Modules
    Model(crate::model::Error),
}

impl From<crate::model::Error> for Error {
    fn from(value: crate::model::Error) -> Self {
        Self::Model(value)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
