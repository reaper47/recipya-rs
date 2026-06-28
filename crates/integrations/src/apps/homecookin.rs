use std::io::{Cursor, Read, Seek};

use schema_org::Recipe;

use crate::{
    Error, Result,
    apps::{
        helpers::{Ingredient, Instruction, read_file},
        mealmaster,
    },
};

#[derive(Default)]
struct RecipeComponents<'a> {
    author: Option<&'a str>,
    description: Option<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    image: Option<&'a str>,
    cook_time: Option<&'a str>,
    prep_time: Option<&'a str>,
    servings: Option<&'a str>,
    title: &'a str,
    r#yield: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(value: RecipeComponents<'_>) -> Self {
        todo!()
    }
}

/// Parses a `Home Cookin` file.
pub fn parse_hc<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    todo!()
}

/// Parses a `Home Cookin` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let cursor = Cursor::new(content.clone());

    let recipes = match mealmaster::parse(cursor) {
        Ok(r) => r,
        Err(_) => parse_txt_helper(&mut content.as_str())?
            .into_iter()
            .map(Recipe::from)
            .collect(),
    };

    Ok(recipes)
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> Result<Vec<RecipeComponents<'s>>> {
    todo!()
}

/// Parses a `Home Cookin` XML file.
pub fn parse_xml<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use super::*;
    }

    mod files {}

    mod results {
        use super::*;
    }
}
