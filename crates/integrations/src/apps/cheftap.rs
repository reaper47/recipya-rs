use std::io::{Read, Seek};

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::take_until;
use nom::character::complete::{digit1, line_ending, not_line_ending, space0};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many_till, many1};
use nom::sequence::{preceded, terminated};

use schema_org::field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum};
use schema_org::{AtType, Recipe};

use super::helpers::read_file;
use crate::Result;
use crate::helpers::{to_is_based_on, to_yield};

pub struct ChefTapRecipe {
    title: String,
    yield_: Option<i16>,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    source: Option<String>,
}

struct RecipeComponents<'a> {
    title: &'a str,
    servings: Option<i16>,
    ingredients: Vec<&'a str>,
    instructions: Vec<&'a str>,
    source: Option<&'a str>,
}

impl From<ChefTapRecipe> for Recipe {
    fn from(r: ChefTapRecipe) -> Self {
        Recipe {
            r#type: Some(AtType::Recipe.to_string()),
            is_based_on: to_is_based_on(&r.source.clone().unwrap_or_default()),
            name: vec![r.title],
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: to_yield(r.yield_.unwrap_or_default() as i64),
            url: r.source.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl From<RecipeComponents<'_>> for ChefTapRecipe {
    fn from(c: RecipeComponents) -> Self {
        Self {
            title: c.title.to_owned(),
            yield_: c.servings,
            ingredients: c
                .ingredients
                .into_iter()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.into()))
                .collect(),
            instructions: c
                .instructions
                .into_iter()
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.into()))
                .collect(),
            source: c.source.map(String::from),
        }
    }
}

/// Parses a ChefTap recipe from the file's content.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let recipe = parse_cheftap_recipe(&content)?;
    Ok(vec![recipe.into()])
}

fn parse_cheftap_recipe(input: &str) -> Result<ChefTapRecipe> {
    Ok(map(recipe, ChefTapRecipe::from)
        .parse(input)
        .map(|(_, r)| r)?)
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents<'_>> {
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
    many1(instruction).parse(input)
}

fn instruction(input: &str) -> IResult<&str, &str> {
    map(
        (not_line_ending, line_ending, opt(line_ending)),
        |(s, _, _)| s,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use schema_org::Recipe;
    use std::io::Cursor;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_recipe1_ok() -> Result<()> {
        let file = recipe1_file();
        let buf = Cursor::new(file);

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(
            got,
            vec![Recipe {
                r#type: Some(AtType::Recipe.to_string()),
                is_based_on: to_is_based_on(
                    "https://www.allrecipes.com/recipe/22390/special-deviled-eggs/".into()
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
            }]
        );
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
