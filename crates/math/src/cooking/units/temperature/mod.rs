mod conversion;
mod operations;
mod scale;

pub use conversion::*;
pub use operations::*;
pub use scale::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

#[derive(Debug, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}
