use std::{
    borrow::Cow,
    io::{Read, Seek},
};

use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use tracing::error;
use winnow::{
    Parser, Result as WResult,
    ascii::{line_ending, multispace0, multispace1, till_line_ending},
    combinator::{delimited, opt, preceded, seq},
    token::{literal, rest, take_until},
};

use schema_org::{
    AtType, Comment, DurationOrText, Recipe, at_context,
    field::{RecipeImageFieldEnum, RecipeYieldFieldEnum},
};

use crate::{
    Error, Result,
    apps::helpers::{
        Ingredient, Instruction, Parsers, ToSections, parse_archive_helper, read_file,
    },
};

#[derive(Default)]
struct RecipeComponents<'a> {
    category: Option<Cow<'a, str>>,
    image: Option<Cow<'a, str>>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    title: Cow<'a, str>,
    prep_time: Option<Cow<'a, str>>,
    total_time: Option<Cow<'a, str>>,
    r#yield: Option<Cow<'a, str>>,
    notes: Vec<Cow<'a, str>>,
}

impl From<RecipeComponents<'_>> for Recipe {
    #[allow(clippy::too_many_lines)]
    fn from(r: RecipeComponents) -> Self {
        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            comment_count: if r.notes.is_empty() {
                vec![]
            } else {
                vec![i32::try_from(r.notes.len()).unwrap_or_default()]
            },
            comment: r
                .notes
                .into_iter()
                .map(|s| Comment {
                    text: vec![s.into()],
                    ..Default::default()
                })
                .collect(),
            image: r.image.map_or(Vec::new(), |s| {
                vec![RecipeImageFieldEnum::URL(s.to_string())]
            }),
            name: vec![r.title.into()],
            prep_time: r
                .prep_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.into())]),
            total_time: r
                .total_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.into())]),
            recipe_category: r.category.map_or(Vec::new(), |s| vec![s.into()]),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            recipe_yield: vec![RecipeYieldFieldEnum::Text(
                r.r#yield.unwrap_or_default().to_string(),
            )],
            ..Default::default()
        }
    }
}

/// Parses a `Umami` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper(
        r,
        &Parsers {
            html: Some(parse_html),
            json: Some(parse_json),
            md: Some(parse_md),
            txt: Some(parse_txt),
            ..Default::default()
        },
    )
}

/// Parses a `Umami` HTML file.
///
/// # Panics
///
/// Panics when a CSS selector is invalid or when an element is inexistant.
pub fn parse_html<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let doc = Html::parse_document(&content);

    let sel_strong = Selector::parse("strong").unwrap();
    let sel_em = Selector::parse("em").unwrap();
    let sel_span = Selector::parse("span").unwrap();

    let paragraphs = doc.select(&Selector::parse("p").unwrap()).collect_vec();
    let h2s = doc.select(&Selector::parse("h2").unwrap()).collect_vec();

    Ok(vec![
        RecipeComponents {
            category: extract_p_text("Recipe Book:", paragraphs.as_slice(), &sel_em),
            image: doc
                .select(&Selector::parse("img").unwrap())
                .next()
                .map(|el| Cow::Borrowed(el.attr("src").unwrap_or_default())),
            ingredients: match h2s.iter().find(|el| {
                el.text()
                    .next()
                    .is_some_and(|s| s.trim_start().starts_with("Ingredient"))
            }) {
                Some(el) => el
                    .next_sibling()
                    .map(|node| {
                        let el = ElementRef::wrap(node).unwrap();
                        el.select(&sel_span)
                            .map(|span| {
                                let text = span.text().collect::<String>();
                                Ingredient::Line(Cow::Owned(text))
                            })
                            .collect_vec()
                    })
                    .unwrap_or_default(),
                None => {
                    return Err(Error::Parse("Ingredients not found".to_string()));
                }
            },
            instructions: match h2s.iter().find(|el| {
                el.text()
                    .next()
                    .is_some_and(|s| s.trim_start().starts_with("Directions"))
            }) {
                Some(el) => el
                    .next_sibling()
                    .map(|node| {
                        let el = ElementRef::wrap(node).unwrap();
                        el.select(&sel_span)
                            .map(|span| {
                                let text = span.text().collect::<String>();
                                Instruction::Line(Cow::Owned(text))
                            })
                            .collect_vec()
                    })
                    .unwrap_or_default(),
                None => {
                    return Err(Error::Parse("Ingredients not found".to_string()));
                }
            },
            title: doc
                .select(&Selector::parse("h1").unwrap())
                .next()
                .map_or(Cow::Borrowed(""), |n| n.text().collect()),
            prep_time: extract_p_text("Active Time:", paragraphs.as_slice(), &sel_strong),
            total_time: extract_p_text("Total Time:", paragraphs.as_slice(), &sel_strong),
            r#yield: extract_p_text("Servings:", paragraphs.as_slice(), &sel_strong),
            notes: h2s
                .iter()
                .find_map(|el| {
                    let starts = el.text().next()?.trim_start().starts_with("Notes");
                    if !starts {
                        return None;
                    }

                    Some(
                        el.next_siblings()
                            .take_while(|sibling| {
                                ElementRef::wrap(*sibling)
                                    .is_none_or(|el| el.value().name() != "h2")
                            })
                            .filter_map(ElementRef::wrap)
                            .filter(|el| el.value().name() == "p")
                            .map(|p| Cow::Owned(p.text().collect::<String>()))
                            .filter(|s: &Cow<str>| !s.trim().is_empty())
                            .collect_vec(),
                    )
                })
                .unwrap_or_default(),
        }
        .into(),
    ])
}

