use std::io::{Read, Seek};

use url::Url;
use winnow::Result as WResult;
use winnow::ascii::tab;
use winnow::combinator::{opt, preceded, repeat, seq};
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::{literal, rest};

use schema_org::Recipe;
use schema_org::field::{
    RecipeDescriptionFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
};
use support::time::parse_duration;

use crate::apps::helpers::{read_file, urls_to_image_object};
use crate::error::{Error, Result};
use crate::helpers::{seconds_to_duration, to_is_based_on, to_yield};

struct SaffronRecipe {
    title: String,
    description: Option<String>,
    source: Option<String>,
    original_url: Option<Url>,
    servings: Option<i16>,
    prep_seconds: Option<i32>,
    cook_seconds: Option<i32>,
    #[allow(unused)]
    total_seconds: Option<i32>,
    cookbook: Option<String>,
    #[allow(unused)]
    section: Option<String>,
    image: Option<String>,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

#[derive(Default)]
struct RecipeComponents<'a> {
    title: &'a str,
    description: Option<&'a str>,
    source: Option<&'a str>,
    original_url: Option<Url>,
    servings: Option<i16>,
    prep_seconds: Option<i32>,
    cook_seconds: Option<i32>,
    total_seconds: Option<i32>,
    cookbook: Option<&'a str>,
    section: Option<&'a str>,
    image: Option<&'a str>,
    ingredients: Vec<&'a str>,
    instructions: Vec<&'a str>,
}

impl From<RecipeComponents<'_>> for SaffronRecipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            title: r.title.into(),
            description: r.description.map(String::from),
            source: r.source.map(String::from),
            original_url: r.original_url,
            servings: r.servings,
            prep_seconds: r.prep_seconds,
            cook_seconds: r.cook_seconds,
            total_seconds: r.total_seconds,
            cookbook: r.cookbook.map(String::from),
            section: r.section.map(String::from),
            image: r.image.map(String::from),
            ingredients: r.ingredients.into_iter().map(String::from).collect(),
            instructions: r.instructions.into_iter().map(String::from).collect(),
        }
    }
}

impl From<SaffronRecipe> for Recipe {
    fn from(r: SaffronRecipe) -> Self {
        Self {
            cook_time: seconds_to_duration(r.cook_seconds.unwrap_or_default()),
            description: r
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
                .unwrap_or_default(),
            headline: r.cookbook.map(|s| vec![s]).unwrap_or_default(),
            image: urls_to_image_object(vec![r.image.unwrap_or_default()]),
            name: vec![r.title],
            is_based_on: to_is_based_on(&r.source.unwrap_or_default()),
            prep_time: seconds_to_duration(r.prep_seconds.unwrap_or_default()),
            recipe_ingredient: r
                .ingredients
                .into_iter()
                .map(RecipeRecipeIngredientFieldEnum::Text)
                .collect(),
            recipe_instructions: r
                .instructions
                .into_iter()
                .map(RecipeRecipeInstructionsFieldEnum::Text)
                .collect(),
            recipe_yield: to_yield(r.servings.unwrap_or_default() as i64),
            url: r
                .original_url
                .map(|u| vec![u.to_string()])
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Parses a Saffron recipe from the file's content.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let mut input = content.as_str();

    let recipe = parse_saffron_recipe(&mut input).map_err(|err| Error::Parse(err.to_string()))?;
    Ok(vec![Recipe::from(recipe)])
}

fn parse_saffron_recipe<'s>(input: &mut &'s str) -> WResult<SaffronRecipe> {
    Ok(parse_recipe.map(SaffronRecipe::from).parse_next(input)?)
}

fn parse_recipe<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_title,
        description: parse_description,
        source: parse_source,
        original_url: parse_original_url,
        servings: parse_servings,
        prep_seconds: parse_prep_seconds,
        cook_seconds: parse_cook_seconds,
        total_seconds: parse_total_seconds,
        cookbook: parse_cookbook,
        section: parse_section,
        image: parse_image,
        ingredients: parse_ingredients,
        instructions: parse_instructions,
    }}
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    parse_metadata("Title: ").parse_next(input)
}

fn parse_description<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(parse_metadata("Description: ")).parse_next(input)
}

fn parse_source<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(parse_metadata("Source: ")).parse_next(input)
}

fn parse_original_url<'s>(input: &mut &'s str) -> WResult<Option<Url>> {
    parse_metadata("Source: ")
        .map(|s: &str| Url::parse(s).ok())
        .parse_next(input)
}

fn parse_servings<'s>(input: &mut &'s str) -> WResult<Option<i16>> {
    parse_metadata("Yield: ")
        .map(|s: &str| s.parse().ok())
        .parse_next(input)
}

fn parse_prep_seconds<'s>(input: &mut &'s str) -> WResult<Option<i32>> {
    parse_metadata("Prep: ").map(parse_time).parse_next(input)
}

fn parse_cook_seconds<'s>(input: &mut &'s str) -> WResult<Option<i32>> {
    parse_metadata("Cook: ").map(parse_time).parse_next(input)
}

fn parse_total_seconds<'s>(input: &mut &'s str) -> WResult<Option<i32>> {
    parse_metadata("Total: ").map(parse_time).parse_next(input)
}

