use crate::cooking::units::{
    length::units::LengthUnit, mass::units::MassUnit, temperature::units::TemperatureUnit,
    volume::units::VolumeUnit,
};

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Length(LengthUnit),
    Mass(MassUnit),
    Temperature(TemperatureUnit),
    Unitless,
    Volume(VolumeUnit),
}
