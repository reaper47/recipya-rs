use scraper::{ElementRef, Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{
        RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    },
};

use crate::{
    Error, Result,
    custom::common::{at_type_recipe, extract_attr, required_text_list},
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let content = &root
        .select(&Selector::parse(".recipe-inner-container")?)
        .next()
        .ok_or_else(|| Error::MissingElement(".recipe-inner-container".to_string()))?;

    let inner = content
        .select(&Selector::parse(".recipe-inner-desc")?)
        .next()
        .ok_or_else(|| Error::MissingElement(".recipe-inner-desc".to_string()))?;

    let sel_li = Selector::parse("li")?;

    let mut ingredients: Vec<(String, Vec<String>)> = Vec::new();
    let mut instructions: Vec<(String, Vec<String>)> = Vec::new();

    inner.children().for_each(|node| {
        if let Some(el) = ElementRef::wrap(node) {
            let text = el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            let name = el.value().name();

            if text.starts_with("For") {
                ingredients.push((text.clone(), Vec::new()));
                instructions.push((text, Vec::new()));
            } else if name == "li" {
                if let Some(v) = ingredients.last_mut() {
                    v.1.push(text);
                }
            } else if name == "ol" {
                let items = el
                    .select(&sel_li)
                    .map(|el| el.text().collect::<Vec<_>>().join(" "))
                    .collect::<Vec<_>>();

                if let Some(v) = instructions.last_mut() {
                    v.1.extend(items);
                }
            }
        }
    });

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        image: extract_attr(content, "img", "src")
            .unwrap_or_default()
            .map(|v| vec![RecipeImageFieldEnum::URL(v.into())])
            .unwrap_or_default(),
        name: required_text_list(content, "h3")?,
        recipe_ingredient: ingredients
            .into_iter()
            .map(|(name, items)| {
                RecipeRecipeIngredientFieldEnum::new_section(
                    &name,
                    &items.iter().map(String::as_str).collect::<Vec<_>>(),
                )
            })
            .collect(),
        recipe_instructions: instructions
            .into_iter()
            .map(|(name, items)| {
                RecipeRecipeInstructionsFieldEnum::new_section(
                    &name,
                    items.iter().map(String::as_str).collect::<Vec<_>>(),
                )
            })
            .collect(),

        url: vec![url.to_string()],
        ..Default::default()
    })
}
