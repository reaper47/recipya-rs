use std::io::{Read, Seek};

use humantime::parse_duration;
use tracing::error;
use winnow::Result as WResult;
use winnow::ascii::{digit1, line_ending, space0, till_line_ending};
use winnow::combinator::{alt, delimited, opt, preceded, repeat, seq, terminated};
use winnow::prelude::*;
use winnow::token::{literal, one_of};

use schema_org::field::{
    ItemListItemListElementFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum,
};
use schema_org::{AtType, Recipe, at_context};

use super::helpers::read_file;
use crate::common::Times;
use crate::helpers::{seconds_to_duration, to_is_based_on, to_yield};
use crate::{Error, Result};

struct AccuChefRecipe {
    title: String,
    category: Option<String>,
    keywords: Vec<String>,
    yield_: Option<i16>,
    times: Times,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
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

impl From<AccuChefRecipe> for Recipe {
    fn from(r: AccuChefRecipe) -> Self {
        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            cook_time: seconds_to_duration(r.times.cook_seconds),
            is_based_on: to_is_based_on(&r.source),
            keywords: r
                .keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: vec![r.title],
            prep_time: seconds_to_duration(r.times.prep_seconds),
            recipe_category: vec![r.category.unwrap_or_default()],
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: to_yield(i64::from(r.yield_.unwrap_or_default())),
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
            ingredients: r.ingredients.into_iter().fold(Vec::new(), |mut acc, ing| {
                if ing.name.is_empty() && ing.quantity.ends_with(':') {
                    acc.push(RecipeRecipeIngredientFieldEnum::new_section(
                        ing.quantity,
                        &[],
                    ));
                } else {
                    let text = format!("{} {}", ing.quantity, ing.name).trim().to_string();

                    if let Some(RecipeRecipeIngredientFieldEnum::ItemList(section)) = acc.last_mut()
                    {
                        section
                            .item_list_element
                            .push(ItemListItemListElementFieldEnum::Text(text));

                        if let Some(i) = section.number_of_items.first_mut() {
                            *i += 1;
                        }
                    } else {
                        acc.push(RecipeRecipeIngredientFieldEnum::Text(text));
                    }
                }

                acc
            }),
            times: Times {
                prep_seconds: r.prep_time.map_or(15 * 60, |s| {
                    let mut s = s.trim().to_string().replace(':', "H");
                    if s.is_empty() {
                        return 15 * 60;
                    }
                    s.push('m');

                    match parse_duration(&s) {
                        Ok(d) => i32::try_from(d.as_secs())
                            .inspect_err(|err| {
                                error!(
                                    "Failed to parse prep time '{d:?}' of an AccuChef recipe: {err}"
                                );
                            })
                            .unwrap_or_default(),
                        Err(err) => {
                            error!("Failed to parse prep time of an AccuChef recipe: {err}");
                            15 * 60
                        }
                    }
                }),
                cook_seconds: 30 * 60,
            },
            instructions: r
                .instructions
                .into_iter()
                .filter(|&s| !s.is_empty())
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.into()))
                .collect(),
            source: r.header.into(),
        }
    }
}

/// Represents the parsed components of an `AccuChef` recipe.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let recipe = parse_accuchef_recipe(&mut content.as_str())?;
    Ok(recipe.into_iter().map(Recipe::from).collect())
}

fn parse_accuchef_recipe(input: &mut &str) -> Result<Vec<AccuChefRecipe>> {
    repeat(0.., recipe.map(|r| Some(AccuChefRecipe::from(r))))
        .parse_next(input)
        .map(|recipes: Vec<_>| recipes.into_iter().flatten().collect())
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        header: parse_header,
        title: parse_title,
        category: parse_category,
        servings: parse_servings,
        prep_time: parse_prep_time,
        ingredients: parse_ingredients,
        instructions: parse_instructions,
        _: parse_footer,
    }}
    .parse_next(input)
}

fn parse_header<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(
        delimited(
            parse_separator,
            literal("AccuChef Import File"),
            till_line_ending,
        ),
        repeat(1.., parse_eol).map(|_: Vec<_>| ()),
    )
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(preceded('A', till_line_ending), parse_eol).parse_next(input)
}

fn parse_category<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(terminated(preceded('B', till_line_ending), parse_eol)).parse_next(input)
}

fn parse_servings(input: &mut &str) -> WResult<Option<i16>> {
    preceded(
        (one_of('M'), literal("Servings"), space0),
        terminated(opt(digit1), (space0, line_ending)),
    )
    .parse_next(input)
    .map(|servings| servings.map(|s| s.parse().unwrap_or_default()))
}

fn parse_prep_time<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(preceded(
        ('P', space0, ':'),
        terminated(till_line_ending, parse_eol),
    ))
    .parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    repeat(1.., parse_ingredient).parse_next(input)
}

