use crate::cooking::units::custom::{
    MassDekagramExt, MassHectogramExt, VolumeAustralianTablespoonExt, VolumeCentilitreExt,
    VolumeDecilitreExt, VolumeImperialCupExt, VolumeImperialDessertSpoonExt, VolumeImperialGillExt,
    VolumeImperialQuartExt, VolumeImperialTablespoonExt, VolumeImperialTeaspoonExt,
    VolumeJiggerExt, VolumeMetricCupExt, VolumeMetricDessertSpoonExt, VolumeMetricTablespoonExt,
    VolumeMetricTeaspoonExt, VolumeUSLegalCupExt,
};
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::{Unit, UnitType};
use crate::Result;

impl UnitConverter for Unit {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        match self {
            Unit::Millilitre(value) => Ok(self.with_value(*value)),
            Unit::Centilitre(value) => Ok(self.with_value(*value)),
            Unit::Decilitre(value) => Ok(self.with_value(*value)),
            Unit::Litre(value) => Ok(self.with_value(*value)),
            Unit::MetricTeaspoon(value) => Ok(self.with_value(*value)),
            Unit::MetricTablespoon(value) => Ok(self.with_value(*value)),
            Unit::MetricDessertSpoon(value) => Ok(self.with_value(*value)),
            Unit::MetricCup(value) => Ok(self.with_value(*value)),
            Unit::AustralianTablespoon(value) => Ok(self.with_value(*value)),
            Unit::ImperialTeaspoon(value) => Ok(self.with_value(*value)),
            Unit::ImperialDessertspoon(value) => Ok(self.with_value(*value)),
            Unit::ImperialTablespoon(value) => Ok(self.with_value(*value)),
            Unit::ImperialFluidOunce(value) => Ok(self.with_value(*value)),
            Unit::ImperialGill(value) => Ok(self.with_value(*value)),
            Unit::ImperialCup(value) => Ok(self.with_value(*value)),
            Unit::ImperialPint(value) => Ok(self.with_value(*value)),
            Unit::ImperialQuart(value) => Ok(self.with_value(*value)),
            Unit::ImperialGallon(value) => Ok(self.with_value(*value)),
            Unit::USLegalCup(value) => Ok(self.with_value(*value)),
            Unit::USTeaspoon(value) => Ok(self.with_value(*value)),
            Unit::USTablespoon(value) => Ok(self.with_value(*value)),
            Unit::USFluidOunce(value) => Ok(self.with_value(*value)),
            Unit::USCup(value) => Ok(self.with_value(*value)),
            Unit::USPint(value) => Ok(self.with_value(*value)),
            Unit::USQuart(value) => Ok(self.with_value(*value)),
            Unit::USGallon(value) => Ok(self.with_value(*value)),
            Unit::Jigger(value) => Ok(self.with_value(*value)),
            Unit::Length(unit) => unit.convert(to),
            Unit::Mass(unit) => unit.convert(to),
            Unit::Temperature(unit) => unit.convert(to),
        }

