use scraper::{Html, Selector};

use schema_org::{
    Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};

use crate::{
    Result,
    custom::common::{at_type_recipe, required_text},
};

#[allow(clippy::too_many_lines)]
pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
    let root = &doc.root_element();

    let section_heading_sel = Selector::parse("h4, h3, h2")?;
    let li_sel = Selector::parse("li")?;

    Ok(Recipe {
        r#type: at_type_recipe(),
        context: at_context(),
        description: doc
            .select(&Selector::parse("strong")?)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .find(|s| s.len() > 80)
            .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
            .unwrap_or_default(),
        image: doc
            .select(&Selector::parse(
                "article img, main img, .content-inner img, img",
            )?)
            .find(|el| {
                el.value().attr("src").is_some_and(|s| {
                    s.contains("squarespace-cdn")
                        && !s.contains("Wordmark")
                        && !s.contains("Stamp")
                        && !s.contains("Divider")
                })
            })
            .and_then(|el| el.value().attr("src"))
            .map(|src| vec![RecipeImageFieldEnum::URL(src.to_string())])
            .unwrap_or_default(),
        name: required_text(root, "title")
            .ok()
            .map(|s| {
                vec![
                    s.split('|')
                        .next()
                        .map(str::trim)
                        .map(ToString::to_string)
                        .unwrap_or_default(),
                ]
            })
            .unwrap_or_default(),
        recipe_ingredient: doc
            .select(&section_heading_sel)
            .find(|el| {
                el.text()
                    .collect::<String>()
                    .trim()
                    .eq_ignore_ascii_case("ingredients")
            })
            .map(|heading| {
                let strong_sel = Selector::parse("strong").unwrap();
                let mut sections: Vec<(String, Vec<String>)> = vec![(String::new(), Vec::new())];

                for sib in heading.next_siblings() {
                    let Some(el) = scraper::ElementRef::wrap(sib) else {
                        continue;
                    };

                    match el.value().name() {
                        "h2" | "h3" | "h4" => break,
                        "ul" => {
                            for li in el.select(&li_sel) {
                                let text = li.text().collect::<String>().trim().to_string();
                                if text.is_empty() {
                                    continue;
                                }

                                let strong_text = li
                                    .select(&strong_sel)
                                    .map(|s| s.text().collect::<String>())
                                    .collect::<String>();

                                let is_section_header =
                                    !strong_text.is_empty() && strong_text.trim() == text.as_str();

                                if is_section_header {
                                    let section_name =
                                        text.trim_end_matches(':').trim().to_string();
                                    sections.push((section_name, Vec::new()));
                                } else {
                                    sections.last_mut().unwrap().1.push(text);
                                }
                            }
                        }
                        "p" | "div" => {
                            let text = el.text().collect::<String>().trim().to_string();
                            let strong_text = el
                                .select(&strong_sel)
                                .map(|s| s.text().collect::<String>())
                                .collect::<String>();

                            if !strong_text.is_empty() && strong_text.trim() == text.as_str() {
                                let section_name = text.trim_end_matches(':').trim().to_string();
                                sections.push((section_name, Vec::new()));
                            }
                        }
                        _ => {}
                    }
                }

                sections
            })
            .map(|sections| {
                sections
                    .into_iter()
                    .filter(|(_, items)| !items.is_empty())
                    .map(|(name, items)| {
                        let items: Vec<&str> = items.iter().map(String::as_str).collect();
                        RecipeRecipeIngredientFieldEnum::new_section(&name, &items)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        recipe_instructions: doc
            .select(&section_heading_sel)
            .find(|el| {
                el.text()
                    .collect::<String>()
                    .trim()
                    .eq_ignore_ascii_case("steps")
            })
            .and_then(|heading| {
                heading.next_siblings().find_map(|sib| {
                    scraper::ElementRef::wrap(sib).filter(|el| el.value().name() == "ol")
                })
            })
            .map(|ol| {
                ol.select(&li_sel)
                    .map(|li| li.text().collect::<String>().trim().to_string())
                    .filter(|s| !s.is_empty())
                    .map(RecipeRecipeInstructionsFieldEnum::Text)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        url: vec![url.to_string()],
        ..Default::default()
    })
}
