use measurements::{Mass, Measurement, Volume};

const DEKAGRAM_IN_KG: f64 = 0.01;

/// A dekagram is a metric unit of mass and weight equal to 10 grams.
pub struct Dekagram {
    pub value: f64,
}

impl Measurement for Dekagram {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("dag", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "kg"
    }

    fn as_base_units(&self) -> f64 {
        self.value * DEKAGRAM_IN_KG
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / DEKAGRAM_IN_KG,
        }
    }
}

pub trait MassDekagramExt {
    fn as_dekagrams(&self) -> f64;
    fn from_dekagrams(decigrams: f64) -> Mass;
}

impl MassDekagramExt for Mass {
    fn as_dekagrams(&self) -> f64 {
        self.as_kilograms() / DEKAGRAM_IN_KG
    }

    fn from_dekagrams(dekagrams: f64) -> Mass {
        Mass::from_kilograms(dekagrams * DEKAGRAM_IN_KG)
    }
}

const HECTOGRAM_IN_KG: f64 = 0.1;

/// A dekagram is a metric unit of mass and weight equal to 100 grams.
pub struct Hectogram {
    pub value: f64,
}

impl Measurement for Hectogram {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("hg", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "kg"
    }

    fn as_base_units(&self) -> f64 {
        self.value * HECTOGRAM_IN_KG
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / HECTOGRAM_IN_KG,
        }
    }
}

pub trait MassHectogramExt {
    fn as_hectograms(&self) -> f64;
    fn from_hectograms(hectograms: f64) -> Mass;
}

impl MassHectogramExt for Mass {
    fn as_hectograms(&self) -> f64 {
        self.as_kilograms() / HECTOGRAM_IN_KG
    }

    fn from_hectograms(hectograms: f64) -> Mass {
        Mass::from_kilograms(hectograms * HECTOGRAM_IN_KG)
    }
}

const CENTILITRE_IN_LITRE: f64 = 0.01;

/// A metric unit of capacity, equal to one hundredth of a liter.
pub struct Centilitre {
    pub value: f64,
}

impl Measurement for Centilitre {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("cl", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "l"
    }

    fn as_base_units(&self) -> f64 {
        self.value * CENTILITRE_IN_LITRE
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / CENTILITRE_IN_LITRE,
        }
    }
}

pub trait VolumeCentilitreExt {
    fn as_centilitres(&self) -> f64;
    fn from_centilitres(centilitres: f64) -> Volume;
}

impl VolumeCentilitreExt for Volume {
    fn as_centilitres(&self) -> f64 {
        self.as_litres() / CENTILITRE_IN_LITRE
    }

    fn from_centilitres(centilitres: f64) -> Volume {
        Volume::from_litres(centilitres * CENTILITRE_IN_LITRE)
    }
}

const DECILITRE_IN_LITRE: f64 = 0.1;

/// Decilitre is volume equivalent to one tenth of a liter (L/10).
pub struct Decilitre {
    pub value: f64,
}

impl Measurement for Decilitre {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("dl", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "l"
    }

    fn as_base_units(&self) -> f64 {
        self.value * DECILITRE_IN_LITRE
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / DECILITRE_IN_LITRE,
        }
    }
}

pub trait VolumeDecilitreExt {
    fn as_decilitres(&self) -> f64;
    fn from_decilitres(decilitres: f64) -> Volume;
}

impl VolumeDecilitreExt for Volume {
    fn as_decilitres(&self) -> f64 {
        self.as_litres() / DECILITRE_IN_LITRE
    }

    fn from_decilitres(decilitres: f64) -> Volume {
        Volume::from_litres(decilitres * DECILITRE_IN_LITRE)
    }
}

const METRIC_TEASPOON_IN_MILLIMETERS: f64 = 5.0;

/// Decilitre is volume equivalent to one tenth of a liter (L/10).
pub struct MetricTeaspoon {
    pub value: f64,
}

