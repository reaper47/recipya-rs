use crate::Result;
use crate::cooking::units::{Unit, UnitType};

pub trait UnitOperations {
    fn abbrev(&self) -> &str;
    fn unit_type(&self) -> UnitType;
    fn value(&self) -> f64;
    fn with_value(&self, value: f64) -> Self;
}

pub trait UnitConverter {
    fn convert(&self, to: UnitType) -> Result<Unit>;
}

pub trait UnitScaler {
    fn scale(&self, factor: f64) -> Result<Unit>;
}
