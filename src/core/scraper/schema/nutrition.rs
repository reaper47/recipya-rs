use std::fmt::Formatter;

use serde::{Deserialize, Deserializer, de};

use crate::core::scraper::schema::AtType;
use crate::core::scraper::schema::RestrictedDiet::{
    DiabeticDiet, GlutenFreeDiet, HalalDiet, HinduDiet, KosherDiet, LowCalorieDiet, LowFatDiet,
    LowLactoseDiet, LowSaltDiet, UnspecifiedDiet, VeganDiet, VegetarianDiet,
};
use crate::core::support::strings::extract_number;

/// Nutritional information about the recipe as described in the [schema](https://schema.org/NutritionInformation).
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum Energy {
    Str(String),
}

impl TryFrom<Energy> for i16 {
    type Error = String;

    fn try_from(value: Energy) -> Result<Self, Self::Error> {
        match value {
            Energy::Str(s) => {
                extract_number(s).map_err(|err| format!("Energy extraction error: {err}"))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Energy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::core::scraper::schema::nutrition::Energy::*;

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
#[derive(Clone, Debug, PartialEq)]
pub enum Mass {
    Str(String),
}

impl TryFrom<Mass> for i16 {
    type Error = String;

    fn try_from(value: Mass) -> Result<Self, Self::Error> {
        match value {
            Mass::Str(s) => {
                extract_number(s).map_err(|err| format!("Mass extraction error: {err}"))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Mass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::core::scraper::schema::nutrition::Mass::*;

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

impl From<String> for RestrictedDiet {
    fn from(value: String) -> Self {
        if value.contains("diabetic") {
            DiabeticDiet
        } else if value.contains("gluten") {
            GlutenFreeDiet
        } else if value.contains("halal") {
            HalalDiet
        } else if value.contains("hindu") {
            HinduDiet
        } else if value.contains("kosher") {
            KosherDiet
        } else if value.contains("low") {
            if value.contains("calorie") {
                LowCalorieDiet
            } else if value.contains("fat") {
                LowFatDiet
            } else if value.contains("lactose") {
                LowLactoseDiet
            } else if value.contains("salt") {
                LowSaltDiet
            } else {
                UnspecifiedDiet
            }
        } else if value.contains("vegan") {
            VeganDiet
        } else if value.contains("vegetarian") {
            VegetarianDiet
        } else {
            UnspecifiedDiet
        }
    }
}

impl<'de> Deserialize<'de> for RestrictedDiet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::core::scraper::schema::nutrition::RestrictedDiet::*;

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
                if v.contains("diabetic") {
                    Ok(DiabeticDiet)
                } else if v.contains("gluten") {
                    Ok(GlutenFreeDiet)
                } else if v.contains("halal") {
                    Ok(HalalDiet)
                } else if v.contains("hindu") {
                    Ok(HinduDiet)
                } else if v.contains("kosher") {
                    Ok(KosherDiet)
                } else if v.contains("low") {
                    if v.contains("calorie") {
                        Ok(LowCalorieDiet)
                    } else if v.contains("fat") {
                        Ok(LowFatDiet)
                    } else if v.contains("lactose") {
                        Ok(LowLactoseDiet)
                    } else if v.contains("salt") {
                        Ok(LowSaltDiet)
                    } else {
                        Ok(UnspecifiedDiet)
                    }
                } else if v.contains("vegan") {
                    Ok(VeganDiet)
                } else if v.contains("vegetarian") {
                    Ok(VegetarianDiet)
                } else {
                    Ok(UnspecifiedDiet)
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
