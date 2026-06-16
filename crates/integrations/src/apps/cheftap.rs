use std::io::{Cursor, Read, Seek};
use std::ops::Deref;

use scraper::{Html, Selector};
use winnow::Result as WResult;
use winnow::ascii::{digit1, line_ending, multispace0, space0, till_line_ending};
use winnow::combinator::{
    alt, delimited, eof, not, opt, peek, preceded, repeat, repeat_till, seq, terminated,
};
use winnow::prelude::*;
use winnow::token::{literal, take_until};
use zip::ZipArchive;

use schema_org::field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum};
use schema_org::{AtType, DurationOrText, Recipe};

use super::helpers::read_file;
use crate::helpers::{to_is_based_on, to_yield};
use crate::{Error, FileFormat, Result};

pub struct ChefTapRecipe {
    title: String,
    r#yield: Option<i16>,
    cook_time: Option<String>,
    prep_time: Option<String>,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    notes: Option<String>,
    source: Option<String>,
}

struct RecipeComponents<'a> {
    title: &'a str,
    cook_time: Option<&'a str>,
    prep_time: Option<&'a str>,
    servings: Option<i16>,
    ingredients: Vec<&'a str>,
    instructions: Vec<&'a str>,
    notes: Option<&'a str>,
    source: Option<&'a str>,
}

impl From<ChefTapRecipe> for Recipe {
    fn from(mut r: ChefTapRecipe) -> Self {
        if let Some(notes) = r.notes {
            let notes = if !notes.to_lowercase().starts_with("note") {
                format!("Notes: {notes}")
            } else {
                notes
            };

            r.instructions
                .push(RecipeRecipeInstructionsFieldEnum::Text(notes));
        }

        Self {
            r#type: AtType::Recipe.to_opt(),
            is_based_on: to_is_based_on(&r.source.clone().unwrap_or_default()),
            cook_time: r
                .cook_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            prep_time: r
                .prep_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            name: vec![r.title],
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: to_yield(i64::from(r.r#yield.unwrap_or_default())),
            url: r.source.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl From<RecipeComponents<'_>> for ChefTapRecipe {
    fn from(c: RecipeComponents) -> Self {
        Self {
            title: c.title.to_owned(),
            cook_time: c.cook_time.map(|s| s.trim().into()),
            prep_time: c.prep_time.map(|s| s.trim().into()),
            r#yield: c.servings,
            ingredients: c
                .ingredients
                .into_iter()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.trim().into()))
                .collect(),
            instructions: c
                .instructions
                .into_iter()
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.trim().into()))
                .collect(),
            notes: c.notes.map(|s| s.trim().into()),
            source: c.source.map(|s| s.trim().into()),
        }
    }
}

/// Parses a ZIP archive containing `ChefTap` recipes and returns a vector of [`Recipe`]s.
pub fn parse_zip<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let mut archive = ZipArchive::new(r)?;
    let mut recipes: Vec<Recipe> = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let cursor = Cursor::new(buf);

        match FileFormat::from_filename(&file_name) {
            FileFormat::Html => {
                let recipe = parse_html(cursor)?;
                recipes.extend_from_slice(&recipe);
            }
            FileFormat::Txt => {
                let parsed_recipes = parse_txt(cursor)?;
                recipes.extend_from_slice(&parsed_recipes);
            }
            _ => {}
        }
    }

    Ok(recipes)
}

