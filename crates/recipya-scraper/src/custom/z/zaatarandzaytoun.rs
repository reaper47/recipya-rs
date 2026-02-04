use schema_org::{AggregateRating, Recipe, field::RecipeRecipeInstructionsFieldEnum};
use scraper::{ElementRef, Html, Selector};
use support::strings::extract_number;

use crate::{
    Result,
    custom::common::{
        extract_ingredients, extract_keywords, extract_metadata_property, required_text_list,
    },
};

pub(crate) fn add_info(doc: &Html, mut recipe: Recipe) -> Recipe {
    let content = doc
        .select(&Selector::parse("div.wprm-recipe-simple").unwrap())
        .next()
        .ok_or(doc.root_element())
        .unwrap();

    recipe.aggregate_rating = extract_rating(&content).unwrap_or_default();
    recipe.date_modified = extract_metadata_property(doc, "og:updated_time").unwrap_or_default();
    recipe.date_published =
        extract_metadata_property(doc, "article:published_time").unwrap_or_default();
    recipe.keywords = extract_keywords(&content, ".wprm-recipe-keyword").unwrap_or_default();
    recipe.recipe_ingredient = extract_ingredients(&content).unwrap_or_default();
    recipe.recipe_instructions = required_text_list(&content, ".wprm-recipe-instructions li")
        .unwrap_or_default()
        .into_iter()
        .map(RecipeRecipeInstructionsFieldEnum::Text)
        .collect();
    recipe
}

fn extract_rating(content: &ElementRef) -> Result<Vec<AggregateRating>> {
    let num_stars = content
        .select(&Selector::parse(".wprm-rating-star-full")?)
        .collect::<Vec<_>>();

    let rating = num_stars.len() as f32;
    let count = extract_number(
        &content
            .select(&Selector::parse(".wprm-recipe-rating-count")?)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default(),
    )
    .unwrap_or(0);

    Ok(vec![AggregateRating::new(rating, count)])
}
