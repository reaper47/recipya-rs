use schema_org::{Recipe, field::RecipeRecipeInstructionsFieldEnum};
use scraper::{Html, Selector};

use crate::custom::common::{extract_author, extract_metadata_property, required_text_list};

pub(crate) fn add_info(doc: &Html, mut recipe: Recipe) -> Recipe {
    let content = doc
        .select(&Selector::parse("div.wprm-recipe").unwrap())
        .next()
        .ok_or(doc.root_element())
        .unwrap();

    recipe.author = extract_author(&content, ".wprm-recipe-author a").unwrap_or_default();
    recipe.date_modified =
        extract_metadata_property(doc, "article:modified_time").unwrap_or_default();
    recipe.date_published =
        extract_metadata_property(doc, "article:published_time").unwrap_or_default();
    recipe.recipe_instructions = required_text_list(&content, ".wprm-recipe-instructions li")
        .unwrap_or_default()
        .into_iter()
        .map(RecipeRecipeInstructionsFieldEnum::Text)
        .collect();

    recipe
}
