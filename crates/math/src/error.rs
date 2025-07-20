use crate::cooking::units::{Unit, UnitType};
use support::impl_display_as_debug;

/// Result type for errors related to mathematics.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to mathematics.
#[derive(Debug)]
pub enum Error {
    InvalidScaleFactor(f64),
    InvalidScale,
    NotDetected,
    RegexFail,
    SystemMismatch,
    UnsupportedUnit(Unit, UnitType),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
