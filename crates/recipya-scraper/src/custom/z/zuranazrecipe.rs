use schema_org::{
    AtType, Recipe, at_context,
    field::{
        RecipeImageFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};
use scraper::{Html, Selector};

use crate::{
    Result,
    custom::common::{extract_attr, extract_author, extract_text_from_elements, required_text},
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    Ok(Recipe {
        r#type: AtType::Recipe.to_opt(),
        context: at_context(),
        author: extract_author(root, "a[itemprop='author']").unwrap_or_default(),
        date_published: extract_attr(root, "meta[itemprop='datePublished']", "content")
            .unwrap_or_default()
            .map(|s| vec![s.to_string()])
            .unwrap_or_default(),
        image: extract_attr(root, "img[itemprop='image']", "src")
            .unwrap_or_default()
            .map(|s| vec![RecipeImageFieldEnum::URL(s.into())])
            .unwrap_or_default(),
        keywords: root
            .select(&Selector::parse(".tags a[rel='tag']")?)
            .filter_map(|el| {
                let text = el.text().collect::<String>().trim().to_string();
                if text.is_empty() {
                    None
                } else {
                    Some(RecipeKeywordsFieldEnum::TextOrURL(
                        text.trim_end_matches(',').trim().to_string(),
                    ))
                }
            })
            .collect::<Vec<_>>(),
        name: vec![required_text(root, "h1[itemprop='name']")?],
        recipe_category: root
            .select(&Selector::parse(
                "span[itemprop='recipeCategory'] a[rel='tag']",
            )?)
            .filter_map(|el| {
                let text = el.text().collect::<String>().trim().to_string();
                if text.is_empty() {
                    None
                } else {
                    Some(text.trim_end_matches(',').trim().to_string())
                }
            })
            .collect::<Vec<_>>(),
        recipe_ingredient: extract_text_from_elements(root, "li[itemprop='ingredients']")
            .unwrap_or_default()
            .into_iter()
            .map(|s| RecipeRecipeIngredientFieldEnum::Text(s))
            .collect(),
        recipe_instructions: extract_text_from_elements(
            root,
            "div[itemprop='recipeInstructions'] ul li",
        )
        .unwrap_or_default()
        .into_iter()
        .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s))
        .collect(),
        url: vec![url.into()],
        ..Default::default()
    })
}
