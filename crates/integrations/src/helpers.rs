use iso8601::Duration;

use schema_org::field::{
    QuantitativeValueValueFieldEnum, RecipeIsBasedOnFieldEnum, RecipeRecipeYieldFieldEnum,
};
use schema_org::{DurationOrText, QuantitativeValue, Recipe};

pub(super) fn seconds_to_duration(secs: i32) -> Vec<DurationOrText> {
    format!("P{secs}S")
        .parse::<Duration>()
        .ok()
        .filter(|&d| d != Duration::default())
        .map(|d| vec![DurationOrText::from(d)])
        .unwrap_or(vec![])
}

pub(super) fn to_is_based_on(value: &str) -> Vec<RecipeIsBasedOnFieldEnum> {
    if value.is_empty() {
        vec![]
    } else {
        vec![RecipeIsBasedOnFieldEnum::URL(value.into())]
    }
}

pub(super) fn to_yield(value: i64) -> Vec<RecipeRecipeYieldFieldEnum> {
    vec![RecipeRecipeYieldFieldEnum::QuantitativeValue(Box::new(
        QuantitativeValue {
            value: vec![QuantitativeValueValueFieldEnum::Number(value as f32)],
            ..Default::default()
        },
    ))]
}

pub(crate) trait ToRecipeSchema {
    fn to_recipe_schema(&self) -> Recipe;
}
