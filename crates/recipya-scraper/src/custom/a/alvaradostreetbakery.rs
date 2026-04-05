use scraper::{ElementRef, Html, Selector};
use unicode_normalization::UnicodeNormalization;

use schema_org::{
    DurationOrText, Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeInstructionsFieldEnum,
        RecipeRecipeYieldFieldEnum,
    },
};

use crate::{
    Result,
    custom::common::{at_type_recipe, extract_attr, extract_ingredients_list},
};

#[allow(clippy::too_many_lines)]
pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let meta = root
        .select(&Selector::parse("ul.recipe__meta li")?)
        .collect::<Vec<_>>();

    let directions = root
        .select(&Selector::parse("div.recipe__main")?)
        .find(|el| el.text().any(|s| s == "Directions"));

    let p_sel = Selector::parse("p")?;
    let ol_sel = Selector::parse("ol")?;
    let li_sel = Selector::parse("li")?;

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        description: directions
            .and_then(|el| {
                let parent = ElementRef::wrap(el.parent()?)?;

                let paragraphs: Vec<_> = parent.select(&p_sel).collect();
                let n = paragraphs.len();

                Some(
                    paragraphs
                        .into_iter()
                        .map(|p| {
                            RecipeDescriptionFieldEnum::Text(
                                p.text().collect::<String>().nfc().collect(),
                            )
                        })
                        .take(n.saturating_sub(1))
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default(),
        image: extract_attr(root, "meta[property='og:image:secure_url']", "content")?
            .map(|s| vec![RecipeImageFieldEnum::URL(s.into())])
            .unwrap_or_default(),
        name: extract_attr(root, "meta[property='og:title']", "content")?
            .map(|s| vec![s.into()])
            .unwrap_or_default(),
        prep_time: extract_meta(meta.as_slice(), "min")
            .into_iter()
            .map(DurationOrText::Text)
            .collect(),
        recipe_ingredient: extract_ingredients_list(
            root,
            ".recipe__ingredients li.metafield-single_line_text_field",
        )
        .unwrap_or_default(),
        recipe_instructions: directions
            .and_then(|el| {
                let mut ancestor = el.parent();
                loop {
                    let node = ancestor?;
                    if let Some(el_ref) = ElementRef::wrap(node)
                        && let Some(ol) = el_ref.select(&ol_sel).next()
                    {
                        return Some(ol);
                    }
                    ancestor = node.parent();
                }
            })
            .map(|ol| {
                ol.select(&li_sel)
                    .map(|li| {
                        RecipeRecipeInstructionsFieldEnum::Text(li.text().collect::<String>())
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        recipe_yield: extract_meta(meta.as_slice(), "serving")
            .into_iter()
            .map(RecipeRecipeYieldFieldEnum::Text)
            .collect(),
        url: vec![url.to_string()],
        ..Default::default()
    })
}

fn extract_meta(meta: &[ElementRef<'_>], needle: &str) -> Vec<String> {
    meta.iter()
        .filter(|el| el.text().any(|s| s.to_lowercase().contains(needle)))
        .map(|el| el.text().collect::<String>().trim().into())
        .collect()
}
