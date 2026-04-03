use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};

use crate::{
    Result,
    custom::common::{at_type_recipe, extract_attr},
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let div_sel = Selector::parse("div.MsoNormal")?;

    let ingredients_start = root
        .select(&div_sel)
        .find(|el| el.text().contains("Ingredients:"));

    let instructions_start = root
        .select(&div_sel)
        .find(|el| el.text().contains("Method:"));

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        date_published: extract_attr(root, "meta[property='article:published_time']", "content")
            .unwrap_or_default()
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        date_modified: extract_attr(root, "meta[property='article:modified_time']", "content")
            .unwrap_or_default()
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        description: extract_attr(root, "meta[property='og:description']", "content")
            .unwrap_or_default()
            .map(|s| {
                vec![RecipeDescriptionFieldEnum::Text(
                    String::from_utf8_lossy(s.as_bytes()).to_string(),
                )]
            })
            .unwrap_or_default(),
        image: extract_attr(root, "meta[property='og:image']", "content")
            .unwrap_or_default()
            .map(|s| vec![RecipeImageFieldEnum::URL(s.into())])
            .unwrap_or_default(),
        recipe_ingredient: ingredients_start
            .map(|start_node| {
                start_node
                    .next_siblings()
                    .filter_map(|node| {
                        node.value()
                            .as_element()
                            .map(|_| ElementRef::wrap(node).unwrap())
                    })
                    .take_while(|el| !el.text().any(|t| t.contains("Method:")))
                    .map(|el| RecipeRecipeIngredientFieldEnum::Text(sanitize_text(&el)))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        recipe_instructions: instructions_start
            .map(|start_node| {
                start_node
                    .next_siblings()
                    .filter_map(ElementRef::wrap)
                    .take_while(|el| !el.text().any(|t| t.contains("Note:")))
                    .map(|el| sanitize_text(&el))
                    .filter(|s| !s.is_empty())
                    .map(RecipeRecipeInstructionsFieldEnum::Text)
                    .collect()
            })
            .unwrap_or_default(),
        name: extract_attr(root, "meta[property='og:title']", "content")
            .unwrap_or_default()
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        url: vec![url.to_string()],
        ..Default::default()
    })
}

fn sanitize_text(el: &ElementRef<'_>) -> String {
    el.text()
        .flat_map(|t| t.chars())
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
