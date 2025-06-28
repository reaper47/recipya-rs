use crate::cooking::units::{LengthUnit, MassUnit, TemperatureUnit, VolumeUnit};

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Length(LengthUnit), 
    Mass(MassUnit),
    Temperature(TemperatureUnit),
    Volume(VolumeUnit),
}