fn extract_p_text<'a>(
    prefix: &'a str,
    paragraphs: &[ElementRef<'a>],
    sel_strong: &Selector,
) -> Option<Cow<'a, str>> {
    paragraphs.iter().find_map(|el| {
        let starts = el.text().next()?.trim_start().starts_with(prefix);
        if !starts {
            return None;
        }

        el.select(sel_strong)
            .next()
            .and_then(|strong| strong.text().next())
            .map(|t| Cow::Borrowed(t.trim()))
    })
}

/// Parses a `Umami` JSON file.
pub fn parse_json<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let recipe: Recipe = serde_json::from_reader(r).map_err(|err| {
        error!("Failed to read Umami JSON file: {err}");
        Error::Parse(err.to_string())
    })?;
    Ok(vec![recipe])
}

/// Parses a `Umami` markdown file.
pub fn parse_md<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    Ok(vec![
        parse_md_helper(&mut content.as_str())
            .map_err(|err| Error::Parse(err.to_string()))?
            .into(),
    ])
}

fn parse_md_helper<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_md_title,
        category: opt(parse_md_category),
        r#yield: opt(parse_md_yield),
        prep_time: opt(parse_md_prep),
        total_time: opt(parse_md_total),
        image: opt(parse_md_image),
        ingredients: parse_md_ingredients,
        instructions: parse_md_instructions,
        notes: parse_md_notes,
    }}
    .parse_next(input)
}

fn parse_md_title<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "# ")
}

fn parse_md_category<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Recipe Book: ")
}

fn parse_md_yield<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Servings: ")
}

fn parse_md_prep<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Active Time: ")
}

fn parse_md_total<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Total Time: ")
}

fn parse_md_image<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    delimited(
        literal("![photo]("),
        take_until(1.., ")\n"),
        (literal(")"), multispace1),
    )
    .map(str::trim)
    .map(Cow::Borrowed)
    .parse_next(input)
}

fn parse_md_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    preceded(
        (literal("## Ingredients"), line_ending),
        take_until(1.., "## Directions"),
    )
    .map(|blocks: &str| {
        blocks
            .split("\n\n")
            .flat_map(|block| {
                block
                    .split('-')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| Ingredient::Line(Cow::Borrowed(s)))
                    .collect_vec()
            })
            .collect()
    })
    .parse_next(input)
}

fn parse_md_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    preceded(
        (literal("## Directions"), line_ending),
        take_until(1.., "\n\n"),
    )
    .map(|blocks: &str| {
        blocks
            .lines()
            .map(|s| Instruction::Line(Cow::Borrowed(s.trim())))
            .collect()
    })
    .parse_next(input)
}

fn parse_md_notes<'s>(input: &mut &'s str) -> WResult<Vec<Cow<'s, str>>> {
    preceded((multispace0, literal("## Notes")), rest)
        .map(|block: &str| {
            block
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(Cow::Borrowed)
                .collect()
        })
        .parse_next(input)
}

