use schema_org::{
    AtType, Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};
use scraper::{Html, Selector};

use crate::{Error, Result, custom::common::extract_metadata_property};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();
    let article_body = doc
        .select(&Selector::parse("div[itemprop='articleBody']")?)
        .next()
        .ok_or(Error::MissingElement("div[itemprop='articleBody']".into()))?;

    Ok(Recipe {
        r#type: AtType::Recipe.to_opt(),
        context: at_context(),
        date_published: root
            .select(&Selector::parse(".article__date time")?)
            .next()
            .map(|e| vec![e.value().attr("datetime").unwrap_or_default().to_string()])
            .unwrap_or_default(),
        description: extract_metadata_property(doc, "og:description")
            .unwrap_or_default()
            .into_iter()
            .map(|v| RecipeDescriptionFieldEnum::Text(v))
            .collect(),
        image: extract_metadata_property(doc, "og:image:secure_url")
            .unwrap_or_default()
            .into_iter()
            .map(|v| RecipeImageFieldEnum::URL(v))
            .collect(),
        name: extract_metadata_property(doc, "og:title")?,
        recipe_ingredient: article_body
            .select(&Selector::parse("ul li")?)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|v| {
                RecipeRecipeIngredientFieldEnum::Text(v.text().collect::<String>().trim().into())
            })
            .collect(),
        recipe_instructions: article_body
            .select(&Selector::parse("ol li")?)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|v| {
                RecipeRecipeInstructionsFieldEnum::Text(v.text().collect::<String>().trim().into())
            })
            .collect(),
        url: vec![url.into()],
        ..Default::default()
    })
}
