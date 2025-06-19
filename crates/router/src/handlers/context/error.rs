use serde::Serialize;
use support::impl_display_as_debug;

/// Result type for errors related to the context.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the context.
#[derive(Debug, Serialize)]
pub enum Error {
    CtxCannotBeNewRootCtx,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
