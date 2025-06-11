use std::borrow::Cow;
use std::io::Read;

use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::combinator::{map, opt, recognize, rest};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, preceded, terminated};

use crate::core::integrations::apps::helpers::{Ingredient, Instruction};
use crate::core::integrations::helpers::{
    seconds_to_duration, sections_to_itemlist, sections_to_vec, to_defined_text, to_is_based_on,
    to_organization_type, to_text, to_yield,
};
use crate::core::integrations::{Error, Result};
use crate::core::model::recipe::Sections;
use crate::core::scraper::schema::{
    AggregateRating, AtType, Energy, Mass, NumberOrText, NutritionInformationSchema,
    RecipeCategory, RecipeSchema,
};

struct RecipeComponents<'a> {
    author: &'a str,
    categories: Vec<&'a str>,
    description: &'a str,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    nutrition: Vec<&'a str>,
    prep_time: &'a str,
    rating: Option<&'a str>,
    source: Option<&'a str>,
    title: &'a str,
    total_time: Option<&'a str>,
    r#yield: i64,
}

impl From<RecipeComponents<'_>> for RecipeSchema {
    fn from(r: RecipeComponents) -> Self {
        let (category, keywords) = match r.categories.as_slice() {
            [first, rest @ ..] => (first.trim().to_string(), rest.to_vec()),
            [] => ("".into(), Vec::new()),
        };

        let rating = r
            .rating
            .and_then(|r| r.trim().split_once(' '))
            .map(|(numerator, denominator)| {
                let numerator: f64 = numerator.parse().unwrap_or_default();
                let denominator: f64 = denominator.parse().unwrap_or_default();
                numerator / denominator
            })
            .unwrap_or_default();

        let parse_time = |s: &str| -> i32 {
            if s == "0:00" {
                return 0;
            }

            let mut s = s.trim_end_matches("\"").replacen(":", "m", 1);
            s.push('s');
            humantime::parse_duration(&s).unwrap_or_default().as_secs() as i32
        };

        let prep_secs = parse_time(r.prep_time);
        let total_secs = r.total_time.map_or(0, parse_time);
        let cook_secs = total_secs.saturating_sub(prep_secs);

        let mut nutrition = NutritionInformationSchema::default();
        r.nutrition.iter().for_each(|s| {
            let (value, key) = s.split_once(' ').unwrap_or_default();
            if value == "0g" || value == "0mg" {
                return;
            }

            let energy = Some(Energy::Str(value.to_string()));
            let mass = Some(Mass::Str(value.to_string()));

            if key == "Calories" {
                nutrition.calories = energy
            } else if key.starts_with("Fat") {
                nutrition.fat_content = mass
            } else if key == "Protein" {
                nutrition.protein_content = mass
            } else if key == "Carbohydrate" {
                nutrition.carbohydrate_content = mass
            } else if key  == "Dietary Fiber" {
                nutrition.fiber_content = mass
            } else if key == "Cholesterol" {
                nutrition.cholesterol_content = mass
            } else if key == "Sodium" {
                nutrition.sodium_content = mass
            } else if key == "Total Sugars" {
                nutrition.sugar_content = mass
            }
        });

        Self {
            at_type: Some(AtType::Recipe),
            aggregate_rating: (rating > 0.0).then(|| AggregateRating {
                at_type: AtType::AggregateRating,
                rating_value: Some(NumberOrText::Number(rating)),
                ..Default::default()
            }),
            author: to_organization_type(r.author.into()),
            cook_time: seconds_to_duration(cook_secs),
            description: to_text(r.description.trim_end_matches("\"").into()),
            is_based_on: to_is_based_on(r.source.map(|s| {
                let mut s = s.trim_end_matches("\"").to_string();
                s.push_str(" [Exported from MasterCook]");
                s
            }).unwrap_or_default()),
            keywords: to_defined_text(keywords.join(",")),
            name: Some(r.title.into()),
            nutrition: if nutrition.is_empty() {
                None
            } else {
                Some(nutrition)
            },
            prep_time: seconds_to_duration(prep_secs),
            recipe_category: if category.is_empty() {
                RecipeCategory::default()
            } else {
                RecipeCategory::Text(category)
            },
            recipe_ingredient: sections_to_vec(Sections::from([(
                "".into(),
                r.ingredients
                    .into_iter()
                    .map(|s| match s {
                        Ingredient::Line(s) => s.to_string(),
                        Ingredient::Section(s) => s.to_string(),
                    })
                    .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" ").replace(" --", ","))
                    .collect(),
            )])),
            recipe_instructions: sections_to_itemlist(Sections::from([(
                "".into(),
                r.instructions
                    .into_iter()
                    .map(|s| match s {
                        Instruction::Line(s) => s.to_string(),
                        Instruction::Section(s) => s.to_string(),
                    })
                    .collect(),
            )])),
            recipe_yield: to_yield(r.r#yield),
            total_time: seconds_to_duration(total_secs),
            ..Default::default()
        }
    }
}

