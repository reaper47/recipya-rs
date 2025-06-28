use measurements::{Measurement, Volume};

const US_LEGAL_CUP_IN_MILLILITERS: f64 = 240.0;

pub struct USLegalCup {
    pub value: f64,
}

impl Measurement for USLegalCup {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("cup", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * US_LEGAL_CUP_IN_MILLILITERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / US_LEGAL_CUP_IN_MILLILITERS,
        }
    }
}

pub trait VolumeUSLegalCupExt {
    fn as_cups_legal(&self) -> f64;
    fn from_cups_legal(cups: f64) -> Volume;
}

impl VolumeUSLegalCupExt for Volume {
    fn as_cups_legal(&self) -> f64 {
        self.as_millilitres() / US_LEGAL_CUP_IN_MILLILITERS
    }

    fn from_cups_legal(cups: f64) -> Volume {
        Volume::from_millilitres(cups * US_LEGAL_CUP_IN_MILLILITERS)
    }
}
