use crate::cooking::units::LengthType;

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Length(LengthType),

    // Mass
    Milligram,
    Gram,
    Dekagram,
    Hectogram,
    Kilogram,
    Ounce,
    Pound,

    // Temperature
    Celsius,
    Fahrenheit,

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