/// Parses a `Umami` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    Ok(vec![
        parse_txt_helper(&mut content.as_str())
            .map_err(|err| Error::Parse(err.to_string()))?
            .into(),
    ])
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_txt_title,
        category: opt(parse_txt_category),
        r#yield: opt(parse_txt_yield),
        prep_time: opt(parse_txt_prep),
        total_time: opt(parse_txt_total),
        ingredients: parse_txt_ingredients,
        instructions: parse_txt_instructions,
        notes: parse_txt_notes,
        ..Default::default()
    }}
    .parse_next(input)
}

fn parse_txt_title<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "")
}

fn parse_txt_category<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Recipe Book: ")
}

fn parse_txt_yield<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Servings: ")
}

fn parse_txt_prep<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Active Time: ")
}

fn parse_txt_total<'s>(input: &mut &'s str) -> WResult<Cow<'s, str>> {
    extract_line(input, "Total Time: ")
}

fn extract_line<'s>(input: &mut &'s str, prefix: &'s str) -> WResult<Cow<'s, str>> {
    delimited(literal(prefix), till_line_ending, multispace1)
        .map(|s: &str| s.trim().trim_matches('*'))
        .map(Cow::Borrowed)
        .parse_next(input)
}

fn parse_txt_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    preceded(
        (literal("Ingredients"), (line_ending, line_ending)),
        take_until(1.., "Directions"),
    )
    .map(|blocks: &str| {
        blocks
            .split("- ")
            .flat_map(|block| {
                block
                    .split('-')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| Ingredient::Line(Cow::Borrowed(s)))
                    .collect_vec()
            })
            .collect()
    })
    .parse_next(input)
}

fn parse_txt_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    preceded(
        (literal("Directions"), (line_ending, line_ending)),
        take_until(1.., "\n\n\n"),
    )
    .map(|blocks: &str| {
        blocks
            .split("\n\n")
            .map(|s| Instruction::Line(Cow::Borrowed(s.trim())))
            .collect()
    })
    .parse_next(input)
}

