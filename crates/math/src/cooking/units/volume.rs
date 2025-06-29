use crate::Error;
use crate::cooking::units::custom::{
    VolumeAustralianTablespoonExt, VolumeCentilitreExt, VolumeDecilitreExt, VolumeImperialCupExt,
    VolumeImperialDessertSpoonExt, VolumeImperialGillExt, VolumeImperialQuartExt,
    VolumeImperialTablespoonExt, VolumeImperialTeaspoonExt, VolumeJiggerExt, VolumeMetricCupExt,
    VolumeMetricDessertSpoonExt, VolumeMetricTablespoonExt, VolumeMetricTeaspoonExt,
    VolumeUSLegalCupExt,
};
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::{Unit, UnitType};

#[derive(Debug, Clone, PartialEq)]
pub enum Volume {
    // Metric
    Millilitre(f64),
    Centilitre(f64),
    Decilitre(f64),
    Litre(f64),
    MetricTeaspoon(f64),
    MetricTablespoon(f64),
    MetricDessertSpoon(f64),
    MetricCup(f64),

    // Australian
    AustralianTablespoon(f64),

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
    USLegalCup(f64),
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

impl UnitOperations for Volume {
    fn unit_type(&self) -> UnitType {
        use VolumeUnit::*;

        match self {
            Volume::Millilitre(_) => UnitType::Volume(Millilitre),
            Volume::Centilitre(_) => UnitType::Volume(Centilitre),
            Volume::Decilitre(_) => UnitType::Volume(Decilitre),
            Volume::Litre(_) => UnitType::Volume(Litre),
            Volume::MetricTeaspoon(_) => UnitType::Volume(MetricTeaspoon),
            Volume::MetricTablespoon(_) => UnitType::Volume(MetricTablespoon),
            Volume::MetricDessertSpoon(_) => UnitType::Volume(MetricDessertSpoon),
            Volume::MetricCup(_) => UnitType::Volume(MetricCup),
            Volume::AustralianTablespoon(_) => UnitType::Volume(AustralianTablespoon),
            Volume::ImperialTeaspoon(_) => UnitType::Volume(ImperialTeaspoon),
            Volume::ImperialDessertspoon(_) => UnitType::Volume(ImperialDessertspoon),
            Volume::ImperialTablespoon(_) => UnitType::Volume(ImperialTablespoon),
            Volume::ImperialFluidOunce(_) => UnitType::Volume(ImperialFluidOunce),
            Volume::ImperialGill(_) => UnitType::Volume(ImperialGill),
            Volume::ImperialCup(_) => UnitType::Volume(ImperialCup),
            Volume::ImperialPint(_) => UnitType::Volume(ImperialPint),
            Volume::ImperialQuart(_) => UnitType::Volume(ImperialQuart),
            Volume::ImperialGallon(_) => UnitType::Volume(ImperialGallon),
            Volume::USLegalCup(_) => UnitType::Volume(USLegalCup),
            Volume::USTeaspoon(_) => UnitType::Volume(USTeaspoon),
            Volume::USTablespoon(_) => UnitType::Volume(USTablespoon),
            Volume::USFluidOunce(_) => UnitType::Volume(USFluidOunce),
            Volume::USCup(_) => UnitType::Volume(USCup),
            Volume::USPint(_) => UnitType::Volume(USPint),
            Volume::USQuart(_) => UnitType::Volume(USQuart),
            Volume::USGallon(_) => UnitType::Volume(USGallon),
            Volume::Jigger(_) => UnitType::Volume(Jigger),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Volume::Millilitre(v) => *v,
            Volume::Centilitre(v) => *v,
            Volume::Decilitre(v) => *v,
            Volume::Litre(v) => *v,
            Volume::MetricTeaspoon(v) => *v,
            Volume::MetricTablespoon(v) => *v,
            Volume::MetricDessertSpoon(v) => *v,
            Volume::MetricCup(v) => *v,
            Volume::AustralianTablespoon(v) => *v,
            Volume::ImperialTeaspoon(v) => *v,
            Volume::ImperialDessertspoon(v) => *v,
            Volume::ImperialTablespoon(v) => *v,
            Volume::ImperialFluidOunce(v) => *v,
            Volume::ImperialGill(v) => *v,
            Volume::ImperialCup(v) => *v,
            Volume::ImperialPint(v) => *v,
            Volume::ImperialQuart(v) => *v,
            Volume::ImperialGallon(v) => *v,
            Volume::USLegalCup(v) => *v,
            Volume::USTeaspoon(v) => *v,
            Volume::USTablespoon(v) => *v,
            Volume::USFluidOunce(v) => *v,
            Volume::USCup(v) => *v,
            Volume::USPint(v) => *v,
            Volume::USQuart(v) => *v,
            Volume::USGallon(v) => *v,
            Volume::Jigger(v) => *v,
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Volume::Millilitre(_) => Volume::Millilitre(value),
            Volume::Centilitre(_) => Volume::Centilitre(value),
            Volume::Decilitre(_) => Volume::Decilitre(value),
            Volume::Litre(_) => Volume::Litre(value),
            Volume::MetricTeaspoon(_) => Volume::MetricTeaspoon(value),
            Volume::MetricTablespoon(_) => Volume::MetricTablespoon(value),
            Volume::MetricDessertSpoon(_) => Volume::MetricDessertSpoon(value),
            Volume::MetricCup(_) => Volume::MetricCup(value),
            Volume::AustralianTablespoon(_) => Volume::AustralianTablespoon(value),
            Volume::ImperialTeaspoon(_) => Volume::ImperialTeaspoon(value),
            Volume::ImperialDessertspoon(_) => Volume::ImperialDessertspoon(value),
            Volume::ImperialTablespoon(_) => Volume::ImperialTablespoon(value),
            Volume::ImperialFluidOunce(_) => Volume::ImperialFluidOunce(value),
            Volume::ImperialGill(_) => Volume::ImperialGill(value),
            Volume::ImperialCup(_) => Volume::ImperialCup(value),
            Volume::ImperialPint(_) => Volume::ImperialPint(value),
            Volume::ImperialQuart(_) => Volume::ImperialQuart(value),
            Volume::ImperialGallon(_) => Volume::ImperialGallon(value),
            Volume::USLegalCup(_) => Volume::USLegalCup(value),
            Volume::USTeaspoon(_) => Volume::USTeaspoon(value),
            Volume::USTablespoon(_) => Volume::USTablespoon(value),
            Volume::USFluidOunce(_) => Volume::USFluidOunce(value),
            Volume::USCup(_) => Volume::USCup(value),
            Volume::USPint(_) => Volume::USPint(value),
            Volume::USQuart(_) => Volume::USQuart(value),
            Volume::USGallon(_) => Volume::USGallon(value),
            Volume::Jigger(_) => Volume::Jigger(value),
        }
    }
}

impl UnitConverter for Volume {
    fn convert(&self, to: UnitType) -> crate::Result<Unit> {
        use VolumeUnit::*;

        match self {
            Volume::Millilitre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_millilitres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(self.with_value(*original_value))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Centilitre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_centilitres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(self.with_value(*original_value))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Decilitre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_decilitres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(self.with_value(*original_value))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Litre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_litres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricDessertSpoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_dessertspoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::AustralianTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons_aus(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialDessertspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_dessertspoons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialFluidOunce(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_fluid_ounces_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialGill(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_gills_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialPint(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_pints_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialQuart(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_quarts_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialGallon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_gallons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => Ok(Unit::Volume(self.with_value(*original_value))),
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USLegalCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups_legal(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USFluidOunce(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_fluid_ounces(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => Ok(Unit::Volume(self.with_value(*original_value))),
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USPint(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_pints(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(self.with_value(*original_value))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USQuart(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_quarts(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(self.with_value(*original_value))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USGallon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_gallons(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(self.with_value(*original_value))),
                        Jigger => Ok(Unit::Volume(Volume::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Jigger(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_jiggers(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Volume::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Volume::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Volume::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Volume::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Volume::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Volume::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Volume::MetricDessertSpoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Volume::MetricCup(value.as_cups_metric()))),
                        AustralianTablespoon => Ok(Unit::Volume(Volume::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        ImperialTeaspoon => Ok(Unit::Volume(Volume::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Volume::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Volume::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Volume::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Volume::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Volume::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Volume::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Volume::ImperialGallon(value.as_gallons_uk())))
                        }
                        USLegalCup => Ok(Unit::Volume(Volume::USLegalCup(value.as_cups_legal()))),
                        USTeaspoon => Ok(Unit::Volume(Volume::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Volume::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Volume::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Volume::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Volume::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Volume::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Volume::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(self.with_value(*original_value))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_unit_operations {
        use super::*;

        fn create_test_volumes() -> Vec<Volume> {
            vec![
                // Metric
                Volume::Millilitre(100.0),
                Volume::Centilitre(10.0),
                Volume::Decilitre(1.0),
                Volume::Litre(0.1),
                Volume::MetricTeaspoon(20.0),
                Volume::MetricTablespoon(6.67),
                Volume::MetricDessertSpoon(10.0),
                Volume::MetricCup(0.4),
                // Australian
                Volume::AustralianTablespoon(5.0),
                // Imperial
                Volume::ImperialTeaspoon(16.91),
                Volume::ImperialDessertspoon(8.45),
                Volume::ImperialTablespoon(5.63),
                Volume::ImperialFluidOunce(3.52),
                Volume::ImperialGill(0.88),
                Volume::ImperialCup(0.44),
                Volume::ImperialPint(0.18),
                Volume::ImperialQuart(0.09),
                Volume::ImperialGallon(0.022),
                // US
                Volume::USLegalCup(0.42),
                Volume::USTeaspoon(20.29),
                Volume::USTablespoon(6.76),
                Volume::USFluidOunce(3.38),
                Volume::USCup(0.42),
                Volume::USPint(0.21),
                Volume::USQuart(0.11),
                Volume::USGallon(0.026),
                // Other
                Volume::Jigger(2.25),
            ]
        }

        #[test]
        fn test_volume_construction_and_value() {
            let vol = Volume::Millilitre(250.0);
            assert_eq!(vol.value(), 250.0);

            let vol2 = Volume::USCup(1.5);
            assert_eq!(vol2.value(), 1.5);

            let vol3 = Volume::ImperialPint(0.5);
            assert_eq!(vol3.value(), 0.5);
        }

        #[test]
        fn test_value_method_all_types() {
            let test_value = 42.0;
            let volumes = vec![
                Volume::Millilitre(test_value),
                Volume::Centilitre(test_value),
                Volume::Decilitre(test_value),
                Volume::Litre(test_value),
                Volume::MetricTeaspoon(test_value),
                Volume::MetricTablespoon(test_value),
                Volume::MetricDessertSpoon(test_value),
                Volume::MetricCup(test_value),
                Volume::AustralianTablespoon(test_value),
                Volume::ImperialTeaspoon(test_value),
                Volume::ImperialDessertspoon(test_value),
                Volume::ImperialTablespoon(test_value),
                Volume::ImperialFluidOunce(test_value),
                Volume::ImperialGill(test_value),
                Volume::ImperialCup(test_value),
                Volume::ImperialPint(test_value),
                Volume::ImperialQuart(test_value),
                Volume::ImperialGallon(test_value),
                Volume::USLegalCup(test_value),
                Volume::USTeaspoon(test_value),
                Volume::USTablespoon(test_value),
                Volume::USFluidOunce(test_value),
                Volume::USCup(test_value),
                Volume::USPint(test_value),
                Volume::USQuart(test_value),
                Volume::USGallon(test_value),
                Volume::Jigger(test_value),
            ];

            for volume in volumes {
                assert_eq!(volume.value(), test_value);
            }
        }

        #[test]
        fn test_with_value_method_all_types() {
            let original_value = 10.0;
            let new_value = 25.0;

            let volumes = vec![
                Volume::Millilitre(original_value),
                Volume::Centilitre(original_value),
                Volume::Decilitre(original_value),
                Volume::Litre(original_value),
                Volume::MetricTeaspoon(original_value),
                Volume::MetricTablespoon(original_value),
                Volume::MetricDessertSpoon(original_value),
                Volume::MetricCup(original_value),
                Volume::AustralianTablespoon(original_value),
                Volume::ImperialTeaspoon(original_value),
                Volume::ImperialDessertspoon(original_value),
                Volume::ImperialTablespoon(original_value),
                Volume::ImperialFluidOunce(original_value),
                Volume::ImperialGill(original_value),
                Volume::ImperialCup(original_value),
                Volume::ImperialPint(original_value),
                Volume::ImperialQuart(original_value),
                Volume::ImperialGallon(original_value),
                Volume::USLegalCup(original_value),
                Volume::USTeaspoon(original_value),
                Volume::USTablespoon(original_value),
                Volume::USFluidOunce(original_value),
                Volume::USCup(original_value),
                Volume::USPint(original_value),
                Volume::USQuart(original_value),
                Volume::USGallon(original_value),
                Volume::Jigger(original_value),
            ];

            for volume in volumes {
                let new_volume = volume.with_value(new_value);
                assert_eq!(new_volume.value(), new_value);
                assert_eq!(volume.value(), original_value);
            }
        }

        #[test]
        fn test_unit_type_method() {
            assert_eq!(
                Volume::Millilitre(100.0).unit_type(),
                UnitType::Volume(VolumeUnit::Millilitre)
            );

            assert_eq!(
                Volume::USCup(2.0).unit_type(),
                UnitType::Volume(VolumeUnit::USCup)
            );

            assert_eq!(
                Volume::ImperialGallon(1.0).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialGallon)
            );

            assert_eq!(
                Volume::Jigger(3.0).unit_type(),
                UnitType::Volume(VolumeUnit::Jigger)
            );
        }

        #[test]
        fn test_unit_type_all_variants() {
            let test_cases = vec![
                (Volume::Millilitre(1.0), VolumeUnit::Millilitre),
                (Volume::Centilitre(1.0), VolumeUnit::Centilitre),
                (Volume::Decilitre(1.0), VolumeUnit::Decilitre),
                (Volume::Litre(1.0), VolumeUnit::Litre),
                (Volume::MetricTeaspoon(1.0), VolumeUnit::MetricTeaspoon),
                (Volume::MetricTablespoon(1.0), VolumeUnit::MetricTablespoon),
                (
                    Volume::MetricDessertSpoon(1.0),
                    VolumeUnit::MetricDessertSpoon,
                ),
                (Volume::MetricCup(1.0), VolumeUnit::MetricCup),
                (
                    Volume::AustralianTablespoon(1.0),
                    VolumeUnit::AustralianTablespoon,
                ),
                (Volume::ImperialTeaspoon(1.0), VolumeUnit::ImperialTeaspoon),
                (
                    Volume::ImperialDessertspoon(1.0),
                    VolumeUnit::ImperialDessertspoon,
                ),
                (
                    Volume::ImperialTablespoon(1.0),
                    VolumeUnit::ImperialTablespoon,
                ),
                (
                    Volume::ImperialFluidOunce(1.0),
                    VolumeUnit::ImperialFluidOunce,
                ),
                (Volume::ImperialGill(1.0), VolumeUnit::ImperialGill),
                (Volume::ImperialCup(1.0), VolumeUnit::ImperialCup),
                (Volume::ImperialPint(1.0), VolumeUnit::ImperialPint),
                (Volume::ImperialQuart(1.0), VolumeUnit::ImperialQuart),
                (Volume::ImperialGallon(1.0), VolumeUnit::ImperialGallon),
                (Volume::USLegalCup(1.0), VolumeUnit::USLegalCup),
                (Volume::USTeaspoon(1.0), VolumeUnit::USTeaspoon),
                (Volume::USTablespoon(1.0), VolumeUnit::USTablespoon),
                (Volume::USFluidOunce(1.0), VolumeUnit::USFluidOunce),
                (Volume::USCup(1.0), VolumeUnit::USCup),
                (Volume::USPint(1.0), VolumeUnit::USPint),
                (Volume::USQuart(1.0), VolumeUnit::USQuart),
                (Volume::USGallon(1.0), VolumeUnit::USGallon),
                (Volume::Jigger(1.0), VolumeUnit::Jigger),
            ];

            for (volume, expected_unit) in test_cases {
                assert_eq!(volume.unit_type(), UnitType::Volume(expected_unit));
            }
        }

        #[test]
        fn test_clone() {
            let original = Volume::Litre(2.5);
            let cloned = original.clone();

            assert_eq!(original, cloned);
            assert_eq!(original.value(), cloned.value());

            let modified = cloned.with_value(5.0);
            assert_eq!(original.value(), 2.5);
            assert_eq!(modified.value(), 5.0);
        }

        #[test]
        fn test_partial_eq() {
            let vol1 = Volume::Millilitre(500.0);
            let vol2 = Volume::Millilitre(500.0);
            let vol3 = Volume::Millilitre(250.0);
            let vol4 = Volume::Litre(0.5);

            assert_eq!(vol1, vol2);
            assert_ne!(vol1, vol3);
            assert_ne!(vol1, vol4);
        }

        #[test]
        fn test_debug_output() {
            let vol = Volume::USCup(1.5);
            let debug_str = format!("{:?}", vol);
            assert!(debug_str.contains("USCup"));
            assert!(debug_str.contains("1.5"));
        }

        #[test]
        fn test_special_float_values() {
            let zero_vol = Volume::Litre(0.0);
            assert_eq!(zero_vol.value(), 0.0);

            let negative_vol = Volume::Millilitre(-100.0);
            assert_eq!(negative_vol.value(), -100.0);

            let large_vol = Volume::Millilitre(f64::MAX / 2.0);
            assert_eq!(large_vol.value(), f64::MAX / 2.0);

            let small_vol = Volume::Millilitre(f64::MIN_POSITIVE);
            assert_eq!(small_vol.value(), f64::MIN_POSITIVE);
        }

        #[test]
        fn test_with_value_preserves_unit_type() {
            let volumes = create_test_volumes();

            for volume in volumes {
                let original_unit_type = volume.unit_type();
                let new_volume = volume.with_value(999.0);

                assert_eq!(new_volume.unit_type(), original_unit_type);
                assert_eq!(new_volume.value(), 999.0);
            }
        }

        #[test]
        fn test_volume_unit_consistency() {
            let volumes = create_test_volumes();

            for volume in volumes {
                match volume.unit_type() {
                    UnitType::Volume(_) => {
                        assert!(true);
                    }
                    _ => {
                        panic!("Volume should always return UnitType::Volume");
                    }
                }
            }
        }

        #[test]
        fn test_volume_unit_enum_completeness() {
            let _units = [
                VolumeUnit::Millilitre,
                VolumeUnit::Centilitre,
                VolumeUnit::Decilitre,
                VolumeUnit::Litre,
                VolumeUnit::MetricTeaspoon,
                VolumeUnit::MetricTablespoon,
                VolumeUnit::MetricDessertSpoon,
                VolumeUnit::MetricCup,
                VolumeUnit::AustralianTablespoon,
                VolumeUnit::ImperialTeaspoon,
                VolumeUnit::ImperialDessertspoon,
                VolumeUnit::ImperialTablespoon,
                VolumeUnit::ImperialFluidOunce,
                VolumeUnit::ImperialGill,
                VolumeUnit::ImperialCup,
                VolumeUnit::ImperialPint,
                VolumeUnit::ImperialQuart,
                VolumeUnit::ImperialGallon,
                VolumeUnit::USLegalCup,
                VolumeUnit::USTeaspoon,
                VolumeUnit::USTablespoon,
                VolumeUnit::USFluidOunce,
                VolumeUnit::USCup,
                VolumeUnit::USPint,
                VolumeUnit::USQuart,
                VolumeUnit::USGallon,
                VolumeUnit::Jigger,
            ];

            assert_eq!(_units.len(), 27);
        }

        // Test VolumeUnit Debug and PartialEq
        #[test]
        fn test_volume_unit_traits() {
            let unit1 = VolumeUnit::Millilitre;
            let unit2 = VolumeUnit::Millilitre;
            let unit3 = VolumeUnit::Litre;

            assert_eq!(unit1, unit2);
            assert_ne!(unit1, unit3);

            let debug_str = format!("{:?}", unit1);
            assert!(debug_str.contains("Millilitre"));
        }

        #[test]
        fn test_unit_operations_integration() {
            let vol = Volume::USFluidOunce(8.0);

            let unit_type = vol.unit_type();
            let value = vol.value();
            let new_vol = vol.with_value(value * 2.0);

            assert_eq!(unit_type, UnitType::Volume(VolumeUnit::USFluidOunce));
            assert_eq!(value, 8.0);
            assert_eq!(new_vol.value(), 16.0);
            assert_eq!(new_vol.unit_type(), unit_type);
        }

        #[test]
        fn test_mathematical_operations() {
            let vol1 = Volume::Millilitre(100.0);
            let vol2 = vol1.with_value(vol1.value() * 2.0);
            let vol3 = vol1.with_value(vol1.value() + 50.0);
            let vol4 = vol1.with_value(vol1.value() / 4.0);

            assert_eq!(vol2.value(), 200.0);
            assert_eq!(vol3.value(), 150.0);
            assert_eq!(vol4.value(), 25.0);
            assert_eq!(vol1.unit_type(), vol2.unit_type());
            assert_eq!(vol1.unit_type(), vol3.unit_type());
            assert_eq!(vol1.unit_type(), vol4.unit_type());
        }

        #[test]
        fn test_floating_point_precision() {
            let vol = Volume::Litre(1.0);
            let vol_third = vol.with_value(vol.value() / 3.0);
            let vol_reconstructed = vol_third.with_value(vol_third.value() * 3.0);

            assert!((vol_reconstructed.value() - 1.0).abs() < f64::EPSILON * 10.0);
        }
    }

    mod tests_conversions {
        use super::*;
        use crate::cooking::units::VolumeUnit::*;

        fn assert_approx_eq(actual: Unit, expected: Unit, threshold: f64) {
            let actual_value = actual.value();
            let expected_value = expected.value();

            assert!(
                (actual_value - expected_value).abs() < threshold,
                "Expected {actual:?}, got {expected:?}, difference: {}",
                (actual_value - expected_value).abs()
            );
        }

        #[test]
        fn test_millilitre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::Millilitre(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(150.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(150.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(200.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(2.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(0.15)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(1.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(1.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(1.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(1.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(2.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(15.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(2.534)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(0.84468)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(2.112676)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(1.5838)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(2.81561)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(1.12624)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(0.879877)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(0.783091)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(0.329954)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(1.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(1.014421)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(1.014421)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(1.2)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(1.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(1.0)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(1.0)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(1.0)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Millilitre(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(1.26239)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_centilitre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::Centilitre(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(1500.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(1.5)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(20.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(8.446816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(8.446821)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(21.12676)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(15.83777)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(28.156064)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(11.262426)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(8.79876993)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(7.8309087)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(3.2995387)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(10.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(10.144206)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(10.14420681)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(12.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(10.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(10.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(10.0)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(10.0)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Centilitre(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(12.623901821)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_decilitre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::Decilitre(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(15000.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(150.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(150.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(200.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(84.46816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(84.46821)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(211.2676)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(158.3777)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(281.56064)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(112.62426504)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(87.9876993)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(78.309087)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(32.995387)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(100.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(101.44206)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(101.4420681)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(120.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(100.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(100.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(100.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(100.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Decilitre(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(126.23901821)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_litre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(1000.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(1.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(200.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.5)).convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(100.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(4.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.2)).convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(60.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(844.6816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(844.6821)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(2112.676056)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(1583.777898)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(2815.6063782)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(1126.2426504)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(879.876993)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(783.09087)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(329.95387)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.2)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(5.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(1.2)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::Litre(243.4609)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(1014.420681)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(1199.9988)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(1000.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(1000.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(1.05668)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(0.264172)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(22.54268)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_metric_tsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(1.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(5.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(1.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(0.5)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(29.05)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(1.4525)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(0.75)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(5.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(5.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(5.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(5.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(10.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(4.2234081552)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(4.2234081552)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(10.56338)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(7.91888)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(14.078031)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(5.6312132)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(4.399384)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(3.9154543)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(1.649769)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(5.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(5.07210)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(5.072103)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(5.999994)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(4.999999)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(5.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(5.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(5.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTeaspoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(6.31195)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_metric_tbsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(2250.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(22.5)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(1.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(0.15)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(2.25)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(30.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(12.6702244)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(12.6702244)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(15.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(23.756668)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(42.234095)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(16.89363)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(13.198154)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(11.746363026848577)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(4.9493080867275)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(15.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(15.216310215)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(15.216310215)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(17.9999820057321)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(14.999997686809676)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(15.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(15.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(15.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricTablespoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(18.935852731707417)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_metric_dessert_spoon_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(1500.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(150.0))
                    .convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(15.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(1.5)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(10.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(20.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(8.446816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(8.446816)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(21.126760563380284)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(15.837778908)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(28.156063)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(11.262426)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(8.79876993195)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(7.83090868)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(3.299538)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertSpoon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(10.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(10.14420681)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(10.14420681)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(11.999988)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(9.9999)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(10.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(10.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(10.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricDessertSpoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(12.6239018)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_metric_cup_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::MetricCup(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(37500.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(375.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(375.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(37.5)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(250.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(250.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(250.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(250.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(500.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(211.1704077)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(211.1705266)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(528.169014)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(395.9444)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(703.9015945)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(281.560662)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(219.969248)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(195.77271)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(82.48846)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(250.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(253.605170)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(253.605170)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(299.99970)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(249.999961)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(250.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(250.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(250.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::MetricCup(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(315.597545)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_australian_tbsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(3000.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(30.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(150.0))
                    .convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(30.0)),
                1e-10,
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(150.0))
                    .convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(3.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(20.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(20.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(20.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(20.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(4.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(4.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(16.89363)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(16.89363)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(42.253521)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(31.675557)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(56.3121275)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(22.524853)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(17.59753)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(15.661817)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(6.5990774)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(20.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(20.288413)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(20.288413)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(23.999976)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(19.99999)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(20.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(20.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(20.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::AustralianTablespoon(56.0))
                    .convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(25.247803)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_tsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(887.9085)),
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(8.879085)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(150.0))
                    .convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(8.879085)),
                1e-10,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(0.8879085)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(5.91939)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(5.91939)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(5.91939)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(5.91939)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(11.83878)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(5.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(5.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(12.5057535)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(9.374999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(16.666672)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(6.66666)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(5.208335)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(4.6354202)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(1.953125)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(28.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(0.6905955)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(6.0047516)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(6.0047516)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(7.103260)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(5.919389)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(5.91939)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(5.91939)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(5.91939)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTeaspoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(7.472579)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_dessert_spoon_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(1065.0)),
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(10.65)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(150.0))
                    .convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(10.65)),
                1e-10,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(150.0))
                    .convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(1.065)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(7.1)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(7.1)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(7.1)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(7.10)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(14.2)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(5.997239)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(5.9972429)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(15.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(11.24482)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(19.9908)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(7.99632)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(6.24712)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(5.559945)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(2.342672)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(7.1)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(7.20238)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(7.202386)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(8.5199914)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(7.1)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(7.1)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(7.1)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(7.1)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialDessertspoon(56.0))
                    .convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(8.962970)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_tablespoon_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::ImperialTablespoon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(2663.724)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(26.63724)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(150.0))
                    .convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(26.63724)),
                1e-10,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTablespoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(2.663724)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(17.75816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(17.75816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(17.75816)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(17.758164)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(35.51632)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(14.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(15.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(37.51723)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(28.12498)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(49.99998)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(19.99999)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(15.624996)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(13.906252)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(5.859373)),
                1e-5,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTablespoon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(17.75816)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(18.014244)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(18.014244)),
                1e-6,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(21.30977)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(17.75816)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(17.75816)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(17.75816)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(17.75816)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialTablespoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(22.417726)),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_fluid_ounce_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::ImperialFluidOunce(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(4261.961250507437)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(15.0))
                    .convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(42.619612)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(150.0))
                    .convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(42.619612)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(4.261961)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(28.41307)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(28.41307)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(28.41307)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(250.0))
                    .convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(28.413075)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(56.82615)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(24.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(24.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(60.02762)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(45.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(80.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(32.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(25.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(22.25)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(9.375)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialFluidOunce(2.534046))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(0.30000016275009356)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(5.0))
                    .convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(28.8228)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(28.8228)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(34.0956)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(236.5882))
                    .convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(28.413)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(473.1765))
                    .convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(28.413)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(946.353))
                    .convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(28.413)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(28.413)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialFluidOunce(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(35.8683)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_gill_conversions() -> Result<()> {
            assert_eq!(
                Unit::Volume(Volume::ImperialGill(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(21309.796875)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(213.09796)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(213.09796)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(21.30979)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(142.0653)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(142.0653)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(142.0653)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(142.0653)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(284.1306)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(119.9999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(120.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(300.1379)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(224.9999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(400.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(160.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(124.999999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(111.25)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(46.874999)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGill(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(142.0653125)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(144.1139)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(144.1139)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(170.4782)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(142.0653)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(142.0653)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(142.0653)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(142.0653)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGill(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(179.34185)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_cup_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(42619.59)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(426.1959)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(426.1959)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(42.61959)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(284.1306)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(284.1306)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(284.1306)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(284.1306)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(568.2612)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(239.9998)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(240.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(600.2759)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(449.9997)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(800.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(320.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(249.99997)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(222.5)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(93.74999)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialCup(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(284.1306)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(288.22795)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(288.22795)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(340.95637)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(284.1306)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(284.1306)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(284.1306)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(284.1306)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialCup(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(358.68367)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_pint_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(85239.1875)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(852.39187)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(852.39187)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(85.23918)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(568.26125)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(568.26125)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(568.26125)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(568.26125)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(1136.5225)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(479.99983)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(480.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(1200.55193)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(899.9996)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(1600.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(640.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(500.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(445.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(187.5)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialPint(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(568.2612500008727)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(576.45596)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(576.45596)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(681.91281)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(568.26125)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(568.26125)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(568.26125)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(568.26125)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialPint(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(717.36742)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_quart_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(170478.3)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(1704.783)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(1704.783)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(170.4783)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(1136.522)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(1136.522)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(1136.522)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(1136.522)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(2273.044)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(959.99925)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(959.999797)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(2401.1028)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(1799.99841)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(3199.99859)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(1279.9995)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(999.99956)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(890.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(374.999835)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialQuart(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(1136.5220000000002)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(1152.91142)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(1152.91142)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(1363.825036)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(1136.522)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(1136.522)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(1136.522)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(1136.522)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialQuart(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(1434.734214)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_gallon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(150.0))
                    .convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(681913.5)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(6819.135)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(6819.135)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(681.9135)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(4546.09)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(4546.09)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(4546.09)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(4546.09)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(9092.18)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(3839.99871)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(3840.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(9604.41549)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(7199.9968)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(12800.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(320.0))
                    .convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(5120.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(3999.9999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(3560.00156)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(1500.0)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGallon(240.0))
                    .convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(4546.090000001815)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(4611.6477)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(15.0))
                    .convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(4611.6477)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(5455.3025)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(4546.09)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(4546.09)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(4546.09)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(3785.412))
                    .convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(4546.09)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::ImperialGallon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(5738.939383)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_legal_cup_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(36000.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(360.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(360.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(36.0)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(240.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(240.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(240.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(240.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(480.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(202.72359)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(202.72370)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(507.042253)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(380.106693)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(675.745530)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(270.298236)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(211.17047)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(187.941808)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(79.188929)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::USLegalCup(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(240.0)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(243.4609)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(243.4609)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(287.99971)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(240.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(240.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(240.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(240.0)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USLegalCup(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(302.97364)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_teaspoon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(739.338239)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(7.3933823)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(7.3933823)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(0.73933823)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(4.92892)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(4.92892)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(4.92892)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(4.9289)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(9.85784)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(4.163369)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(4.163369)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(10.4132146)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(7.8063170)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(13.877903)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(5.55116)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(4.3368447)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(3.859793)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(1.62631)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::USTeaspoon(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(4.928921594018646)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(5.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(5.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(5.9147)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(4.92892)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(4.92892)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(4.92892)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(4.92892)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTeaspoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(6.22222)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_tablespoon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(2218.0147)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(22.180147)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(22.180147)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(2.218014)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(14.78676)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(14.78676)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(14.78676)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(14.78676)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(29.57352)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(12.490108)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(12.490108)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(31.239643)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(23.418951)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(41.633709)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(16.6534851)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(13.0105)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(11.57938)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(4.87895)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::USTablespoon(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(14.786764782055936)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(15.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(15.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(17.7441)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(14.78676)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(14.78676)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(14.78676)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(14.78676)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USTablespoon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(18.666666)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_floz_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(150.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(4436.02943)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(44.36029)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(44.36029)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(4.436029)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(5.0))
                    .convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(29.57352)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(15.0))
                    .convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(29.57352)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(29.57352)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(29.57352)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(59.147)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(5.0))
                    .convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(24.980217)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(24.980217)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(62.4792)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(46.83790)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(400.0))
                    .convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(83.26741)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(33.30697)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(500.0))
                    .convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(26.021068)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(890.0))
                    .convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(23.15876)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(1500.0))
                    .convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(9.7579)),
                1e-4,
            );
            assert_eq!(
                Unit::Volume(Volume::USFluidOunce(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(29.573529564111873)),
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(30.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(30.0)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(35.4882))
                    .convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(35.4882)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(29.57352)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(29.57352)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(29.57352)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(29.57352)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USFluidOunce(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(37.33333)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_cup_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USCup(2.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(473.1764)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(354.882354)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(354.882354)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(35.488235)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(236.5882)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(236.5882)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(236.5882)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(236.58823)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(473.17647)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(199.841737)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(199.841737)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(499.83430)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(374.70321)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(666.13934)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(266.455762)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(208.16854)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(185.2700)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(78.0632)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(236.5882)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(239.9999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(239.9999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(283.905599)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(236.5882)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(236.5882)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(236.5882)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(236.5882)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USCup(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(298.66666)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_pint_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USPint(2.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(709.76470)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(709.76470)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(70.97647)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(473.176472)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(473.176472)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(473.176472)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(473.17647)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(399.68347)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(399.68347)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(999.6686)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(749.406436)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(1332.2786)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(532.91152)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(416.33709)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(370.54017)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(156.1264)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(473.176472)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(479.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(479.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(567.8111)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(473.176472)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(473.176472)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(473.176472)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(473.176472)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USPint(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(597.3333)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_quart_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(2.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(1892.7058)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(1419.52941)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(1419.52941)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(141.95294)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(1892.7058)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(799.3669)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(799.3669)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(1999.33720)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(1498.81287)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(2664.55739)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(1065.8230)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(832.67418)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(741.0803)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(312.2528)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(946.3529)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(959.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(959.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(1135.62239)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(946.3529)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(946.3529)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(946.3529)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(946.3529)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USQuart(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(1194.6666)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_gallon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(2.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(7570.82356)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(5678.11767)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(5678.11767)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(567.81176)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(3785.411784)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(3785.411784)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(10.0))
                    .convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(3785.411784)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(3785.41178)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(7570.82356)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(3197.46779)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(15.0))
                    .convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(3197.46960)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(7997.34883)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(45.0))
                    .convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(5995.25149)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(10658.2295)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(4263.2922)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(3330.69673)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(2964.32140)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(1249.0112)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(3785.411784)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(3839.999999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(3839.999999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(4542.48959)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(3785.411784)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(3785.411784)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(3785.411784)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(3785.411784)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::USGallon(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(4778.66667)),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_jigger_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(2.0)).convert(UnitType::Volume(Millilitre))?,
                Unit::Volume(Volume::Millilitre(88.7205886)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(15.0)).convert(UnitType::Volume(Centilitre))?,
                Unit::Volume(Volume::Centilitre(66.54044)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(150.0)).convert(UnitType::Volume(Decilitre))?,
                Unit::Volume(Volume::Decilitre(66.54044)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(150.0)).convert(UnitType::Volume(Litre))?,
                Unit::Volume(Volume::Litre(6.654044)),
                1e-5,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
                Unit::Volume(Volume::MetricTeaspoon(44.3602)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
                Unit::Volume(Volume::MetricTablespoon(44.3602)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
                Unit::Volume(Volume::MetricDessertSpoon(44.3602)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(250.0)).convert(UnitType::Volume(MetricCup))?,
                Unit::Volume(Volume::MetricCup(44.36029)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(40.0))
                    .convert(UnitType::Volume(AustralianTablespoon))?,
                Unit::Volume(Volume::AustralianTablespoon(88.7205)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
                Unit::Volume(Volume::ImperialTeaspoon(37.47032)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
                Unit::Volume(Volume::ImperialTablespoon(37.470346)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(15.0))
                    .convert(UnitType::Volume(ImperialDessertspoon))?,
                Unit::Volume(Volume::ImperialDessertspoon(93.71893)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
                Unit::Volume(Volume::ImperialFluidOunce(70.25685)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(400.0)).convert(UnitType::Volume(ImperialGill))?,
                Unit::Volume(Volume::ImperialGill(124.90112)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(320.0)).convert(UnitType::Volume(ImperialCup))?,
                Unit::Volume(Volume::ImperialCup(49.96045542)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(500.0)).convert(UnitType::Volume(ImperialPint))?,
                Unit::Volume(Volume::ImperialPint(39.03160)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(890.0)).convert(UnitType::Volume(ImperialQuart))?,
                Unit::Volume(Volume::ImperialQuart(34.73814)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
                Unit::Volume(Volume::ImperialGallon(14.6368)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(240.0)).convert(UnitType::Volume(USLegalCup))?,
                Unit::Volume(Volume::USLegalCup(44.3602)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(5.0)).convert(UnitType::Volume(USTeaspoon))?,
                Unit::Volume(Volume::USTeaspoon(44.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(15.0)).convert(UnitType::Volume(USTablespoon))?,
                Unit::Volume(Volume::USTablespoon(44.99999)),
                1e-4,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
                Unit::Volume(Volume::USFluidOunce(53.2323)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(236.5882)).convert(UnitType::Volume(USCup))?,
                Unit::Volume(Volume::USCup(44.3602)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(473.1765)).convert(UnitType::Volume(USPint))?,
                Unit::Volume(Volume::USPint(44.3602)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(946.353)).convert(UnitType::Volume(USQuart))?,
                Unit::Volume(Volume::USQuart(44.3602)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(3785.412)).convert(UnitType::Volume(USGallon))?,
                Unit::Volume(Volume::USGallon(44.3602)),
                1e-3,
            );
            assert_approx_eq(
                Unit::Volume(Volume::Jigger(56.0)).convert(UnitType::Volume(Jigger))?,
                Unit::Volume(Volume::Jigger(56.0)),
                1e-4,
            );
            Ok(())
        }
    }
}
