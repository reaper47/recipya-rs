use std::io::Read;

use humantime::parse_duration;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::char;
use nom::character::complete::{digit1, not_line_ending, space0};
use nom::combinator::{map, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use tracing::error;

use crate::core::integrations::{Error, IntegrationRecipe, Result};
use crate::core::model::recipe::{Sections, TimesForCreate};

struct AccuChefRecipe {
    title: String,
    category: Option<String>,
    keywords: Vec<String>,
    yield_: Option<i16>,
    times: TimesForCreate,
    ingredients: Sections,
    instructions: Sections,
    source: String,
}

struct RecipeComponents<'a> {
    header: &'a str,
    title: &'a str,
    category: Option<&'a str>,
    servings: Option<i16>,
    prep_time: Option<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<&'a str>,
}

struct Ingredient<'a> {
    name: &'a str,
    quantity: &'a str,
}

impl From<AccuChefRecipe> for IntegrationRecipe {
    fn from(r: AccuChefRecipe) -> Self {
        Self {
            title: r.title,
            category: r.category,
            keywords: r.keywords,
            yield_: r.yield_,
            times: Some(r.times),
            ingredients: r.ingredients,
            instructions: r.instructions,
            source: Some(r.source).filter(|s| !s.is_empty()),
            ..Default::default()
        }
    }
}

impl From<RecipeComponents<'_>> for AccuChefRecipe {
    fn from(r: RecipeComponents) -> Self {
        Self {
            title: r.title.into(),
            category: r.category.map(String::from),
            keywords: vec![],
            yield_: r.servings,
            ingredients: Sections::from([(
                "".into(),
                r.ingredients
                    .into_iter()
                    .map(|ing| format!("{} {}", ing.quantity, ing.name))
                    .collect(),
            )]),
            times: TimesForCreate {
                prep_seconds: r
                    .prep_time
                    .map(|s| {
                        let mut s = s.trim().to_string().replace(":", "H");
                        if s.is_empty() {
                            return 15 * 60;
                        }
                        s.push('m');

                        match parse_duration(&s) {
                            Ok(d) => d.as_secs() as i32,
                            Err(err) => {
                                error!("Failed to parse prep time of an AccuChef recipe: {err}");
                                15 * 60
                            }
                        }
                    })
                    .unwrap_or(15 * 60),
                cook_seconds: 30 * 60,
            },
            instructions: Sections::from([(
                "".into(),
                r.instructions.into_iter().map(String::from).collect(),
            )]),
            source: r.header.into(),
        }
    }
}

/// Represents the parsed components of an AccuChef recipe.
pub fn parse<R>(mut r: R) -> Result<Vec<IntegrationRecipe>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;

    let recipe = parse_accuchef_recipe(&content)?;
    Ok(recipe.into_iter().map(IntegrationRecipe::from).collect())
}

