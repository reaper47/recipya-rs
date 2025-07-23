use measurements::{Measurement, Volume};

const IMPERIAL_TEASPOON_IN_MILLILITRES: f64 = 5.91939;

/// Represents a volume measured in Imperial teaspoons (UK pre-metric standard).
pub struct ImperialTeaspoon {
    pub value: f64,
}

impl Measurement for ImperialTeaspoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("tsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * IMPERIAL_TEASPOON_IN_MILLILITRES
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / IMPERIAL_TEASPOON_IN_MILLILITRES,
        }
    }
}

pub trait VolumeImperialTeaspoonExt {
    /// Convert Volume to a floating point value in UK/Imperial Teaspoons (tsp)
    fn as_teaspoons_uk(&self) -> f64;
    fn from_teaspoons_uk(teaspoons: f64) -> Volume;
}

impl VolumeImperialTeaspoonExt for Volume {
    fn as_teaspoons_uk(&self) -> f64 {
        self.as_millilitres() / IMPERIAL_TEASPOON_IN_MILLILITRES
    }

    fn from_teaspoons_uk(teaspoons: f64) -> Volume {
        Volume::from_millilitres(teaspoons * IMPERIAL_TEASPOON_IN_MILLILITRES)
    }
}

const IMPERIAL_TABLESPOON_IN_MILLILITRES: f64 = 17.75816;

/// Represents a volume measured in Imperial tablespoons (UK pre-metric standard).
pub struct ImperialTablespoon {
    pub value: f64,
}

impl Measurement for ImperialTablespoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("tbsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * IMPERIAL_TABLESPOON_IN_MILLILITRES
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / IMPERIAL_TABLESPOON_IN_MILLILITRES,
        }
    }
}

pub trait VolumeImperialTablespoonExt {
    /// Convert Volume to a floating point value in UK/Imperial Tablespoons (tbsp)
    fn as_tablespoons_uk(&self) -> f64;
    fn from_tablespoons_uk(tablespoons: f64) -> Volume;
}

impl VolumeImperialTablespoonExt for Volume {
    fn as_tablespoons_uk(&self) -> f64 {
        self.as_millilitres() / IMPERIAL_TABLESPOON_IN_MILLILITRES
    }

    fn from_tablespoons_uk(tablespoons: f64) -> Volume {
        Volume::from_millilitres(tablespoons * IMPERIAL_TABLESPOON_IN_MILLILITRES)
    }
}

const IMPERIAL_DESSERTSPOON_IN_MILLILITERS: f64 = 7.1;

pub struct ImperialDessertSpoon {
    pub value: f64,
}

impl Measurement for ImperialDessertSpoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("dsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * IMPERIAL_DESSERTSPOON_IN_MILLILITERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / IMPERIAL_DESSERTSPOON_IN_MILLILITERS,
        }
    }
}

pub trait VolumeImperialDessertSpoonExt {
    fn as_dessertspoons_uk(&self) -> f64;
    fn from_dessertspoons_uk(spoons: f64) -> Volume;
}

impl VolumeImperialDessertSpoonExt for Volume {
    fn as_dessertspoons_uk(&self) -> f64 {
        self.as_millilitres() / IMPERIAL_DESSERTSPOON_IN_MILLILITERS
    }

    fn from_dessertspoons_uk(spoons: f64) -> Volume {
        Volume::from_millilitres(spoons * IMPERIAL_DESSERTSPOON_IN_MILLILITERS)
    }
}

const IMPERIAL_CUP_IN_MILLILITERS: f64 = 284.1306;

pub struct ImperialCup {
    pub value: f64,
}

impl Measurement for ImperialCup {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("cup", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * IMPERIAL_CUP_IN_MILLILITERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / IMPERIAL_CUP_IN_MILLILITERS,
        }
    }
}

pub trait VolumeImperialCupExt {
    fn as_cups_uk(&self) -> f64;
    fn from_cups_uk(cups: f64) -> Volume;
}

impl VolumeImperialCupExt for Volume {
    fn as_cups_uk(&self) -> f64 {
        self.as_millilitres() / IMPERIAL_CUP_IN_MILLILITERS
    }

    fn from_cups_uk(cups: f64) -> Volume {
        Volume::from_millilitres(cups * IMPERIAL_CUP_IN_MILLILITERS)
    }
}

const IMPERIAL_GILL_IN_MILLILITERS: f64 = 142.0653125;

pub struct ImperialGill {
    pub value: f64,
}

impl Measurement for ImperialGill {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("gill", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * IMPERIAL_GILL_IN_MILLILITERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / IMPERIAL_GILL_IN_MILLILITERS,
        }
    }
}

pub trait VolumeImperialGillExt {
    fn as_gills_uk(&self) -> f64;
    fn from_gills_uk(gills: f64) -> Volume;
}

impl VolumeImperialGillExt for Volume {
    fn as_gills_uk(&self) -> f64 {
        self.as_millilitres() / IMPERIAL_GILL_IN_MILLILITERS
    }

    fn from_gills_uk(gills: f64) -> Volume {
        Volume::from_millilitres(gills * IMPERIAL_GILL_IN_MILLILITERS)
    }
}

const IMPERIAL_QUART_IN_LITERS: f64 = 1.136522;

pub struct ImperialQuart {
    pub value: f64,
}

impl Measurement for ImperialQuart {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("qt", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "l"
    }

    fn as_base_units(&self) -> f64 {
        self.value * IMPERIAL_QUART_IN_LITERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / IMPERIAL_QUART_IN_LITERS,
        }
    }
}

pub trait VolumeImperialQuartExt {
    fn as_quarts_uk(&self) -> f64;
    fn from_quarts_uk(quarts: f64) -> Volume;
}

impl VolumeImperialQuartExt for Volume {
    fn as_quarts_uk(&self) -> f64 {
        self.as_litres() / IMPERIAL_QUART_IN_LITERS
    }

    fn from_quarts_uk(quarts: f64) -> Volume {
        Volume::from_litres(quarts * IMPERIAL_QUART_IN_LITERS)
    }
}
