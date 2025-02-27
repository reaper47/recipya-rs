use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[macro_export]
macro_rules! impl_display_as_debug {
    ($type:ty) => {
        impl core::fmt::Display for $type {
            fn fmt(
                &self,
                fmt: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                write!(fmt, "{self:?}")
            }
        }
    };
}

#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    Server(String),

    #[from]
    Io(std::io::Error),
    #[from]
    SetGlobalDefault(tracing::subscriber::SetGlobalDefaultError),
    #[from]
    Token(crate::core::auth::token::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