        /*
        match (self, to) {
            // === VOLUME ===
            // From millilitre
            (Unit::Millilitre(value), UnitType::Millilitre) => Ok(Unit::Millilitre(*value)),
            (Unit::Millilitre(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_millilitres(*value).as_centilitres(),
            )),
            (Unit::Millilitre(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_millilitres(*value).as_decilitres(),
            )),
            (Unit::Millilitre(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_millilitres(*value).as_litres()))
            }
            (Unit::Millilitre(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_millilitres(*value).as_teaspoons_metric(),
            )),
            (Unit::Millilitre(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_millilitres(*value).as_tablespoons_metric(),
            )),
            (Unit::Millilitre(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_millilitres(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::Millilitre(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_millilitres(*value).as_cups_metric(),
            )),
            (Unit::Millilitre(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_millilitres(*value).as_tablespoons_aus()),
            ),
            (Unit::Millilitre(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_millilitres(*value).as_teaspoons_uk(),
            )),
            (Unit::Millilitre(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_millilitres(*value).as_tablespoons_uk()),
            ),
            (Unit::Millilitre(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_millilitres(*value).as_dessertspoons_uk()),
            ),
            (Unit::Millilitre(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_millilitres(*value).as_fluid_ounces_uk()),
            ),
            (Unit::Millilitre(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_millilitres(*value).as_gills_uk(),
            )),
            (Unit::Millilitre(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_millilitres(*value).as_cups_uk(),
            )),
            (Unit::Millilitre(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_millilitres(*value).as_pints_uk(),
            )),
            (Unit::Millilitre(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_millilitres(*value).as_quarts_uk(),
            )),
            (Unit::Millilitre(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_millilitres(*value).as_gallons_uk(),
            )),
            (Unit::Millilitre(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_millilitres(*value).as_cups_legal(),
            )),
            (Unit::Millilitre(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_millilitres(*value).as_fluid_ounces(),
            )),
            (Unit::Millilitre(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_millilitres(*value).as_cups()))
            }
            (Unit::Millilitre(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_millilitres(*value).as_pints()))
            }
            (Unit::Millilitre(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_millilitres(*value).as_quarts()))
            }
            (Unit::Millilitre(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_millilitres(*value).as_gallons(),
            )),
            (Unit::Millilitre(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_millilitres(*value).as_teaspoons(),
            )),
            (Unit::Millilitre(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_millilitres(*value).as_tablespoons(),
            )),
            (Unit::Millilitre(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_millilitres(*value).as_jiggers()))
            }

            // From centilitre
            (Unit::Centilitre(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_centilitres(*value).as_millilitres(),
            )),
            (Unit::Centilitre(value), UnitType::Centilitre) => Ok(Unit::Centilitre(*value)),
            (Unit::Centilitre(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_centilitres(*value).as_decilitres(),
            )),
            (Unit::Centilitre(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_centilitres(*value).as_litres()))
            }
            (Unit::Centilitre(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_centilitres(*value).as_teaspoons_metric(),
            )),
            (Unit::Centilitre(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_centilitres(*value).as_tablespoons_metric(),
            )),
            (Unit::Centilitre(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_centilitres(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::Centilitre(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_centilitres(*value).as_cups_metric(),
            )),
            (Unit::Centilitre(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_centilitres(*value).as_tablespoons_aus()),
            ),
            (Unit::Centilitre(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_centilitres(*value).as_teaspoons_uk(),
            )),
            (Unit::Centilitre(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_centilitres(*value).as_tablespoons_uk()),
            ),
            (Unit::Centilitre(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_centilitres(*value).as_dessertspoons_uk()),
            ),
            (Unit::Centilitre(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_centilitres(*value).as_fluid_ounces_uk()),
            ),
            (Unit::Centilitre(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_centilitres(*value).as_gills_uk(),
            )),
            (Unit::Centilitre(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_centilitres(*value).as_cups_uk(),
            )),
            (Unit::Centilitre(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_centilitres(*value).as_pints_uk(),
            )),
            (Unit::Centilitre(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_centilitres(*value).as_quarts_uk(),
            )),
            (Unit::Centilitre(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_centilitres(*value).as_gallons_uk(),
            )),
            (Unit::Centilitre(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_centilitres(*value).as_cups_legal(),
            )),
            (Unit::Centilitre(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_centilitres(*value).as_fluid_ounces(),
            )),
            (Unit::Centilitre(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_centilitres(*value).as_cups()))
            }
            (Unit::Centilitre(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_centilitres(*value).as_pints()))
            }
            (Unit::Centilitre(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_centilitres(*value).as_quarts()))
            }
            (Unit::Centilitre(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_centilitres(*value).as_gallons(),
            )),
            (Unit::Centilitre(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_centilitres(*value).as_teaspoons(),
            )),
            (Unit::Centilitre(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_centilitres(*value).as_tablespoons(),
            )),
            (Unit::Centilitre(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_centilitres(*value).as_jiggers()))
            }

            // From decilitre
            (Unit::Decilitre(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_decilitres(*value).as_millilitres(),
            )),
            (Unit::Decilitre(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_decilitres(*value).as_centilitres(),
            )),
            (Unit::Decilitre(value), UnitType::Decilitre) => Ok(Unit::Decilitre(*value)),
            (Unit::Decilitre(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_decilitres(*value).as_litres()))
            }
            (Unit::Decilitre(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_decilitres(*value).as_teaspoons_metric(),
            )),
            (Unit::Decilitre(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_decilitres(*value).as_tablespoons_metric(),
            )),
            (Unit::Decilitre(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_decilitres(*value).as_dessertspoons_metric(),
            )),
            (Unit::Decilitre(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_decilitres(*value).as_cups_metric(),
            )),
            (Unit::Decilitre(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_decilitres(*value).as_tablespoons_aus()),
            ),
            (Unit::Decilitre(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_decilitres(*value).as_teaspoons_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_decilitres(*value).as_tablespoons_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_decilitres(*value).as_dessertspoons_uk()),
            ),
            (Unit::Decilitre(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_decilitres(*value).as_fluid_ounces_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_decilitres(*value).as_gills_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_decilitres(*value).as_cups_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_decilitres(*value).as_pints_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_decilitres(*value).as_quarts_uk(),
            )),
            (Unit::Decilitre(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_decilitres(*value).as_gallons_uk(),
            )),
            (Unit::Decilitre(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_decilitres(*value).as_cups_legal(),
            )),
            (Unit::Decilitre(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_decilitres(*value).as_fluid_ounces(),
            )),
            (Unit::Decilitre(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_decilitres(*value).as_cups()))
            }
            (Unit::Decilitre(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_decilitres(*value).as_pints()))
            }
            (Unit::Decilitre(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_decilitres(*value).as_quarts()))
            }
            (Unit::Decilitre(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_decilitres(*value).as_gallons()))
            }
            (Unit::Decilitre(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_decilitres(*value).as_teaspoons(),
            )),
            (Unit::Decilitre(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_decilitres(*value).as_tablespoons(),
            )),
            (Unit::Decilitre(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_decilitres(*value).as_jiggers()))
            }

            // From litre
            (Unit::Litre(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_litres(*value).as_millilitres(),
            )),
            (Unit::Litre(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_litres(*value).as_centilitres(),
            )),
            (Unit::Litre(value), UnitType::Decilitre) => {
                Ok(Unit::Decilitre(Volume::from_litres(*value).as_decilitres()))
            }
            (Unit::Litre(value), UnitType::Litre) => Ok(Unit::Litre(*value)),
            (Unit::Litre(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_litres(*value).as_teaspoons_metric(),
            )),
            (Unit::Litre(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_litres(*value).as_tablespoons_metric(),
            )),
            (Unit::Litre(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_litres(*value).as_dessertspoons_metric(),
            )),
            (Unit::Litre(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_litres(*value).as_cups_metric(),
            )),
            (Unit::Litre(value), UnitType::AustralianTablespoon) => Ok(Unit::AustralianTablespoon(
                Volume::from_litres(*value).as_tablespoons_aus(),
            )),
            (Unit::Litre(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_litres(*value).as_teaspoons_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_litres(*value).as_tablespoons_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialDessertspoon) => Ok(Unit::ImperialDessertspoon(
                Volume::from_litres(*value).as_dessertspoons_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_litres(*value).as_fluid_ounces_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_litres(*value).as_gills_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialCup) => {
                Ok(Unit::ImperialCup(Volume::from_litres(*value).as_cups_uk()))
            }
            (Unit::Litre(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_litres(*value).as_pints_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_litres(*value).as_quarts_uk(),
            )),
            (Unit::Litre(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_litres(*value).as_gallons_uk(),
            )),
            (Unit::Litre(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_litres(*value).as_cups_legal(),
            )),
            (Unit::Litre(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_litres(*value).as_fluid_ounces(),
            )),
            (Unit::Litre(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_litres(*value).as_cups()))
            }
            (Unit::Litre(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_litres(*value).as_pints()))
            }
            (Unit::Litre(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_litres(*value).as_quarts()))
            }
            (Unit::Litre(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_litres(*value).as_gallons()))
            }
            (Unit::Litre(value), UnitType::USTeaspoon) => {
                Ok(Unit::USTeaspoon(Volume::from_litres(*value).as_teaspoons()))
            }
            (Unit::Litre(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_litres(*value).as_tablespoons(),
            )),
            (Unit::Litre(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_litres(*value).as_jiggers()))
            }

            // From Metric Teaspoon
            (Unit::MetricTeaspoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_teaspoons_metric(*value).as_millilitres(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_teaspoons_metric(*value).as_centilitres(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_teaspoons_metric(*value).as_decilitres(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::Litre) => Ok(Unit::Litre(
                Volume::from_teaspoons_metric(*value).as_litres(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::MetricTeaspoon) => {
                Ok(Unit::MetricTeaspoon(*value))
            }
            (Unit::MetricTeaspoon(value), UnitType::MetricTablespoon) => {
                Ok(Unit::MetricTablespoon(
                    Volume::from_teaspoons_metric(*value).as_tablespoons_metric(),
                ))
            }
            (Unit::MetricTeaspoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_teaspoons_metric(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::MetricTeaspoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_teaspoons_metric(*value).as_cups_metric(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::AustralianTablespoon) => {
                Ok(Unit::AustralianTablespoon(
                    Volume::from_teaspoons_metric(*value).as_tablespoons_aus(),
                ))
            }
            (Unit::MetricTeaspoon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_teaspoons_metric(*value).as_teaspoons_uk()),
            ),
            (Unit::MetricTeaspoon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_teaspoons_metric(*value).as_tablespoons_uk()),
            ),
            (Unit::MetricTeaspoon(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(
                    Volume::from_teaspoons_metric(*value).as_dessertspoons_uk(),
                ))
            }
            (Unit::MetricTeaspoon(value), UnitType::ImperialFluidOunce) => {
                Ok(Unit::ImperialFluidOunce(
                    Volume::from_teaspoons_metric(*value).as_fluid_ounces_uk(),
                ))
            }
            (Unit::MetricTeaspoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_teaspoons_metric(*value).as_gills_uk(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_teaspoons_metric(*value).as_cups_uk(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_teaspoons_metric(*value).as_pints_uk(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_teaspoons_metric(*value).as_quarts_uk(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_teaspoons_metric(*value).as_gallons_uk(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_teaspoons_metric(*value).as_cups_legal(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_teaspoons_metric(*value).as_fluid_ounces(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_teaspoons_metric(*value).as_cups()))
            }
            (Unit::MetricTeaspoon(value), UnitType::USPint) => Ok(Unit::USPint(
                Volume::from_teaspoons_metric(*value).as_pints(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_teaspoons_metric(*value).as_quarts(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_teaspoons_metric(*value).as_gallons(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_teaspoons_metric(*value).as_teaspoons(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_teaspoons_metric(*value).as_tablespoons(),
            )),
            (Unit::MetricTeaspoon(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_teaspoons_metric(*value).as_jiggers(),
            )),

            // From Metric Tablespoon
            (Unit::MetricTablespoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_tablespoons_metric(*value).as_millilitres(),
            )),
            (Unit::MetricTablespoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_tablespoons_metric(*value).as_centilitres(),
            )),
            (Unit::MetricTablespoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_tablespoons_metric(*value).as_decilitres(),
            )),
            (Unit::MetricTablespoon(value), UnitType::Litre) => Ok(Unit::Litre(
                Volume::from_tablespoons_metric(*value).as_litres(),
            )),
            (Unit::MetricTablespoon(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_tablespoons_metric(*value).as_teaspoons_metric(),
            )),
            (Unit::MetricTablespoon(value), UnitType::MetricTablespoon) => {
                Ok(Unit::MetricTablespoon(*value))
            }
            (Unit::MetricTablespoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_tablespoons_metric(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::MetricTablespoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_tablespoons_metric(*value).as_cups_metric(),
            )),
            (Unit::MetricTablespoon(value), UnitType::AustralianTablespoon) => {
                Ok(Unit::AustralianTablespoon(
                    Volume::from_tablespoons_metric(*value).as_tablespoons_aus(),
                ))
            }
            (Unit::MetricTablespoon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_tablespoons_metric(*value).as_teaspoons_uk()),
            ),
            (Unit::MetricTablespoon(value), UnitType::ImperialTablespoon) => {
                Ok(Unit::ImperialTablespoon(
                    Volume::from_tablespoons_metric(*value).as_tablespoons_uk(),
                ))
            }
            (Unit::MetricTablespoon(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(
                    Volume::from_tablespoons_metric(*value).as_dessertspoons_uk(),
                ))
            }
            (Unit::MetricTablespoon(value), UnitType::ImperialFluidOunce) => {
                Ok(Unit::ImperialFluidOunce(
                    Volume::from_tablespoons_metric(*value).as_fluid_ounces_uk(),
                ))
            }
            (Unit::MetricTablespoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_tablespoons_metric(*value).as_gills_uk(),
            )),
            (Unit::MetricTablespoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_tablespoons_metric(*value).as_cups_uk(),
            )),
            (Unit::MetricTablespoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_tablespoons_metric(*value).as_pints_uk(),
            )),
            (Unit::MetricTablespoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_tablespoons_metric(*value).as_quarts_uk(),
            )),
            (Unit::MetricTablespoon(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_tablespoons_metric(*value).as_gallons_uk(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_tablespoons_metric(*value).as_cups_legal(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_tablespoons_metric(*value).as_fluid_ounces(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USCup) => Ok(Unit::USCup(
                Volume::from_tablespoons_metric(*value).as_cups(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USPint) => Ok(Unit::USPint(
                Volume::from_tablespoons_metric(*value).as_pints(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_tablespoons_metric(*value).as_quarts(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_tablespoons_metric(*value).as_gallons(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_tablespoons_metric(*value).as_teaspoons(),
            )),
            (Unit::MetricTablespoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_tablespoons_metric(*value).as_tablespoons(),
            )),
            (Unit::MetricTablespoon(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_tablespoons_metric(*value).as_jiggers(),
            )),

            // From Metric Dessert Spoon
            (Unit::MetricDessertSpoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_dessertspoons_metric(*value).as_millilitres(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_dessertspoons_metric(*value).as_centilitres(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_dessertspoons_metric(*value).as_decilitres(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::Litre) => Ok(Unit::Litre(
                Volume::from_dessertspoons_metric(*value).as_litres(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::MetricTeaspoon) => {
                Ok(Unit::MetricTeaspoon(
                    Volume::from_dessertspoons_metric(*value).as_teaspoons_metric(),
                ))
            }
            (Unit::MetricDessertSpoon(value), UnitType::MetricTablespoon) => {
                Ok(Unit::MetricTablespoon(
                    Volume::from_dessertspoons_metric(*value).as_tablespoons_metric(),
                ))
            }
            (Unit::MetricDessertSpoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(*value))
            }
            (Unit::MetricDessertSpoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_dessertspoons_metric(*value).as_cups_metric(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::AustralianTablespoon) => {
                Ok(Unit::AustralianTablespoon(
                    Volume::from_dessertspoons_metric(*value).as_tablespoons_aus(),
                ))
            }
            (Unit::MetricDessertSpoon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_dessertspoons_metric(*value).as_teaspoons_uk()),
            ),
            (Unit::MetricDessertSpoon(value), UnitType::ImperialTablespoon) => {
                Ok(Unit::ImperialTablespoon(
                    Volume::from_dessertspoons_metric(*value).as_tablespoons_uk(),
                ))
            }
            (Unit::MetricDessertSpoon(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(
                    Volume::from_dessertspoons_metric(*value).as_dessertspoons_uk(),
                ))
            }
            (Unit::MetricDessertSpoon(value), UnitType::ImperialFluidOunce) => {
                Ok(Unit::ImperialFluidOunce(
                    Volume::from_dessertspoons_metric(*value).as_fluid_ounces_uk(),
                ))
            }
            (Unit::MetricDessertSpoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_dessertspoons_metric(*value).as_gills_uk(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_dessertspoons_metric(*value).as_cups_uk(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_dessertspoons_metric(*value).as_pints_uk(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_dessertspoons_metric(*value).as_quarts_uk(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::ImperialGallon) => Ok(
                Unit::ImperialGallon(Volume::from_dessertspoons_metric(*value).as_gallons_uk()),
            ),
            (Unit::MetricDessertSpoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_dessertspoons_metric(*value).as_cups_legal(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_dessertspoons_metric(*value).as_fluid_ounces(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USCup) => Ok(Unit::USCup(
                Volume::from_dessertspoons_metric(*value).as_cups(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USPint) => Ok(Unit::USPint(
                Volume::from_dessertspoons_metric(*value).as_pints(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_dessertspoons_metric(*value).as_quarts(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_dessertspoons_metric(*value).as_gallons(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_dessertspoons_metric(*value).as_teaspoons(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_dessertspoons_metric(*value).as_tablespoons(),
            )),
            (Unit::MetricDessertSpoon(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_dessertspoons_metric(*value).as_jiggers(),
            )),

            // From Metric Cup
            (Unit::MetricCup(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_cups_metric(*value).as_millilitres(),
            )),
            (Unit::MetricCup(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_cups_metric(*value).as_centilitres(),
            )),
            (Unit::MetricCup(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_cups_metric(*value).as_decilitres(),
            )),
            (Unit::MetricCup(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_cups_metric(*value).as_litres()))
            }
            (Unit::MetricCup(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_cups_metric(*value).as_teaspoons_metric(),
            )),
            (Unit::MetricCup(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_cups_metric(*value).as_tablespoons_metric(),
            )),
            (Unit::MetricCup(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_cups_metric(*value).as_dessertspoons_metric(),
            )),
            (Unit::MetricCup(value), UnitType::MetricCup) => Ok(Unit::MetricCup(*value)),
            (Unit::MetricCup(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_cups_metric(*value).as_tablespoons_aus()),
            ),
            (Unit::MetricCup(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_cups_metric(*value).as_teaspoons_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_cups_metric(*value).as_tablespoons_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_cups_metric(*value).as_dessertspoons_uk()),
            ),
            (Unit::MetricCup(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_cups_metric(*value).as_fluid_ounces_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_cups_metric(*value).as_gills_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_cups_metric(*value).as_cups_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_cups_metric(*value).as_pints_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_cups_metric(*value).as_quarts_uk(),
            )),
            (Unit::MetricCup(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_cups_metric(*value).as_gallons_uk(),
            )),
            (Unit::MetricCup(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_cups_metric(*value).as_cups_legal(),
            )),
            (Unit::MetricCup(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_cups_metric(*value).as_fluid_ounces(),
            )),
            (Unit::MetricCup(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_cups_metric(*value).as_cups()))
            }
            (Unit::MetricCup(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_cups_metric(*value).as_pints()))
            }
            (Unit::MetricCup(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_cups_metric(*value).as_quarts()))
            }
            (Unit::MetricCup(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_cups_metric(*value).as_gallons(),
            )),
            (Unit::MetricCup(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_cups_metric(*value).as_teaspoons(),
            )),
            (Unit::MetricCup(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_cups_metric(*value).as_tablespoons(),
            )),
            (Unit::MetricCup(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_cups_metric(*value).as_jiggers()))
            }

            // From Australian Tablespoon
            (Unit::AustralianTablespoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_tablespoons_aus(*value).as_millilitres(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_tablespoons_aus(*value).as_centilitres(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_tablespoons_aus(*value).as_decilitres(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::Litre) => Ok(Unit::Litre(
                Volume::from_tablespoons_aus(*value).as_litres(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::MetricTeaspoon) => Ok(
                Unit::MetricTeaspoon(Volume::from_tablespoons_aus(*value).as_teaspoons_metric()),
            ),
            (Unit::AustralianTablespoon(value), UnitType::MetricTablespoon) => {
                Ok(Unit::MetricTablespoon(
                    Volume::from_tablespoons_aus(*value).as_tablespoons_metric(),
                ))
            }
            (Unit::AustralianTablespoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_tablespoons_aus(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::AustralianTablespoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_tablespoons_aus(*value).as_cups_metric(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::AustralianTablespoon) => {
                Ok(Unit::AustralianTablespoon(*value))
            }
            (Unit::AustralianTablespoon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_tablespoons_aus(*value).as_teaspoons_uk()),
            ),
            (Unit::AustralianTablespoon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_tablespoons_aus(*value).as_tablespoons_uk()),
            ),
            (Unit::AustralianTablespoon(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(
                    Volume::from_tablespoons_aus(*value).as_dessertspoons_uk(),
                ))
            }
            (Unit::AustralianTablespoon(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_tablespoons_aus(*value).as_fluid_ounces_uk()),
            ),
            (Unit::AustralianTablespoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_tablespoons_aus(*value).as_gills_uk(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_tablespoons_aus(*value).as_cups_uk(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_tablespoons_aus(*value).as_pints_uk(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::ImperialQuart) => Ok(
                Unit::ImperialQuart(Volume::from_tablespoons_aus(*value).as_quarts_uk()),
            ),
            (Unit::AustralianTablespoon(value), UnitType::ImperialGallon) => Ok(
                Unit::ImperialGallon(Volume::from_tablespoons_aus(*value).as_gallons_uk()),
            ),
            (Unit::AustralianTablespoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_tablespoons_aus(*value).as_cups_legal(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_tablespoons_aus(*value).as_fluid_ounces(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_tablespoons_aus(*value).as_cups()))
            }
            (Unit::AustralianTablespoon(value), UnitType::USPint) => Ok(Unit::USPint(
                Volume::from_tablespoons_aus(*value).as_pints(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_tablespoons_aus(*value).as_quarts(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_tablespoons_aus(*value).as_gallons(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_tablespoons_aus(*value).as_teaspoons(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_tablespoons_aus(*value).as_tablespoons(),
            )),
            (Unit::AustralianTablespoon(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_tablespoons_aus(*value).as_jiggers(),
            )),

            // From Imperial Teaspoon
            (Unit::ImperialTeaspoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_teaspoons_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_teaspoons_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_teaspoons_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_teaspoons_uk(*value).as_litres()))
            }
            (Unit::ImperialTeaspoon(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_teaspoons_uk(*value).as_teaspoons_metric(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::MetricTablespoon) => Ok(
                Unit::MetricTablespoon(Volume::from_teaspoons_uk(*value).as_tablespoons_metric()),
            ),
            (Unit::ImperialTeaspoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_teaspoons_uk(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::ImperialTeaspoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_teaspoons_uk(*value).as_cups_metric(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_teaspoons_uk(*value).as_tablespoons_aus()),
            ),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialTeaspoon) => {
                Ok(Unit::ImperialTeaspoon(*value))
            }
            (Unit::ImperialTeaspoon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_teaspoons_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_teaspoons_uk(*value).as_dessertspoons_uk()),
            ),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_teaspoons_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_teaspoons_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_teaspoons_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_teaspoons_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_teaspoons_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_teaspoons_uk(*value).as_gallons_uk(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_teaspoons_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_teaspoons_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_teaspoons_uk(*value).as_cups()))
            }
            (Unit::ImperialTeaspoon(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_teaspoons_uk(*value).as_pints()))
            }
            (Unit::ImperialTeaspoon(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_teaspoons_uk(*value).as_quarts()))
            }
            (Unit::ImperialTeaspoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_teaspoons_uk(*value).as_gallons(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_teaspoons_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_teaspoons_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialTeaspoon(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_teaspoons_uk(*value).as_jiggers()))
            }

            // From Imperial Dessertspoon
            (Unit::ImperialDessertspoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_dessertspoons_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_dessertspoons_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_dessertspoons_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::Litre) => Ok(Unit::Litre(
                Volume::from_dessertspoons_uk(*value).as_litres(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::MetricTeaspoon) => Ok(
                Unit::MetricTeaspoon(Volume::from_dessertspoons_uk(*value).as_teaspoons_metric()),
            ),
            (Unit::ImperialDessertspoon(value), UnitType::MetricTablespoon) => {
                Ok(Unit::MetricTablespoon(
                    Volume::from_dessertspoons_uk(*value).as_tablespoons_metric(),
                ))
            }
            (Unit::ImperialDessertspoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_dessertspoons_uk(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::ImperialDessertspoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_dessertspoons_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_dessertspoons_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_dessertspoons_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_dessertspoons_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(*value))
            }
            (Unit::ImperialDessertspoon(value), UnitType::ImperialFluidOunce) => {
                Ok(Unit::ImperialFluidOunce(
                    Volume::from_dessertspoons_uk(*value).as_fluid_ounces_uk(),
                ))
            }
            (Unit::ImperialDessertspoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_dessertspoons_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_dessertspoons_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_dessertspoons_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialQuart) => Ok(
                Unit::ImperialQuart(Volume::from_dessertspoons_uk(*value).as_quarts_uk()),
            ),
            (Unit::ImperialDessertspoon(value), UnitType::ImperialGallon) => Ok(
                Unit::ImperialGallon(Volume::from_dessertspoons_uk(*value).as_gallons_uk()),
            ),
            (Unit::ImperialDessertspoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_dessertspoons_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_dessertspoons_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_dessertspoons_uk(*value).as_cups()))
            }
            (Unit::ImperialDessertspoon(value), UnitType::USPint) => Ok(Unit::USPint(
                Volume::from_dessertspoons_uk(*value).as_pints(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_dessertspoons_uk(*value).as_quarts(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_dessertspoons_uk(*value).as_gallons(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_dessertspoons_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_dessertspoons_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialDessertspoon(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_dessertspoons_uk(*value).as_jiggers(),
            )),

            // From Imperial Tablespoon
            (Unit::ImperialTablespoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_tablespoons_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_tablespoons_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_tablespoons_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_tablespoons_uk(*value).as_litres()))
            }
            (Unit::ImperialTablespoon(value), UnitType::MetricTeaspoon) => Ok(
                Unit::MetricTeaspoon(Volume::from_tablespoons_uk(*value).as_teaspoons_metric()),
            ),
            (Unit::ImperialTablespoon(value), UnitType::MetricTablespoon) => Ok(
                Unit::MetricTablespoon(Volume::from_tablespoons_uk(*value).as_tablespoons_metric()),
            ),
            (Unit::ImperialTablespoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_tablespoons_uk(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::ImperialTablespoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_tablespoons_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_tablespoons_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialTablespoon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_tablespoons_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialTablespoon(value), UnitType::ImperialTablespoon) => {
                Ok(Unit::ImperialTablespoon(*value))
            }
            (Unit::ImperialTablespoon(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(
                    Volume::from_tablespoons_uk(*value).as_dessertspoons_uk(),
                ))
            }
            (Unit::ImperialTablespoon(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_tablespoons_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialTablespoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_tablespoons_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_tablespoons_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_tablespoons_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_tablespoons_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::ImperialGallon) => Ok(
                Unit::ImperialGallon(Volume::from_tablespoons_uk(*value).as_gallons_uk()),
            ),
            (Unit::ImperialTablespoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_tablespoons_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_tablespoons_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_tablespoons_uk(*value).as_cups()))
            }
            (Unit::ImperialTablespoon(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_tablespoons_uk(*value).as_pints()))
            }
            (Unit::ImperialTablespoon(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_tablespoons_uk(*value).as_quarts(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_tablespoons_uk(*value).as_gallons(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_tablespoons_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_tablespoons_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialTablespoon(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_tablespoons_uk(*value).as_jiggers(),
            )),

            // From Imperial Fluid Ounce
            (Unit::ImperialFluidOunce(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_fluid_ounces_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_fluid_ounces_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_fluid_ounces_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::Litre) => Ok(Unit::Litre(
                Volume::from_fluid_ounces_uk(*value).as_litres(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::MetricTeaspoon) => Ok(
                Unit::MetricTeaspoon(Volume::from_fluid_ounces_uk(*value).as_teaspoons_metric()),
            ),
            (Unit::ImperialFluidOunce(value), UnitType::MetricTablespoon) => {
                Ok(Unit::MetricTablespoon(
                    Volume::from_fluid_ounces_uk(*value).as_tablespoons_metric(),
                ))
            }
            (Unit::ImperialFluidOunce(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_fluid_ounces_uk(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::ImperialFluidOunce(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_fluid_ounces_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_fluid_ounces_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_fluid_ounces_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_fluid_ounces_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialDessertspoon) => {
                Ok(Unit::ImperialDessertspoon(
                    Volume::from_fluid_ounces_uk(*value).as_dessertspoons_uk(),
                ))
            }
            (Unit::ImperialFluidOunce(value), UnitType::ImperialFluidOunce) => {
                Ok(Unit::ImperialFluidOunce(*value))
            }
            (Unit::ImperialFluidOunce(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_fluid_ounces_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_fluid_ounces_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_fluid_ounces_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_fluid_ounces_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::ImperialGallon) => Ok(
                Unit::ImperialGallon(Volume::from_fluid_ounces_uk(*value).as_gallons_uk()),
            ),
            (Unit::ImperialFluidOunce(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_fluid_ounces_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_fluid_ounces_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_fluid_ounces_uk(*value).as_cups()))
            }
            (Unit::ImperialFluidOunce(value), UnitType::USPint) => Ok(Unit::USPint(
                Volume::from_fluid_ounces_uk(*value).as_pints(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::USQuart) => Ok(Unit::USQuart(
                Volume::from_fluid_ounces_uk(*value).as_quarts(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_fluid_ounces_uk(*value).as_gallons(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_fluid_ounces_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_fluid_ounces_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialFluidOunce(value), UnitType::Jigger) => Ok(Unit::Jigger(
                Volume::from_fluid_ounces_uk(*value).as_jiggers(),
            )),

            // From Imperial Gill
            (Unit::ImperialGill(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_gills_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialGill(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_gills_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialGill(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_gills_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialGill(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_gills_uk(*value).as_litres()))
            }
            (Unit::ImperialGill(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_gills_uk(*value).as_teaspoons_metric(),
            )),
            (Unit::ImperialGill(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_gills_uk(*value).as_tablespoons_metric(),
            )),
            (Unit::ImperialGill(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_gills_uk(*value).as_dessertspoons_metric()),
            ),
            (Unit::ImperialGill(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_gills_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialGill(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_gills_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialGill(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_gills_uk(*value).as_teaspoons_uk(),
            )),
            (Unit::ImperialGill(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_gills_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialGill(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_gills_uk(*value).as_dessertspoons_uk()),
            ),
            (Unit::ImperialGill(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_gills_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialGill(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(*value)),
            (Unit::ImperialGill(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_gills_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialGill(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_gills_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialGill(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_gills_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialGill(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_gills_uk(*value).as_gallons_uk(),
            )),
            (Unit::ImperialGill(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_gills_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialGill(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_gills_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialGill(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_gills_uk(*value).as_cups()))
            }
            (Unit::ImperialGill(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_gills_uk(*value).as_pints()))
            }
            (Unit::ImperialGill(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_gills_uk(*value).as_quarts()))
            }
            (Unit::ImperialGill(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_gills_uk(*value).as_gallons()))
            }
            (Unit::ImperialGill(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_gills_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialGill(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_gills_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialGill(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_gills_uk(*value).as_jiggers()))
            }

            // From Imperial Cup
            (Unit::ImperialCup(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_cups_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialCup(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_cups_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialCup(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_cups_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialCup(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_cups_uk(*value).as_litres()))
            }
            (Unit::ImperialCup(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_cups_uk(*value).as_teaspoons_metric(),
            )),
            (Unit::ImperialCup(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_cups_uk(*value).as_tablespoons_metric(),
            )),
            (Unit::ImperialCup(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_cups_uk(*value).as_dessertspoons_metric()),
            ),
            (Unit::ImperialCup(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_cups_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialCup(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_cups_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialCup(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_cups_uk(*value).as_teaspoons_uk(),
            )),
            (Unit::ImperialCup(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_cups_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialCup(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_cups_uk(*value).as_dessertspoons_uk()),
            ),
            (Unit::ImperialCup(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_cups_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialCup(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_cups_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialCup(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(*value)),
            (Unit::ImperialCup(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_cups_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialCup(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_cups_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialCup(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_cups_uk(*value).as_gallons_uk(),
            )),
            (Unit::ImperialCup(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_cups_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialCup(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_cups_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialCup(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_cups_uk(*value).as_cups()))
            }
            (Unit::ImperialCup(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_cups_uk(*value).as_pints()))
            }
            (Unit::ImperialCup(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_cups_uk(*value).as_quarts()))
            }
            (Unit::ImperialCup(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_cups_uk(*value).as_gallons()))
            }
            (Unit::ImperialCup(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_cups_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialCup(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_cups_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialCup(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_cups_uk(*value).as_jiggers()))
            }

            // From Imperial Pint
            (Unit::ImperialPint(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_pints_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialPint(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_pints_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialPint(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_pints_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialPint(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_pints_uk(*value).as_litres()))
            }
            (Unit::ImperialPint(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_pints_uk(*value).as_teaspoons_metric(),
            )),
            (Unit::ImperialPint(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_pints_uk(*value).as_tablespoons_metric(),
            )),
            (Unit::ImperialPint(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_pints_uk(*value).as_dessertspoons_metric()),
            ),
            (Unit::ImperialPint(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_pints_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialPint(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_pints_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialPint(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_pints_uk(*value).as_teaspoons_uk(),
            )),
            (Unit::ImperialPint(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_pints_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialPint(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_pints_uk(*value).as_dessertspoons_uk()),
            ),
            (Unit::ImperialPint(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_pints_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialPint(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_pints_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialPint(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_pints_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialPint(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(*value)),
            (Unit::ImperialPint(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_pints_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialPint(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_pints_uk(*value).as_gallons_uk(),
            )),
            (Unit::ImperialPint(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_pints_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialPint(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_pints_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialPint(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_pints_uk(*value).as_cups()))
            }
            (Unit::ImperialPint(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_pints_uk(*value).as_pints()))
            }
            (Unit::ImperialPint(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_pints_uk(*value).as_quarts()))
            }
            (Unit::ImperialPint(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_pints_uk(*value).as_gallons()))
            }
            (Unit::ImperialPint(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_pints_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialPint(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_pints_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialPint(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_pints_uk(*value).as_jiggers()))
            }

            // From Imperial Quart
            (Unit::ImperialQuart(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_quarts_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialQuart(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_quarts_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialQuart(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_quarts_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialQuart(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_quarts_uk(*value).as_litres()))
            }
            (Unit::ImperialQuart(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_quarts_uk(*value).as_teaspoons_metric(),
            )),
            (Unit::ImperialQuart(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_quarts_uk(*value).as_tablespoons_metric(),
            )),
            (Unit::ImperialQuart(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_quarts_uk(*value).as_dessertspoons_metric()),
            ),
            (Unit::ImperialQuart(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_quarts_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialQuart(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_quarts_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialQuart(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_quarts_uk(*value).as_teaspoons_uk(),
            )),
            (Unit::ImperialQuart(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_quarts_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialQuart(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_quarts_uk(*value).as_dessertspoons_uk()),
            ),
            (Unit::ImperialQuart(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_quarts_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialQuart(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_quarts_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialQuart(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_quarts_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialQuart(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_quarts_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialQuart(value), UnitType::ImperialQuart) => {
                Ok(Unit::ImperialQuart(*value))
            }
            (Unit::ImperialQuart(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_quarts_uk(*value).as_gallons_uk(),
            )),
            (Unit::ImperialQuart(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_quarts_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialQuart(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_quarts_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialQuart(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_quarts_uk(*value).as_cups()))
            }
            (Unit::ImperialQuart(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_quarts_uk(*value).as_pints()))
            }
            (Unit::ImperialQuart(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_quarts_uk(*value).as_quarts()))
            }
            (Unit::ImperialQuart(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_quarts_uk(*value).as_gallons()))
            }
            (Unit::ImperialQuart(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_quarts_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialQuart(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_quarts_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialQuart(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_quarts_uk(*value).as_jiggers()))
            }

            // From Imperial Gallon
            (Unit::ImperialGallon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_gallons_uk(*value).as_millilitres(),
            )),
            (Unit::ImperialGallon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_gallons_uk(*value).as_centilitres(),
            )),
            (Unit::ImperialGallon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_gallons_uk(*value).as_decilitres(),
            )),
            (Unit::ImperialGallon(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_gallons_uk(*value).as_litres()))
            }
            (Unit::ImperialGallon(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_gallons_uk(*value).as_teaspoons_metric(),
            )),
            (Unit::ImperialGallon(value), UnitType::MetricTablespoon) => Ok(
                Unit::MetricTablespoon(Volume::from_gallons_uk(*value).as_tablespoons_metric()),
            ),
            (Unit::ImperialGallon(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_gallons_uk(*value).as_dessertspoons_metric()),
            ),
            (Unit::ImperialGallon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_gallons_uk(*value).as_tablespoons_aus(),
            )),
            (Unit::ImperialGallon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_gallons_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialGallon(value), UnitType::ImperialTeaspoon) => Ok(
                Unit::ImperialTeaspoon(Volume::from_gallons_uk(*value).as_teaspoons_uk()),
            ),
            (Unit::ImperialGallon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_gallons_uk(*value).as_tablespoons_uk()),
            ),
            (Unit::ImperialGallon(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_gallons_uk(*value).as_dessertspoons_uk()),
            ),
            (Unit::ImperialGallon(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_gallons_uk(*value).as_fluid_ounces_uk()),
            ),
            (Unit::ImperialGallon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_gallons_uk(*value).as_gills_uk(),
            )),
            (Unit::ImperialGallon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_gallons_uk(*value).as_cups_uk(),
            )),
            (Unit::ImperialGallon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_gallons_uk(*value).as_pints_uk(),
            )),
            (Unit::ImperialGallon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_gallons_uk(*value).as_quarts_uk(),
            )),
            (Unit::ImperialGallon(value), UnitType::ImperialGallon) => {
                Ok(Unit::ImperialGallon(*value))
            }
            (Unit::ImperialGallon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_gallons_uk(*value).as_cups_legal(),
            )),
            (Unit::ImperialGallon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_gallons_uk(*value).as_fluid_ounces(),
            )),
            (Unit::ImperialGallon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_gallons_uk(*value).as_cups()))
            }
            (Unit::ImperialGallon(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_gallons_uk(*value).as_pints()))
            }
            (Unit::ImperialGallon(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_gallons_uk(*value).as_quarts()))
            }
            (Unit::ImperialGallon(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_gallons_uk(*value).as_gallons()))
            }
            (Unit::ImperialGallon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_gallons_uk(*value).as_teaspoons(),
            )),
            (Unit::ImperialGallon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_gallons_uk(*value).as_tablespoons(),
            )),
            (Unit::ImperialGallon(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_gallons_uk(*value).as_jiggers()))
            }

            // From US Legal Cup
            (Unit::USLegalCup(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_cups_legal(*value).as_millilitres(),
            )),
            (Unit::USLegalCup(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_cups_legal(*value).as_centilitres(),
            )),
            (Unit::USLegalCup(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_cups_legal(*value).as_decilitres(),
            )),
            (Unit::USLegalCup(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_cups_legal(*value).as_litres()))
            }
            (Unit::USLegalCup(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_cups_legal(*value).as_teaspoons_metric(),
            )),
            (Unit::USLegalCup(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_cups_legal(*value).as_tablespoons_metric(),
            )),
            (Unit::USLegalCup(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_cups_legal(*value).as_dessertspoons_metric()),
            ),
            (Unit::USLegalCup(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_cups_legal(*value).as_tablespoons_aus(),
            )),
            (Unit::USLegalCup(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_cups_legal(*value).as_teaspoons_uk()),
            ),
            (Unit::USLegalCup(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_cups_legal(*value).as_teaspoons_uk(),
            )),
            (Unit::USLegalCup(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_cups_legal(*value).as_tablespoons_uk()),
            ),
            (Unit::USLegalCup(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_cups_legal(*value).as_dessertspoons_uk()),
            ),
            (Unit::USLegalCup(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_cups_legal(*value).as_fluid_ounces_uk()),
            ),
            (Unit::USLegalCup(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_cups_legal(*value).as_gills_uk(),
            )),
            (Unit::USLegalCup(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_cups_legal(*value).as_cups_uk(),
            )),
            (Unit::USLegalCup(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_cups_legal(*value).as_pints_uk(),
            )),
            (Unit::USLegalCup(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_cups_legal(*value).as_quarts_uk(),
            )),
            (Unit::USLegalCup(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_cups_legal(*value).as_gallons_uk(),
            )),
            (Unit::USLegalCup(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(*value)),
            (Unit::USLegalCup(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_cups_legal(*value).as_fluid_ounces(),
            )),
            (Unit::USLegalCup(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_cups_legal(*value).as_cups()))
            }
            (Unit::USLegalCup(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_cups_legal(*value).as_pints()))
            }
            (Unit::USLegalCup(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_cups_legal(*value).as_quarts()))
            }
            (Unit::USLegalCup(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_cups_legal(*value).as_gallons()))
            }
            (Unit::USLegalCup(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_cups_legal(*value).as_teaspoons(),
            )),
            (Unit::USLegalCup(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_cups_legal(*value).as_tablespoons(),
            )),
            (Unit::USLegalCup(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_cups_legal(*value).as_jiggers()))
            }

            // From US Teaspoon
            (Unit::USTeaspoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_teaspoons(*value).as_millilitres(),
            )),
            (Unit::USTeaspoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_teaspoons(*value).as_centilitres(),
            )),
            (Unit::USTeaspoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_teaspoons(*value).as_decilitres(),
            )),
            (Unit::USTeaspoon(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_teaspoons(*value).as_litres()))
            }
            (Unit::USTeaspoon(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_teaspoons(*value).as_teaspoons_metric(),
            )),
            (Unit::USTeaspoon(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_teaspoons(*value).as_tablespoons_metric(),
            )),
            (Unit::USTeaspoon(value), UnitType::MetricDessertSpoon) => Ok(
                Unit::MetricDessertSpoon(Volume::from_teaspoons(*value).as_dessertspoons_metric()),
            ),
            (Unit::USTeaspoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_teaspoons(*value).as_tablespoons_aus(),
            )),
            (Unit::USTeaspoon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_teaspoons(*value).as_teaspoons_uk()),
            ),
            (Unit::USTeaspoon(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_teaspoons(*value).as_teaspoons_uk(),
            )),
            (Unit::USTeaspoon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_teaspoons(*value).as_tablespoons_uk()),
            ),
            (Unit::USTeaspoon(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_teaspoons(*value).as_dessertspoons_uk()),
            ),
            (Unit::USTeaspoon(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_teaspoons(*value).as_fluid_ounces_uk()),
            ),
            (Unit::USTeaspoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_teaspoons(*value).as_gills_uk(),
            )),
            (Unit::USTeaspoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_teaspoons(*value).as_cups_uk(),
            )),
            (Unit::USTeaspoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_teaspoons(*value).as_pints_uk(),
            )),
            (Unit::USTeaspoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_teaspoons(*value).as_quarts_uk(),
            )),
            (Unit::USTeaspoon(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_teaspoons(*value).as_cups_legal(),
            )),
            (Unit::USTeaspoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_teaspoons(*value).as_cups_legal(),
            )),
            (Unit::USTeaspoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_teaspoons(*value).as_fluid_ounces(),
            )),
            (Unit::USTeaspoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_teaspoons(*value).as_cups()))
            }
            (Unit::USTeaspoon(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_teaspoons(*value).as_pints()))
            }
            (Unit::USTeaspoon(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_teaspoons(*value).as_quarts()))
            }
            (Unit::USTeaspoon(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_teaspoons(*value).as_gallons()))
            }
            (Unit::USTeaspoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(*value)),
            (Unit::USTeaspoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_teaspoons(*value).as_tablespoons(),
            )),
            (Unit::USTeaspoon(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_teaspoons(*value).as_jiggers()))
            }

            // From US Tablespoon
            (Unit::USTablespoon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_tablespoons(*value).as_millilitres(),
            )),
            (Unit::USTablespoon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_tablespoons(*value).as_centilitres(),
            )),
            (Unit::USTablespoon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_tablespoons(*value).as_decilitres(),
            )),
            (Unit::USTablespoon(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_tablespoons(*value).as_litres()))
            }
            (Unit::USTablespoon(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_tablespoons(*value).as_teaspoons_metric(),
            )),
            (Unit::USTablespoon(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_tablespoons(*value).as_tablespoons_metric(),
            )),
            (Unit::USTablespoon(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_tablespoons(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::USTablespoon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_tablespoons(*value).as_tablespoons_aus(),
            )),
            (Unit::USTablespoon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_tablespoons(*value).as_teaspoons_uk()),
            ),
            (Unit::USTablespoon(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_tablespoons(*value).as_teaspoons_uk(),
            )),
            (Unit::USTablespoon(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_tablespoons(*value).as_tablespoons_uk()),
            ),
            (Unit::USTablespoon(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_tablespoons(*value).as_dessertspoons_uk()),
            ),
            (Unit::USTablespoon(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_tablespoons(*value).as_fluid_ounces_uk()),
            ),
            (Unit::USTablespoon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_tablespoons(*value).as_gills_uk(),
            )),
            (Unit::USTablespoon(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_tablespoons(*value).as_cups_uk(),
            )),
            (Unit::USTablespoon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_tablespoons(*value).as_pints_uk(),
            )),
            (Unit::USTablespoon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_tablespoons(*value).as_quarts_uk(),
            )),
            (Unit::USTablespoon(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_tablespoons(*value).as_cups_legal(),
            )),
            (Unit::USTablespoon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_tablespoons(*value).as_cups_legal(),
            )),
            (Unit::USTablespoon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_tablespoons(*value).as_fluid_ounces(),
            )),
            (Unit::USTablespoon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_tablespoons(*value).as_cups()))
            }
            (Unit::USTablespoon(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_tablespoons(*value).as_pints()))
            }
            (Unit::USTablespoon(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_tablespoons(*value).as_quarts()))
            }
            (Unit::USTablespoon(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_tablespoons(*value).as_gallons(),
            )),
            (Unit::USTablespoon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_tablespoons(*value).as_teaspoons(),
            )),
            (Unit::USTablespoon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(*value)),
            (Unit::USTablespoon(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_tablespoons(*value).as_jiggers()))
            }

            // From US Fluid Ounce
            (Unit::USFluidOunce(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_fluid_ounces(*value).as_millilitres(),
            )),
            (Unit::USFluidOunce(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_fluid_ounces(*value).as_centilitres(),
            )),
            (Unit::USFluidOunce(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_fluid_ounces(*value).as_decilitres(),
            )),
            (Unit::USFluidOunce(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_fluid_ounces(*value).as_litres()))
            }
            (Unit::USFluidOunce(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_fluid_ounces(*value).as_teaspoons_metric(),
            )),
            (Unit::USFluidOunce(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_fluid_ounces(*value).as_tablespoons_metric(),
            )),
            (Unit::USFluidOunce(value), UnitType::MetricDessertSpoon) => {
                Ok(Unit::MetricDessertSpoon(
                    Volume::from_fluid_ounces(*value).as_dessertspoons_metric(),
                ))
            }
            (Unit::USFluidOunce(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_fluid_ounces(*value).as_tablespoons_aus(),
            )),
            (Unit::USFluidOunce(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_fluid_ounces(*value).as_teaspoons_uk()),
            ),
            (Unit::USFluidOunce(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_fluid_ounces(*value).as_teaspoons_uk(),
            )),
            (Unit::USFluidOunce(value), UnitType::ImperialTablespoon) => Ok(
                Unit::ImperialTablespoon(Volume::from_fluid_ounces(*value).as_tablespoons_uk()),
            ),
            (Unit::USFluidOunce(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_fluid_ounces(*value).as_dessertspoons_uk()),
            ),
            (Unit::USFluidOunce(value), UnitType::ImperialFluidOunce) => Ok(
                Unit::ImperialFluidOunce(Volume::from_fluid_ounces(*value).as_fluid_ounces_uk()),
            ),
            (Unit::USFluidOunce(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_fluid_ounces(*value).as_gills_uk(),
            )),
            (Unit::USFluidOunce(value), UnitType::ImperialCup) => Ok(Unit::ImperialCup(
                Volume::from_fluid_ounces(*value).as_cups_uk(),
            )),
            (Unit::USFluidOunce(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_fluid_ounces(*value).as_pints_uk(),
            )),
            (Unit::USFluidOunce(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_fluid_ounces(*value).as_quarts_uk(),
            )),
            (Unit::USFluidOunce(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_fluid_ounces(*value).as_cups_legal(),
            )),
            (Unit::USFluidOunce(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_fluid_ounces(*value).as_cups_legal(),
            )),
            (Unit::USFluidOunce(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(*value)),
            (Unit::USFluidOunce(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_fluid_ounces(*value).as_cups()))
            }
            (Unit::USFluidOunce(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_fluid_ounces(*value).as_pints()))
            }
            (Unit::USFluidOunce(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_fluid_ounces(*value).as_quarts()))
            }
            (Unit::USFluidOunce(value), UnitType::USGallon) => Ok(Unit::USGallon(
                Volume::from_fluid_ounces(*value).as_gallons(),
            )),
            (Unit::USFluidOunce(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_fluid_ounces(*value).as_teaspoons(),
            )),
            (Unit::USFluidOunce(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_fluid_ounces(*value).as_tablespoons(),
            )),
            (Unit::USFluidOunce(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_fluid_ounces(*value).as_jiggers()))
            }

            // From US Cup
            (Unit::USCup(value), UnitType::Millilitre) => {
                Ok(Unit::Millilitre(Volume::from_cups(*value).as_millilitres()))
            }
            (Unit::USCup(value), UnitType::Centilitre) => {
                Ok(Unit::Centilitre(Volume::from_cups(*value).as_centilitres()))
            }
            (Unit::USCup(value), UnitType::Decilitre) => {
                Ok(Unit::Decilitre(Volume::from_cups(*value).as_decilitres()))
            }
            (Unit::USCup(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_cups(*value).as_litres()))
            }
            (Unit::USCup(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_cups(*value).as_teaspoons_metric(),
            )),
            (Unit::USCup(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_cups(*value).as_tablespoons_metric(),
            )),
            (Unit::USCup(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_cups(*value).as_dessertspoons_metric(),
            )),
            (Unit::USCup(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_cups(*value).as_tablespoons_aus(),
            )),
            (Unit::USCup(value), UnitType::AustralianTablespoon) => Ok(Unit::AustralianTablespoon(
                Volume::from_cups(*value).as_teaspoons_uk(),
            )),
            (Unit::USCup(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_cups(*value).as_teaspoons_uk(),
            )),
            (Unit::USCup(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_cups(*value).as_tablespoons_uk(),
            )),
            (Unit::USCup(value), UnitType::ImperialDessertspoon) => Ok(Unit::ImperialDessertspoon(
                Volume::from_cups(*value).as_dessertspoons_uk(),
            )),
            (Unit::USCup(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_cups(*value).as_fluid_ounces_uk(),
            )),
            (Unit::USCup(value), UnitType::ImperialGill) => {
                Ok(Unit::ImperialGill(Volume::from_cups(*value).as_gills_uk()))
            }
            (Unit::USCup(value), UnitType::ImperialCup) => {
                Ok(Unit::ImperialCup(Volume::from_cups(*value).as_cups_uk()))
            }
            (Unit::USCup(value), UnitType::ImperialPint) => {
                Ok(Unit::ImperialPint(Volume::from_cups(*value).as_pints_uk()))
            }
            (Unit::USCup(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_cups(*value).as_quarts_uk(),
            )),
            (Unit::USCup(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_cups(*value).as_cups_legal(),
            )),
            (Unit::USCup(value), UnitType::USLegalCup) => {
                Ok(Unit::USLegalCup(Volume::from_cups(*value).as_cups_legal()))
            }
            (Unit::USCup(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_cups(*value).as_fluid_ounces(),
            )),
            (Unit::USCup(value), UnitType::USCup) => Ok(Unit::USCup(*value)),
            (Unit::USCup(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_cups(*value).as_pints()))
            }
            (Unit::USCup(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_cups(*value).as_quarts()))
            }
            (Unit::USCup(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_cups(*value).as_gallons()))
            }
            (Unit::USCup(value), UnitType::USTeaspoon) => {
                Ok(Unit::USTeaspoon(Volume::from_cups(*value).as_teaspoons()))
            }
            (Unit::USCup(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_cups(*value).as_tablespoons(),
            )),
            (Unit::USCup(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_cups(*value).as_jiggers()))
            }

            // From US Pint
            (Unit::USPint(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_pints(*value).as_millilitres(),
            )),
            (Unit::USPint(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_pints(*value).as_centilitres(),
            )),
            (Unit::USPint(value), UnitType::Decilitre) => {
                Ok(Unit::Decilitre(Volume::from_pints(*value).as_decilitres()))
            }
            (Unit::USPint(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_pints(*value).as_litres()))
            }
            (Unit::USPint(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_pints(*value).as_teaspoons_metric(),
            )),
            (Unit::USPint(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_pints(*value).as_tablespoons_metric(),
            )),
            (Unit::USPint(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_pints(*value).as_dessertspoons_metric(),
            )),
            (Unit::USPint(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_pints(*value).as_tablespoons_aus(),
            )),
            (Unit::USPint(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_pints(*value).as_teaspoons_uk()),
            ),
            (Unit::USPint(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_pints(*value).as_teaspoons_uk(),
            )),
            (Unit::USPint(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_pints(*value).as_tablespoons_uk(),
            )),
            (Unit::USPint(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_pints(*value).as_dessertspoons_uk()),
            ),
            (Unit::USPint(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_pints(*value).as_fluid_ounces_uk(),
            )),
            (Unit::USPint(value), UnitType::ImperialGill) => {
                Ok(Unit::ImperialGill(Volume::from_pints(*value).as_gills_uk()))
            }
            (Unit::USPint(value), UnitType::ImperialCup) => {
                Ok(Unit::ImperialCup(Volume::from_pints(*value).as_cups_uk()))
            }
            (Unit::USPint(value), UnitType::ImperialPint) => {
                Ok(Unit::ImperialPint(Volume::from_pints(*value).as_pints_uk()))
            }
            (Unit::USPint(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_pints(*value).as_quarts_uk(),
            )),
            (Unit::USPint(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_pints(*value).as_cups_legal(),
            )),
            (Unit::USPint(value), UnitType::USLegalCup) => {
                Ok(Unit::USLegalCup(Volume::from_pints(*value).as_cups_legal()))
            }
            (Unit::USPint(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_pints(*value).as_fluid_ounces(),
            )),
            (Unit::USPint(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_pints(*value).as_cups()))
            }
            (Unit::USPint(value), UnitType::USPint) => Ok(Unit::USPint(*value)),
            (Unit::USPint(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_pints(*value).as_quarts()))
            }
            (Unit::USPint(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_pints(*value).as_gallons()))
            }
            (Unit::USPint(value), UnitType::USTeaspoon) => {
                Ok(Unit::USTeaspoon(Volume::from_pints(*value).as_teaspoons()))
            }
            (Unit::USPint(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_pints(*value).as_tablespoons(),
            )),
            (Unit::USPint(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_pints(*value).as_jiggers()))
            }

            // From US Quart
            (Unit::USQuart(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_quarts(*value).as_millilitres(),
            )),
            (Unit::USQuart(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_quarts(*value).as_centilitres(),
            )),
            (Unit::USQuart(value), UnitType::Decilitre) => {
                Ok(Unit::Decilitre(Volume::from_quarts(*value).as_decilitres()))
            }
            (Unit::USQuart(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_quarts(*value).as_litres()))
            }
            (Unit::USQuart(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_quarts(*value).as_teaspoons_metric(),
            )),
            (Unit::USQuart(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_quarts(*value).as_tablespoons_metric(),
            )),
            (Unit::USQuart(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_quarts(*value).as_dessertspoons_metric(),
            )),
            (Unit::USQuart(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_quarts(*value).as_tablespoons_aus(),
            )),
            (Unit::USQuart(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_quarts(*value).as_teaspoons_uk()),
            ),
            (Unit::USQuart(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_quarts(*value).as_teaspoons_uk(),
            )),
            (Unit::USQuart(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_quarts(*value).as_tablespoons_uk(),
            )),
            (Unit::USQuart(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_quarts(*value).as_dessertspoons_uk()),
            ),
            (Unit::USQuart(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_quarts(*value).as_fluid_ounces_uk(),
            )),
            (Unit::USQuart(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_quarts(*value).as_gills_uk(),
            )),
            (Unit::USQuart(value), UnitType::ImperialCup) => {
                Ok(Unit::ImperialCup(Volume::from_quarts(*value).as_cups_uk()))
            }
            (Unit::USQuart(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_quarts(*value).as_pints_uk(),
            )),
            (Unit::USQuart(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_quarts(*value).as_quarts_uk(),
            )),
            (Unit::USQuart(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_quarts(*value).as_cups_legal(),
            )),
            (Unit::USQuart(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_quarts(*value).as_cups_legal(),
            )),
            (Unit::USQuart(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_quarts(*value).as_fluid_ounces(),
            )),
            (Unit::USQuart(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_quarts(*value).as_cups()))
            }
            (Unit::USQuart(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_quarts(*value).as_pints()))
            }
            (Unit::USQuart(value), UnitType::USQuart) => Ok(Unit::USQuart(*value)),
            (Unit::USQuart(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_quarts(*value).as_gallons()))
            }
            (Unit::USQuart(value), UnitType::USTeaspoon) => {
                Ok(Unit::USTeaspoon(Volume::from_quarts(*value).as_teaspoons()))
            }
            (Unit::USQuart(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_quarts(*value).as_tablespoons(),
            )),
            (Unit::USQuart(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_quarts(*value).as_jiggers()))
            }

            // From US Gallon
            (Unit::USGallon(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_gallons(*value).as_millilitres(),
            )),
            (Unit::USGallon(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_gallons(*value).as_centilitres(),
            )),
            (Unit::USGallon(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_gallons(*value).as_decilitres(),
            )),
            (Unit::USGallon(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_gallons(*value).as_litres()))
            }
            (Unit::USGallon(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_gallons(*value).as_teaspoons_metric(),
            )),
            (Unit::USGallon(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_gallons(*value).as_tablespoons_metric(),
            )),
            (Unit::USGallon(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_gallons(*value).as_dessertspoons_metric(),
            )),
            (Unit::USGallon(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_gallons(*value).as_tablespoons_aus(),
            )),
            (Unit::USGallon(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_gallons(*value).as_teaspoons_uk()),
            ),
            (Unit::USGallon(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_gallons(*value).as_teaspoons_uk(),
            )),
            (Unit::USGallon(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_gallons(*value).as_tablespoons_uk(),
            )),
            (Unit::USGallon(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_gallons(*value).as_dessertspoons_uk()),
            ),
            (Unit::USGallon(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_gallons(*value).as_fluid_ounces_uk(),
            )),
            (Unit::USGallon(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_gallons(*value).as_gills_uk(),
            )),
            (Unit::USGallon(value), UnitType::ImperialCup) => {
                Ok(Unit::ImperialCup(Volume::from_gallons(*value).as_cups_uk()))
            }
            (Unit::USGallon(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_gallons(*value).as_pints_uk(),
            )),
            (Unit::USGallon(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_gallons(*value).as_quarts_uk(),
            )),
            (Unit::USGallon(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_gallons(*value).as_cups_legal(),
            )),
            (Unit::USGallon(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_gallons(*value).as_cups_legal(),
            )),
            (Unit::USGallon(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_gallons(*value).as_fluid_ounces(),
            )),
            (Unit::USGallon(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_gallons(*value).as_cups()))
            }
            (Unit::USGallon(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_gallons(*value).as_pints()))
            }
            (Unit::USGallon(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_gallons(*value).as_quarts()))
            }
            (Unit::USGallon(value), UnitType::USGallon) => Ok(Unit::USGallon(*value)),
            (Unit::USGallon(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_gallons(*value).as_teaspoons(),
            )),
            (Unit::USGallon(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_gallons(*value).as_tablespoons(),
            )),
            (Unit::USGallon(value), UnitType::Jigger) => {
                Ok(Unit::Jigger(Volume::from_gallons(*value).as_jiggers()))
            }

            // From Jigger
            (Unit::Jigger(value), UnitType::Millilitre) => Ok(Unit::Millilitre(
                Volume::from_jiggers(*value).as_millilitres(),
            )),
            (Unit::Jigger(value), UnitType::Centilitre) => Ok(Unit::Centilitre(
                Volume::from_jiggers(*value).as_centilitres(),
            )),
            (Unit::Jigger(value), UnitType::Decilitre) => Ok(Unit::Decilitre(
                Volume::from_jiggers(*value).as_decilitres(),
            )),
            (Unit::Jigger(value), UnitType::Litre) => {
                Ok(Unit::Litre(Volume::from_jiggers(*value).as_litres()))
            }
            (Unit::Jigger(value), UnitType::MetricTeaspoon) => Ok(Unit::MetricTeaspoon(
                Volume::from_jiggers(*value).as_teaspoons_metric(),
            )),
            (Unit::Jigger(value), UnitType::MetricTablespoon) => Ok(Unit::MetricTablespoon(
                Volume::from_jiggers(*value).as_tablespoons_metric(),
            )),
            (Unit::Jigger(value), UnitType::MetricDessertSpoon) => Ok(Unit::MetricDessertSpoon(
                Volume::from_jiggers(*value).as_dessertspoons_metric(),
            )),
            (Unit::Jigger(value), UnitType::MetricCup) => Ok(Unit::MetricCup(
                Volume::from_jiggers(*value).as_tablespoons_aus(),
            )),
            (Unit::Jigger(value), UnitType::AustralianTablespoon) => Ok(
                Unit::AustralianTablespoon(Volume::from_jiggers(*value).as_teaspoons_uk()),
            ),
            (Unit::Jigger(value), UnitType::ImperialTeaspoon) => Ok(Unit::ImperialTeaspoon(
                Volume::from_jiggers(*value).as_teaspoons_uk(),
            )),
            (Unit::Jigger(value), UnitType::ImperialTablespoon) => Ok(Unit::ImperialTablespoon(
                Volume::from_jiggers(*value).as_tablespoons_uk(),
            )),
            (Unit::Jigger(value), UnitType::ImperialDessertspoon) => Ok(
                Unit::ImperialDessertspoon(Volume::from_jiggers(*value).as_dessertspoons_uk()),
            ),
            (Unit::Jigger(value), UnitType::ImperialFluidOunce) => Ok(Unit::ImperialFluidOunce(
                Volume::from_jiggers(*value).as_fluid_ounces_uk(),
            )),
            (Unit::Jigger(value), UnitType::ImperialGill) => Ok(Unit::ImperialGill(
                Volume::from_jiggers(*value).as_gills_uk(),
            )),
            (Unit::Jigger(value), UnitType::ImperialCup) => {
                Ok(Unit::ImperialCup(Volume::from_jiggers(*value).as_cups_uk()))
            }
            (Unit::Jigger(value), UnitType::ImperialPint) => Ok(Unit::ImperialPint(
                Volume::from_jiggers(*value).as_pints_uk(),
            )),
            (Unit::Jigger(value), UnitType::ImperialQuart) => Ok(Unit::ImperialQuart(
                Volume::from_jiggers(*value).as_quarts_uk(),
            )),
            (Unit::Jigger(value), UnitType::ImperialGallon) => Ok(Unit::ImperialGallon(
                Volume::from_jiggers(*value).as_cups_legal(),
            )),
            (Unit::Jigger(value), UnitType::USLegalCup) => Ok(Unit::USLegalCup(
                Volume::from_jiggers(*value).as_cups_legal(),
            )),
            (Unit::Jigger(value), UnitType::USFluidOunce) => Ok(Unit::USFluidOunce(
                Volume::from_jiggers(*value).as_fluid_ounces(),
            )),
            (Unit::Jigger(value), UnitType::USCup) => {
                Ok(Unit::USCup(Volume::from_jiggers(*value).as_cups()))
            }
            (Unit::Jigger(value), UnitType::USPint) => {
                Ok(Unit::USPint(Volume::from_jiggers(*value).as_pints()))
            }
            (Unit::Jigger(value), UnitType::USQuart) => {
                Ok(Unit::USQuart(Volume::from_jiggers(*value).as_quarts()))
            }
            (Unit::Jigger(value), UnitType::USGallon) => {
                Ok(Unit::USGallon(Volume::from_jiggers(*value).as_gallons()))
            }
            (Unit::Jigger(value), UnitType::USTeaspoon) => Ok(Unit::USTeaspoon(
                Volume::from_jiggers(*value).as_teaspoons(),
            )),
            (Unit::Jigger(value), UnitType::USTablespoon) => Ok(Unit::USTablespoon(
                Volume::from_jiggers(*value).as_tablespoons(),
            )),
            (Unit::Jigger(value), UnitType::Jigger) => Ok(Unit::Jigger(*value)),

            (other, other_type) => Err(Error::UnsupportedUnit(other.clone(), other_type)),
        }*/
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn assert_approx_eq(actual: Unit, expected: Unit, threshold: f64) {
        let actual = actual.value();
        let expected = expected.value();

        assert!(
            (actual - expected).abs() < threshold,
            "Expected {actual}, got {expected}, difference: {}",
            (actual - expected).abs()
        );
    }
    
