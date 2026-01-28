use schema_org::{
    AggregateRating, Recipe, at_context,
    field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum},
};
use scraper::{ElementRef, Html, Selector};
use support::strings::extract_number;

use crate::{
    Error, Result,
    custom::common::{
        at_type_recipe, extract_author, extract_category, extract_cuisines, extract_description,
        extract_duration, extract_image_urls, extract_keywords, extract_metadata_property,
        extract_yield, normalize_text, required_text, required_text_list,
    },
};

pub fn parse_zaatar_and_zaytoun_dot_com(doc: &Html, url: &str) -> Result<Recipe> {
    let content = doc
        .select(&Selector::parse("div.wprm-recipe-simple").unwrap())
        .next()
        .ok_or(Error::DomainNotImplemented)?;

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        recipe_yield: extract_yield(&content, ".wprm-recipe-servings")?,
        recipe_cuisine: extract_cuisines(&content, ".wprm-recipe-cuisine", Some("Lebanese"))?,
        recipe_ingredient: extract_ingredients(&content)?,
        cook_time: extract_duration(
            &content,
            ".wprm-recipe-cook-time-container span:nth-child(n+3)",
        )?,
        recipe_instructions: required_text_list(&content, ".wprm-recipe-instructions li")?
            .into_iter()
            .map(|ins| RecipeRecipeInstructionsFieldEnum::Text(ins.into()))
            .collect(),
        recipe_category: extract_category(&content, ".wprm-recipe-course")?,
        date_modified: extract_metadata_property(doc, "og:updated_time")?,
        keywords: extract_keywords(&content, ".wprm-recipe-keyword")?,
        aggregate_rating: extract_rating(&content)?,
        date_published: extract_metadata_property(doc, "article:published_time")?,
        author: extract_author(&content, ".wprm-recipe-author")?,
        image: extract_image_urls(&doc.root_element(), ".wp-block-image img")?,
        description: extract_description(&content, ".wprm-recipe-summary p")?,
        url: vec![url.into()],
        name: vec![required_text(&content, ".wprm-recipe-name")?],
        ..Default::default()
    })
}

fn extract_ingredients(content: &ElementRef) -> Result<Vec<RecipeRecipeIngredientFieldEnum>> {
    let h4_sel = Selector::parse("h4")?;
    let ingredients_container_sel = Selector::parse(".wprm-recipe-ingredients-container")?;
    let ingredient_group_sel = Selector::parse(".wprm-recipe-ingredient-group")?;
    let ingredients_li_sel = Selector::parse(".wprm-recipe-ingredients li")?;

    Ok(content
        .select(&ingredients_container_sel)
        .next()
        .map(|container| {
            let groups = container.select(&ingredient_group_sel).collect::<Vec<_>>();

            if groups.is_empty() {
                container
                    .select(&ingredients_li_sel)
                    .map(|li| normalize_text(&li))
                    .map(RecipeRecipeIngredientFieldEnum::Text)
                    .collect::<Vec<_>>()
            } else {
                groups
                    .iter()
                    .flat_map(|group| parse_ingredient_group(group, &h4_sel, &ingredients_li_sel))
                    .collect()
            }
        })
        .unwrap_or_default())
}

fn parse_ingredient_group(
    group: &ElementRef,
    h4_sel: &Selector,
    li_sel: &Selector,
) -> Vec<RecipeRecipeIngredientFieldEnum> {
    let group_name = group
        .select(h4_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let ingredients = group
        .select(li_sel)
        .map(|s| normalize_text(&s))
        .collect::<Vec<_>>();

    match group_name {
        Some(name) => {
            vec![RecipeRecipeIngredientFieldEnum::new_section(
                &name,
                ingredients.iter().map(|s| s.as_str()).collect(),
            )]
        }
        None => ingredients
            .into_iter()
            .map(RecipeRecipeIngredientFieldEnum::Text)
            .collect(),
    }
}

fn extract_rating(content: &ElementRef) -> Result<Vec<AggregateRating>> {
    let num_stars = content
        .select(&Selector::parse(".wprm-rating-star-full")?)
        .collect::<Vec<_>>();

    let rating = num_stars.len() as f32;
    let count = extract_number(
        &content
            .select(&Selector::parse(".wprm-recipe-rating-count")?)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default(),
    )
    .unwrap_or(0);

    Ok(vec![AggregateRating::new(rating, count)])
}