impl Measurement for MetricTeaspoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("tsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * METRIC_TEASPOON_IN_MILLIMETERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / METRIC_TEASPOON_IN_MILLIMETERS,
        }
    }
}

pub trait VolumeMetricTeaspoonExt {
    fn as_teaspoons_metric(&self) -> f64;
    fn from_teaspoons_metric(teaspoons: f64) -> Volume;
}

impl VolumeMetricTeaspoonExt for Volume {
    fn as_teaspoons_metric(&self) -> f64 {
        self.as_millilitres() / METRIC_TEASPOON_IN_MILLIMETERS
    }

    fn from_teaspoons_metric(teaspoons: f64) -> Volume {
        Volume::from_millilitres(teaspoons * METRIC_TEASPOON_IN_MILLIMETERS)
    }
}

const METRIC_TABLESPOON_IN_MILLIMETERS: f64 = 15.0;

/// Decilitre is volume equivalent to one tenth of a liter (L/10).
pub struct MetricTablespoon {
    pub value: f64,
}

impl Measurement for MetricTablespoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("tbsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * METRIC_TABLESPOON_IN_MILLIMETERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / METRIC_TABLESPOON_IN_MILLIMETERS,
        }
    }
}

pub trait VolumeMetricTablespoonExt {
    fn as_tablespoons_metric(&self) -> f64;
    fn from_tablespoons_metric(teaspoons: f64) -> Volume;
}

impl VolumeMetricTablespoonExt for Volume {
    fn as_tablespoons_metric(&self) -> f64 {
        self.as_millilitres() / METRIC_TABLESPOON_IN_MILLIMETERS
    }

    fn from_tablespoons_metric(teaspoons: f64) -> Volume {
        Volume::from_millilitres(teaspoons * METRIC_TABLESPOON_IN_MILLIMETERS)
    }
}

const METRIC_DESSERTSPOON_IN_MILLILITERS: f64 = 10.0;

/// US Dessert spoon is a unit of culinary measure equivalent to 2 metric teaspoons.
pub struct MetricDessertSpoon {
    pub value: f64,
}

impl Measurement for MetricDessertSpoon {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("dsp", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * METRIC_DESSERTSPOON_IN_MILLILITERS
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / METRIC_DESSERTSPOON_IN_MILLILITERS,
        }
    }
}

pub trait VolumeMetricDessertSpoonExt {
    fn as_dessertspoons_metric(&self) -> f64;
    fn from_dessertspoons_metric(spoons: f64) -> Volume;
}

impl VolumeMetricDessertSpoonExt for Volume {
    fn as_dessertspoons_metric(&self) -> f64 {
        self.as_millilitres() / METRIC_DESSERTSPOON_IN_MILLILITERS
    }

    fn from_dessertspoons_metric(spoons: f64) -> Volume {
        Volume::from_millilitres(spoons * METRIC_DESSERTSPOON_IN_MILLILITERS)
    }
}

const METRIC_CUP_IN_MILLILITRES: f64 = 250.0;

/// In Australia, a standard measuring cup holds 250 milliliters of liquid.
pub struct MetricCup {
    pub value: f64,
}

impl Measurement for MetricCup {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        ("cup", self.value)
    }

    fn get_base_units_name(&self) -> &'static str {
        "ml"
    }

    fn as_base_units(&self) -> f64 {
        self.value * METRIC_CUP_IN_MILLILITRES
    }

    fn from_base_units(units: f64) -> Self {
        Self {
            value: units / METRIC_CUP_IN_MILLILITRES,
        }
    }
}

pub trait VolumeMetricCupExt {
    fn as_cups_metric(&self) -> f64;
    fn from_cups_metric(cups: f64) -> Volume;
}

impl VolumeMetricCupExt for Volume {
    fn as_cups_metric(&self) -> f64 {
        self.as_millilitres() / METRIC_CUP_IN_MILLILITRES
    }

    fn from_cups_metric(cups: f64) -> Volume {
        Volume::from_millilitres(cups * METRIC_CUP_IN_MILLILITRES)
    }
}
