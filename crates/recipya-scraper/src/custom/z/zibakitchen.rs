use schema_org::{
    AtType, Recipe, at_context,
    field::{
        RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeKeywordsFieldEnum,
    },
};
use scraper::{Html, Selector};

use crate::{
    Result,
    custom::common::{
        extract_attr, extract_duration, extract_ingredients_list, extract_instructions_list,
        extract_yield,
    },
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    Ok(Recipe {
        r#type: AtType::Recipe.to_opt(),
        context: at_context(),
        author: extract_attr(root, "a[rel='author']", "href")
            .unwrap_or_default()
            .map(|s| {
                vec![RecipeAuthorFieldEnum::new_person(
                    s.trim_end_matches('/')
                        .rsplit_once('/')
                        .unwrap_or_default()
                        .1,
                )]
            })
            .unwrap_or_default(),
        cook_time: extract_duration(root, "meta[itemprop='cookTime']", "content"),
        date_published: extract_attr(root, "span[itemprop='datePublished']", "content")
            .unwrap_or_default()
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        description: {
            let s: String = root
                .select(&Selector::parse(".pf-content p")?)
                .map(|el| el.text().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n\n");

            (!s.is_empty())
                .then_some(RecipeDescriptionFieldEnum::Text(s))
                .into_iter()
                .collect()
        },
        keywords: root
            .select(&Selector::parse(".post_details > a[rel='tag']")?)
            .flat_map(|el| el.text())
            .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into()))
            .collect::<Vec<_>>(),
        image: root
            .select(&Selector::parse("img[itemprop='image']")?)
            .filter_map(|el| el.attr("src"))
            .map(|s| RecipeImageFieldEnum::URL(s.into()))
            .collect::<Vec<_>>(),
        name: extract_attr(root, "a[itemprop='name']", "title")?
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        prep_time: extract_duration(root, "meta[itemprop='prepTime']", "content"),
        recipe_category: root
            .select(&Selector::parse(".post_details p a[rel='tag']")?)
            .filter_map(|el| el.text().next())
            .collect::<Vec<_>>()
            .first()
            .map(|s| vec![s.rsplit_once('(').unwrap_or((s, "")).0.trim().to_string()])
            .unwrap_or_default(),
        recipe_ingredient: extract_ingredients_list(root, "li[itemprop='recipeIngredient']")?,
        recipe_instructions: extract_instructions_list(root, "li[itemprop='recipeInstructions']")?,
        recipe_yield: extract_yield(root, ".recipe-serves span").unwrap_or_default(),
        url: vec![url.into()],
        ..Default::default()
    })
}
