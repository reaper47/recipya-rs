use crate::Result;
use crate::cooking::units::traits::UnitConverter;
use crate::cooking::units::{Unit, UnitType};

impl UnitConverter for Unit {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        match self {
            Unit::Length(unit) => unit.convert(to),
            Unit::Mass(unit) => unit.convert(to),
            Unit::Temperature(unit) => unit.convert(to),
            Unit::Volume(unit) => unit.convert(to),
        }
    }
}
