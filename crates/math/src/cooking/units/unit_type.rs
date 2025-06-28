use crate::cooking::units::{LengthUnit, MassUnit};
use crate::cooking::units::temperature::TemperatureUnit;

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Length(LengthUnit), 
    Mass(MassUnit),
    Temperature(TemperatureUnit),
    
    // === VOLUME ===
    // Metric
    Millilitre,
    Centilitre,
    Decilitre,
    Litre,
    MetricTeaspoon,
    MetricTablespoon,
    MetricDessertSpoon,
    MetricCup,

    // Australian
    AustralianTablespoon,

    // Imperial
    ImperialTeaspoon,
    ImperialDessertspoon,
    ImperialTablespoon,
    ImperialFluidOunce,
    ImperialGill,
    ImperialCup,
    ImperialPint,
    ImperialQuart,
    ImperialGallon,
    USLegalCup,
    USTeaspoon,
    USTablespoon,
    USFluidOunce,
    USCup,
    USPint,
    USQuart,
    USGallon,

    // Other
    Jigger,
}
