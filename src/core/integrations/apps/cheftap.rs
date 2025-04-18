use std::io::Read;

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::take_until;
use nom::character::complete::{digit1, line_ending, not_line_ending, space0};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many_till, many1};
use nom::sequence::{preceded, terminated};

use crate::core::integrations::error::{Error, Result};
use crate::core::model::recipe::Sections;

/// Represents the parsed components of a ChefTap recipe.
#[derive(Debug, Default, PartialEq)]
pub struct ChefTapRecipe {
    title: String,
    yield_: Option<i16>,
    ingredients: Sections,
    instructions: Sections,
    source: Option<String>,
}

#[derive(Debug)]
struct RecipeComponents<'a> {
    title: &'a str,
    servings: Option<i16>,
    ingredients: Vec<&'a str>,
    instructions: Vec<&'a str>,
    source: Option<&'a str>,
}

impl ChefTapRecipe {
    /// Parses a ChefTap recipe from the file's content.
    pub fn parse<R>(mut r: R) -> Result<Self>
    where
        R: Read,
    {
        let mut content = String::new();
        r.read_to_string(&mut content)?;

        let recipe = parse_cheftap_recipe(&content)?;
        Ok(recipe)
    }
}

impl From<RecipeComponents<'_>> for ChefTapRecipe {
    fn from(c: RecipeComponents) -> Self {
        Self {
            title: c.title.to_owned(),
            yield_: c.servings,
            ingredients: Sections::from([(
                "".into(),
                c.ingredients.into_iter().map(String::from).collect(),
            )]),
            instructions: Sections::from([(
                "".into(),
                c.instructions.into_iter().map(String::from).collect(),
            )]),
            source: c.source.map(String::from),
        }
    }
}

fn parse_cheftap_recipe(input: &str) -> Result<ChefTapRecipe> {
    map(recipe, ChefTapRecipe::from)
        .parse(input)
        .map(|(_, r)| r)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (title, source, servings, ingredients, instructions),
        |(title, source, servings, ingredients, instructions)| RecipeComponents {
            title,
            servings,
            ingredients,
            instructions,
            source,
        },
    )
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, many1(line_ending)).parse(input)
}

fn source(input: &str) -> IResult<&str, Option<&str>> {
    opt(terminated(not_line_ending, many1(line_ending))).parse(input)
}

fn servings(input: &str) -> IResult<&str, Option<i16>> {
    let kw = "yields ";
    opt(map_res(
        preceded(
            terminated(take_until(kw), (tag(kw), space0)),
            terminated(digit1, (not_line_ending, line_ending)),
        ),
        str::parse,
    ))
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<&str>> {
    map(
        many_till(
            ingredient,
            (line_ending, tag("Directions"), many1(line_ending)),
        ),
        |(v, _)| v,
    )
    .parse(input)
}

fn ingredient(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, line_ending).parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<&str>> {
    println!("{input}");
    let a = many1(instruction).parse(input);
    println!("{a:?}");
    a
}

fn instruction(input: &str) -> IResult<&str, &str> {
    let a = map(
        (not_line_ending, line_ending, opt(line_ending)),
        |(s, _, _)| s,
    )
    .parse(input);
    println!("{a:?}");
    a
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_recipe1_ok() -> Result<()> {
        let file = recipe1_file();
        let buf = Cursor::new(file);

        let got = ChefTapRecipe::parse(buf)?;

        pretty_assertions::assert_eq!(got, ChefTapRecipe {
            title: "Deviled Eggs".into(),
            yield_: Some(6),
            ingredients: Sections::from([
                ("".into(), vec![
                    "6 large eggs".into(),
                    "¼ cup mayonnaise".into(),
                    "2 tablespoons finely chopped onion".into(),
                    "1 tablespoon prepared horseradish".into(),
                    "1 tablespoon prepared mustard".into(),
                    "1/4 teaspoon paprika, or as needed, for garnish".into(),
                    "salt and pepper to taste".into(),
                ])
            ]),
            instructions: Sections::from([
                ("".into(), vec![
                    "Place eggs in a medium saucepan and cover with cold water. Bring water to a boil and immediately remove from heat. Cover and let eggs stand in hot water for 10 to 12 minutes. Remove from hot water, cool, and peel.".into(),
                    "Slice each egg in half lengthwise and remove yolks; set aside egg white halves and place yolks in a medium bowl. Use a fork to mash yolks, then mix in mayonnaise, relish, onion, horseradish, and mustard until well combined.".into(),
                    "Use a spoon or pastry bag to fill egg white halves with yolk mixture. Garnish with paprika, salt, and pepper. Chill in the refrigerator until serving.".into(),
                ])
            ]),
            source: Some("https://www.allrecipes.com/recipe/22390/special-deviled-eggs/".into()),
        });
        Ok(())
    }

    fn recipe1_file<'a>() -> &'a str {
        r##"Deviled Eggs

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
"##
    }
}
