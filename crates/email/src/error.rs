use derive_more::From;

use support::impl_display_as_debug;

/// Result type for errors related to emails.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to emailing.
#[derive(Debug, From)]
pub enum Error {
    EmailNotSetup,
    MissingConfig,
    SmtpNotInitialized,

    General(String),
    RenderFail,
    SendFail(String),

    #[from(lettre::error::Error)]
    BuildMessage,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