/// Parses a MasterCook MX2 file.
pub fn parse_mx2<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    Ok(vec![])
}

/// Parses a MasterCook MXP file.
pub fn parse_mxp<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    Ok(vec![])
}

/// Parses a MasterCook MZ2 file.
pub fn parse_mz2<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    Ok(vec![])
}

/// Parses a MasterCook TXT file.
pub fn parse_txt<R>(mut r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;

    let recipes = parse_txt_helper(&content)?
        .into_iter()
        .map(RecipeSchema::from)
        .collect::<Vec<_>>();

    Ok(recipes)
}

fn parse_txt_helper(input: &str) -> Result<Vec<RecipeComponents>> {
    many1(map(recipe_txt, |r| r))
        .parse(input)
        .map(|(_, r)| r)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe_txt(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (
            header,
            title,
            author,
            serving_size,
            prep_time,
            categories,
            line_ending,
            ingredients,
            line_ending,
            instructions,
            description,
            opt(source),
            opt(servings),
            opt(total_time),
            opt(rating),
            metasection,
            nutrition,
            flush,
        ),
        |(
            _,
            title,
            author,
            serving_size,
            prep_time,
            categories,
            _,
            ingredients,
            _,
            instructions,
            description,
            source,
            _,
            total_time,
            rating,
            _,
            nutrition,
            _,
        )| RecipeComponents {
            author,
            categories,
            description,
            ingredients,
            instructions,
            nutrition,
            prep_time,
            rating,
            source,
            title,
            total_time,
            r#yield: serving_size,
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    recognize((
        space0,
        line_ending,
        tag("* Exported from MasterCook *"),
        line_ending,
        line_ending,
    ))
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    delimited(space1, take_until("\n"), (line_ending, line_ending)).parse(input)
}

fn author(input: &str) -> IResult<&str, &str> {
    delimited(tag("Recipe By     :"), take_until("\n"), line_ending).parse(input)
}

fn serving_size(input: &str) -> IResult<&str, i64> {
    map(
        delimited(tag("Serving Size  : "), digit1, space1),
        |s: &str| s.parse::<i64>().unwrap_or_default(),
    )
    .parse(input)
}

fn prep_time(input: &str) -> IResult<&str, &str> {
    delimited(tag("Preparation Time :"), take_until("\n"), line_ending).parse(input)
}

fn categories(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(
        tag(","),
        delimited(tag("Categories    :"), take_until("\n"), line_ending),
    )
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    map(
        (
            (
                tag("  Amount  Measure       Ingredient -- Preparation Method"),
                line_ending,
            ),
            (
                tag("--------  ------------  --------------------------------"),
                line_ending,
            ),
            many1(ingredient),
        ),
        |(_, _, v)| v,
    )
    .parse(input)
}

fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(delimited(space1, take_until("\n"), line_ending), |s| {
        Ingredient::Line(Cow::Borrowed(s))
    })
    .parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    map(
        take_until("Description:"),
        |content: &str| {
            content
                .split("\n\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| Instruction::Line(Cow::Borrowed(s)))
                .collect()
        },
    )
    .parse(input)
}

fn description(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Description:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn source(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Source:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn servings(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Yield:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn total_time(input: &str) -> IResult<&str, &str> {
    delimited(
        (tag("Start to Finish Time:"), line_ending, tag("  \"")),
        take_until("\n"),
        line_ending,
    )
    .parse(input)
}

fn rating(input: &str) -> IResult<&str, &str> {
    delimited(tag("Ratings       : "), take_until("\n"), line_ending).parse(input)
}

fn metasection(input: &str) -> IResult<&str, &str> {
    delimited(
        (opt(line_ending), space0),
        tag("- - - - - - - - - - - - - - - - - - -"),
        (space0, line_ending, line_ending),
    )
    .parse(input)
}

fn nutrition(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag("Per Serving (excluding unknown items): "),
        map(
            terminated(take_until("\n"), line_ending),
            |content: &str| {
                content.split(';')
                    .map(|s| s.trim())
                    .collect::<Vec<&str>>()
            }
        )
    )
    .parse(input)
}

fn flush(input: &str) -> IResult<&str, &str> {
    alt((
        take_until("\n* Exported from MasterCook *"),
        rest,
    )).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn test_txt1() -> Result<()> {
            let file = files::txt();
            let buf = Cursor::new(file);

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, results::txt());
            Ok(())
        }
    }

    mod files {
        pub fn txt<'a>() -> &'a str {
            r##"                      
* Exported from MasterCook *

                               Best Chicken

Recipe By     :Macpoule
Serving Size  : 18    Preparation Time :0:30
Categories    : Dinner

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
  6             pounds  chicken breast -- for braising meat
  12       tablespoons  oil -- flaked

Cut the chicken into many pieces

Mix it with love

Kiss it

Eat

Description:
  "The best chicken in the universe!"
Source:
  "My mother's recipe cookbook"
Yield:
  "36 cups"
Start to Finish Time:
  "1:45"
Ratings       : 5 10
                                    - - - - - - - - - - - - - - - - - - - 

Per Serving (excluding unknown items): 340 Calories; 23g Fat (62.1% calories from fat); 32g Protein; 0g Carbohydrate; 0g Dietary Fiber; 97mg Cholesterol; 95mg Sodium; 0g Total Sugars; 1mcg Vitamin D; 17mg Calcium; 1mg Iron; 333mg Potassium; 263mg Phosphorus.  Exchanges: .

Suggested Wine: Columbia; Pinot Gris; 1996

NOTES : This recipe is extremely difficult to prepare.

Nutr. Assoc. : 0 0

                      
* Exported from MasterCook *

                             Delicious Ramen

Recipe By     :Macpoule
Serving Size  : 24    Preparation Time :0:00
Categories    : 

  Amount  Measure       Ingredient -- Preparation Method
--------  ------------  --------------------------------
  4                     egg
  12             heads  Algood Preserves, Strawberry
  1           teaspoon  salt

Cook for 12 hours

Make sure to boil

Mix all ingredients together and eat

Description:
  "Ramen has never been soooo delicious"
Source:
  "My mother's recipe cookbook"

                                    - - - - - - - - - - - - - - - - - - - 

Per Serving (excluding unknown items): 34 Calories; 1g Fat (21.3% calories from fat); 1g Protein; 6g Carbohydrate; 0g Dietary Fiber; 31mg Cholesterol; 108mg Sodium; 4g Total Sugars; trace Vitamin D; 5mg Calcium; trace Iron; 12mg Potassium; 17mg Phosphorus.  Exchanges: .


Nutr. Assoc. : 0 0 0
"##
        }
    }

    mod results {
        use super::*;

        pub fn txt() -> Vec<RecipeSchema> {
            vec![
                RecipeSchema {
                    at_type: Some(AtType::Recipe),
                    author: to_organization_type("Macpoule".into()),
                    aggregate_rating: Some(AggregateRating {
                        at_type: AtType::AggregateRating,
                        rating_value: Some(NumberOrText::Number(0.5)),
                        ..Default::default()
                    }),
                    cook_time: seconds_to_duration(4500),
                    description: to_text("The best chicken in the universe!".into()),
                    is_based_on: to_is_based_on(
                        "My mother's recipe cookbook [Exported from MasterCook]".into(),
                    ),
                    name: Some("Best Chicken".into()),
                    nutrition: Some(NutritionInformationSchema {
                        at_type: Some(AtType::NutritionInformation),
                        calories: Some(Energy::Str("340".into())),
                        cholesterol_content: Some(Mass::Str("97mg".into())),
                        fat_content: Some(Mass::Str("23g".into())),
                        protein_content: Some(Mass::Str("32g".into())),
                        sodium_content: Some(Mass::Str("95mg".into())),
                        ..Default::default()
                    }),
                    prep_time: seconds_to_duration(30 * 60),
                    recipe_category: RecipeCategory::Text("Dinner".into()),
                    recipe_ingredient: sections_to_vec(Sections::from([(
                        "".into(),
                        vec![
                            "6 pounds chicken breast, for braising meat".into(),
                            "12 tablespoons oil, flaked".into(),
                        ],
                    )])),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            "Cut the chicken into many pieces".into(),
                            "Mix it with love".into(),
                            "Kiss it".into(),
                            "Eat".into(),
                        ],
                    )])),
                    recipe_yield: to_yield(18),
                    total_time: seconds_to_duration(6300),
                    ..Default::default()
                },
                RecipeSchema {
                    at_type: Some(AtType::Recipe),
                    author: to_organization_type("Macpoule".into()),
                    description: to_text("Ramen has never been soooo delicious".into()),
                    is_based_on: to_is_based_on(
                        "My mother's recipe cookbook [Exported from MasterCook]".into(),
                    ),
                    name: Some("Delicious Ramen".into()),
                    nutrition: Some(NutritionInformationSchema {
                        at_type: Some(AtType::NutritionInformation),
                        calories: Some(Energy::Str("34".into())),
                        carbohydrate_content: Some(Mass::Str("6g".into())),
                        cholesterol_content: Some(Mass::Str("31mg".into())),
                        fat_content: Some(Mass::Str("1g".into())),
                        protein_content: Some(Mass::Str("1g".into())),
                        sodium_content: Some(Mass::Str("108mg".into())),
                        sugar_content: Some(Mass::Str("4g".into())),
                        ..Default::default()
                    }),
                    recipe_ingredient: sections_to_vec(Sections::from([(
                        "".into(),
                        vec![
                            "4 egg".into(),
                            "12 heads Algood Preserves, Strawberry".into(),
                            "1 teaspoon salt".into(),
                        ],
                    )])),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            "Cook for 12 hours".into(),
                            "Make sure to boil".into(),
                            "Mix all ingredients together and eat".into(),
                        ],
                    )])),
                    recipe_yield: to_yield(24),
                    ..Default::default()
                },
            ]
        }
    }
}
