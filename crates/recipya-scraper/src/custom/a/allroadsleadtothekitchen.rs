use scraper::{Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{RecipeAuthorFieldEnum, RecipeImageFieldEnum},
};

use crate::{
    Error, Result,
    custom::common::{
        at_type_recipe, extract_attr, extract_description, extract_duration,
        extract_ingredients_list, extract_instructions_list, extract_vec_string, extract_yield,
        optional_text,
    },
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();
    let content = root
        .select(&Selector::parse(".ccm-card")?)
        .next()
        .ok_or_else(|| Error::MissingElement("ccm-card".to_string()))?;

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        author: optional_text(root, "span[itemprop='author'] span")
            .unwrap_or_default()
            .map(|s| vec![RecipeAuthorFieldEnum::new_person(s.as_ref())])
            .unwrap_or_default(),
        cook_time: extract_duration(&content, "span[itemprop='cookTime']", "content"),
        description: extract_description(root, "div[itemprop='description']").unwrap_or_default(),
        image: extract_attr(&content, "img[itemprop='image']", "src")
            .unwrap_or_default()
            .into_iter()
            .map(|s| RecipeImageFieldEnum::URL(s.into()))
            .collect(),
        name: extract_vec_string(&content, "h3"),
        prep_time: extract_duration(&content, "span[itemprop='prepTime']", "content"),
        recipe_ingredient: extract_ingredients_list(root, "li[itemprop='recipeIngredient']")?,
        recipe_instructions: extract_instructions_list(root, "li[itemprop='recipeInstructions']")?,
        total_time: extract_duration(&content, "span[itemprop='totalTime']", "content"),
        recipe_yield: extract_yield(&content, "span[itemprop='recipeYield']").unwrap_or_default(),
        url: vec![url.to_string()],
        ..Default::default()
    })
}
