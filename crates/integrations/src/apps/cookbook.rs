use std::{
    borrow::Cow,
    io::{Read, Seek},
};

use winnow::{
    Parser, Result as WResult,
    ascii::{digit1, line_ending, multispace1, space0, space1, till_line_ending},
    combinator::{delimited, repeat, seq, terminated},
    token::{literal, take_until},
};

use schema_org::{AtType, Recipe};

use crate::{
    Error, Result,
    apps::{
        helpers::{
            Ingredient, Instruction, Parsers, ToSections, parse_archive_helper_no_images, read_file,
        },
        recipya::at_context,
    },
};

#[derive(Default)]
struct RecipeComponents<'a> {
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    title: &'a str,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            name: vec![r.title.into()],
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            ..Default::default()
        }
    }
}

/// Parses a `CookBook` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper_no_images(
        r,
        &Parsers {
            txt: Some(parse_txt),
            ..Default::default()
        },
    )
}

/// Parses a `CookBook` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let recipe =
        parse_txt_helper(&mut content.as_str()).map_err(|err| Error::Parse(err.to_string()))?;
    Ok(vec![recipe.into()])
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_title,
        ingredients: parse_ingredients,
        instructions: parse_instructions,
    }}
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, multispace1).parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    terminated(take_until(1.., "\n\n"), (line_ending, line_ending, space0))
        .map(|s: &str| {
            s.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(|s| Ingredient::Line(Cow::Borrowed(s)))
                .collect()
        })
        .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    repeat(
        1..,
        delimited(
            (digit1, literal('.'), space1),
            till_line_ending,
            (line_ending, space0),
        ),
    )
    .map(|lines: Vec<&str>| {
        lines
            .into_iter()
            .map(|s| Instruction::Line(Cow::Borrowed(s)))
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
        fn test_cookbook_txt_ok() -> Result<()> {
            let buf = Cursor::new(files::txt());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, vec![results::txt()]);
            Ok(())
        }
    }

    mod files {
        pub fn txt<'a>() -> &'a str {
            r"Homemade Lemonade

            5 lemons, unwaxed
            1 litre water
            1 large handful ice cubes
            180 grams caster sugar, or brown sugar

            1. Put the lemons in a blender and blitz along with half of the sugar, half the ice cubes and 500ml water.
            2. Strain the juice into a jug to get rid of any bits.
            3. Put the lemon pulp (from the straining) back into the food processor. Add the rest of the sugar, ice cubes and 500ml water and blitz again.
            4. Strain it into the jug with the first lot of juice and discard the pulp.
            5. Serve with slices of lemon and lots of ice.

            Shared from CookBook
            https://cookbookmanager.com"
        }
    }

    mod results {
        use schema_org::{
            AtType,
            field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum},
        };

        use crate::apps::recipya::at_context;

        use super::*;

        pub fn txt() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Homemade Lemonade".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("5 lemons, unwaxed".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 litre water".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 large handful ice cubes".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "180 grams caster sugar, or brown sugar".into(),
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Put the lemons in a blender and blitz along with half of the sugar, half the ice cubes and 500ml water.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Strain the juice into a jug to get rid of any bits.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Put the lemon pulp (from the straining) back into the food processor. Add the rest of the sugar, ice cubes and 500ml water and blitz again.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Strain it into the jug with the first lot of juice and discard the pulp.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Serve with slices of lemon and lots of ice.".into()),
                ],
                ..Default::default()
            }
        }
    }
}
