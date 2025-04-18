use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, not_line_ending, space0, tab};
use nom::combinator::{eof, map, map_opt, opt};
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};
use url::Url;

use crate::core::integrations::error::{Error, Result};
use crate::core::integrations::IntegrationRecipe;
use crate::core::model::recipe::{Sections, TimesForCreate};
use crate::core::support::time::parse_duration;

struct SaffronRecipe {
    title: String,
    description: Option<String>,
    source: Option<String>,
    original_url: Option<Url>,
    servings: Option<i16>,
    prep_seconds: Option<i32>,
    cook_seconds: Option<i32>,
    total_seconds: Option<i32>,
    cookbook: Option<String>,
    section: Option<String>,
    image: Option<String>,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

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

impl From<SaffronRecipe> for IntegrationRecipe {
    fn from(r: SaffronRecipe) -> Self {
        Self {
            cookbook: r.cookbook,
            description: r.description,
            images: vec![r.image.unwrap_or_default()].into_iter().filter(|s| !s.is_empty()).collect(),
            ingredients: Sections::from([("".into(), r.ingredients)]),
            instructions: Sections::from([("".into(), r.instructions)]),
            source: r.original_url.map(|u| u.to_string()).or(r.source),
            times: if r.prep_seconds.is_some() || r.cook_seconds.is_some() {
                Some(TimesForCreate {
                    prep_seconds: r.prep_seconds.unwrap_or_default(),
                    cook_seconds: r.cook_seconds.unwrap_or_default(),
                })
            } else {
                None
            },
            title: r.title,
            yield_: r.servings,
            ..Default::default()
        }
    }
}

/// Parses a Saffron recipe from the file's content.
pub fn parse<R>(mut r: R) -> Result<Vec<IntegrationRecipe>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;

    let recipe = parse_saffron_recipe(&content)?;
    Ok(vec![IntegrationRecipe::from(recipe)])
}

fn parse_saffron_recipe(input: &str) -> Result<SaffronRecipe> {
    map(recipe, SaffronRecipe::from)
        .parse(input)
        .map(|(_, r)| r)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (
            title,
            description,
            source,
            original_url,
            servings,
            prep_seconds,
            cook_seconds,
            total_seconds,
            cookbook,
            section,
            image,
            ingredients,
            instructions,
        ),
        |(
            title,
            description,
            source,
            original_url,
            servings,
            prep_seconds,
            cook_seconds,
            total_seconds,
            cookbook,
            section,
            image,
            ingredients,
            instructions,
        )| {
            RecipeComponents {
                title,
                description: description.filter(|s| !s.is_empty()),
                source,
                original_url,
                servings,
                prep_seconds,
                cook_seconds,
                total_seconds,
                cookbook,
                section,
                image: image.filter(|s| !s.is_empty()),
                ingredients,
                instructions,
            }
        },
    )
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    preceded(tag("Title: "), rest).parse(input)
}

fn description(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(tag("Description: "), rest)).parse(input)
}

fn source(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(tag("Source: "), rest)).parse(input)
}

fn original_url(input: &str) -> IResult<&str, Option<Url>> {
    map(preceded(tag("Original URL: "), rest), |s| {
        Url::parse(s).ok()
    })
    .parse(input)
}

fn servings(input: &str) -> IResult<&str, Option<i16>> {
    map(preceded(tag("Yield: "), rest), |s| s.parse().ok()).parse(input)
}

fn prep_seconds(input: &str) -> IResult<&str, Option<i32>> {
    opt(map_opt(preceded(tag("Prep: "), rest), parse_time)).parse(input)
}

fn cook_seconds(input: &str) -> IResult<&str, Option<i32>> {
    opt(map_opt(preceded(tag("Cook: "), rest), parse_time)).parse(input)
}

fn total_seconds(input: &str) -> IResult<&str, Option<i32>> {
    opt(map_opt(preceded(tag("Total: "), rest), parse_time)).parse(input)
}

fn parse_time(s: &str) -> Option<i32> {
    if let Ok((_, (hours, minutes))) = parse_duration(s) {
        Some((hours * 60 * 60 + minutes * 60) as i32)
    } else {
        None
    }
}

fn cookbook(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(tag("Cookbook: "), rest)).parse(input)
}

fn section(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(tag("Section: "), rest)).parse(input)
}

fn image(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded((tag("Image: "), space0), rest)).parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(preceded(tag("Ingredients: "), rest), many1(tabbed_line)).parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(preceded(tag("Instructions: "), rest), many1(tabbed_line)).parse(input)
}

fn tabbed_line(input: &str) -> IResult<&str, &str> {
    preceded(tab, rest).parse(input)
}

fn rest(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, alt((line_ending, eof))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use files::*;
    use std::io::Cursor;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_recipe1_ok() -> Result<()> {
        let file = recipe1_file();
        let buf = Cursor::new(file);

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(got, vec![IntegrationRecipe {
            title: "Apple Puff Pancake".into(),
            description: Some("Apples are baked into an oven-puffed pancake for breakfast. This is so delicious that you don't need to add any syrup. A great alternative to regular pancakes.".into()),
            source: Some("https://www.allrecipes.com/recipe/50936/apple-puff-pancake/".into()),
            yield_: Some(9),
            times: Some(TimesForCreate {
                prep_seconds: 900,
                cook_seconds: 1800,
            }),
            cookbook: Some("First Cookbook".into()),
            ingredients: Sections::from([
                ("".into(), vec![
                    "6 eggs".into(),
                    "1.5 cups milk".into(),
                    "1 teaspoon vanilla extract".into(),
                    "1 cup all-purpose flour".into(),
                    "3 tablespoons sugar".into(),
                    "0.5 teaspoon salt".into(),
                    "0.25 teaspoon ground cinnamon".into(),
                    "2 tablespoons butter".into(),
                    "2 apples - peeled, cored and sliced".into(),
                    "3 tablespoons brown sugar".into(),
                ])
            ]),
            instructions: Sections::from([
                ("".into(), vec![
                    "Preheat the oven to 425 degrees F (220 degrees C).".into(),
                    "Blend eggs, milk, and vanilla with an electric mixer in a large bowl. Add flour, sugar, salt, and cinnamon; mix just until blended. Set batter aside.".into(),
                    "Melt butter in a 9x9-inch square pan. Arrange apple slices in the bottom of the pan; pour batter over them. Sprinkle brown sugar on top.".into(),
                    "Bake in the preheated oven until puffed and lightly browned, about 20 minutes.".into(),
                ])
            ]),
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
            vec![IntegrationRecipe {
                title: "Yay".into(),
                description: None,
                source: Some("Mom".into()),
                times: Some(TimesForCreate {
                    prep_seconds: 4500,
                    cook_seconds: 0,
                }),
                cookbook: Some("First Cookbook".into()),
                ingredients: Sections::from([
                    ("".into(), vec!["1 kg chicken".into(), "1 egg".into()])
                ]),
                instructions: Sections::from([
                    ("".into(), vec!["Mix stuff".into(), "Eat a melon".into(), "Profit".into()])
                ]),
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
