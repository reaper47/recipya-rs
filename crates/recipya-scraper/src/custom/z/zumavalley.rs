use schema_org::{
    AtType, Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};
use scraper::{Element, Html, Selector};

use crate::{Result, custom::common::extract_metadata_property};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    Ok(Recipe {
        r#type: AtType::Recipe.to_opt(),
        context: at_context(),
        description: extract_metadata_property(doc, "og:description")
            .unwrap_or_default()
            .into_iter()
            .map(|s| RecipeDescriptionFieldEnum::Text(s))
            .collect::<Vec<_>>(),
        image: extract_metadata_property(doc, "og:image:secure_url")
            .unwrap_or_default()
            .into_iter()
            .map(|s| RecipeImageFieldEnum::URL(s))
            .collect::<Vec<_>>(),
        name: extract_metadata_property(doc, "og:title")?,
        recipe_ingredient: root
            .select(&Selector::parse("h3")?)
            .find(|el| el.text().any(|t| t.contains("Ingredients")))
            .and_then(|h3| h3.next_sibling_element())
            .map(|p| {
                p.inner_html()
                    .split("<br>")
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .map(|s| RecipeRecipeIngredientFieldEnum::Text(s))
                    .collect()
            })
            .unwrap_or_default(),
        recipe_instructions: root
            .select(&Selector::parse("ol li")?)
            .map(|n| RecipeRecipeInstructionsFieldEnum::Text(n.text().collect()))
            .collect::<Vec<_>>(),
        url: vec![url.into()],
        ..Default::default()
    })
}
