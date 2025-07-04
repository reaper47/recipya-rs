mod conversion;
mod operations;
mod scale;

pub use conversion::*;
pub use operations::*;
pub use scale::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    Millimetre(f64),
    Centimetre(f64),
    Metre(f64),
    Kilometre(f64),
    Inch(f64),
    Foot(f64),
}

#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Millimetre,
    Centimetre,
    Metre,
    Kilometre,
    Inch,
    Foot,
}