    mod test_convert_volume {
        use super::*;

        #[test]
        fn test_millilitre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Millilitre(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(150.0),
            );
            assert_eq!(
                Unit::Millilitre(150.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(15.0),
            );
            assert_eq!(
                Unit::Millilitre(200.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(2.0),
            );
            assert_eq!(
                Unit::Millilitre(150.0).convert(UnitType::Litre)?,
                Unit::Litre(0.15),
            );
            assert_eq!(
                Unit::Millilitre(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(1.0),
            );
            assert_eq!(
                Unit::Millilitre(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(1.0),
            );
            assert_eq!(
                Unit::Millilitre(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(1.0),
            );
            assert_eq!(
                Unit::Millilitre(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(1.0),
            );
            assert_eq!(
                Unit::Millilitre(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(2.0),
            );
            assert_approx_eq(
                Unit::Millilitre(15.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(2.534),
                1e-4,
            );
            assert_approx_eq(
                Unit::Millilitre(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(0.84468),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(2.112676),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(1.5838),
                1e-4,
            );
            assert_approx_eq(
                Unit::Millilitre(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(2.81561),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(1.12624),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(0.879877),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(0.783091),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(0.329954),
                1e-5,
            );
            assert_eq!(
                Unit::Millilitre(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(1.0),
            );
            assert_approx_eq(
                Unit::Millilitre(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(1.014421),
                1e-6,
            );
            assert_approx_eq(
                Unit::Millilitre(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(1.014421),
                1e-6,
            );
            assert_approx_eq(
                Unit::Millilitre(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(1.2),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(1.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::Millilitre(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(1.0),
                1e-6,
            );
            assert_approx_eq(
                Unit::Millilitre(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(1.0),
                1e-6,
            );
            assert_approx_eq(
                Unit::Millilitre(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(1.0),
                1e-6,
            );
            assert_approx_eq(
                Unit::Millilitre(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(1.26239),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_centilitre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Centilitre(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(1500.0),
            );
            assert_eq!(
                Unit::Centilitre(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(15.0),
            );
            assert_eq!(
                Unit::Centilitre(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(15.0),
            );
            assert_eq!(
                Unit::Centilitre(150.0).convert(UnitType::Litre)?,
                Unit::Litre(1.5),
            );
            assert_eq!(
                Unit::Centilitre(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(10.0),
            );
            assert_eq!(
                Unit::Centilitre(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(10.0),
            );
            assert_eq!(
                Unit::Centilitre(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(10.0),
            );
            assert_eq!(
                Unit::Centilitre(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(10.0),
            );
            assert_eq!(
                Unit::Centilitre(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(20.0),
            );
            assert_approx_eq(
                Unit::Centilitre(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(8.446816),
                1e-4,
            );
            assert_approx_eq(
                Unit::Centilitre(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(8.446821),
                1e-5,
            );
            assert_approx_eq(
                Unit::Centilitre(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(21.12676),
                1e-5,
            );
            assert_approx_eq(
                Unit::Centilitre(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(15.83777),
                1e-4,
            );
            assert_approx_eq(
                Unit::Centilitre(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(28.156064),
                1e-5,
            );
            assert_approx_eq(
                Unit::Centilitre(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(11.262426),
                1e-5,
            );
            assert_approx_eq(
                Unit::Centilitre(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(8.79876993),
                1e-5,
            );
            assert_approx_eq(
                Unit::Centilitre(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(7.8309087),
                1e-5,
            );
            assert_approx_eq(
                Unit::Centilitre(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(3.2995387),
                1e-5,
            );
            assert_eq!(
                Unit::Centilitre(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(10.0),
            );
            assert_approx_eq(
                Unit::Centilitre(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(10.144206),
                1e-6,
            );
            assert_approx_eq(
                Unit::Centilitre(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(10.14420681),
                1e-6,
            );
            assert_approx_eq(
                Unit::Centilitre(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(12.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Centilitre(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(10.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Centilitre(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(10.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Centilitre(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(10.0),
                1e-6,
            );
            assert_approx_eq(
                Unit::Centilitre(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(10.0),
                1e-6,
            );
            assert_approx_eq(
                Unit::Centilitre(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(12.623901821),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_decilitre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Decilitre(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(15000.0),
            );
            assert_eq!(
                Unit::Decilitre(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(150.0),
            );
            assert_eq!(
                Unit::Decilitre(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(150.0),
            );
            assert_eq!(
                Unit::Decilitre(150.0).convert(UnitType::Litre)?,
                Unit::Litre(15.0),
            );
            assert_eq!(
                Unit::Decilitre(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(100.0),
            );
            assert_eq!(
                Unit::Decilitre(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(100.0),
            );
            assert_eq!(
                Unit::Decilitre(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(100.0),
            );
            assert_eq!(
                Unit::Decilitre(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(100.0),
            );
            assert_eq!(
                Unit::Decilitre(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(200.0),
            );
            assert_approx_eq(
                Unit::Decilitre(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(84.46816),
                1e-4,
            );
            assert_approx_eq(
                Unit::Decilitre(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(84.46821),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(211.2676),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(158.3777),
                1e-4,
            );
            assert_approx_eq(
                Unit::Decilitre(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(281.56064),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(112.62426504),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(87.9876993),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(78.309087),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(32.995387),
                1e-5,
            );
            assert_eq!(
                Unit::Decilitre(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(100.0),
            );
            assert_approx_eq(
                Unit::Decilitre(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(101.44206),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(101.4420681),
                1e-6,
            );
            assert_approx_eq(
                Unit::Decilitre(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(120.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Decilitre(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(100.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Decilitre(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(100.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Decilitre(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(100.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(100.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::Decilitre(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(126.23901821),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_litre_conversions() -> Result<()> {
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(1000.0),
            );
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(100.0),
            );
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(10.0),
            );
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::Litre)?,
                Unit::Litre(1.0),
            );
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(200.0),
            );
            assert_eq!(
                Unit::Litre(1.5).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(100.0),
            );
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(100.0),
            );
            assert_eq!(
                Unit::Litre(1.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(4.0),
            );
            assert_eq!(
                Unit::Litre(1.2).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(60.0),
            );
            assert_approx_eq(
                Unit::Litre(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(844.6816),
                1e-4,
            );
            assert_approx_eq(
                Unit::Litre(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(844.6821),
                1e-5,
            );
            assert_approx_eq(
                Unit::Litre(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(2112.676056),
                1e-5,
            );
            assert_approx_eq(
                Unit::Litre(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(1583.777898),
                1e-4,
            );
            assert_approx_eq(
                Unit::Litre(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(2815.6063782),
                1e-5,
            );
            assert_approx_eq(
                Unit::Litre(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(1126.2426504),
                1e-5,
            );
            assert_approx_eq(
                Unit::Litre(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(879.876993),
                1e-5,
            );
            assert_approx_eq(
                Unit::Litre(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(783.09087),
                1e-5,
            );
            assert_approx_eq(
                Unit::Litre(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(329.95387),
                1e-5,
            );
            assert_eq!(
                Unit::Litre(1.2).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(5.0),
            );
            assert_approx_eq(
                Unit::Litre(1.2).convert(UnitType::USTeaspoon)?,
                Unit::Litre(243.4609),
                1e-4,
            );
            assert_approx_eq(
                Unit::Litre(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(1014.420681),
                1e-6,
            );
            assert_approx_eq(
                Unit::Litre(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(1199.9988),
                1e-3,
            );
            assert_approx_eq(
                Unit::Litre(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(1000.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Litre(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(1000.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::Litre(1.0).convert(UnitType::USQuart)?,
                Unit::USQuart(1.05668),
                1e-3,
            );
            assert_approx_eq(
                Unit::Litre(1.0).convert(UnitType::USGallon)?,
                Unit::USGallon(0.264172),
                1e-3,
            );
            assert_approx_eq(
                Unit::Litre(1.0).convert(UnitType::Jigger)?,
                Unit::Jigger(22.54268),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_metric_tsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::MetricTeaspoon(1.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(5.0),
            );
            assert_eq!(
                Unit::MetricTeaspoon(1.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(0.5),
            );
            assert_eq!(
                Unit::MetricTeaspoon(29.05).convert(UnitType::Decilitre)?,
                Unit::Decilitre(1.4525),
            );
            assert_eq!(
                Unit::MetricTeaspoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(0.75),
            );
            assert_eq!(
                Unit::MetricTeaspoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(5.0),
            );
            assert_eq!(
                Unit::MetricTeaspoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(5.0),
            );
            assert_eq!(
                Unit::MetricTeaspoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(5.0),
            );
            assert_eq!(
                Unit::MetricTeaspoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(5.0),
            );
            assert_eq!(
                Unit::MetricTeaspoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(10.0),
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(4.2234081552),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(4.2234081552),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(10.56338),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(7.91888),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(14.078031),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(5.6312132),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(4.399384),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(3.9154543),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(1.649769),
                1e-5,
            );
            assert_eq!(
                Unit::MetricTeaspoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(5.0),
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(5.07210),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(5.072103),
                1e-6,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(5.999994),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(4.999999),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(5.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(5.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(5.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTeaspoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(6.31195),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_metric_tbsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::MetricTablespoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(2250.0),
            );
            assert_eq!(
                Unit::MetricTablespoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(22.5),
            );
            assert_eq!(
                Unit::MetricTablespoon(1.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(0.15),
            );
            assert_eq!(
                Unit::MetricTablespoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(2.25),
            );
            assert_eq!(
                Unit::MetricTablespoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(15.0),
            );
            assert_eq!(
                Unit::MetricTablespoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(15.0),
            );
            assert_eq!(
                Unit::MetricTablespoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(15.0),
            );
            assert_eq!(
                Unit::MetricTablespoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(15.0),
            );
            assert_eq!(
                Unit::MetricTablespoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(30.0),
            );
            assert_approx_eq(
                Unit::MetricTablespoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(12.6702244),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(12.6702244),
                1e-5,
            );
            assert_eq!(
                Unit::ImperialDessertspoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(15.0),
            );
            assert_approx_eq(
                Unit::MetricTablespoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(23.756668),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(42.234095),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(16.89363),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(13.198154),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(11.746363026848577),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(4.9493080867275),
                1e-5,
            );
            assert_eq!(
                Unit::MetricTablespoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(15.0),
            );
            assert_approx_eq(
                Unit::MetricTablespoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(15.216310215),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(15.216310215),
                1e-6,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(17.9999820057321),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(14.999997686809676),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(15.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(15.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(15.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricTablespoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(18.935852731707417),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_metric_dessert_spoon_conversions() -> Result<()> {
            assert_eq!(
                Unit::MetricDessertSpoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(1500.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(15.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(15.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(1.5),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(10.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(10.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(10.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(10.0),
            );
            assert_eq!(
                Unit::MetricDessertSpoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(20.0),
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(8.446816),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::MetricDessertSpoon(8.446816),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(21.126760563380284),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(15.837778908),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(28.156063),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(11.262426),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(8.79876993195),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(7.83090868),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(3.299538),
                1e-5,
            );
            assert_eq!(
                Unit::MetricDessertSpoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(10.0),
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(10.14420681),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(10.14420681),
                1e-6,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(11.999988),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(9.9999),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(10.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(10.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(10.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricDessertSpoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(12.6239018),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_metric_cup_conversions() -> Result<()> {
            assert_eq!(
                Unit::MetricCup(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(37500.0),
            );
            assert_eq!(
                Unit::MetricCup(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(375.0),
            );
            assert_eq!(
                Unit::MetricCup(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(375.0),
            );
            assert_eq!(
                Unit::MetricCup(150.0).convert(UnitType::Litre)?,
                Unit::Litre(37.5),
            );
            assert_eq!(
                Unit::MetricCup(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(250.0),
            );
            assert_eq!(
                Unit::MetricCup(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(250.0),
            );
            assert_eq!(
                Unit::MetricCup(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(250.0),
            );
            assert_eq!(
                Unit::MetricCup(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(250.0),
            );
            assert_eq!(
                Unit::MetricCup(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(500.0),
            );
            assert_approx_eq(
                Unit::MetricCup(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(211.1704077),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricCup(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(211.1705266),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricCup(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(528.169014),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricCup(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(395.9444),
                1e-4,
            );
            assert_approx_eq(
                Unit::MetricCup(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(703.9015945),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricCup(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(281.560662),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricCup(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(219.969248),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricCup(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(195.77271),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricCup(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(82.48846),
                1e-5,
            );
            assert_eq!(
                Unit::MetricCup(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(250.0),
            );
            assert_approx_eq(
                Unit::MetricCup(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(253.605170),
                1e-5,
            );
            assert_approx_eq(
                Unit::MetricCup(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(253.605170),
                1e-6,
            );
            assert_approx_eq(
                Unit::MetricCup(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(299.99970),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricCup(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(249.999961),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricCup(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(250.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricCup(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(250.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricCup(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(250.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::MetricCup(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(315.597545),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_australian_tbsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::AustralianTablespoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(3000.0),
            );
            assert_eq!(
                Unit::AustralianTablespoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(30.0),
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(30.0),
                1e-10,
            );
            assert_eq!(
                Unit::AustralianTablespoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(3.0),
            );
            assert_eq!(
                Unit::AustralianTablespoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(20.0),
            );
            assert_eq!(
                Unit::AustralianTablespoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(20.0),
            );
            assert_eq!(
                Unit::AustralianTablespoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(20.0),
            );
            assert_eq!(
                Unit::AustralianTablespoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(20.0),
            );
            assert_eq!(
                Unit::AustralianTablespoon(4.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(4.0),
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(16.89363),
                1e-4,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(16.89363),
                1e-4,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(42.253521),
                1e-5,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(31.675557),
                1e-4,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(56.3121275),
                1e-5,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(22.524853),
                1e-5,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(17.59753),
                1e-5,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(15.661817),
                1e-5,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(6.5990774),
                1e-5,
            );
            assert_eq!(
                Unit::AustralianTablespoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(20.0),
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(20.288413),
                1e-5,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(20.288413),
                1e-6,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(23.999976),
                1e-3,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(19.99999),
                1e-3,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(20.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(20.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(20.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::AustralianTablespoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(25.247803),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_tsp_conversions() -> Result<()> {
            assert_eq!(
                Unit::ImperialTeaspoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(887.9085),
            );
            assert_eq!(
                Unit::ImperialTeaspoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(8.879085),
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(8.879085),
                1e-10,
            );
            assert_eq!(
                Unit::ImperialTeaspoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(0.8879085),
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(5.91939),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(5.91939),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(5.91939),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(5.91939),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialTeaspoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(11.83878),
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(5.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(5.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(12.5057535),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(9.374999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(16.666672),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(6.66666),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(5.208335),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(4.6354202),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(1.953125),
                1e-5,
            );
            assert_eq!(
                Unit::ImperialTeaspoon(28.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(0.6905955),
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(6.0047516),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(6.0047516),
                1e-6,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(7.103260),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(5.919389),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(5.91939),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(5.91939),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(5.91939),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTeaspoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(7.472579),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_dessert_spoon_conversions() -> Result<()> {
            assert_eq!(
                Unit::ImperialDessertspoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(1065.0),
            );
            assert_eq!(
                Unit::ImperialDessertspoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(10.65),
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(10.65),
                1e-10,
            );
            assert_eq!(
                Unit::ImperialDessertspoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(1.065),
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(7.1),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(7.1),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(7.1),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(88.75),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(47.97791),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(5.997239),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(5.9972429),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(15.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(11.24482),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(19.9908),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(7.99632),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(6.24712),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(5.559945),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(2.342672),
                1e-5,
            );
            assert_eq!(
                Unit::ImperialDessertspoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(7.1),
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(7.20238),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(7.202386),
                1e-6,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(8.5199914),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(7.1),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(7.1),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(7.1),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(7.1),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialDessertspoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(8.962970),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_tablespoon_conversions() -> Result<()> {
            assert_eq!(
                Unit::ImperialTablespoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(2663.724),
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(26.63724),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(26.63724),
                1e-10,
            );
            assert_eq!(
                Unit::ImperialTablespoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(2.663724),
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(17.75816),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(17.75816),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(17.75816),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(221.977),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(119.9999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(14.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(15.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(37.51723),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(28.12498),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(49.99998),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(19.99999),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(15.624996),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(13.906252),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(5.859373),
                1e-5,
            );
            assert_eq!(
                Unit::ImperialTablespoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(17.75816),
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(18.014244),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(18.014244),
                1e-6,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(21.30977),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(17.75816),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(17.75816),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(17.75816),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(17.75816),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialTablespoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(22.417726),
                1e-6,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_fluid_ounce_conversions() -> Result<()> {
            assert_eq!(
                Unit::ImperialFluidOunce(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(4261.961250507437),
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(42.619612),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(42.619612),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(150.0).convert(UnitType::Litre)?,
                Unit::Litre(4.261961),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(28.41307),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(28.41307),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(28.41307),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(355.16343),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(192.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(24.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(24.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(60.02762),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(45.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(80.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(32.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(25.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(22.25),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(9.375),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialFluidOunce(2.534046).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(0.30000016275009356),
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(28.8228),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(28.8228),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(34.0956),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(28.413),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(28.413),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(28.413),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(28.413),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialFluidOunce(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(35.8683),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_gill_conversions() -> Result<()> {
            assert_eq!(
                Unit::ImperialGill(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(21309.796875),
            );
            assert_approx_eq(
                Unit::ImperialGill(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(213.09796),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialGill(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(213.09796),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialGill(150.0).convert(UnitType::Litre)?,
                Unit::Litre(21.30979),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialGill(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(142.0653),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(142.0653),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(142.0653),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(1775.8164),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(959.9996),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(119.9999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(120.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(300.1379),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(224.9999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(400.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(160.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(124.999999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(111.25),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(46.874999),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialGill(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(142.0653125),
            );
            assert_approx_eq(
                Unit::ImperialGill(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(144.1139),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(144.1139),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGill(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(170.4782),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGill(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(142.0653),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGill(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(142.0653),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGill(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(142.0653),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGill(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(142.0653),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGill(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(179.34185),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_cup_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::ImperialCup(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(42619.59),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(426.1959),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialCup(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(426.1959),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialCup(150.0).convert(UnitType::Litre)?,
                Unit::Litre(42.61959),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialCup(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(284.1306),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(284.1306),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(284.1306),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(3551.6325),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(1919.9991),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(239.9998),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(240.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(600.2759),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(449.9997),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(800.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(320.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(249.99997),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(222.5),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(93.74999),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialCup(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(284.1306),
            );
            assert_approx_eq(
                Unit::ImperialCup(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(288.22795),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(288.22795),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialCup(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(340.95637),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialCup(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(284.1306),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialCup(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(284.1306),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialCup(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(284.1306),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialCup(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(284.1306),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialCup(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(358.68367),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_pint_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::ImperialPint(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(85239.1875),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(852.39187),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialPint(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(852.39187),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialPint(150.0).convert(UnitType::Litre)?,
                Unit::Litre(85.23918),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialPint(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(568.26125),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(568.26125),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(568.26125),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(7103.2656),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(3839.9987),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(479.99983),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(480.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(1200.55193),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(899.9996),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(1600.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(640.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(500.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(445.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(187.5),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialPint(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(568.2612500008727),
            );
            assert_approx_eq(
                Unit::ImperialPint(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(576.45596),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(576.45596),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialPint(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(681.91281),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(568.26125),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(568.26125),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(568.26125),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(568.26125),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialPint(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(717.36742),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_quart_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::ImperialQuart(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(170478.3),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(1704.783),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialQuart(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(1704.783),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialQuart(150.0).convert(UnitType::Litre)?,
                Unit::Litre(170.4783),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialQuart(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(1136.522),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(1136.522),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(1136.522),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(14206.525),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(7679.994),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(959.99925),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(959.999797),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(2401.1028),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(1799.99841),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(3199.99859),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(1279.9995),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(999.99956),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(890.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(374.999835),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialQuart(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(1136.5220000000002),
            );
            assert_approx_eq(
                Unit::ImperialQuart(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(1152.91142),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(1152.91142),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialQuart(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(1363.825036),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(1136.522),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(1136.522),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(1136.522),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(1136.522),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialQuart(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(1434.734214),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_imperial_gallon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::ImperialGallon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(681913.5),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(6819.135),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialGallon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(6819.135),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialGallon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(681.9135),
                1e-5,
            );
            assert_approx_eq(
                Unit::ImperialGallon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(4546.09),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(4546.09),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(4546.09),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(56826.125),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(30719.98972),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(3839.99871),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(3840.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(9604.41549),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(7199.9968),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(12800.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(5120.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(3999.9999),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(3560.00156),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(1500.0),
                1e-4,
            );
            assert_eq!(
                Unit::ImperialGallon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(4546.090000001815),
            );
            assert_approx_eq(
                Unit::ImperialGallon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(4611.6477),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(4611.6477),
                1e-4,
            );
            assert_approx_eq(
                Unit::ImperialGallon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(5455.3025),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(4546.09),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(4546.09),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(4546.09),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(4546.09),
                1e-3,
            );
            assert_approx_eq(
                Unit::ImperialGallon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(5738.939383),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_legal_cup_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USLegalCup(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(36000.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(360.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::USLegalCup(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(360.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::USLegalCup(150.0).convert(UnitType::Litre)?,
                Unit::Litre(36.0),
                1e-5,
            );
            assert_approx_eq(
                Unit::USLegalCup(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(240.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(240.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(240.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(3000.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(1621.7887),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(202.72359),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(202.72370),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(507.042253),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(380.106693),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(675.745530),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(270.298236),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(211.17047),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(187.941808),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(79.188929),
                1e-4,
            );
            assert_eq!(
                Unit::USLegalCup(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(240.0),
            );
            assert_approx_eq(
                Unit::USLegalCup(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(243.4609),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(243.4609),
                1e-4,
            );
            assert_approx_eq(
                Unit::USLegalCup(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(287.99971),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(240.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(240.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(240.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(240.0),
                1e-3,
            );
            assert_approx_eq(
                Unit::USLegalCup(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(302.97364),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_teaspoon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USTeaspoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(739.338239),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(7.3933823),
                1e-5,
            );
            assert_approx_eq(
                Unit::USTeaspoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(7.3933823),
                1e-5,
            );
            assert_approx_eq(
                Unit::USTeaspoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(0.73933823),
                1e-5,
            );
            assert_approx_eq(
                Unit::USTeaspoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(4.92892),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(4.92892),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(4.92892),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(61.611519),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(33.30695),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(4.163369),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(4.163369),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(10.4132146),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(7.8063170),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(13.877903),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(5.55116),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(4.3368447),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(3.859793),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(30.805759),
                1e-4,
            );
            assert_eq!(
                Unit::USTeaspoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(4.928921594018646),
            );
            assert_approx_eq(
                Unit::USTeaspoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(5.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(5.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTeaspoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(5.9147),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(4.92892),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(4.92892),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(4.92892),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(4.92892),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTeaspoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(6.22222),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_tablespoon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USTablespoon(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(2218.0147),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(22.180147),
                1e-5,
            );
            assert_approx_eq(
                Unit::USTablespoon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(22.180147),
                1e-5,
            );
            assert_approx_eq(
                Unit::USTablespoon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(2.218014),
                1e-5,
            );
            assert_approx_eq(
                Unit::USTablespoon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(14.78676),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(14.78676),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(14.78676),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(184.834559),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(99.92086),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(12.490108),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(12.490108),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(31.239643),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(23.418951),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(41.633709),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(16.6534851),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(13.0105),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(11.57938),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(92.417279),
                1e-4,
            );
            assert_eq!(
                Unit::USTablespoon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(14.786764782055936),
            );
            assert_approx_eq(
                Unit::USTablespoon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(15.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(15.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USTablespoon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(17.7441),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(14.78676),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(14.78676),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(14.78676),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(14.78676),
                1e-3,
            );
            assert_approx_eq(
                Unit::USTablespoon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(18.666666),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_floz_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USFluidOunce(150.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(4436.02943),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(44.36029),
                1e-5,
            );
            assert_approx_eq(
                Unit::USFluidOunce(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(44.36029),
                1e-5,
            );
            assert_approx_eq(
                Unit::USFluidOunce(150.0).convert(UnitType::Litre)?,
                Unit::Litre(4.436029),
                1e-5,
            );
            assert_approx_eq(
                Unit::USFluidOunce(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(29.57352),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(29.57352),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(29.57352),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(369.6691),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(199.84173),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(24.980217),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(24.980217),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(62.4792),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(46.83790),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(83.26741),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(33.30697),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(26.021068),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(23.15876),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(184.8345),
                1e-4,
            );
            assert_eq!(
                Unit::USFluidOunce(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(29.573529564111873),
            );
            assert_approx_eq(
                Unit::USFluidOunce(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(30.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(30.0),
                1e-4,
            );
            assert_approx_eq(
                Unit::USFluidOunce(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(35.4882),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(29.57352),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(29.57352),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(29.57352),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(29.57352),
                1e-3,
            );
            assert_approx_eq(
                Unit::USFluidOunce(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(37.33333),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_cup_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USCup(2.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(473.1764),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(354.882354),
                1e-5,
            );
            assert_approx_eq(
                Unit::USCup(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(354.882354),
                1e-5,
            );
            assert_approx_eq(
                Unit::USCup(150.0).convert(UnitType::Litre)?,
                Unit::Litre(35.488235),
                1e-5,
            );
            assert_approx_eq(
                Unit::USCup(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(236.5882),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(236.5882),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(236.5882),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(2957.35295),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(1598.7338),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(199.841737),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(199.841737),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(499.83430),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(374.70321),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(666.13934),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(266.455762),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(208.16854),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(185.2700),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(1478.67647),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(236.5882),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(239.9999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(239.9999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USCup(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(283.905599),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(236.5882),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(236.5882),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(236.5882),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(236.5882),
                1e-3,
            );
            assert_approx_eq(
                Unit::USCup(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(298.66666),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_pint_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USPint(2.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(946.3529),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(709.76470),
                1e-5,
            );
            assert_approx_eq(
                Unit::USPint(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(709.76470),
                1e-5,
            );
            assert_approx_eq(
                Unit::USPint(150.0).convert(UnitType::Litre)?,
                Unit::Litre(70.97647),
                1e-5,
            );
            assert_approx_eq(
                Unit::USPint(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(473.176472),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(473.176472),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(473.176472),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(5914.7059),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(3197.4677),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(399.68347),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(399.68347),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(999.6686),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(749.406436),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(1332.2786),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(532.91152),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(416.33709),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(370.54017),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(2957.3529),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(473.176472),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(479.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(479.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USPint(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(567.8111),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(473.176472),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(473.176472),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(473.176472),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(473.176472),
                1e-3,
            );
            assert_approx_eq(
                Unit::USPint(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(597.3333),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_us_quart_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USQuart(2.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(1892.7058),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(1419.52941),
                1e-5,
            );
            assert_approx_eq(
                Unit::USQuart(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(1419.52941),
                1e-5,
            );
            assert_approx_eq(
                Unit::USQuart(150.0).convert(UnitType::Litre)?,
                Unit::Litre(141.95294),
                1e-5,
            );
            assert_approx_eq(
                Unit::USQuart(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(946.3529),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(946.3529),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(946.3529),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(11829.41182),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(6394.9355),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(799.3669),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(799.3669),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(1999.33720),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(1498.81287),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(2664.55739),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(1065.8230),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(832.67418),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(741.0803),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(5914.70591),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(946.3529),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(959.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(959.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USQuart(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(1135.62239),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(946.3529),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(946.3529),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(946.3529),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(946.3529),
                1e-3,
            );
            assert_approx_eq(
                Unit::USQuart(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(1194.6666),
                1e-4,
            );
            Ok(())
        }
        
        #[test]
        fn test_us_gallon_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::USGallon(2.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(7570.82356),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(5678.11767),
                1e-5,
            );
            assert_approx_eq(
                Unit::USGallon(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(5678.11767),
                1e-5,
            );
            assert_approx_eq(
                Unit::USGallon(150.0).convert(UnitType::Litre)?,
                Unit::Litre(567.81176),
                1e-5,
            );
            assert_approx_eq(
                Unit::USGallon(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(3785.411784),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(3785.411784),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(3785.411784),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(47317.6473),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(25579.7423),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(3197.46779),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(3197.46960),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(7997.34883),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(5995.25149),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(10658.2295),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(4263.2922),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(3330.69673),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(2964.32140),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(23658.82365),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(3785.411784),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(3839.999999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(3839.999999),
                1e-4,
            );
            assert_approx_eq(
                Unit::USGallon(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(4542.48959),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(3785.411784),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(3785.411784),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(3785.411784),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(3785.411784),
                1e-3,
            );
            assert_approx_eq(
                Unit::USGallon(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(4778.66667),
                1e-4,
            );
            Ok(())
        }

        #[test]
        fn test_jigger_conversions() -> Result<()> {
            assert_approx_eq(
                Unit::Jigger(2.0).convert(UnitType::Millilitre)?,
                Unit::Millilitre(88.7205886),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(15.0).convert(UnitType::Centilitre)?,
                Unit::Centilitre(66.54044),
                1e-5,
            );
            assert_approx_eq(
                Unit::Jigger(150.0).convert(UnitType::Decilitre)?,
                Unit::Decilitre(66.54044),
                1e-5,
            );
            assert_approx_eq(
                Unit::Jigger(150.0).convert(UnitType::Litre)?,
                Unit::Litre(6.654044),
                1e-5,
            );
            assert_approx_eq(
                Unit::Jigger(5.0).convert(UnitType::MetricTeaspoon)?,
                Unit::MetricTeaspoon(44.3602),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(15.0).convert(UnitType::MetricTablespoon)?,
                Unit::MetricTablespoon(44.3602),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(10.0).convert(UnitType::MetricDessertSpoon)?,
                Unit::MetricDessertSpoon(44.3602),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(250.0).convert(UnitType::MetricCup)?,
                Unit::MetricCup(554.503678),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(40.0).convert(UnitType::AustralianTablespoon)?,
                Unit::AustralianTablespoon(299.7626),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(5.0).convert(UnitType::ImperialTeaspoon)?,
                Unit::ImperialTeaspoon(37.47032),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(15.0).convert(UnitType::ImperialTablespoon)?,
                Unit::ImperialTablespoon(37.470346),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(15.0).convert(UnitType::ImperialDessertspoon)?,
                Unit::ImperialDessertspoon(93.71893),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(45.0).convert(UnitType::ImperialFluidOunce)?,
                Unit::ImperialFluidOunce(70.25685),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(400.0).convert(UnitType::ImperialGill)?,
                Unit::ImperialGill(124.90112),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(320.0).convert(UnitType::ImperialCup)?,
                Unit::ImperialCup(49.96045542),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(500.0).convert(UnitType::ImperialPint)?,
                Unit::ImperialPint(39.03160),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(890.0).convert(UnitType::ImperialQuart)?,
                Unit::ImperialQuart(34.73814),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(1500.0).convert(UnitType::ImperialGallon)?,
                Unit::ImperialGallon(277.2518),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(240.0).convert(UnitType::USLegalCup)?,
                Unit::USLegalCup(44.3602),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(5.0).convert(UnitType::USTeaspoon)?,
                Unit::USTeaspoon(44.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(15.0).convert(UnitType::USTablespoon)?,
                Unit::USTablespoon(44.99999),
                1e-4,
            );
            assert_approx_eq(
                Unit::Jigger(35.4882).convert(UnitType::USFluidOunce)?,
                Unit::USFluidOunce(53.2323),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(236.5882).convert(UnitType::USCup)?,
                Unit::USCup(44.3602),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(473.1765).convert(UnitType::USPint)?,
                Unit::USPint(44.3602),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(946.353).convert(UnitType::USQuart)?,
                Unit::USQuart(44.3602),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(3785.412).convert(UnitType::USGallon)?,
                Unit::USGallon(44.3602),
                1e-3,
            );
            assert_approx_eq(
                Unit::Jigger(56.0).convert(UnitType::Jigger)?,
                Unit::Jigger(56.0),
                1e-4,
            );
            Ok(())
        }
    }
}
