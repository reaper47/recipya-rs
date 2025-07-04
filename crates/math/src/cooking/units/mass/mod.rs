mod conversion;
mod operations;
mod scale;

pub use conversion::*;
pub use operations::*;
pub use scale::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Mass {
    Milligram(f64),
    Gram(f64),
    Dekagram(f64),
    Hectogram(f64),
    Kilogram(f64),
    Ounce(f64),
    Pound(f64),
}

#[derive(Debug, PartialEq)]
pub enum MassUnit {
    Milligram,
    Gram,
    Dekagram,
    Hectogram,
    Kilogram,
    Ounce,
    Pound,
}