fn parse_time(s: &str) -> Option<i32> {
    if let Ok((_, (hours, minutes))) = parse_duration(s) {
        Some((hours * 60 * 60 + minutes * 60) as i32)
    } else {
        None
    }
}

fn parse_cookbook<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(parse_metadata("Cookbook: ")).parse_next(input)
}

fn parse_section<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(parse_metadata("Section: ")).parse_next(input)
}

fn parse_image<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(parse_metadata("Image: ")).parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    preceded(parse_metadata("Ingredients: "), repeat(1.., tabbed_line)).parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    preceded(parse_metadata("Instructions: "), repeat(1.., tabbed_line)).parse_next(input)
}

fn tabbed_line<'s>(input: &mut &'s str) -> WResult<&'s str> {
    preceded(tab, rest).parse_next(input)
}

fn parse_metadata<'s>(text: &str) -> impl Parser<&'s str, &'s str, ContextError> {
    preceded(literal(text), rest)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use files::*;
    use schema_org::Recipe;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_recipe1_ok() -> Result<()> {
        let file = recipe1_file();
        let buf = Cursor::new(file);

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(got, vec![Recipe {
            cook_time: seconds_to_duration(1800),
            description: vec![RecipeDescriptionFieldEnum::Text("Apples are baked into an oven-puffed pancake for breakfast. This is so delicious that you don't need to add any syrup. A great alternative to regular pancakes.".into())],
            headline: vec!["First Cookbook".into()],
            is_based_on: to_is_based_on("KMKIDMAN5".into()),
            name: vec!["Apple Puff Pancake".into()],
            prep_time: seconds_to_duration(900),
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("6 eggs".into()),
                RecipeRecipeIngredientFieldEnum::Text("1.5 cups milk".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon vanilla extract".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup all-purpose flour".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 tablespoons sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("0.5 teaspoon salt".into()),
                RecipeRecipeIngredientFieldEnum::Text("0.25 teaspoon ground cinnamon".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tablespoons butter".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 apples - peeled, cored and sliced".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 tablespoons brown sugar".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 425 degrees F (220 degrees C).".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Blend eggs, milk, and vanilla with an electric mixer in a large bowl. Add flour, sugar, salt, and cinnamon; mix just until blended. Set batter aside.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Melt butter in a 9x9-inch square pan. Arrange apple slices in the bottom of the pan; pour batter over them. Sprinkle brown sugar on top.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Bake in the preheated oven until puffed and lightly browned, about 20 minutes.".into()),
            ],
            recipe_yield: to_yield(9),
            url: vec!["https://www.allrecipes.com/recipe/50936/apple-puff-pancake/".into()],
            ..Default::default()
        }]);
        Ok(())
    }

    #[test]
    fn test_recipe2_ok() -> Result<()> {
        let file = recipe2_file();
        let buf = Cursor::new(file);

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(
            got,
            vec![Recipe {
                cook_time: seconds_to_duration(0),
                headline: vec!["First Cookbook".into()],
                is_based_on: to_is_based_on("Mom".into()),
                name: vec!["Yay".into()],
                prep_time: seconds_to_duration(4500),
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1 kg chicken".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 egg".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Mix stuff".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Eat a melon".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Profit".into()),
                ],
                recipe_yield: to_yield(0),
                ..Default::default()
            }]
        );
        Ok(())
    }

    mod files {
        pub fn recipe1_file<'a>() -> &'a str {
            r#"Title: Apple Puff Pancake
Description: Apples are baked into an oven-puffed pancake for breakfast. This is so delicious that you don't need to add any syrup. A great alternative to regular pancakes.
Source: KMKIDMAN5
Original URL: https://www.allrecipes.com/recipe/50936/apple-puff-pancake/
Yield: 9
Prep: 15 minutes
Cook: 30 minutes
Total: 45 minutes
Cookbook: First Cookbook
Section: First Section
Image:
Ingredients:
	6 eggs
	1.5 cups milk
	1 teaspoon vanilla extract
	1 cup all-purpose flour
	3 tablespoons sugar
	0.5 teaspoon salt
	0.25 teaspoon ground cinnamon
	2 tablespoons butter
	2 apples - peeled, cored and sliced
	3 tablespoons brown sugar
Instructions:
	Preheat the oven to 425 degrees F (220 degrees C).
	Blend eggs, milk, and vanilla with an electric mixer in a large bowl. Add flour, sugar, salt, and cinnamon; mix just until blended. Set batter aside.
	Melt butter in a 9x9-inch square pan. Arrange apple slices in the bottom of the pan; pour batter over them. Sprinkle brown sugar on top.
	Bake in the preheated oven until puffed and lightly browned, about 20 minutes."#
        }

        pub fn recipe2_file<'a>() -> &'a str {
            r#"Title: Yay
Description:
Source: Mom
Original URL:
Yield:
Prep: 1 hour and 15 minutes
Cookbook: First Cookbook
Section: First Section
Image:
Ingredients:
	1 kg chicken
	1 egg
Instructions:
	Mix stuff
	Eat a melon
	Profit"#
        }
    }
}
