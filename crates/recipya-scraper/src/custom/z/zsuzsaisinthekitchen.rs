use schema_org::{
    AtType, Recipe, at_context,
    field::{
        RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    },
};
use scraper::{Html, Selector};

use crate::{
    Result,
    custom::common::{extract_attr, extract_ingredients, extract_metadata_property, optional_text},
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let name = extract_metadata_property(doc, "og:title")?;
    let title = name.first().cloned().unwrap();

    Ok(Recipe {
        r#type: AtType::Recipe.to_opt(),
        context: at_context(),
        author: optional_text(root, "span[itemprop='author'] span[itemprop='name']")
            .unwrap_or_default()
            .map(|s| vec![RecipeAuthorFieldEnum::new_person(&s)])
            .unwrap_or_default(),
        date_published: extract_attr(root, "abbr[itemprop='datePublished']", "title")
            .unwrap_or_default()
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        description: root
            .select(&Selector::parse(
                "div[itemprop='description articleBody'] div.MsoNormal",
            )?)
            .next()
            .map(|el| el.text().collect::<String>())
            .map(|s| vec![RecipeDescriptionFieldEnum::Text(s.trim().into())])
            .unwrap_or_default(),
        image: extract_attr(root, "meta[itemprop='image_url']", "content")
            .unwrap_or_default()
            .map(|s| vec![RecipeImageFieldEnum::URL(s.into())])
            .unwrap_or_default(),
        name,
        recipe_ingredient: root
            .select(&Selector::parse("div.MsoNormal")?)
            .find(|el| el.text().any(|t| t.contains(&title)))
            .map(|node| {
                node.next_siblings()
                    .take_while(|n| match n.value().as_element() {
                        Some(el) => el.name() != "ul",
                        None => true,
                    })
                    .filter_map(|node| {
                        println!("{}", node.children().collect::<Vec<_>>().len());
                        node.value().as_element().and_then(|_| {
                            let text = scraper::ElementRef::wrap(node)?
                                .text()
                                .collect::<String>()
                                .trim()
                                .to_string();

                            if text.is_empty() {
                                None
                            } else {
                                Some(RecipeRecipeIngredientFieldEnum::Text(text))
                            }
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        recipe_instructions: root
            .select(&Selector::parse("div > ul:first-of-type > li")?)
            .map(|n| {
                RecipeRecipeInstructionsFieldEnum::Text(
                    n.text()
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" "),
                )
            })
            .collect::<Vec<_>>(),
        url: vec![url.into()],
        ..Default::default()
    })
}
