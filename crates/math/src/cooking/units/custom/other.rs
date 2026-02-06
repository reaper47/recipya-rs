use measurements::{Measurement, Volume};

const JIGGER_IN_MILLILITRES: f64 = 44.360_294_3;

/// A jigger is a bartender’s measuring tool for alcohol.
///
/// A jigger (also called an alcohol jigger or bar jigger) is an
/// hourglass-shaped measuring tool used to ensure accurate amounts
/// of alcohol are poured into drinks
/// ([source](https://barsandbartending.com)).
pub struct Jigger {
    pub value: f64,
}

impl Measurement for Jigger {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("jigger", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * JIGGER_IN_MILLILITRES
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / JIGGER_IN_MILLILITRES,
        }
    }
}

pub trait VolumeJiggerExt {
    fn as_jiggers(&self) -> f64;
    fn from_jiggers(jiggers: f64) -> Volume;
}

impl VolumeJiggerExt for Volume {
    fn as_jiggers(&self) -> f64 {
        self.as_millilitres() / JIGGER_IN_MILLILITRES
    }

    fn from_jiggers(jiggers: f64) -> Volume {
        Self::from_millilitres(jiggers * JIGGER_IN_MILLILITRES)
    }
}
