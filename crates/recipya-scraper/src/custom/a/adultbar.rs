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

    let ingredients = content
        .first()
        .map(|c| {
            c.select(&row_sel)
                .map(|row| {
                    row.text()
                        .map(str::trim)
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

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
            .map(|v| vec![RecipeDescriptionFieldEnum::Text(v.into())])
            .unwrap_or_default(),
        keywords: extract_attr(root, "meta[name='keywords']", "content")
            .unwrap_or_default()
            .map(|v| vec![RecipeKeywordsFieldEnum::TextOrURL(v.into())])
            .unwrap_or_default(),
        image: extract_attr(root, "meta[property='twitter:image']", "content")
            .unwrap_or_default()
            .map(|v| vec![RecipeImageFieldEnum::URL(v.into())])
            .unwrap_or_default(),
        name: extract_attr(root, "meta[property='og:title']", "content")
            .unwrap_or_default()
            .map(|v| vec![v.trim_end_matches("Recipe").trim().into()])
            .unwrap_or_default(),
        recipe_ingredient: ingredients
            .into_iter()
            .map(RecipeRecipeIngredientFieldEnum::Text)
            .collect(),
        recipe_instructions: content
            .get(1)
            .map(|c| {
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
            })
            .unwrap_or_default(),
        recipe_yield: if servings == 0 {
            vec![]
        } else {
            vec![RecipeRecipeYieldFieldEnum::Text(servings.to_string())]
        },
        url: vec![url.to_string()],
        ..Default::default()
    })
}
