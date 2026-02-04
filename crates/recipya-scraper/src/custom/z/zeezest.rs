use schema_org::{
    DurationOrText, Recipe, at_context,
    field::{
        RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    },
};
use scraper::{ElementRef, Html, Selector};

use crate::{
    Error, Result,
    custom::common::{
        at_type_recipe, extract_author, extract_description, extract_metadata_property,
        normalize_text, required_text,
    },
};

pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let ingredients_container_sel = Selector::parse(".recipes-module__gO5IBa__ingredientsList")?;
    let ingredient_groups_sel = Selector::parse("li.recipes-module__gO5IBa__listHeading")?;
    let ingredient_sel = Selector::parse("li.ingredient p")?;
    let instructions_sel = Selector::parse("ol.recipes-module__gO5IBa__customOrderList")?;

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        author: extract_author(root, ".recepiescovercard-module__V__cIq__schefDetails > p")
            .unwrap_or_default(),
        cook_time: get_metadata_box(root, "img[alt='Cooking Time']")
            .map(|s| vec![DurationOrText::Text(s)])
            .unwrap_or_default(),
        description: extract_description(root, ".recepiescovercard-module__V__cIq__disc")
            .unwrap_or_default(),
        image: extract_metadata_property(doc, "og:image")
            .unwrap_or_default()
            .into_iter()
            .map(RecipeImageFieldEnum::URL)
            .collect(),
        name: required_text(root, ".recepiescovercard-module__V__cIq__title")
            .ok()
            .map(|s| vec![s])
            .unwrap_or_default(),
        prep_time: get_metadata_box(root, "img[alt='Preparation Time']")
            .map(|s| vec![DurationOrText::Text(s)])
            .unwrap_or_default(),
        recipe_category: get_metadata_box(root, "img[alt='Meal Type']")
            .map(|s| vec![s])
            .unwrap_or_default(),
        recipe_ingredient: root
            .select(&ingredients_container_sel)
            .next()
            .ok_or_else(|| Error::MissingElement("ingredients".into()))?
            .select(&ingredient_groups_sel)
            .map(|group: ElementRef<'_>| {
                (
                    group
                        .select(&Selector::parse("p").unwrap())
                        .next()
                        .map(|s| normalize_text(&s)),
                    group
                        .select(&ingredient_sel)
                        .map(|ingredient| normalize_text(&ingredient))
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(name, ings)| {
                RecipeRecipeIngredientFieldEnum::new_section(
                    &name.unwrap_or_default(),
                    ings.iter()
                        .map(String::as_str)
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
            })
            .collect::<Vec<_>>(),
        recipe_instructions: root
            .select(&instructions_sel)
            .next()
            .ok_or_else(|| Error::MissingElement("instructions".into()))?
            .select(&Selector::parse("li")?)
            .map(|li| RecipeRecipeInstructionsFieldEnum::Text(normalize_text(&li)))
            .collect::<Vec<_>>(),
        url: vec![url.into()],
        ..Default::default()
    })
}

fn get_metadata_box(root: &ElementRef, css_selector: &str) -> Option<String> {
    match Selector::parse(css_selector) {
        Ok(sel) => root.select(&sel).next().and_then(|img| {
            let parent = img.parent()?.next_sibling()?.last_child()?;
            let parent_el = scraper::ElementRef::wrap(parent)?;

            let text: String = parent_el.text().collect();
            let cleaned = text
                .split_whitespace()
                .filter(|s| *s != "Preparation" && *s != "Time")
                .collect::<Vec<_>>()
                .join(" ");

            if cleaned.is_empty() {
                None
            } else {
                Some(cleaned)
            }
        }),
        Err(_) => None,
    }
}
