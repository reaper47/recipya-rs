use crate::impl_display_as_debug;
use derive_more::From;

/// Result type for errors related to emails.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to emailing.
#[derive(Debug, From)]
pub enum Error {
    EmailNotSetup,
    MissingConfig,
    SendGridNotInitialized,
    SmtpNotInitialized,

    General(String),
    RenderFail,
    SendFail(String),

    #[from]
    BuildMessage(lettre::error::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
