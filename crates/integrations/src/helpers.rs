use iso8601::Duration;

use recipe_schema::components::{
    CreativeWorkOrItemListOrText, CreativeWorkOrText, DefinedTermOrTextOrURL, HowTo,
    OrganizationTypeOrText, QuantitativeValue, QuantitativeValueOrText, Sections, TextOrTextObject,
};
use recipe_schema::{AtType::HowToStep, RecipeSchema};

pub(super) fn seconds_to_duration(secs: i32) -> Option<Duration> {
    Some(format!("P{secs}S").parse::<Duration>().unwrap_or_default())
        .filter(|&d| d != Duration::default())
}

pub(super) fn sections_to_vec(sections: Sections) -> Option<Vec<String>> {
    let mut elements = vec![];
    sections.into_iter().for_each(|(section_name, els)| {
        elements.push(format!("<section>{section_name}</section>"));
        elements.extend(els.into_iter().map(|item| item.text));
    });
    Some(elements).filter(|v| !v.is_empty())
}

pub(super) fn sections_to_itemlist(sections: Sections) -> Option<CreativeWorkOrItemListOrText> {
    let elements = sections
        .into_iter()
        .flat_map(|(section_name, els)| {
            els.into_iter().map(move |item| HowTo {
                at_type: HowToStep,
                name: Some(section_name.clone()).filter(|s| !s.is_empty()),
                text: item.text,
                ..Default::default()
            })
        })
        .filter(|h| h.text != "<section></section>")
        .collect::<Vec<_>>();

    if elements.is_empty() {
        None
    } else {
        Some(CreativeWorkOrItemListOrText::ItemList(elements))
    }
}

pub(super) fn to_defined_text(value: String) -> Option<DefinedTermOrTextOrURL> {
    Some(DefinedTermOrTextOrURL::Text(value)).filter(|t| match t {
        DefinedTermOrTextOrURL::Text(s) => !s.is_empty(),
        _ => false,
    })
}

pub(super) fn to_is_based_on(value: String) -> Option<CreativeWorkOrText> {
    Some(CreativeWorkOrText::Text(value)).filter(|s| match s {
        CreativeWorkOrText::Text(s) => !s.is_empty(),
        _ => false,
    })
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

pub(super) fn to_yield(value: i64) -> QuantitativeValueOrText {
    QuantitativeValueOrText::QuantitativeValue(QuantitativeValue { value })
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
    fn to_recipe_schema(&self) -> RecipeSchema;
}