/// Parses a single HTML file and returns a [`Recipe`] if one is found.
pub fn parse_html<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let doc = Html::parse_document(&read_file(r)?);

    let title = doc
        .select(&Selector::parse("h1").unwrap())
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .unwrap_or_default();

    let source = doc
        .select(&Selector::parse("a").unwrap())
        .map(|el| el.attr("href"))
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .unwrap_or_default()
        .map(String::from);

    let mut recipe = ChefTapRecipe {
        title,
        r#yield: None,
        cook_time: None,
        prep_time: None,
        ingredients: vec![],
        instructions: vec![],
        notes: None,
        source,
    };

    let mut notes = Vec::new();
    let mut keywords = Vec::new();
    let mut _total_time: Option<String> = None;

    for el in doc.select(&Selector::parse("p").unwrap()).skip(1) {
        let text = el.text().collect::<String>();
        if text.contains("•") {
            keywords.extend_from_slice(
                &text
                    .split("•")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>(),
            );
        } else if text.starts_with("Cook Time ") {
            recipe.cook_time = Some(text[10..].trim().to_string());
        } else if text.starts_with("Prep Time ") {
            recipe.prep_time = Some(text[10..].trim().to_string());
        } else if text.starts_with("Total Time ") {
            _total_time = Some(text[10..].trim().to_string());
        } else if text.starts_with("Yield ") {
            if recipe.r#yield.is_none() {
                recipe.r#yield = text[6..]
                    .trim()
                    .to_string()
                    .parse::<i16>()
                    .ok()
                    .and_then(Some);
            }
        } else if text.starts_with(|c: char| c.is_ascii_digit()) {
            recipe
                .ingredients
                .push(RecipeRecipeIngredientFieldEnum::Text(text));
        } else if !recipe.ingredients.is_empty() {
            recipe
                .instructions
                .push(RecipeRecipeInstructionsFieldEnum::Text(text));
        } else {
            notes.push(text);
        }
    }

    for note in notes.into_iter().filter(|s| s.deref() != recipe.title) {
        recipe.notes = Some(note);
    }

    Ok(vec![recipe.into()])
}

/// Parses a `ChefTap` recipe in the text format from the file's content.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let recipe = parse_cheftap_recipe(&mut content.as_str())?;
    Ok(vec![recipe.into()])
}

fn parse_cheftap_recipe(input: &mut &str) -> Result<ChefTapRecipe> {
    parse_recipe
        .map(ChefTapRecipe::from)
        .parse_next(input)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn parse_recipe<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_title,
        source: parse_source,
        notes: parse_notes,
        cook_time: parse_cook_time,
        prep_time: parse_prep_time,
        _: parse_total_time,
        servings: parse_servings,
        _: parse_other_servings,
        ingredients: parse_ingredients,
        _: parse_directions_line,
        instructions: parse_instructions,
    }}
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(
        till_line_ending,
        repeat::<_, _, Vec<_>, _, _>(1.., line_ending),
    )
    .parse_next(input)
}

fn parse_source<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(terminated(
        till_line_ending,
        repeat::<_, _, Vec<_>, _, _>(1.., line_ending),
    ))
    .parse_next(input)
}

fn parse_notes<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(preceded(
        not(is_time_header),
        terminated(till_line_ending, (line_ending, line_ending)),
    ))
    .parse_next(input)
}

fn is_time_header(input: &mut &str) -> WResult<()> {
    preceded(multispace0, alt(("Cook Time".void(), "Prep Time".void()))).parse_next(input)
}

fn parse_cook_time<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(delimited(
        multispace0,
        preceded(literal("Cook Time "), till_line_ending),
        multispace0,
    ))
    .parse_next(input)
}

fn parse_prep_time<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(delimited(
        space0,
        preceded(literal("Prep Time "), till_line_ending),
        multispace0,
    ))
    .parse_next(input)
}

fn parse_total_time<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(delimited(
        space0,
        preceded(literal("Total Time "), till_line_ending),
        multispace0,
    ))
    .parse_next(input)
}

fn parse_servings(input: &mut &str) -> WResult<Option<i16>> {
    let kw = "yields ";
    let kw2 = "Yield";
    opt(preceded(
        alt((
            terminated(take_until(0.., kw), (literal(kw), space0)),
            terminated(take_until(0.., kw2), (literal(kw2), space0)),
        )),
        terminated(digit1, (till_line_ending, line_ending)),
    )
    .try_map(str::parse))
    .parse_next(input)
}

fn parse_other_servings<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(delimited(
        multispace0,
        preceded(literal("Yield "), till_line_ending),
        multispace0,
    ))
    .parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    repeat_till(0.., parse_ingredient, line_ending)
        .map(|(v, _)| v)
        .parse_next(input)
}

fn parse_ingredient<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, line_ending).parse_next(input)
}

fn parse_directions_line<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(delimited(literal("Directions"), line_ending, line_ending)).parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    repeat_till(1.., parse_instruction, peek(eof))
        .map(|(v, _)| v)
        .parse_next(input)
}

