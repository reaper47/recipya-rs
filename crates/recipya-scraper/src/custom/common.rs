use reqwest::Url;
use schema_org::{
    AtType, DurationOrText,
    field::{
        RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeKeywordsFieldEnum, RecipeRecipeYieldFieldEnum,
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
        Some(author) => Ok(vec![RecipeAuthorFieldEnum::new_person(&author)]),
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
        Some(text) => Ok(vec![RecipeDescriptionFieldEnum::Text(text)]),
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
        Ok(vec![value.into()])
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
        .ok_or(Error::DomainNotImplemented)?
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
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