fn parse_ingredient<'s>(input: &mut &'s str) -> WResult<Ingredient<'s>> {
    (
        preceded(one_of('H'), till_line_ending),
        line_ending,
        preceded(one_of('I'), till_line_ending),
        line_ending,
    )
        .map(|(quantity, _, name, _): (&str, _, &str, _)| Ingredient {
            name: name.trim(),
            quantity: quantity.trim(),
        })
        .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    repeat(1.., parse_instruction).parse_next(input)
}

fn parse_instruction<'s>(input: &mut &'s str) -> WResult<&'s str> {
    preceded('J', terminated(till_line_ending, parse_eol)).parse_next(input)
}

fn parse_footer<'s>(input: &mut &'s str) -> WResult<&'s str> {
    (literal("Z.....End of recipe definition"), opt(parse_eol))
        .take()
        .parse_next(input)
}

fn parse_separator<'s>(input: &mut &'s str) -> WResult<&'s str> {
    literal("*****").parse_next(input)
}

fn parse_eol<'s>(input: &mut &'s str) -> WResult<&'s str> {
    alt((literal("\r\n"), literal("\n"))).parse_next(input)
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::io::Cursor;

    use schema_org::{
        AtType,
        field::{
            RecipeIsBasedOnFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeYieldFieldEnum,
        },
    };

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_recipe1_ok() -> Result<()> {
        let file = recipe1_file();
        let buf = Cursor::new(file);

        let got = parse(buf)?;

        pretty_assertions::assert_eq!(
            got,
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: seconds_to_duration(1800),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("AccuChef Import File")],
                    name: vec!["24 Hour Fruit Salad".into()],
                    prep_time: seconds_to_duration(900),
                    recipe_category: vec!["Fruit".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Dressing:", &[
                            "3 Egg Yolks",
                            "2 Tbls Sugar",
                            "1 Tbls Butter",
                            "1 Tbls Lemon Juice",
                            "1 Cup Cool Whip",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Fruit:", &[
                            "1 Can Pineapple Chunks (20 Oz)",
                            "2 Cans Mandarin Oranges",
                            "1 Can Royal Ann Cherries",
                            "1 Package Small Colored Marshmellows",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Mix egg yolks, sugar and butter by hand slightly. Mixture should not be".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("frothy. Microwave like scrambled eggs. Cool and blend in lemon juice and".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("cool whip. Drain the fruit and fold in dressing. Add marshmellows.".into()),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: seconds_to_duration(1800),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("AccuChef Import File")],
                    name: vec!["7 Layer Salad".into()],
                    prep_time: seconds_to_duration(900),
                    recipe_category: vec!["Salad".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 Head Lettuce, Shredded".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Cup Celery, Thinly Sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Cup Green Onion, Sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Cup Green Pepper, Chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Pkg Frozen Peas (Or Peas & Onions)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 Oz Can Water Chestnuts".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Cup Miracle Whip".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Cup Sour Cream".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Cup Grated Cheddar Cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Hard Boiled Eggs, Chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Bacon, Crinkled".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Tomtato Cubes".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Mix sour cream and miracle whip. Layer all the ingredients down to water".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("chestnuts and then cover with miracle whip mixture. Refrigerate over".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("night and then place eggs, bacon and tomatoes on top.".into()),
                    ],

                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: seconds_to_duration(1800),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("AccuChef Import File")],
                    name: vec!["Aebleskiver".into()],
                    prep_time: seconds_to_duration(900),
                    recipe_category: vec!["Bread".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("3 C Jiffy Mix".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Tbsp Shortening".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 Eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Milk".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0\\fswiss\\fcharset0 Arial;".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("}{\\f1\\fnil Lucida Casual;}}~".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("\\viewkind4\\uc1\\pard\\lang1033\\f0\\fs18\\par~".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("\\f1 Separate whites from yolks. Beat whites until stiff. Mix jiffy mix,".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("shortening and yolks. Add enough milk to make a thin batter. Fold in the".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("whites. Cook in aebleskiver pan with melted margarine or butter as".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("cooking fat. Cook on medium low heat until done.\\par~".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("}~".into()),
                    ],
                    recipe_yield: vec![RecipeRecipeYieldFieldEnum::new_quantitative_value(24.0)],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: seconds_to_duration(1800),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("AccuChef Import File")],
                    name: vec!["Ambrosia Delight".into()],
                    prep_time: seconds_to_duration(3600),
                    recipe_category: vec!["Dessert".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 Lrg Can Fruit Cocktail".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Lrg Can Crushed Pineapple".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Pkgs Frozen Strawberries".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Or 3 Bananas".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Orange Sherbet".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Mix all together about 2 hours before serving. Serve a with a scoop of".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("orange sherbet on top.".into()),
                    ],
                    ..Default::default()
                }
            ]
        );
        Ok(())
    }

    fn recipe1_file<'a>() -> &'a str {
        r##"*****AccuChef Import File (C)SIVART Software https://www.AccuChef.com
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
*****AccuChef Import File (C)SIVART Software https://www.AccuChef.com
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
*****AccuChef Import File (C)SIVART Software https://www.AccuChef.com
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
*****AccuChef Import File (C)SIVART Software https://www.AccuChef.com
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