fn parse_accuchef_recipe(input: &str) -> Result<Vec<AccuChefRecipe>> {
    many0(map(recipe, |r| Some(AccuChefRecipe::from(r))))
        .parse(input)
        .map(|(_, recipes)| recipes.into_iter().flatten().collect())
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents> {
    map(
        (
            header,
            title,
            category,
            servings,
            prep_time,
            ingredients,
            instructions,
            footer,
        ),
        |(header, title, category, servings, prep_time, ingredients, instructions, _)| {
            RecipeComponents {
                header,
                title,
                category,
                servings: servings.and_then(|s| s.parse::<i16>().ok()),
                prep_time,
                ingredients,
                instructions,
            }
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    terminated(
        delimited(separator, tag("AccuChef Import File"), not_line_ending),
        many1(eol),
    )
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    terminated(preceded(char('A'), not_line_ending), eol).parse(input)
}

fn category(input: &str) -> IResult<&str, Option<&str>> {
    opt(terminated(preceded(char('B'), not_line_ending), eol)).parse(input)
}

fn servings(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = (char('M'), tag("Servings"), space0).parse(input)?;
    let (input, num) = opt(terminated(digit1, space0)).parse(input)?;
    let (input, _) = eol.parse(input)?;
    Ok((input, num))
}

fn prep_time(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(
        (char('P'), space0, char(':')),
        terminated(not_line_ending, eol),
    ))
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    many1(ingredient).parse(input)
}

fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(
        (
            preceded(char('H'), not_line_ending),
            eol,
            preceded(char('I'), not_line_ending),
            eol,
        ),
        |(quantity, _, name, _)| Ingredient {
            name: if name.trim().is_empty() { "" } else { name },
            quantity: if quantity.trim().is_empty() {
                ""
            } else {
                quantity
            },
        },
    )
    .parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<&str>> {
    many1(instruction).parse(input)
}

fn instruction(input: &str) -> IResult<&str, &str> {
    preceded(char('J'), terminated(not_line_ending, eol)).parse(input)
}

fn footer(input: &str) -> IResult<&str, &str> {
    recognize((tag("Z.....End of recipe definition"), opt(eol))).parse(input)
}

fn separator(input: &str) -> IResult<&str, &str> {
    recognize(tag("*****")).parse(input)
}

fn eol(input: &str) -> IResult<&str, &str> {
    recognize(alt((tag("\r\n"), tag("\n")))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::default::Default;
    use std::io::Cursor;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_recipe1_ok() -> Result<()> {
        let file = recipe1_file();
        let buf = Cursor::new(file);

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(
            got,
            vec![IntegrationRecipe {
                title: "24 Hour Fruit Salad".into(),
                category: Some("Fruit".into()),
                keywords: vec![],
                yield_: None,
                times: Some(
                   TimesForCreate {
                       prep_seconds: 900,
                       cook_seconds: 1800,
                   },
                ),
                ingredients: Sections::from([
                    ("".into(), vec![
                        "Dressing: ".into(),
                        "3 Egg Yolks".into(),
                        "2 Tbls Sugar".into(),
                        "1 Tbls Butter".into(),
                        "1 Tbls Lemon Juice".into(),
                        "1 Cup Cool Whip".into(),
                        "Fruit: ".into(),
                        "1 Can Pineapple Chunks (20 Oz)".into(),
                        "2 Cans Mandarin Oranges".into(),
                        "1 Can Royal Ann Cherries".into(),
                        "1 Package Small Colored Marshmellows".into(),
                    ])
                ]),
                instructions: Sections::from([
                    ("".into(), vec![
                        "Mix egg yolks, sugar and butter by hand slightly. Mixture should not be".into(),
                        "frothy. Microwave like scrambled eggs. Cool and blend in lemon juice and".into(),
                        "cool whip. Drain the fruit and fold in dressing. Add marshmellows.".into(),
                    ])
                ]),
                source: Some("AccuChef Import File".into()),
                ..Default::default()
            }, IntegrationRecipe {
                title: "7 Layer Salad".into(),
                category: Some("Salad".into()),
                keywords: vec![],
                yield_: None,
                times: Some(
                    TimesForCreate {
                        prep_seconds: 900,
                        cook_seconds: 1800,
                    },
                ),
                ingredients: Sections::from([
                    ("".into(), vec![
                        "1 Head Lettuce, Shredded".into(),
                        "1 Cup Celery, Thinly Sliced".into(),
                        "1 Cup Green Onion, Sliced".into(),
                        "1 Cup Green Pepper, Chopped".into(),
                        "1 Pkg Frozen Peas (Or Peas & Onions)".into(),
                        "8 Oz Can Water Chestnuts".into(),
                        "1/2 Cup Miracle Whip".into(),
                        "1/2 Cup Sour Cream".into(),
                        "1/2 Cup Grated Cheddar Cheese".into(),
                        " Hard Boiled Eggs, Chopped".into(),
                        " Bacon, Crinkled".into(),
                        " Tomtato Cubes".into(),
                    ])
                ]),
                instructions: Sections::from([
                    ("".into(), vec![
                        "Mix sour cream and miracle whip. Layer all the ingredients down to water".into(),
                        "chestnuts and then cover with miracle whip mixture. Refrigerate over".into(),
                        "night and then place eggs, bacon and tomatoes on top.".into(),
                    ])
                ]),
                source: Some("AccuChef Import File".into()),
                ..Default::default()
            }, IntegrationRecipe {
                title: "Aebleskiver".into(),
                category: Some("Bread".into()),
                keywords: vec![],
                yield_: Some(24),
                times: Some(
                    TimesForCreate {
                        prep_seconds: 900,
                        cook_seconds: 1800,
                    },
                ),
                ingredients: Sections::from([
                    ("".into(), vec![
                       "3 C Jiffy Mix".into(),
                       "2 Tbsp Shortening".into(),
                       "3 Eggs".into(),
                       " Milk".into(),
                    ])
                ]),
                instructions: Sections::from([
                    ("".into(), vec![
                        "{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0\\fswiss\\fcharset0 Arial;".into(),
                        "}{\\f1\\fnil Lucida Casual;}}~".into(),
                        "\\viewkind4\\uc1\\pard\\lang1033\\f0\\fs18\\par~".into(),
                        "\\f1 Separate whites from yolks. Beat whites until stiff. Mix jiffy mix,".into(),
                        "shortening and yolks. Add enough milk to make a thin batter. Fold in the".into(),
                        "whites. Cook in aebleskiver pan with melted margarine or butter as".into(),
                        "cooking fat. Cook on medium low heat until done.\\par~".into(),
                        "}~".into(),
                        "".into(),
                    ])
                ]),
                source: Some("AccuChef Import File".into()),
                ..Default::default()
            }, IntegrationRecipe {
                title: "Ambrosia Delight".into(),
                category: Some("Dessert".into()),
                keywords: vec![],
                yield_: None,
                times: Some(TimesForCreate {
                    prep_seconds: 3600,
                    cook_seconds: 1800,
                }),
                ingredients: Sections::from([
                    ("".into(), vec![
                        "1 Lrg Can Fruit Cocktail".into(),
                        "1 Lrg Can Crushed Pineapple".into(),
                        "2 Pkgs Frozen Strawberries".into(),
                        "2 Or 3 Bananas".into(),
                        " Orange Sherbet".into(),
                    ])
                ]),
                instructions: Sections::from([
                    ("".into(), vec![
                        "Mix all together about 2 hours before serving. Serve a with a scoop of".into(),
                        "orange sherbet on top.".into(),
                    ])
                ]),
                source: Some("AccuChef Import File".into()),
                ..Default::default()
            }]
        );
        Ok(())
    }

    fn recipe1_file<'a>() -> &'a str {
        r##"*****AccuChef Import File (C)SIVART Software http://www.AccuChef.com
A24 Hour Fruit Salad
BFruit
MServings
HDressing:
I
H3
IEgg Yolks
H2 Tbls
ISugar
H1 Tbls
IButter
H1 Tbls
ILemon Juice
H1 Cup
ICool Whip
HFruit:
I
H1 Can
IPineapple Chunks (20 Oz)
H2 Cans
IMandarin Oranges
H1 Can
IRoyal Ann Cherries
H1 Package
ISmall Colored Marshmellows
JMix egg yolks, sugar and butter by hand slightly. Mixture should not be
Jfrothy. Microwave like scrambled eggs. Cool and blend in lemon juice and
Jcool whip. Drain the fruit and fold in dressing. Add marshmellows.
Z.....End of recipe definition
*****AccuChef Import File (C)SIVART Software http://www.AccuChef.com
A7 Layer Salad
BSalad
MServings
H1
IHead Lettuce, Shredded
H1 Cup
ICelery, Thinly Sliced
H1 Cup
IGreen Onion, Sliced
H1 Cup
IGreen Pepper, Chopped
H1 Pkg
IFrozen Peas (Or Peas & Onions)
H8 Oz Can
IWater Chestnuts
H1/2 Cup
IMiracle Whip
H1/2 Cup
ISour Cream
H1/2 Cup
IGrated Cheddar Cheese
H
IHard Boiled Eggs, Chopped
H
IBacon, Crinkled
H
ITomtato Cubes
JMix sour cream and miracle whip. Layer all the ingredients down to water
Jchestnuts and then cover with miracle whip mixture. Refrigerate over
Jnight and then place eggs, bacon and tomatoes on top.
Z.....End of recipe definition
*****AccuChef Import File (C)SIVART Software http://www.AccuChef.com
AAebleskiver
BBread
MServings 24
P  :
H3 C
IJiffy Mix
H2 Tbsp
IShortening
H3
IEggs
H
IMilk
J{\rtf1\ansi\ansicpg1252\deff0{\fonttbl{\f0\fswiss\fcharset0 Arial;
J}{\f1\fnil Lucida Casual;}}~
J\viewkind4\uc1\pard\lang1033\f0\fs18\par~
J\f1 Separate whites from yolks. Beat whites until stiff. Mix jiffy mix,
Jshortening and yolks. Add enough milk to make a thin batter. Fold in the
Jwhites. Cook in aebleskiver pan with melted margarine or butter as
Jcooking fat. Cook on medium low heat until done.\par~
J}~
J
Z.....End of recipe definition
*****AccuChef Import File (C)SIVART Software http://www.AccuChef.com
AAmbrosia Delight
BDessert
MServings
P  : 60
H1 Lrg Can
IFruit Cocktail
H1 Lrg Can
ICrushed Pineapple
H2 Pkgs
IFrozen Strawberries
H2 Or 3
IBananas
H
IOrange Sherbet
JMix all together about 2 hours before serving. Serve a with a scoop of
Jorange sherbet on top.
Z.....End of recipe definition"##
    }
}