fn parse_instruction<'s>(input: &mut &'s str) -> WResult<&'s str> {
    (till_line_ending, line_ending, opt(line_ending))
        .map(|(s, _, _)| s)
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use schema_org::{DurationOrText, Recipe};

    use test_fixtures::open_test_file;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_cheftap_recipe1_txt_ok() -> Result<()> {
        let file = recipe1_txt_file();
        let buf = Cursor::new(file);

        let got = parse_txt(buf)?;

        pretty_assertions::assert_eq!(got, vec![recipe1()]);
        Ok(())
    }

    #[test]
    fn test_cheftap_recipe2_txt_ok() -> Result<()> {
        let file = recipe2_txt_file();
        let buf = Cursor::new(file);

        let got = parse_txt(buf)?;

        pretty_assertions::assert_eq!(got, vec![recipe2()]);
        Ok(())
    }

    #[test]
    fn test_cheftap_recipe2_html_ok() -> Result<()> {
        let file = recipe2_html_file();
        let buf = Cursor::new(file);

        let got = parse_html(buf)?;

        pretty_assertions::assert_eq!(got, vec![recipe2()]);
        Ok(())
    }

    #[test]
    fn test_cheftap_recipe3_txt_ok() -> Result<()> {
        let file = recipe3_txt_file();
        let buf = Cursor::new(file);

        let got = parse_txt(buf)?;

        pretty_assertions::assert_eq!(got, vec![recipe3()]);
        Ok(())
    }

    #[test]
    fn test_cheftap_recipe3_html_ok() -> Result<()> {
        let file = recipe3_html_file();
        let buf = Cursor::new(file);

        let got = parse_html(buf)?;

        pretty_assertions::assert_eq!(got, vec![recipe3()]);
        Ok(())
    }

    #[test]
    fn test_cheftap_zip_txt_ok() -> Result<()> {
        let buf = open_test_file("integrations/cheftap_export_txt.zip");

        let got = parse_zip(buf)?;

        assert_eq!(got.len(), 2);
        assert!(got.contains(&recipe2()));
        assert!(got.contains(&recipe3()));
        Ok(())
    }

    fn recipe1_txt_file<'a>() -> &'a str {
        r"Deviled Eggs

https://www.allrecipes.com/recipe/22390/special-deviled-eggs/


Original recipe (1X) yields 6 servings
6 large eggs
¼ cup mayonnaise
2 tablespoons finely chopped onion
1 tablespoon prepared horseradish
1 tablespoon prepared mustard
1/4 teaspoon paprika, or as needed, for garnish
salt and pepper to taste

Directions

Place eggs in a medium saucepan and cover with cold water. Bring water to a boil and immediately remove from heat. Cover and let eggs stand in hot water for 10 to 12 minutes. Remove from hot water, cool, and peel.

Slice each egg in half lengthwise and remove yolks; set aside egg white halves and place yolks in a medium bowl. Use a fork to mash yolks, then mix in mayonnaise, relish, onion, horseradish, and mustard until well combined.

Use a spoon or pastry bag to fill egg white halves with yolk mixture. Garnish with paprika, salt, and pepper. Chill in the refrigerator until serving.
"
    }

    fn recipe2_txt_file<'a>() -> &'a str {
        r"Cuban-Style Yuca

        https://www.allrecipes.com/recipe/150114/cuban-style-yuca/


        Cook Time 15 Minutes

        Prep Time 10 Minutes

        Total Time 25 Minutes

        Yield 6
        2 pounds yuca, peeled and sliced lengthwise
        0.5 teaspoon salt
        0.25 cup olive oil
        0.5 onion, diced
        4 cloves garlic, minced
        0.5 teaspoon fresh lemon juice

        Place yuca into a pan and fill with enough water to cover. Stir in salt. Bring to a boil over medium-high heat, cover, and cook until tender, about 15 minutes. Drain; place yuca on a serving plate.

        Meanwhile, place olive oil, onion, garlic, and lemon juice into a pan. Cook over medium heat for about 5 minutes.

        Pour the hot olive oil mixture over the yuca; serve immediately.

"
    }

    fn recipe2_html_file<'a>() -> &'a str {
        r#"<html><body><H1><p dir="ltr">Cuban-Style Yuca</p>
        </H1><br/><br/>

        <p dir="ltr">Cook Time 15 Minutes</p>

        <p dir="ltr">Prep Time 10 Minutes</p>

        <p dir="ltr">Total Time 25 Minutes</p>

        <p dir="ltr">Yield 6</p>

        <br/>
        <a href='https://www.allrecipes.com/recipe/150114/cuban-style-yuca/'>Original Article</a><br/><br/>
        <p dir="ltr">Cuban-Style Yuca</p>
        <p dir="ltr">Cook Time 15 Minutes</p>
        <p dir="ltr">Prep Time 10 Minutes</p>
        <p dir="ltr">Total Time 25 Minutes</p>
        <p dir="ltr">Yield 6</p>
        <p dir="ltr">2 pounds yuca, peeled and sliced lengthwise</p>
        <p dir="ltr">0.5 teaspoon salt</p>
        <p dir="ltr">0.25 cup olive oil</p>
        <p dir="ltr">0.5 onion, diced</p>
        <p dir="ltr">4 cloves garlic, minced</p>
        <p dir="ltr">0.5 teaspoon fresh lemon juice</p>
        <p dir="ltr">Place yuca into a pan and fill with enough water to cover. Stir in salt. Bring to a boil over medium-high heat, cover, and cook until tender, about 15 minutes. Drain; place yuca on a serving plate.</p>
        <p dir="ltr">Meanwhile, place olive oil, onion, garlic, and lemon juice into a pan. Cook over medium heat for about 5 minutes.</p>
        <p dir="ltr">Pour the hot olive oil mixture over the yuca; serve immediately.</p>

        </body></html>"#
    }

    fn recipe3_txt_file<'a>() -> &'a str {
        r"Chewy Maple Cookies

        https://www.allrecipes.com/recipe/11030/chewy-maple-cookies/

        These are notes


        Cook Time 15 Minutes

        Prep Time 15 Minutes

        Total Time 30 Minutes

        Yield 18

        Yield 3 dozen
        1 cup packed brown sugar
        0.5 cup shortening
        0.5 cup real maple syrup
        1 egg
        0.5 teaspoon vanilla extract
        1.5 cups all-purpose flour
        2 teaspoons baking powder
        0.5 teaspoon salt
        1 cup flaked coconut

        Preheat the oven to 375 degrees F (190 degrees C). Grease cookie sheets.

        In a mixing bowl, beat together brown sugar and shortening until fluffy. Mix in maple syrup, egg, and vanilla until well combined.

        Combine flour, baking powder, and salt in a separate bowl. Add flour mixture to creamed mixture a little at a time, mixing well after each addition. Stir in coconut. Drop by tablespoonfuls 2 inches apart onto the prepared baking sheets.

        Bake in the preheated oven until edges are set, 10 to 12 minutes.

"
    }

    fn recipe3_html_file<'a>() -> &'a str {
        r#"<html><body><H1><p dir="ltr">Chewy Maple Cookies</p>
        </H1><br/><br/>

        <p dir="ltr">Cook Time 15 Minutes</p>

        <p dir="ltr">Prep Time 15 Minutes</p>

        <p dir="ltr">Total Time 30 Minutes</p>

        <p dir="ltr">Yield 18</p>

        <p dir="ltr">Yield 3 dozen</p>

        <br/>
        <a href='https://www.allrecipes.com/recipe/11030/chewy-maple-cookies/'>Original Article</a><br/><br/>
        <p dir="ltr">cheese &#8226; dinner</p>
        <br/><p dir="ltr">These are notes</p>
        <br/><br/><p dir="ltr">Chewy Maple Cookies</p>
        <p dir="ltr">Cook Time 15 Minutes</p>
        <p dir="ltr">Prep Time 15 Minutes</p>
        <p dir="ltr">Total Time 30 Minutes</p>
        <p dir="ltr">Yield 18</p>
        <p dir="ltr">Yield 3 dozen</p>
        <p dir="ltr">1 cup packed brown sugar</p>
        <p dir="ltr">0.5 cup shortening</p>
        <p dir="ltr">0.5 cup real maple syrup</p>
        <p dir="ltr">1 egg</p>
        <p dir="ltr">0.5 teaspoon vanilla extract</p>
        <p dir="ltr">1.5 cups all-purpose flour</p>
        <p dir="ltr">2 teaspoons baking powder</p>
        <p dir="ltr">0.5 teaspoon salt</p>
        <p dir="ltr">1 cup flaked coconut</p>
        <p dir="ltr">Preheat the oven to 375 degrees F (190 degrees C). Grease cookie sheets.</p>
        <p dir="ltr">In a mixing bowl, beat together brown sugar and shortening until fluffy. Mix in maple syrup, egg, and vanilla until well combined.</p>
        <p dir="ltr">Combine flour, baking powder, and salt in a separate bowl. Add flour mixture to creamed mixture a little at a time, mixing well after each addition. Stir in coconut. Drop by tablespoonfuls 2 inches apart onto the prepared baking sheets.</p>
        <p dir="ltr">Bake in the preheated oven until edges are set, 10 to 12 minutes.</p>

        </body></html>"#
    }

    fn recipe1() -> Recipe {
        Recipe {
            r#type: AtType::Recipe.to_opt(),
            is_based_on: to_is_based_on(
                "https://www.allrecipes.com/recipe/22390/special-deviled-eggs/"
            ),
            name: vec!["Deviled Eggs".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("6 large eggs".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup mayonnaise".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tablespoons finely chopped onion".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 tablespoon prepared horseradish".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text("1 tablespoon prepared mustard".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/4 teaspoon paprika, or as needed, for garnish".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text("salt and pepper to taste".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Place eggs in a medium saucepan and cover with cold water. Bring water to a boil and immediately remove from heat. Cover and let eggs stand in hot water for 10 to 12 minutes. Remove from hot water, cool, and peel.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Slice each egg in half lengthwise and remove yolks; set aside egg white halves and place yolks in a medium bowl. Use a fork to mash yolks, then mix in mayonnaise, relish, onion, horseradish, and mustard until well combined.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Use a spoon or pastry bag to fill egg white halves with yolk mixture. Garnish with paprika, salt, and pepper. Chill in the refrigerator until serving.".into()
                ),
            ],
            recipe_yield: to_yield(6),
            url: vec!["https://www.allrecipes.com/recipe/22390/special-deviled-eggs/".into()],
            ..Default::default()
        }
    }

    fn recipe2() -> Recipe {
        Recipe {
            r#type: AtType::Recipe.to_opt(),
            is_based_on: to_is_based_on(
                "https://www.allrecipes.com/recipe/150114/cuban-style-yuca/"
            ),
            cook_time: vec![DurationOrText::Text("15 Minutes".into())],
            prep_time: vec![DurationOrText::Text("10 Minutes".into())],
            name: vec!["Cuban-Style Yuca".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("2 pounds yuca, peeled and sliced lengthwise".into()),
                RecipeRecipeIngredientFieldEnum::Text("0.5 teaspoon salt".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "0.25 cup olive oil".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "0.5 onion, diced".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text("4 cloves garlic, minced".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "0.5 teaspoon fresh lemon juice".into()
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Place yuca into a pan and fill with enough water to cover. Stir in salt. Bring to a boil over medium-high heat, cover, and cook until tender, about 15 minutes. Drain; place yuca on a serving plate.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Meanwhile, place olive oil, onion, garlic, and lemon juice into a pan. Cook over medium heat for about 5 minutes.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Pour the hot olive oil mixture over the yuca; serve immediately.".into()
                ),
            ],
            recipe_yield: to_yield(6),
            url: vec!["https://www.allrecipes.com/recipe/150114/cuban-style-yuca/".into()],
            ..Default::default()
        }
    }

    fn recipe3() -> Recipe {
        Recipe {
            r#type: AtType::Recipe.to_opt(),
            is_based_on: to_is_based_on(
                "https://www.allrecipes.com/recipe/11030/chewy-maple-cookies/"
            ),
            cook_time: vec![DurationOrText::Text("15 Minutes".into())],
            prep_time: vec![DurationOrText::Text("15 Minutes".into())],
            name: vec!["Chewy Maple Cookies".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 cup packed brown sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("0.5 cup shortening".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "0.5 cup real maple syrup".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 egg".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text("0.5 teaspoon vanilla extract".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1.5 cups all-purpose flour".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 teaspoons baking powder".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "0.5 teaspoon salt".into()
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup flaked coconut".into()
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Preheat the oven to 375 degrees F (190 degrees C). Grease cookie sheets.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "In a mixing bowl, beat together brown sugar and shortening until fluffy. Mix in maple syrup, egg, and vanilla until well combined.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Combine flour, baking powder, and salt in a separate bowl. Add flour mixture to creamed mixture a little at a time, mixing well after each addition. Stir in coconut. Drop by tablespoonfuls 2 inches apart onto the prepared baking sheets.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Bake in the preheated oven until edges are set, 10 to 12 minutes.".into()
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Notes: These are notes".into()
                ),
            ],
            recipe_yield: to_yield(18),
            url: vec!["https://www.allrecipes.com/recipe/11030/chewy-maple-cookies/".into()],
            ..Default::default()
        }
    }
}
