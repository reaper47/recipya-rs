use reqwest::Url;
use schema_org::{
    AtType, DurationOrText,
    field::{
        RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeYieldFieldEnum,
    },
};
use scraper::{ElementRef, Html, Selector};
use support::strings::extract_number;

use crate::{Error, Result};

pub(crate) fn at_type_recipe() -> Option<String> {
    AtType::Recipe.to_opt()
}

pub(crate) fn extract_author(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeAuthorFieldEnum>> {
    match optional_text(fragment, css_selector)? {
        Some(author) => Ok(vec![RecipeAuthorFieldEnum::new_person(author.trim())]),
        None => Ok(vec![]),
    }
}

pub(crate) fn extract_category(fragment: &ElementRef, css_selector: &str) -> Result<Vec<String>> {
    match text_list(fragment, css_selector, ",")?.get(0) {
        Some(category) => Ok(vec![category.clone()]),
        None => Ok(vec![]),
    }
}

pub(crate) fn extract_cuisines(
    fragment: &ElementRef,
    css_selector: &str,
    default_cuisine: Option<&str>,
) -> Result<Vec<String>> {
    let cuisines = text_list(fragment, css_selector, ",")?;
    if cuisines.is_empty() {
        match default_cuisine {
            Some(cuisine) => Ok(vec![cuisine.to_string()]),
            None => Err(Error::DomainNotImplemented),
        }
    } else {
        Ok(cuisines)
    }
}

pub(crate) fn extract_description(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeDescriptionFieldEnum>> {
    match optional_text(fragment, css_selector)? {
        Some(s) => Ok(vec![RecipeDescriptionFieldEnum::Text(s.trim().into())]),
        None => Ok(vec![]),
    }
}

pub(crate) fn extract_duration(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<DurationOrText>> {
    Ok(optional_text(fragment, css_selector)?
        .map(|s| vec![DurationOrText::Text(s)])
        .unwrap_or_default())
}

pub(crate) fn extract_image_urls(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeImageFieldEnum>> {
    Ok(fragment
        .select(&Selector::parse(css_selector)?)
        .filter_map(|el| {
            let src = el.attr("src")?;
            let url = Url::parse(src).ok()?;
            matches!(url.scheme(), "http" | "https").then(|| RecipeImageFieldEnum::URL(url.into()))
        })
        .collect())
}

pub(crate) fn extract_ingredients(
    content: &ElementRef,
) -> Result<Vec<RecipeRecipeIngredientFieldEnum>> {
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

pub(crate) fn extract_keywords(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeKeywordsFieldEnum>> {
    let keywords = text_list(fragment, css_selector, ",")?;
    if keywords.is_empty() {
        Ok(vec![RecipeKeywordsFieldEnum::TextOrURL("Lebanese".into())])
    } else {
        Ok(keywords
            .into_iter()
            .map(RecipeKeywordsFieldEnum::TextOrURL)
            .collect())
    }
}

pub(crate) fn extract_attr<'a>(
    fragment: &'a ElementRef,
    css_selector: &'a str,
    attr: &'a str,
) -> Result<Option<&'a str>> {
    Ok(fragment
        .select(&Selector::parse(css_selector)?)
        .next()
        .ok_or(Error::MissingElement(css_selector.into()))?
        .value()
        .attr(attr))
}

pub(crate) fn extract_metadata_property(fragment: &Html, property: &str) -> Result<Vec<String>> {
    let value = fragment
        .select(&Selector::parse(&format!("meta[property='{property}']"))?)
        .next()
        .ok_or(Error::Parse("Meta data property not found".into()))?
        .value()
        .attr("content")
        .unwrap_or_default();

    if value.is_empty() {
        Ok(vec![])
    } else {
        Ok(vec![value.trim().into()])
    }
}

pub(crate) fn extract_yield(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeRecipeYieldFieldEnum>> {
    match optional_text(fragment, css_selector)? {
        Some(text) => match extract_number(&text).ok() {
            Some(n) => Ok(vec![RecipeRecipeYieldFieldEnum::Text(n)]),
            None => Ok(vec![]),
        },
        None => Ok(vec![]),
    }
}

pub(crate) fn required_text(fragment: &ElementRef, css_selector: &str) -> Result<String> {
    Ok(fragment
        .select(&Selector::parse(css_selector).unwrap())
        .next()
        .ok_or(Error::DomainNotImplemented)?
        .text()
        .next()
        .ok_or(Error::MissingElement(css_selector.into()))?
        .to_string())
}

pub(crate) fn required_text_list<'a>(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<String>> {
    let text = fragment
        .select(&Selector::parse(css_selector)?)
        .map(|s| normalize_text(&s))
        .collect::<Vec<_>>();

    if text.is_empty() {
        Err(Error::DomainNotImplemented)
    } else {
        Ok(text)
    }
}

pub(crate) fn optional_text(fragment: &ElementRef, css_selector: &str) -> Result<Option<String>> {
    let text = fragment
        .select(&Selector::parse(css_selector)?)
        .map(|s| s.text().next().unwrap_or_default())
        .collect::<Vec<_>>();

    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text.join(" ")))
    }
}

pub(crate) fn text_list(
    fragment: &ElementRef,
    css_selector: &str,
    delimiter: &str,
) -> Result<Vec<String>> {
    match optional_text(fragment, css_selector)? {
        Some(text) => Ok(text
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .collect()),
        None => Ok(Vec::new()),
    }
}

pub(crate) fn normalize_text(li: &ElementRef) -> String {
    li.text()
        .flat_map(str::split_whitespace)
        .filter(|s| *s != "▢")
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn extract_text_from_elements(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<String>> {
    Ok(fragment
        .select(&Selector::parse(css_selector)?)
        .filter_map(|el| {
            let text = el.text().collect::<String>().trim().to_string();
            if text.is_empty() {
                None
            } else {
                Some(text.trim_end_matches(',').trim().to_string())
            }
        })
        .collect::<Vec<_>>())
}
