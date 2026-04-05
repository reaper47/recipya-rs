use itertools::Itertools;
use scraper::{Element, ElementRef, Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum, RecipeYieldFieldEnum,
    },
};
use support::strings::extract_number;

use crate::{
    Error, Result,
    custom::common::{at_type_recipe, extract_attr, extract_vec_string},
};

#[allow(clippy::too_many_lines)]
pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();
    let content = &root
        .select(&Selector::parse("#content")?)
        .next()
        .ok_or_else(|| Error::MissingElement("#content".to_string()))?;

    let entry_content = &root
        .select(&Selector::parse(".entry-content")?)
        .next()
        .ok_or_else(|| Error::MissingElement(".entry_content".to_string()))?;

    let mut keywords = content
        .select(&Selector::parse(".cat-links a")?)
        .map(|s| s.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>();

    let p_sel = Selector::parse("p")?;

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        description: entry_content
            .first_element_child()
            .map_or(Vec::new(), |el| {
                vec![RecipeDescriptionFieldEnum::Text(
                    el.text().collect::<Vec<_>>().join(" "),
                )]
            }),
        date_created: content
            .select(&Selector::parse(".post-date")?)
            .next()
            .map(|s| s.text().collect::<Vec<_>>().join(" "))
            .into_iter()
            .collect::<Vec<_>>(),
        recipe_category: keywords.pop().map(|s| vec![s]).unwrap_or_default(),
        keywords: keywords
            .into_iter()
            .map(RecipeKeywordsFieldEnum::TextOrURL)
            .collect(),
        image: extract_attr(content, "div.header-img", "style")
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let url = s
                    .split("url(")
                    .nth(1)
                    .and_then(|s| s.strip_suffix(");"))
                    .unwrap_or_default();
                RecipeImageFieldEnum::URL(url.into())
            })
            .collect(),
        name: extract_vec_string(content, "h1.entry-title"),
        recipe_ingredient: entry_content
            .select(&p_sel)
            .find(|el| el.text().contains("Ingredients:"))
            .ok_or_else(|| Error::MissingElement("ingredients".into()))?
            .children()
            .fold(Vec::new(), |mut acc, node| {
                if let Some(el) = node.value().as_element()
                    && el.name() == "br"
                {
                    acc.push(String::new());
                } else if let Some(text) = node.value().as_text() {
                    let t = text.trim();
                    if !t.is_empty() {
                        match acc.last_mut() {
                            Some(last) => last.push_str(t),
                            None => acc.push(t.into()),
                        }
                    }
                }
                acc
            })
            .into_iter()
            .filter(|s| !s.is_empty() && !s.contains("Ingredients"))
            .map(RecipeRecipeIngredientFieldEnum::Text)
            .collect::<Vec<_>>(),
        recipe_instructions: entry_content
            .select(&p_sel)
            .find(|el| el.text().contains("Directions:"))
            .ok_or_else(|| Error::MissingElement("instructions".into()))?
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .filter(|el| el.value().name() == "p")
            .map(|el| {
                RecipeRecipeInstructionsFieldEnum::Text(
                    el.text().collect::<Vec<_>>().join(" ").trim().into(),
                )
            })
            .collect::<Vec<_>>(),
        recipe_yield: extract_number::<String>(
            entry_content
                .select(&p_sel)
                .find(|el| el.text().any(|t| t.contains("servings")))
                .map(|el| el.text().collect::<Vec<_>>().join(" "))
                .unwrap_or_default()
                .trim(),
        )
        .map_or_else(
            |_| vec![RecipeYieldFieldEnum::Text("1".into())],
            |s| vec![RecipeYieldFieldEnum::Text(s)],
        ),
        url: vec![url.to_string()],
        ..Default::default()
    })
}
