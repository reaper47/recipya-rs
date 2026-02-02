use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::{Energy, Mass};

///<https://schema.org/NutritionInformation>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct NutritionInformation {
    ///<https://schema.org/calories>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub calories: Vec<Energy>,
    ///<https://schema.org/carbohydrateContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub carbohydrate_content: Vec<Mass>,
    ///<https://schema.org/cholesterolContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cholesterol_content: Vec<Mass>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/fatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fat_content: Vec<Mass>,
    ///<https://schema.org/fiberContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fiber_content: Vec<Mass>,
    ///<https://schema.org/proteinContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub protein_content: Vec<Mass>,
    ///<https://schema.org/saturatedFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub saturated_fat_content: Vec<Mass>,
    ///<https://schema.org/servingSize>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub serving_size: Vec<String>,
    ///<https://schema.org/sodiumContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sodium_content: Vec<Mass>,
    ///<https://schema.org/sugarContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sugar_content: Vec<Mass>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    ///<https://schema.org/transFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trans_fat_content: Vec<Mass>,
    ///<https://schema.org/unsaturatedFatContent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unsaturated_fat_content: Vec<Mass>,
}

impl NutritionInformation {
    /// Checks whether all fields of the schema are `None`.
    pub const fn is_empty(&self) -> bool {
        self.calories.is_empty()
            && self.carbohydrate_content.is_empty()
            && self.cholesterol_content.is_empty()
            && self.fat_content.is_empty()
            && self.fiber_content.is_empty()
            && self.protein_content.is_empty()
            && self.saturated_fat_content.is_empty()
            && self.serving_size.is_empty()
            && self.sodium_content.is_empty()
            && self.sugar_content.is_empty()
            && self.trans_fat_content.is_empty()
            && self.unsaturated_fat_content.is_empty()
    }

    /// Checks whether the nutrition is per 100g. If the serving size is not provided, it is assumed to be per 100g.
    pub fn is_per_100g(&self) -> bool {
        match &self.serving_size.first() {
            Some(serving_size) => {
                let normalized = serving_size.to_lowercase();

                normalized.contains("100 g")
                    || normalized.contains("100g")
                    || normalized.contains("100 gram")
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let empty = NutritionInformation::default();
        assert!(empty.is_empty());

        let non_empty = NutritionInformation {
            calories: vec![Energy::new("100 kcal")],
            ..Default::default()
        };
        assert!(!non_empty.is_empty());
    }

    mod tests_is_per_100g {
        use super::*;

        fn nutrition_with_serving(serving_size: Vec<String>) -> NutritionInformation {
            NutritionInformation {
                serving_size,
                ..Default::default()
            }
        }

        #[test]
        fn test_no_serving_size_is_per_100g() {
            let nutrition = nutrition_with_serving(vec![]);

            assert!(nutrition.is_per_100g());
        }

        #[test]
        fn test_empty_vec_is_per_100g() {
            let nutrition = nutrition_with_serving(vec![]);

            assert!(nutrition.is_per_100g());
        }

        #[test]
        fn test_various_formats_is_per_100g() {
            for case in [
                "per 100g",
                "Per 100G",
                "PER 100 GRAM",
                "100 g",
                "100g",
                "Nutrition per 100g",
                "  per 100g  ",
            ] {
                let nutrition = nutrition_with_serving(vec![case.to_string()]);

                assert!(nutrition.is_per_100g(), "Failed for: '{case}'");
            }
        }

        #[test]
        fn test_serving_sizes_is_per_serving() {
            for case in [
                "1 serving",
                "1 cup",
                "2 tablespoons",
                "150g",
                "per 100ml",
                "1 slice",
            ] {
                let nutrition = nutrition_with_serving(vec![case.to_string()]);

                assert!(!nutrition.is_per_100g(), "Should fail for: '{case}'");
            }
        }
    }
}
