use scraper::{Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        RecipeRecipeYieldFieldEnum,
    },
};
use support::strings::extract_number;

use crate::{
    Result,
    custom::common::{at_type_recipe, extract_attr},
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let content = root
        .select(&Selector::parse(".card-deck.mb-1 > div")?)
        .collect::<Vec<_>>();

    let row_sel = Selector::parse(".row")?;
    let ol_sel = Selector::parse("ol li")?;

    let ingredients = content.first().map_or_default(|c| {
        c.select(&row_sel)
            .map(|row| {
                row.text()
                    .map(str::trim)
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
    });

    let servings = ingredients
        .iter()
        .filter(|s| s.contains("ml"))
        .fold(0, |acc, s| {
            let i: i32 = extract_number(s).unwrap_or_default();
            i + acc
        });

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        description: extract_attr(root, "meta[property='og:description']", "content")
            .unwrap_or_default()
            .map_or_default(|v| vec![RecipeDescriptionFieldEnum::Text(v.into())]),
        keywords: extract_attr(root, "meta[name='keywords']", "content")
            .unwrap_or_default()
            .map_or_default(|v| vec![RecipeKeywordsFieldEnum::TextOrURL(v.into())]),
        image: extract_attr(root, "meta[property='twitter:image']", "content")
            .unwrap_or_default()
            .map_or_default(|v| vec![RecipeImageFieldEnum::URL(v.into())]),
        name: extract_attr(root, "meta[property='og:title']", "content")
            .unwrap_or_default()
            .map_or_default(|v| vec![v.trim_end_matches("Recipe").trim().into()]),
        recipe_ingredient: ingredients
            .into_iter()
            .map(RecipeRecipeIngredientFieldEnum::Text)
            .collect(),
        recipe_instructions: content.get(1).map_or_default(|c| {
            c.select(&ol_sel)
                .map(|row| {
                    row.text()
                        .map(str::trim)
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .map(RecipeRecipeInstructionsFieldEnum::Text)
                .collect::<Vec<_>>()
        }),
        recipe_yield: if servings == 0 {
            vec![]
        } else {
            vec![RecipeRecipeYieldFieldEnum::Text(servings.to_string())]
        },
        url: vec![url.to_string()],
        ..Default::default()
    })
}
