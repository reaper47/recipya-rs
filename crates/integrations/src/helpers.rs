use iso8601::Duration;

use schema_org::field::{
    QuantitativeValueValueFieldEnum, RecipeIsBasedOnFieldEnum, RecipeRecipeYieldFieldEnum,
};
use schema_org::{AtType, CreativeWork, DurationOrText, QuantitativeValue, Recipe, at_context};

/// Converts seconds to a `DurationOrText` enum.
pub fn seconds_to_duration(secs: i32) -> Vec<DurationOrText> {
    format!("P{secs}S")
        .parse::<Duration>()
        .ok()
        .filter(|&d| d != Duration::default())
        .map(|d| vec![DurationOrText::from(d)])
        .unwrap_or(vec![])
}

/// Converts a quantity value to a `RecipeRecipeYieldFieldEnum` enum.
pub fn to_is_based_on(value: &str) -> Vec<RecipeIsBasedOnFieldEnum> {
    if value.is_empty() {
        vec![]
    } else {
        match url::Url::parse(value) {
            Ok(_) => vec![RecipeIsBasedOnFieldEnum::URL(value.into())],
            Err(_) => {
                vec![RecipeIsBasedOnFieldEnum::CreativeWork(Box::new(
                    CreativeWork {
                        r#type: AtType::CreativeWork.to_opt(),
                        text: vec![value.trim().into()],
                        ..Default::default()
                    },
                ))]
            }
        }
    }
}

/// Converts a quantity value to a `RecipeRecipeYieldFieldEnum` enum.
#[allow(clippy::cast_precision_loss)]
pub fn to_yield(value: i64) -> Vec<RecipeRecipeYieldFieldEnum> {
    if value == 0 {
        vec![]
    } else {
        vec![RecipeRecipeYieldFieldEnum::QuantitativeValue(Box::new(
            QuantitativeValue {
                r#type: AtType::QuantitativeValue.to_string(),
                context: at_context(),
                value: vec![QuantitativeValueValueFieldEnum::Number(value as f64)],
                ..Default::default()
            },
        ))]
    }
}

/// Trait for converting a value to a `Recipe` schema.
pub trait ToRecipeSchema {
    fn to_recipe_schema(&self) -> Recipe;
}
