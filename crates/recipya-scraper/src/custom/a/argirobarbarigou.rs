use scraper::{Html, Selector};

use schema_org::{
    DurationOrText, Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        RecipeRecipeYieldFieldEnum,
    },
};
use support::strings::extract_number;

use crate::{
    Error, Result,
    custom::common::{at_type_recipe, extract_attr},
};

#[allow(clippy::too_many_lines)]
pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();
    let container = &root
        .select(&Selector::parse(
            ".container.recipe-container div.row.recipe-header",
        )?)
        .last()
        .ok_or_else(|| Error::MissingElement("container".to_string()))?;

    let times = container
        .select(&Selector::parse(".time")?)
        .last()
        .map(|e| e.text().collect::<String>());

    let times = times.and_then(|s| {
        let parts = s.split('|').collect::<Vec<_>>();
        match parts.len() {
            2 => Some((
                format!(
                    "PT{}M",
                    extract_number(parts[0].trim()).unwrap_or_else(|_| "5".to_string())
                ),
                Some(format!(
                    "PT{}M",
                    extract_number(parts[1].trim()).unwrap_or_else(|_| "15".to_string())
                )),
            )),
            1 => Some((
                format!(
                    "PT{}M",
                    extract_number(parts[0].trim()).unwrap_or_else(|_| "5".to_string())
                ),
                None,
            )),
            _ => None,
        }
    });

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        description: extract_attr(root, "meta[property='og:description']", "content")
            .unwrap_or_default()
            .map(|s| vec![RecipeDescriptionFieldEnum::Text(s.trim().to_string())])
            .unwrap_or_default(),
        keywords: container
            .select(&Selector::parse(
                ".special-diets-wrapper ul.special-diets-list li a span.full-name",
            )?)
            .map(|e| RecipeKeywordsFieldEnum::TextOrURL(e.text().collect::<String>()))
            .collect(),
        image: extract_attr(root, "meta[property='og:image:secure_url']", "content")
            .unwrap_or_default()
            .map(|s| vec![RecipeImageFieldEnum::URL(s.trim().to_string())])
            .unwrap_or_default(),
        name: extract_attr(root, "meta[property='og:title']", "content")
            .unwrap_or_default()
            .map(|s| vec![s.trim_end_matches("| ARGIRO BARBARIGOU").trim().to_string()])
            .unwrap_or_default(),
        prep_time: times
            .clone()
            .map(|(prep, _)| DurationOrText::Text(prep))
            .into_iter()
            .collect(),
        cook_time: times
            .and_then(|(_, cook)| cook)
            .map(DurationOrText::Text)
            .into_iter()
            .collect(),
        recipe_yield: container
            .select(&Selector::parse(".serves")?)
            .map(|el| {
                RecipeRecipeYieldFieldEnum::Text(
                    extract_number(&el.text().collect::<Vec<_>>().join(" "))
                        .unwrap_or(1)
                        .to_string(),
                )
            })
            .collect(),
        recipe_ingredient: container
            .select(&Selector::parse("ul.ingred-list li")?)
            .map(|el| {
                RecipeRecipeIngredientFieldEnum::Text(
                    el.text().collect::<Vec<_>>().join(" ").trim().into(),
                )
            })
            .collect(),
        recipe_instructions: container
            .select(&Selector::parse("div.drecipeSteps ol li")?)
            .map(|el| {
                RecipeRecipeInstructionsFieldEnum::Text(
                    el.text().collect::<Vec<_>>().join(" ").trim().into(),
                )
            })
            .collect(),
        url: vec![url.to_string()],
        ..Default::default()
    })
}
