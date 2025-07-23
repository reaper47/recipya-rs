use measurements::{Measurement, Volume};

const AUSTRALIAN_TABLESPOON_IN_MILLILITRES: f64 = 20.0;

/// In Australia, a standard measuring tablespoon holds 20 milliliters of liquid.
pub struct AustralianTablespoon {
    pub value: f64,
}

impl Measurement for AustralianTablespoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("tbsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * AUSTRALIAN_TABLESPOON_IN_MILLILITRES
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / AUSTRALIAN_TABLESPOON_IN_MILLILITRES,
        }
    }
}

pub trait VolumeAustralianTablespoonExt {
    fn as_tablespoons_aus(&self) -> f64;
    fn from_tablespoons_aus(tablespoons: f64) -> Volume;
}

impl VolumeAustralianTablespoonExt for Volume {
    fn as_tablespoons_aus(&self) -> f64 {
        self.as_millilitres() / AUSTRALIAN_TABLESPOON_IN_MILLILITRES
    }

    fn from_tablespoons_aus(tablespoons: f64) -> Volume {
        Volume::from_millilitres(tablespoons * AUSTRALIAN_TABLESPOON_IN_MILLILITRES)
    }
}