fn parse_txt_notes<'s>(input: &mut &'s str) -> WResult<Vec<Cow<'s, str>>> {
    preceded((multispace0, literal("Notes")), rest)
        .map(|block: &str| {
            block
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(Cow::Borrowed)
                .collect()
        })
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use std::io::Cursor;

        use super::*;

        #[test]
        fn test_umami_html_ok() -> Result<()> {
            let buf = Cursor::new(files::html());

            let got = parse_html(buf)?;

            assert_eq!(got.len(), 1);
            pretty_assertions::assert_eq!(got, vec![results::recipe1()]);
            Ok(())
        }

        #[test]
        fn test_umami_json_ok() -> Result<()> {
            let buf = Cursor::new(files::json());

            let got = parse_json(buf)?;

            assert_eq!(got.len(), 1);
            pretty_assertions::assert_eq!(got, vec![results::recipe1_json()]);
            Ok(())
        }

        #[test]
        fn test_umami_md_ok() -> Result<()> {
            let buf = Cursor::new(files::md());

            let got = parse_md(buf)?;

            assert_eq!(got.len(), 1);
            pretty_assertions::assert_eq!(got, vec![results::recipe1()]);
            Ok(())
        }

        #[test]
        fn test_umami_txt_ok() -> Result<()> {
            let buf = Cursor::new(files::txt());

            let got = parse_txt(buf)?;

            assert_eq!(got.len(), 1);
            let mut expected = results::recipe1();
            expected.image.clear();
            pretty_assertions::assert_eq!(got, vec![expected]);
            Ok(())
        }
    }

    mod files {
        pub fn html<'a>() -> &'a str {
            r#"<html><body><h1>Crunchy lettuce wraps with grilled Bulgogi mushrooms</h1><p>Recipe Book: <em>Best Recipes</em></p><p>Servings: <strong>4 servings</strong></p><p>Active Time: <strong>20 minutes</strong></p><p>Total Time: <strong>40 minutes</strong></p><img alt="Recipe photo" style="max-height:400px" src="https://www.umami.recipes/api/image/recipes/Usk18Na0NKoBkOeOvfCF/images/xGLWh4MqGj0qWdXBPzdPWs?w=1080&amp;q=75"/><h2>Ingredients</h2><ul><li><p><span>14 oz oyster mushrooms</span></p></li><li><p><span>2 romaine hearts</span></p></li><li><p><span>4 carrots</span></p></li><li><p><span>1 cucumber</span></p></li><li><p><span>4 scallions</span></p></li><li><p><span>1 clove garlic</span></p></li><li><p><span>1 oz ginger</span></p></li><li><p><span>2 tbsp soy sauce</span></p></li><li><p><span>4 tbsp apple juice</span></p></li><li><p><span>1 tbsp honey</span></p></li><li><p><span>1 tbsp sesame oil</span></p></li><li><p><span>1 tbsp gochujang</span></p></li><li><p><span>1 ⅘ oz kimchi</span></p></li><li><p><span>0.7 oz sesame seeds</span></p></li></ul><h2>Directions</h2><ol><li><p><span>Separate individual leaves from lettuce hearts. Cut carrots and cucumbers into thin strips. Cut spring onions into thin rings.</span></p></li><li><p><span>Finely grate garlic and ginger into a small bowl. Next add soy sauce, apple juice, honey, sesame oil, Gochujang to the bowl and mix well. Then, either on the hot grill, or on a grill pan, sear oyster mushrooms (preferably with a grill press). After approx. 2 min. brush them with the prepared Bulgogi sauce, flip, and grill for approx. 1 min. more. Brush the other side, and flip again to grill for approx. another 1 min.</span></p></li><li><p><span>Fill lettuce leaves with grilled oyster mushrooms, carrot, cucumber, and kimchi. Garnish with sesame seeds and spring onions to serve.</span></p></li></ol><h2>Notes</h2><p><span>Some recipe notes</span></p><p><span>And more here</span></p></body></html>"#
        }

        pub fn json<'a>() -> &'a str {
            r#"{
              "@context": "https://schema.org",
              "@type": "Recipe",
              "name": "Crunchy lettuce wraps with grilled Bulgogi mushrooms",
              "url": "https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF",
              "image": [
                "https://www.umami.recipes/api/image/recipes/Usk18Na0NKoBkOeOvfCF/images/xGLWh4MqGj0qWdXBPzdPWs?w=2048&q=75"
              ],
              "author": {
                "@type": "Person",
                "name": "Marc-Andre Charland"
              },
              "datePublished": "2026-06-23T20:56:33.761Z",
              "prepTime": "P0Y0M0DT0H20M0S",
              "cookTime": "P0Y0M0DT0H0M0S",
              "totalTime": "P0Y0M0DT0H20M0S",
              "keywords": "Crunchy lettuce wraps with grilled Bulgogi mushrooms, Best Recipes",
              "recipeYield": "4 servings",
              "recipeCategory": "Best Recipes",
              "nutrition": {
                "@context": "https://schema.org",
                "@type": "NutritionInformation",
                "calories": "208 cal",
                "carbohydrateContent": "26 g",
                "fatContent": "6 g",
                "proteinContent": "9 g"
              },
              "recipeIngredient": [
                "14 oz oyster mushrooms",
                "2 romaine hearts",
                "4 carrots",
                "1 cucumber",
                "4 scallions",
                "1 clove garlic",
                "1 oz ginger",
                "2 tbsp soy sauce",
                "4 tbsp apple juice",
                "1 tbsp honey",
                "1 tbsp sesame oil",
                "1 tbsp gochujang",
                "1 ⅘ oz kimchi",
                "0.7 oz sesame seeds"
              ],
              "recipeInstructions": [
                {
                  "@type": "HowToStep",
                  "text": "Separate individual leaves from lettuce hearts. Cut carrots and cucumbers into thin strips. Cut spring onions into thin rings.",
                  "url": "https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF?start=true&step=1"
                },
                {
                  "@type": "HowToStep",
                  "text": "Finely grate garlic and ginger into a small bowl. Next add soy sauce, apple juice, honey, sesame oil, Gochujang to the bowl and mix well. Then, either on the hot grill, or on a grill pan, sear oyster mushrooms (preferably with a grill press). After approx. 2 min. brush them with the prepared Bulgogi sauce, flip, and grill for approx. 1 min. more. Brush the other side, and flip again to grill for approx. another 1 min.",
                  "url": "https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF?start=true&step=2"
                },
                {
                  "@type": "HowToStep",
                  "text": "Fill lettuce leaves with grilled oyster mushrooms, carrot, cucumber, and kimchi. Garnish with sesame seeds and spring onions to serve.",
                  "url": "https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF?start=true&step=3"
                }
              ]
            }"#
        }

        pub fn md<'a>() -> &'a str {
            r"# Crunchy lettuce wraps with grilled Bulgogi mushrooms

            Recipe Book: *Best Recipes*
            Servings: **4 servings**
            Active Time: **20 minutes**
            Total Time: **40 minutes**

            ![photo](https://www.umami.recipes/api/image/recipes/Usk18Na0NKoBkOeOvfCF/images/xGLWh4MqGj0qWdXBPzdPWs?w=1080&q=75)

            ## Ingredients
            - 14 oz oyster mushrooms
            - 2 romaine hearts
            - 4 carrots
            - 1 cucumber
            - 4 scallions
            - 1 clove garlic
            - 1 oz ginger
            - 2 tbsp soy sauce
            - 4 tbsp apple juice
            - 1 tbsp honey
            - 1 tbsp sesame oil
            - 1 tbsp gochujang
            - 1 ⅘ oz kimchi
            - 0.7 oz sesame seeds

            ## Directions
            1. Separate individual leaves from lettuce hearts. Cut carrots and cucumbers into thin strips. Cut spring onions into thin rings.
            2. Finely grate garlic and ginger into a small bowl. Next add soy sauce, apple juice, honey, sesame oil, Gochujang to the bowl and mix well. Then, either on the hot grill, or on a grill pan, sear oyster mushrooms (preferably with a grill press). After approx. 2 min. brush them with the prepared Bulgogi sauce, flip, and grill for approx. 1 min. more. Brush the other side, and flip again to grill for approx. another 1 min.
            3. Fill lettuce leaves with grilled oyster mushrooms, carrot, cucumber, and kimchi. Garnish with sesame seeds and spring onions to serve.

            ## Notes
            Some recipe notes

            And more here"
        }

        pub fn txt<'a>() -> &'a str {
            r"Crunchy lettuce wraps with grilled Bulgogi mushrooms

            Recipe Book: Best Recipes
            Servings: 4 servings
            Active Time: 20 minutes
            Total Time: 40 minutes

            Ingredients

            - 14 oz oyster mushrooms
            - 2 romaine hearts
            - 4 carrots
            - 1 cucumber
            - 4 scallions
            - 1 clove garlic
            - 1 oz ginger
            - 2 tbsp soy sauce
            - 4 tbsp apple juice
            - 1 tbsp honey
            - 1 tbsp sesame oil
            - 1 tbsp gochujang
            - 1 ⅘ oz kimchi
            - 0.7 oz sesame seeds

            Directions

            1. Separate individual leaves from lettuce hearts. Cut carrots and cucumbers into thin strips. Cut spring onions into thin rings.

            2. Finely grate garlic and ginger into a small bowl. Next add soy sauce, apple juice, honey, sesame oil, Gochujang to the bowl and mix well. Then, either on the hot grill, or on a grill pan, sear oyster mushrooms (preferably with a grill press). After approx. 2 min. brush them with the prepared Bulgogi sauce, flip, and grill for approx. 1 min. more. Brush the other side, and flip again to grill for approx. another 1 min.

            3. Fill lettuce leaves with grilled oyster mushrooms, carrot, cucumber, and kimchi. Garnish with sesame seeds and spring onions to serve.


            Notes

            Some recipe notes

            And more here"
        }
    }

    mod results {
        use schema_org::{
            Energy, HowToStep, Mass, NutritionInformation,
            field::{
                RecipeAuthorFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
                RecipeRecipeInstructionsFieldEnum,
            },
        };

        use super::*;

        pub fn recipe1() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                comment_count: vec![2],
                comment: vec![
                    Comment {
                        text: vec!["Some recipe notes".into()],
                        ..Default::default()
                    },
                    Comment {
                        text: vec!["And more here".into()],
                        ..Default::default()
                    }
                ],
                name: vec!["Crunchy lettuce wraps with grilled Bulgogi mushrooms".into()],
                image: vec![RecipeImageFieldEnum::URL("https://www.umami.recipes/api/image/recipes/Usk18Na0NKoBkOeOvfCF/images/xGLWh4MqGj0qWdXBPzdPWs?w=1080&q=75".into())],
                prep_time: vec![DurationOrText::Text("20 minutes".into())],
                total_time: vec![DurationOrText::Text("40 minutes".into())],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("4 servings".into())],
                recipe_category: vec!["Best Recipes".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("14 oz oyster mushrooms".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 romaine hearts".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 carrots".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cucumber".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 scallions".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 clove garlic".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 oz ginger".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tbsp soy sauce".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 tbsp apple juice".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tbsp honey".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tbsp sesame oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tbsp gochujang".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ⅘ oz kimchi".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.7 oz sesame seeds".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Separate individual leaves from lettuce hearts. Cut carrots and cucumbers into thin strips. Cut spring onions into thin rings.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Finely grate garlic and ginger into a small bowl. Next add soy sauce, apple juice, honey, sesame oil, Gochujang to the bowl and mix well. Then, either on the hot grill, or on a grill pan, sear oyster mushrooms (preferably with a grill press). After approx. 2 min. brush them with the prepared Bulgogi sauce, flip, and grill for approx. 1 min. more. Brush the other side, and flip again to grill for approx. another 1 min.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Fill lettuce leaves with grilled oyster mushrooms, carrot, cucumber, and kimchi. Garnish with sesame seeds and spring onions to serve.".into()),
                ],
                ..Default::default()
            }
        }

        pub fn recipe1_json() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Crunchy lettuce wraps with grilled Bulgogi mushrooms".into()],
                url: vec!["https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF".into()],
                image: vec![RecipeImageFieldEnum::URL("https://www.umami.recipes/api/image/recipes/Usk18Na0NKoBkOeOvfCF/images/xGLWh4MqGj0qWdXBPzdPWs?w=2048&q=75".into())],
                author: vec![RecipeAuthorFieldEnum::new_org("Marc-Andre Charland")],
                date_published: vec!["2026-06-23T20:56:33.761Z".into()],
                prep_time: vec![DurationOrText::Text("P0Y0M0DT0H20M0S".into())],
                cook_time: vec![DurationOrText::Text("P0Y0M0DT0H0M0S".into())],
                total_time: vec![DurationOrText::Text("P0Y0M0DT0H20M0S".into())],
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("Crunchy lettuce wraps with grilled Bulgogi mushrooms, Best Recipes".into()),
                ],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("4 servings".into())],
                recipe_category: vec!["Best Recipes".into()],
                nutrition: vec![NutritionInformation {
                    r#type: AtType::NutritionInformation.to_opt(),
                    context: at_context(),
                    calories: vec![Energy::new("208 cal")],
                    carbohydrate_content: vec![Mass::new("26 g")],
                    fat_content: vec![Mass::new("6 g")],
                    protein_content: vec![Mass::new("9 g")],
                    ..Default::default()
                }],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("14 oz oyster mushrooms".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 romaine hearts".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 carrots".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cucumber".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 scallions".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 clove garlic".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 oz ginger".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tbsp soy sauce".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 tbsp apple juice".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tbsp honey".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tbsp sesame oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tbsp gochujang".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ⅘ oz kimchi".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.7 oz sesame seeds".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Separate individual leaves from lettuce hearts. Cut carrots and cucumbers into thin strips. Cut spring onions into thin rings.".into()],
                        url: vec!["https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF?start=true&step=1".into()],
                        ..Default::default()
                    }.into()),
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Finely grate garlic and ginger into a small bowl. Next add soy sauce, apple juice, honey, sesame oil, Gochujang to the bowl and mix well. Then, either on the hot grill, or on a grill pan, sear oyster mushrooms (preferably with a grill press). After approx. 2 min. brush them with the prepared Bulgogi sauce, flip, and grill for approx. 1 min. more. Brush the other side, and flip again to grill for approx. another 1 min.".into()],
                        url: vec!["https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF?start=true&step=2".into()],
                        ..Default::default()
                    }.into()),
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Fill lettuce leaves with grilled oyster mushrooms, carrot, cucumber, and kimchi. Garnish with sesame seeds and spring onions to serve.".into()],
                        url: vec!["https://www.umami.recipes/recipe/Usk18Na0NKoBkOeOvfCF?start=true&step=3".into()],
                        ..Default::default()
                    }.into()),
                ],
                ..Default::default()
            }
        }
    }
}
