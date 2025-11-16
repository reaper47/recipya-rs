use iso8601::Duration;

use schema_org::field::{
    QuantitativeValueValueFieldEnum, RecipeIsBasedOnFieldEnum, RecipeKeywordsFieldEnum,
    RecipeRecipeYieldFieldEnum,
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

pub(super) fn to_defined_text(value: &str) -> Vec<RecipeKeywordsFieldEnum> {
    if value.is_empty() {
        vec![]
    } else {
        vec![RecipeKeywordsFieldEnum::TextOrURL(value.into())]
    }
}

pub(super) fn to_is_based_on(value: &str) -> Vec<RecipeIsBasedOnFieldEnum> {
    if value.is_empty() {
        vec![]
    } else {
        vec![RecipeIsBasedOnFieldEnum::URL(value.into())]
    }
}

pub(super) fn to_text(value: String) -> Option<TextOrTextObject> {
    Some(TextOrTextObject::Text(value)).filter(|t| match t {
        TextOrTextObject::Text(s) => !s.is_empty(),
        _ => false,
    })
}

pub(super) fn to_organization_type(value: String) -> Option<OrganizationTypeOrText> {
    (!value.is_empty()).then_some(OrganizationTypeOrText::Text(value))
}

pub(super) fn to_yield(value: i64) -> Vec<RecipeRecipeYieldFieldEnum> {
    vec![RecipeRecipeYieldFieldEnum::QuantitativeValue(Box::new(
        QuantitativeValue {
            value: vec![QuantitativeValueValueFieldEnum::Number(value as f32)],
            ..Default::default()
        },
    ))]
}

pub(super) fn vec_to_howto(values: Vec<&str>) -> Option<CreativeWorkOrItemListOrText> {
    Some(CreativeWorkOrItemListOrText::ItemList(
        values
            .into_iter()
            .map(|v| HowTo {
                at_type: HowToStep,
                text: v.into(),
                ..Default::default()
            })
            .collect(),
    ))
}

pub(crate) trait ToRecipeSchema {
    fn to_recipe_schema(&self) -> Recipe;
}
