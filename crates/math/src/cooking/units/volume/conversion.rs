use crate::Error;
use crate::Result;
use crate::cooking::units::UnitType;
use crate::cooking::units::custom::{
    VolumeAustralianTablespoonExt, VolumeCentilitreExt, VolumeDecilitreExt, VolumeImperialCupExt,
    VolumeImperialDessertSpoonExt, VolumeImperialGillExt, VolumeImperialQuartExt,
    VolumeImperialTablespoonExt, VolumeImperialTeaspoonExt, VolumeJiggerExt, VolumeMetricCupExt,
    VolumeMetricDessertSpoonExt, VolumeMetricTablespoonExt, VolumeMetricTeaspoonExt,
};
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::unit::Unit;
use crate::cooking::units::volume::units::Volume;
use crate::cooking::units::volume::units::VolumeUnit;

impl UnitConverter for Volume {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        use VolumeUnit::*;

        match self {
            Volume::Millilitre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_millilitres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(self.with_value(*original_value))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Centilitre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_centilitres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(self.with_value(*original_value))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Decilitre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_decilitres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(self.with_value(*original_value))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Litre(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_litres(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricDessertspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_dessertspoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::MetricCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::AustralianTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::AustralianDessertspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_dessertspoons_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => {
                            Ok(Unit::Volume(self.with_value(*original_value)))
                        }
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::AustralianTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons_aus(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::AustralianCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups_metric(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialDessertspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_dessertspoons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialFluidOunce(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_fluid_ounces_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialGill(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_gills_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialPint(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_pints_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialQuart(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_quarts_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => Ok(Unit::Volume(self.with_value(*original_value))),
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::ImperialGallon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_gallons_uk(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => Ok(Unit::Volume(self.with_value(*original_value))),
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USTeaspoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_teaspoons(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USTablespoon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_tablespoons(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => Ok(Unit::Volume(self.with_value(*original_value))),
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USFluidOunce(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_fluid_ounces(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => Ok(Unit::Volume(self.with_value(*original_value))),
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USCup(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_cups(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Volume::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(self.with_value(*original_value))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USPint(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_pints(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(self.with_value(*original_value))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USQuart(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_quarts(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(self.with_value(*original_value))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::USGallon(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_gallons(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(self.with_value(*original_value))),
                        Jigger => Ok(Unit::Volume(Self::Jigger(value.as_jiggers()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Volume(self.clone()), to)),
            },
            Volume::Jigger(original_value) => match to {
                UnitType::Volume(unit) => {
                    let value = measurements::Volume::from_jiggers(*original_value);

                    match unit {
                        Millilitre => Ok(Unit::Volume(Self::Millilitre(value.as_millilitres()))),
                        Centilitre => Ok(Unit::Volume(Self::Centilitre(value.as_centilitres()))),
                        Decilitre => Ok(Unit::Volume(Self::Decilitre(value.as_decilitres()))),
                        Litre => Ok(Unit::Volume(Self::Litre(value.as_litres()))),
                        MetricTeaspoon => Ok(Unit::Volume(Self::MetricTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        MetricTablespoon => Ok(Unit::Volume(Self::MetricTablespoon(
                            value.as_tablespoons_metric(),
                        ))),
                        MetricDessertSpoon => Ok(Unit::Volume(Self::MetricDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        MetricCup => Ok(Unit::Volume(Self::MetricCup(value.as_cups_metric()))),
                        AustralianTeaspoon => Ok(Unit::Volume(Self::AustralianTeaspoon(
                            value.as_teaspoons_metric(),
                        ))),
                        AustralianDessertspoon => Ok(Unit::Volume(Self::AustralianDessertspoon(
                            value.as_dessertspoons_metric(),
                        ))),
                        AustralianTablespoon => Ok(Unit::Volume(Self::AustralianTablespoon(
                            value.as_tablespoons_aus(),
                        ))),
                        AustralianCup => {
                            Ok(Unit::Volume(Self::AustralianCup(value.as_cups_metric())))
                        }
                        ImperialTeaspoon => Ok(Unit::Volume(Self::ImperialTeaspoon(
                            value.as_teaspoons_uk(),
                        ))),
                        ImperialDessertspoon => Ok(Unit::Volume(Self::ImperialDessertspoon(
                            value.as_dessertspoons_uk(),
                        ))),
                        ImperialTablespoon => Ok(Unit::Volume(Self::ImperialTablespoon(
                            value.as_tablespoons_uk(),
                        ))),
                        ImperialFluidOunce => Ok(Unit::Volume(Self::ImperialFluidOunce(
                            value.as_fluid_ounces_uk(),
                        ))),
                        ImperialGill => Ok(Unit::Volume(Self::ImperialGill(value.as_gills_uk()))),
                        ImperialCup => Ok(Unit::Volume(Self::ImperialCup(value.as_cups_uk()))),
                        ImperialPint => Ok(Unit::Volume(Self::ImperialPint(value.as_pints_uk()))),
                        ImperialQuart => {
                            Ok(Unit::Volume(Self::ImperialQuart(value.as_quarts_uk())))
                        }
                        ImperialGallon => {
                            Ok(Unit::Volume(Self::ImperialGallon(value.as_gallons_uk())))
                        }
                        USTeaspoon => Ok(Unit::Volume(Self::USTeaspoon(value.as_teaspoons()))),
                        USTablespoon => {
                            Ok(Unit::Volume(Self::USTablespoon(value.as_tablespoons())))
                        }
                        USFluidOunce => {
                            Ok(Unit::Volume(Self::USFluidOunce(value.as_fluid_ounces())))
                        }
                        USCup => Ok(Unit::Volume(Self::USCup(value.as_cups()))),
                        USPint => Ok(Unit::Volume(Self::USPint(value.as_pints()))),
                        USQuart => Ok(Unit::Volume(Self::USQuart(value.as_quarts()))),
                        USGallon => Ok(Unit::Volume(Self::USGallon(value.as_gallons()))),
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
    use crate::cooking::units::volume::units::VolumeUnit::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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
            Unit::Volume(Volume::Millilitre(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(2.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(2.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::Millilitre(15.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(2.534)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Millilitre(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
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
            Unit::Volume(Volume::Millilitre(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::Millilitre(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(0.329954)),
            1e-5,
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
            Unit::Volume(Volume::Millilitre(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::Centilitre(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(20.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(20.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::Centilitre(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(8.446816)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Centilitre(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
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
            Unit::Volume(Volume::Centilitre(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::Centilitre(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(3.2995387)),
            1e-5,
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
            Unit::Volume(Volume::Centilitre(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::Decilitre(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(100.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(100.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(100.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(100.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(100.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(200.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(200.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::Decilitre(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(84.46816)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Decilitre(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
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
            Unit::Volume(Volume::Decilitre(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::Decilitre(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(32.995387)),
            1e-5,
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
            Unit::Volume(Volume::MetricDessertspoon(100.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(1.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(4.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(1000.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(10.0)).convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(1000.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(1.2)).convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(60.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(2000.0)),
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
            Unit::Volume(Volume::Litre(15.0)).convert(UnitType::Volume(ImperialDessertspoon))?,
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
            Unit::Volume(Volume::MetricTeaspoon(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
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
            Unit::Volume(Volume::MetricDessertspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(10.0)),
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
            Unit::Volume(Volume::MetricTeaspoon(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(14.078031)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTeaspoon(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(5.6312132)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTeaspoon(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(4.399384)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTeaspoon(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(3.9154543)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTeaspoon(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(1.649769)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTeaspoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(5.07210)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTeaspoon(15.0)).convert(UnitType::Volume(USTablespoon))?,
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
            Unit::Volume(Volume::MetricTeaspoon(3785.412)).convert(UnitType::Volume(USGallon))?,
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
            Unit::Volume(Volume::MetricTablespoon(150.0)).convert(UnitType::Volume(Millilitre))?,
            Unit::Volume(Volume::Millilitre(2250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(15.0)).convert(UnitType::Volume(Centilitre))?,
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
            Unit::Volume(Volume::MetricDessertspoon(15.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(15.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(15.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(15.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(30.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(30.0)),
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
            Unit::Volume(Volume::MetricTablespoon(320.0)).convert(UnitType::Volume(ImperialCup))?,
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
        assert_approx_eq(
            Unit::Volume(Volume::MetricTablespoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(15.216310215)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTablespoon(15.0)).convert(UnitType::Volume(USTablespoon))?,
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
            Unit::Volume(Volume::MetricTablespoon(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(14.999997686809676)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTablespoon(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(15.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTablespoon(946.353)).convert(UnitType::Volume(USQuart))?,
            Unit::Volume(Volume::USQuart(15.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricTablespoon(3785.412)).convert(UnitType::Volume(USGallon))?,
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
            Unit::Volume(Volume::MetricDessertspoon(150.0))
                .convert(UnitType::Volume(Millilitre))?,
            Unit::Volume(Volume::Millilitre(1500.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(15.0)).convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(15.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(150.0)).convert(UnitType::Volume(Decilitre))?,
            Unit::Volume(Volume::Decilitre(15.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(150.0)).convert(UnitType::Volume(Litre))?,
            Unit::Volume(Volume::Litre(1.5)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(5.0))
                .convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(15.0))
                .convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(20.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(20.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(5.0))
                .convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(8.446816)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(15.0))
                .convert(UnitType::Volume(ImperialTablespoon))?,
            Unit::Volume(Volume::MetricDessertspoon(8.446816)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(15.0))
                .convert(UnitType::Volume(ImperialDessertspoon))?,
            Unit::Volume(Volume::ImperialDessertspoon(21.126760563380284)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(45.0))
                .convert(UnitType::Volume(ImperialFluidOunce))?,
            Unit::Volume(Volume::ImperialFluidOunce(15.837778908)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(400.0))
                .convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(28.156063)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(320.0))
                .convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(11.262426)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(500.0))
                .convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(8.79876993195)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(890.0))
                .convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(7.83090868)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(3.299538)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(10.14420681)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(15.0))
                .convert(UnitType::Volume(USTablespoon))?,
            Unit::Volume(Volume::USTablespoon(10.14420681)),
            1e-6,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(35.4882))
                .convert(UnitType::Volume(USFluidOunce))?,
            Unit::Volume(Volume::USFluidOunce(11.999988)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(9.9999)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(10.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(946.353)).convert(UnitType::Volume(USQuart))?,
            Unit::Volume(Volume::USQuart(10.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(3785.412))
                .convert(UnitType::Volume(USGallon))?,
            Unit::Volume(Volume::USGallon(10.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricDessertspoon(56.0)).convert(UnitType::Volume(Jigger))?,
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
            Unit::Volume(Volume::MetricCup(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(500.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(500.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricCup(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(211.1704077)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::MetricCup(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
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
            Unit::Volume(Volume::MetricCup(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::MetricCup(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(82.48846)),
            1e-5,
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
    fn test_australian_tsp_conversions() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(150.0))
                .convert(UnitType::Volume(Millilitre))?,
            Unit::Volume(Volume::Millilitre(750.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(15.0)).convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(7.5)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(150.0)).convert(UnitType::Volume(Decilitre))?,
            Unit::Volume(Volume::Decilitre(7.5)),
            1e-10,
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(150.0)).convert(UnitType::Volume(Litre))?,
            Unit::Volume(Volume::Litre(0.75)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(5.0))
                .convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(15.0))
                .convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(5.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(4.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(10.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(5.0))
                .convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(4.2234)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(15.0))
                .convert(UnitType::Volume(ImperialTablespoon))?,
            Unit::Volume(Volume::ImperialTablespoon(4.2234)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(15.0))
                .convert(UnitType::Volume(ImperialDessertspoon))?,
            Unit::Volume(Volume::ImperialDessertspoon(10.56338)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(45.0))
                .convert(UnitType::Volume(ImperialFluidOunce))?,
            Unit::Volume(Volume::ImperialFluidOunce(7.91888)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(400.0))
                .convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(14.07803)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(320.0))
                .convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(5.63121)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(500.0))
                .convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(4.3993)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(890.0))
                .convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(3.915454)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(1.64976)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(5.0721)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(15.0))
                .convert(UnitType::Volume(USTablespoon))?,
            Unit::Volume(Volume::USTablespoon(5.0721)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(35.4882))
                .convert(UnitType::Volume(USFluidOunce))?,
            Unit::Volume(Volume::USFluidOunce(5.99999)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(4.999)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(5.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(946.353)).convert(UnitType::Volume(USQuart))?,
            Unit::Volume(Volume::USQuart(5.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(3785.412))
                .convert(UnitType::Volume(USGallon))?,
            Unit::Volume(Volume::USGallon(5.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianTeaspoon(56.0)).convert(UnitType::Volume(Jigger))?,
            Unit::Volume(Volume::Jigger(6.31195)),
            1e-4,
        );
        Ok(())
    }

    #[test]
    fn test_australian_dsp_conversions() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(150.0))
                .convert(UnitType::Volume(Millilitre))?,
            Unit::Volume(Volume::Millilitre(1500.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(15.0))
                .convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(15.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(150.0))
                .convert(UnitType::Volume(Decilitre))?,
            Unit::Volume(Volume::Decilitre(15.0)),
            1e-10,
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(150.0)).convert(UnitType::Volume(Litre))?,
            Unit::Volume(Volume::Litre(1.5)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(5.0))
                .convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(15.0))
                .convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(250.0))
                .convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(10.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(4.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(2.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianDessertspoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(20.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(5.0))
                .convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(8.44681)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(15.0))
                .convert(UnitType::Volume(ImperialTablespoon))?,
            Unit::Volume(Volume::ImperialTablespoon(8.44681)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(15.0))
                .convert(UnitType::Volume(ImperialDessertspoon))?,
            Unit::Volume(Volume::ImperialDessertspoon(21.12676)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(45.0))
                .convert(UnitType::Volume(ImperialFluidOunce))?,
            Unit::Volume(Volume::ImperialFluidOunce(15.8377)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(400.0))
                .convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(28.15606)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(320.0))
                .convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(11.26242)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(500.0))
                .convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(8.79876)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(890.0))
                .convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(7.83090)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(3.29953)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(5.0))
                .convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(10.1442)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(15.0))
                .convert(UnitType::Volume(USTablespoon))?,
            Unit::Volume(Volume::USTablespoon(10.1442)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(35.4882))
                .convert(UnitType::Volume(USFluidOunce))?,
            Unit::Volume(Volume::USFluidOunce(11.9999)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(236.5882))
                .convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(9.99999)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(473.1765))
                .convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(10.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(946.353))
                .convert(UnitType::Volume(USQuart))?,
            Unit::Volume(Volume::USQuart(10.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(3785.412))
                .convert(UnitType::Volume(USGallon))?,
            Unit::Volume(Volume::USGallon(10.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianDessertspoon(56.0)).convert(UnitType::Volume(Jigger))?,
            Unit::Volume(Volume::Jigger(12.6239)),
            1e-4,
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
            Unit::Volume(Volume::AustralianTablespoon(150.0)).convert(UnitType::Volume(Litre))?,
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
            Unit::Volume(Volume::MetricDessertspoon(20.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTablespoon(250.0))
                .convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(20.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTablespoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(20.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTablespoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(20.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTablespoon(4.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(4.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTablespoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(40.0)),
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
            Unit::Volume(Volume::AustralianTablespoon(56.0)).convert(UnitType::Volume(Jigger))?,
            Unit::Volume(Volume::Jigger(25.247803)),
            1e-6,
        );
        Ok(())
    }

    #[test]
    fn test_australian_cup_conversions() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(150.0)).convert(UnitType::Volume(Millilitre))?,
            Unit::Volume(Volume::Millilitre(37500.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(15.0)).convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(375.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(150.0)).convert(UnitType::Volume(Decilitre))?,
            Unit::Volume(Volume::Decilitre(375.0)),
            1e-10,
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(150.0)).convert(UnitType::Volume(Litre))?,
            Unit::Volume(Volume::Litre(37.5)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(15.0))
                .convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(250.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(4.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(50.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianCup(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(500.0)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(211.1704)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(15.0))
                .convert(UnitType::Volume(ImperialTablespoon))?,
            Unit::Volume(Volume::ImperialTablespoon(211.1705)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(15.0))
                .convert(UnitType::Volume(ImperialDessertspoon))?,
            Unit::Volume(Volume::ImperialDessertspoon(528.16901)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(45.0))
                .convert(UnitType::Volume(ImperialFluidOunce))?,
            Unit::Volume(Volume::ImperialFluidOunce(395.9445)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(703.9016)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(281.5607)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(219.9692)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(195.7727)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(82.4885)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(253.60517)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(15.0)).convert(UnitType::Volume(USTablespoon))?,
            Unit::Volume(Volume::USTablespoon(253.60517)),
            1e-6,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
            Unit::Volume(Volume::USFluidOunce(299.9997)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(249.999)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(250.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(946.353)).convert(UnitType::Volume(USQuart))?,
            Unit::Volume(Volume::USQuart(250.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(3785.412)).convert(UnitType::Volume(USGallon))?,
            Unit::Volume(Volume::USGallon(250.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::AustralianCup(56.0)).convert(UnitType::Volume(Jigger))?,
            Unit::Volume(Volume::Jigger(315.597545)),
            1e-6,
        );
        Ok(())
    }

    #[test]
    fn test_imperial_tsp_conversions() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(150.0)).convert(UnitType::Volume(Millilitre))?,
            Unit::Volume(Volume::Millilitre(887.9085)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(15.0)).convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(8.879085)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(150.0)).convert(UnitType::Volume(Decilitre))?,
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
            Unit::Volume(Volume::MetricDessertspoon(5.91939)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(5.91939)),
            1e-4,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(5.91939)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(5.91939)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(11.83878)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(11.83878)),
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
            Unit::Volume(Volume::ImperialTeaspoon(320.0)).convert(UnitType::Volume(ImperialCup))?,
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
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(6.0047516)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(15.0)).convert(UnitType::Volume(USTablespoon))?,
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
            Unit::Volume(Volume::ImperialTeaspoon(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(5.919389)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(5.91939)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(946.353)).convert(UnitType::Volume(USQuart))?,
            Unit::Volume(Volume::USQuart(5.91939)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTeaspoon(3785.412)).convert(UnitType::Volume(USGallon))?,
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
            Unit::Volume(Volume::ImperialDessertspoon(150.0)).convert(UnitType::Volume(Litre))?,
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
            Unit::Volume(Volume::MetricDessertspoon(7.1)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialDessertspoon(250.0))
                .convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(7.10)),
            1e-4,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(7.1)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(7.1)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialDessertspoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(14.2)),
            1e-5,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(14.2)),
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
            Unit::Volume(Volume::ImperialDessertspoon(56.0)).convert(UnitType::Volume(Jigger))?,
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
            Unit::Volume(Volume::ImperialTablespoon(15.0)).convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(26.63724)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTablespoon(150.0)).convert(UnitType::Volume(Decilitre))?,
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
            Unit::Volume(Volume::MetricDessertspoon(17.75816)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTablespoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(17.758164)),
            1e-4,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(17.75816)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(17.75816)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTablespoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(35.51632)),
            1e-4,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(35.51632)),
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
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTablespoon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
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
            Unit::Volume(Volume::ImperialTablespoon(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(17.75816)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTablespoon(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(17.75816)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialTablespoon(946.353)).convert(UnitType::Volume(USQuart))?,
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
            Unit::Volume(Volume::ImperialFluidOunce(15.0)).convert(UnitType::Volume(Centilitre))?,
            Unit::Volume(Volume::Centilitre(42.619612)),
            1e-5,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(150.0)).convert(UnitType::Volume(Decilitre))?,
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
            Unit::Volume(Volume::MetricDessertspoon(28.41307)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(28.413075)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(28.4131)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(28.4131)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(56.82615)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(500.0))
                .convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(56.8262)),
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
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(5.0)).convert(UnitType::Volume(USTeaspoon))?,
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
            Unit::Volume(Volume::ImperialFluidOunce(236.5882)).convert(UnitType::Volume(USCup))?,
            Unit::Volume(Volume::USCup(28.413)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(473.1765)).convert(UnitType::Volume(USPint))?,
            Unit::Volume(Volume::USPint(28.413)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialFluidOunce(946.353)).convert(UnitType::Volume(USQuart))?,
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
            Unit::Volume(Volume::ImperialGill(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(142.0653)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(142.0653)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(142.0653)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(142.0653)),
            1e-4,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(142.0653125)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(142.0653125)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(284.1306)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(284.1306)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
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
            Unit::Volume(Volume::ImperialGill(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(400.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(160.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(124.999999)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(111.25)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGill(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(46.874999)),
            1e-4,
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
            Unit::Volume(Volume::ImperialGill(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::ImperialCup(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(284.1306)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(284.1306)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(284.1306)),
            1e-4,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialCup(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(284.1306)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialCup(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(284.1306)),
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(568.2612)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(568.2612)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
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
            Unit::Volume(Volume::ImperialCup(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(222.5)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(93.74999)),
            1e-4,
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
            Unit::Volume(Volume::ImperialCup(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::ImperialPint(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(568.26125)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(568.26125)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(568.26125)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(568.26125)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(568.2613)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(568.2613)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(1136.5225)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(1136.5225)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
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
            Unit::Volume(Volume::ImperialPint(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(1600.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(640.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(500.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(445.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialPint(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(187.5)),
            1e-4,
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
            Unit::Volume(Volume::ImperialPint(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::ImperialQuart(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
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
            Unit::Volume(Volume::MetricDessertspoon(1136.522)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(1136.522)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(1136.522)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(1136.522)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(2273.044)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(2273.044)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
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
            Unit::Volume(Volume::ImperialQuart(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(3199.99859)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(1279.9995)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(999.99956)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(890.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(374.999835)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(1152.91142)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(15.0)).convert(UnitType::Volume(USTablespoon))?,
            Unit::Volume(Volume::USTablespoon(1152.91142)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialQuart(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::ImperialQuart(3785.412)).convert(UnitType::Volume(USGallon))?,
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
            Unit::Volume(Volume::ImperialGallon(150.0)).convert(UnitType::Volume(Millilitre))?,
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
            Unit::Volume(Volume::ImperialGallon(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
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
            Unit::Volume(Volume::MetricDessertspoon(4546.09)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(4546.09)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(4546.09)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(4546.09)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(9092.18)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(9092.18)),
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
            Unit::Volume(Volume::ImperialGallon(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(12800.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(5120.0)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(3999.9999)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(3560.00156)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(1500.0))
                .convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(1500.0)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(5.0)).convert(UnitType::Volume(USTeaspoon))?,
            Unit::Volume(Volume::USTeaspoon(4611.6477)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialGallon(15.0)).convert(UnitType::Volume(USTablespoon))?,
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
            Unit::Volume(Volume::ImperialGallon(3785.412)).convert(UnitType::Volume(USGallon))?,
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
            Unit::Volume(Volume::USTeaspoon(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(4.92892)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(4.92892)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(4.9289)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(4.9289)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(4.9289)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(9.85784)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(9.8578)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(4.163369)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTeaspoon(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
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
            Unit::Volume(Volume::USTeaspoon(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::USTeaspoon(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(1.62631)),
            1e-4,
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
            Unit::Volume(Volume::USTeaspoon(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::USTablespoon(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(14.78676)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(14.78676)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(14.78676)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(14.78676)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(14.78676)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(14.78676)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(29.57352)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(29.5735)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
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
            Unit::Volume(Volume::USTablespoon(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(41.633709)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(16.6534851)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(13.0105)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(11.57938)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USTablespoon(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(4.87895)),
            1e-4,
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
            Unit::Volume(Volume::USTablespoon(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::USFluidOunce(5.0)).convert(UnitType::Volume(MetricTeaspoon))?,
            Unit::Volume(Volume::MetricTeaspoon(29.57352)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(15.0)).convert(UnitType::Volume(MetricTablespoon))?,
            Unit::Volume(Volume::MetricTablespoon(29.57352)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(10.0))
                .convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(29.57352)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(29.57352)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(5.0))
                .convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(29.5735)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(29.5735)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(40.0))
                .convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(59.147)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(59.147)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
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
            Unit::Volume(Volume::USFluidOunce(400.0)).convert(UnitType::Volume(ImperialGill))?,
            Unit::Volume(Volume::ImperialGill(83.26741)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(320.0)).convert(UnitType::Volume(ImperialCup))?,
            Unit::Volume(Volume::ImperialCup(33.30697)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(500.0)).convert(UnitType::Volume(ImperialPint))?,
            Unit::Volume(Volume::ImperialPint(26.021068)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(890.0)).convert(UnitType::Volume(ImperialQuart))?,
            Unit::Volume(Volume::ImperialQuart(23.15876)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USFluidOunce(1500.0)).convert(UnitType::Volume(ImperialGallon))?,
            Unit::Volume(Volume::ImperialGallon(9.7579)),
            1e-4,
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
            Unit::Volume(Volume::USFluidOunce(35.4882)).convert(UnitType::Volume(USFluidOunce))?,
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
            Unit::Volume(Volume::MetricDessertspoon(236.5882)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USCup(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(236.58823)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USCup(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(236.5882)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USCup(10.0)).convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(236.5882)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USCup(40.0)).convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(473.17647)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USCup(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(473.17647)),
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
            Unit::Volume(Volume::USCup(15.0)).convert(UnitType::Volume(ImperialDessertspoon))?,
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
            Unit::Volume(Volume::MetricDessertspoon(473.176472)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USPint(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(473.17647)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USPint(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(473.1764)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USPint(10.0)).convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(473.1764)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USPint(40.0)).convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(946.3529)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USPint(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(946.3529)),
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
            Unit::Volume(Volume::USPint(15.0)).convert(UnitType::Volume(ImperialDessertspoon))?,
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
            Unit::Volume(Volume::USQuart(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(946.3529)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(946.3529)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(946.3529)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(946.3529)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(40.0)).convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(1892.7058)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(1892.7058)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(799.3669)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
            Unit::Volume(Volume::ImperialTablespoon(799.3669)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(15.0)).convert(UnitType::Volume(ImperialDessertspoon))?,
            Unit::Volume(Volume::ImperialDessertspoon(1999.33720)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USQuart(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::USGallon(10.0)).convert(UnitType::Volume(MetricDessertSpoon))?,
            Unit::Volume(Volume::MetricDessertspoon(3785.411784)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(3785.41178)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(3785.4117)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(10.0))
                .convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(3785.4117)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(40.0)).convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(7570.82356)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(7570.8235)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(5.0)).convert(UnitType::Volume(ImperialTeaspoon))?,
            Unit::Volume(Volume::ImperialTeaspoon(3197.46779)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(15.0)).convert(UnitType::Volume(ImperialTablespoon))?,
            Unit::Volume(Volume::ImperialTablespoon(3197.46960)),
            1e-3,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(15.0)).convert(UnitType::Volume(ImperialDessertspoon))?,
            Unit::Volume(Volume::ImperialDessertspoon(7997.34883)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::USGallon(45.0)).convert(UnitType::Volume(ImperialFluidOunce))?,
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
            Unit::Volume(Volume::MetricDessertspoon(44.3602)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Jigger(250.0)).convert(UnitType::Volume(MetricCup))?,
            Unit::Volume(Volume::MetricCup(44.36029)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Jigger(5.0)).convert(UnitType::Volume(AustralianTeaspoon))?,
            Unit::Volume(Volume::AustralianTeaspoon(44.3603)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Jigger(10.0)).convert(UnitType::Volume(AustralianDessertspoon))?,
            Unit::Volume(Volume::AustralianDessertspoon(44.3603)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Jigger(40.0)).convert(UnitType::Volume(AustralianTablespoon))?,
            Unit::Volume(Volume::AustralianTablespoon(88.7205)),
            1e-4,
        );
        assert_approx_eq(
            Unit::Volume(Volume::Jigger(500.0)).convert(UnitType::Volume(AustralianCup))?,
            Unit::Volume(Volume::AustralianCup(88.7205)),
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
            Unit::Volume(Volume::Jigger(15.0)).convert(UnitType::Volume(ImperialDessertspoon))?,
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
