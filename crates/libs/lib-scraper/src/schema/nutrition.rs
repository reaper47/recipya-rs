use std::fmt::Formatter;

use serde::{de, Deserialize, Deserializer};

use crate::schema::AtType;

/// Nutritional information about the recipe as described in the [schema](https://schema.org/NutritionInformation).
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NutritionInformationSchema {
    #[serde(rename = "@type")]
    pub at_type: Option<AtType>,
    /// The number of calories (kcal).
    pub calories: Option<Energy>,
    /// The number of grams of carbohydrates.
    pub carbohydrate_content: Option<Mass>,
    /// The number of milligrams of cholesterol.
    pub cholesterol_content: Option<Mass>,
    /// The number of grams of fat.
    pub fat_content: Option<Mass>,
    /// The number of grams of fiber.
    pub fiber_content: Option<Mass>,
    /// The number of grams of protein.
    pub protein_content: Option<Mass>,
    /// The number of grams of saturated fat.
    pub saturated_fat_content: Option<Mass>,
    /// The serving size, in terms of the number of volume or mass.
    pub serving_size: Option<String>,
    /// The number of milligrams of sodium.
    pub sodium_content: Option<Mass>,
    /// The number of grams of sugar.
    pub sugar_content: Option<Mass>,
    /// The number of grams of trans fat.
    pub trans_fat_content: Option<Mass>,
    /// The number of grams of unsaturated fat.
    pub unsaturated_fat_content: Option<Mass>,
}

/// Properties that take Energy as values are of the form '<Number> <Energy unit of measure>'.
#[derive(Debug, PartialEq)]
pub enum Energy {
    Str(String),
}

impl<'de> Deserialize<'de> for Energy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::nutrition::Energy::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Energy;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing an energy value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Str(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Str(v))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// Properties that take Mass as values are of the form '<Number> <Mass unit of measure>'. E.g., '7 kg'.
#[derive(Debug, PartialEq)]
pub enum Mass {
    Str(String),
}

impl<'de> Deserialize<'de> for Mass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::nutrition::Mass::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Mass;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing an energy value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Str(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Str(v))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// A diet restricted to certain foods or preparations for cultural, religious, health or lifestyle reasons.
#[derive(Debug, Default, PartialEq)]
pub enum RestrictedDiet {
    DiabeticDiet,
    GlutenFreeDiet,
    HalalDiet,
    HinduDiet,
    KosherDiet,
    LowCalorieDiet,
    LowFatDiet,
    LowLactoseDiet,
    LowSaltDiet,
    VeganDiet,
    VegetarianDiet,
    #[default]
    UnspecifiedDiet,
}

impl<'de> Deserialize<'de> for RestrictedDiet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::nutrition::RestrictedDiet::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = RestrictedDiet;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.trim() {
                    "DiabeticDiet" => Ok(DiabeticDiet),
                    "GlutenFreeDiet" => Ok(GlutenFreeDiet),
                    "HalalDiet" => Ok(HalalDiet),
                    "HinduDiet" => Ok(HinduDiet),
                    "KosherDiet" => Ok(KosherDiet),
                    "LowCalorieDiet" => Ok(LowCalorieDiet),
                    "LowFatDiet" => Ok(LowFatDiet),
                    "LowLactoseDiet" => Ok(LowLactoseDiet),
                    "LowSaltDiet" => Ok(LowSaltDiet),
                    "VeganDiet" => Ok(VeganDiet),
                    "VegetarianDiet" => Ok(VegetarianDiet),
                    _ => Ok(UnspecifiedDiet),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
