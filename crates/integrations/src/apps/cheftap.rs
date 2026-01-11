use std::io::{Read, Seek};

use winnow::Result as WResult;
use winnow::ascii::{digit1, line_ending, space0, till_line_ending};
use winnow::combinator::{eof, opt, peek, preceded, repeat, repeat_till, seq, terminated};
use winnow::prelude::*;

use schema_org::field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum};
use schema_org::{AtType, Recipe};
use winnow::token::{literal, take_until};

use super::helpers::read_file;
use crate::helpers::{to_is_based_on, to_yield};
use crate::{Error, Result};

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
            r#type: AtType::Recipe.to_opt(),
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
        servings: parse_servings,
        ingredients: parse_ingredients,
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

fn parse_servings(input: &mut &str) -> WResult<Option<i16>> {
    let kw = "yields ";
    opt(preceded(
        terminated(take_until(0.., kw), (literal(kw), space0)),
        terminated(digit1, (till_line_ending, line_ending)),
    )
    .try_map(str::parse))
    .parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    repeat_till(
        0..,
        parse_ingredient,
        (
            line_ending,
            literal("Directions"),
            repeat(1.., line_ending).fold(|| (), |_, _| ()),
        ),
    )
    .map(|(v, _)| v)
    .parse_next(input)
}

fn parse_ingredient<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, line_ending).parse_next(input)
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
