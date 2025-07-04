mod conversion;
mod operations;
mod scale;

pub use conversion::*;
pub use operations::*;
pub use scale::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Volume {
    // Metric
    Millilitre(f64),
    Centilitre(f64),
    Decilitre(f64),
    Litre(f64),
    MetricTeaspoon(f64),
    MetricTablespoon(f64),
    MetricDessertspoon(f64),
    MetricCup(f64),

    // Australian
    AustralianTeaspoon(f64),
    AustralianDessertspoon(f64),
    AustralianTablespoon(f64),
    AustralianCup(f64),

    // Imperial
    ImperialTeaspoon(f64),
    ImperialDessertspoon(f64),
    ImperialTablespoon(f64),
    ImperialFluidOunce(f64),
    ImperialGill(f64),
    ImperialCup(f64),
    ImperialPint(f64),
    ImperialQuart(f64),
    ImperialGallon(f64),
    USTeaspoon(f64),
    USTablespoon(f64),
    USFluidOunce(f64),
    USCup(f64),
    USPint(f64),
    USQuart(f64),
    USGallon(f64),

    // Other
    Jigger(f64),
}

#[derive(Debug, PartialEq)]
pub enum VolumeUnit {
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
    AustralianTeaspoon,
    AustralianDessertspoon,
    AustralianTablespoon,
    AustralianCup,

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

    // US Customary
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
