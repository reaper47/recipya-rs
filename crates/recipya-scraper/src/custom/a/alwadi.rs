use itertools::Itertools;
use scraper::{Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{
        RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        RecipeRecipeYieldFieldEnum,
    },
};
use support::strings::extract_number;

use crate::{
    Error, Result,
    custom::common::{at_type_recipe, extract_attr, extract_description, extract_vec_string},
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();
    let content = &root
        .select(&Selector::parse(".details .container")?)
        .last()
        .ok_or_else(|| Error::MissingElement("container".to_string()))?;

    let recipe_yield = if let Some(el) = content.select(&Selector::parse(".serving")?).next()
        && let Some(next) = el.last_child()
        && let Some(text_node) = next.value().as_text()
    {
        extract_number(text_node)
            .ok()
            .map_or_else(|| Vec::new(), |n| vec![RecipeRecipeYieldFieldEnum::Text(n)])
    } else {
        vec![]
    };

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        description: extract_description(content, ".description").unwrap_or_default(),
        cook_time: extract_vec_string(content, ".duration")
            .iter()
            .map(|s| schema_org::DurationOrText::Text(s.trim().into()))
            .collect(),
        image: extract_attr(root, "meta[property='og:image']", "content")
            .unwrap_or_default()
            .map(|s| vec![RecipeImageFieldEnum::URL(s.into())])
            .unwrap_or_default(),
        recipe_category: extract_vec_string(content, ".category > a"),
        recipe_yield,
        recipe_ingredient: content
            .select(&Selector::parse(".recipeIngredients table tr")?)
            .skip(1)
            .filter_map(|s| {
                let s = s.text().flat_map(|t| t.split_whitespace()).join(" ");
                if s.is_empty() {
                    None
                } else {
                    Some(RecipeRecipeIngredientFieldEnum::Text(s))
                }
            })
            .collect(),
        recipe_instructions: content
            .select(&Selector::parse(".preparationSteps ol li")?)
            .filter_map(|s| {
                let s = s.text().flat_map(|t| t.split_whitespace()).join(" ");
                if s.is_empty() {
                    None
                } else {
                    Some(RecipeRecipeInstructionsFieldEnum::Text(s))
                }
            })
            .collect(),
        name: extract_vec_string(content, ".title"),
        url: vec![url.to_string()],
        ..Default::default()
    })
}
