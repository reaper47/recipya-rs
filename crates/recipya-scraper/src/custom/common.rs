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

pub fn at_type_recipe() -> Option<String> {
    AtType::Recipe.to_opt()
}

pub fn extract_author(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeAuthorFieldEnum>> {
    (optional_text(fragment, css_selector)?).map_or_else(
        || Ok(vec![]),
        |author| Ok(vec![RecipeAuthorFieldEnum::new_person(author.trim())]),
    )
}

pub fn extract_category(fragment: &ElementRef, css_selector: &str) -> Result<Vec<String>> {
    text_list(fragment, css_selector, ",")?
        .first()
        .map_or_else(|| Ok(vec![]), |category| Ok(vec![category.clone()]))
}

pub fn extract_cuisines(
    fragment: &ElementRef,
    css_selector: &str,
    default_cuisine: Option<&str>,
) -> Result<Vec<String>> {
    let cuisines = text_list(fragment, css_selector, ",")?;
    if cuisines.is_empty() {
        default_cuisine.map_or(Err(Error::DomainNotImplemented), |cuisine| {
            Ok(vec![cuisine.to_string()])
        })
    } else {
        Ok(cuisines)
    }
}

pub fn extract_description(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeDescriptionFieldEnum>> {
    (optional_text(fragment, css_selector)?).map_or_else(
        || Ok(vec![]),
        |s| Ok(vec![RecipeDescriptionFieldEnum::Text(s.trim().into())]),
    )
}

pub fn extract_duration(fragment: &ElementRef, css_selector: &str) -> Result<Vec<DurationOrText>> {
    Ok(optional_text(fragment, css_selector)?
        .map(|s| vec![DurationOrText::Text(s)])
        .unwrap_or_default())
}

pub fn extract_image_urls(
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

pub fn extract_ingredients(content: &ElementRef) -> Result<Vec<RecipeRecipeIngredientFieldEnum>> {
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
                ingredients
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .as_slice(),
            )]
        }
        None => ingredients
            .into_iter()
            .map(RecipeRecipeIngredientFieldEnum::Text)
            .collect(),
    }
}

pub fn extract_keywords(
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

pub fn extract_attr<'a>(
    fragment: &'a ElementRef,
    css_selector: &'a str,
    attr: &'a str,
) -> Result<Option<&'a str>> {
    Ok(fragment
        .select(&Selector::parse(css_selector)?)
        .next()
        .ok_or_else(|| Error::MissingElement(css_selector.into()))?
        .value()
        .attr(attr))
}

pub fn extract_metadata_property(fragment: &Html, property: &str) -> Result<Vec<String>> {
    let value = fragment
        .select(&Selector::parse(&format!("meta[property='{property}']"))?)
        .next()
        .ok_or_else(|| Error::Parse("Meta property not found".into()))?
        .value()
        .attr("content")
        .unwrap_or_default();

    if value.is_empty() {
        Ok(vec![])
    } else {
        Ok(vec![value.trim().into()])
    }
}

pub fn extract_yield(
    fragment: &ElementRef,
    css_selector: &str,
) -> Result<Vec<RecipeRecipeYieldFieldEnum>> {
    (optional_text(fragment, css_selector)?).map_or_else(
        || Ok(vec![]),
        |text| {
            extract_number(&text).ok().map_or_else(
                || Ok(vec![]),
                |n| Ok(vec![RecipeRecipeYieldFieldEnum::Text(n)]),
            )
        },
    )
}

pub fn required_text(fragment: &ElementRef, css_selector: &str) -> Result<String> {
    Ok(fragment
        .select(&Selector::parse(css_selector).unwrap())
        .next()
        .ok_or(Error::DomainNotImplemented)?
        .text()
        .next()
        .ok_or_else(|| Error::MissingElement(css_selector.into()))?
        .to_string())
}

pub fn required_text_list(fragment: &ElementRef, css_selector: &str) -> Result<Vec<String>> {
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

pub fn optional_text(fragment: &ElementRef, css_selector: &str) -> Result<Option<String>> {
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

pub fn text_list(
    fragment: &ElementRef,
    css_selector: &str,
    delimiter: &str,
) -> Result<Vec<String>> {
    (optional_text(fragment, css_selector)?).map_or_else(
        || Ok(Vec::new()),
        |text| {
            Ok(text
                .split(delimiter)
                .map(|s| s.trim().to_string())
                .collect())
        },
    )
}

pub fn normalize_text(li: &ElementRef) -> String {
    li.text()
        .flat_map(str::split_whitespace)
        .filter(|s| *s != "▢")
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn extract_text_from_elements(
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
