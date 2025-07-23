use crate::cooking::units::{Unit, UnitType};

/// Result type for errors related to mathematics.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to mathematics.
#[derive(Debug)]
pub enum Error {
    InvalidMeasurementSystem(i16),
    InvalidScaleFactor(f64),
    InvalidScale,
    NotDetected,
    RegexFail,
    SystemMismatch,
    UnsupportedUnit(Unit, UnitType),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
