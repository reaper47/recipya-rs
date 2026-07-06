use std::borrow::Cow;
use std::io::{Cursor, Read, Seek};

use itertools::Itertools;
use winnow::ascii::{digit1, line_ending, multispace1, space0, till_line_ending};
use winnow::combinator::{alt, delimited, opt, peek, repeat, repeat_till, seq, terminated};
use winnow::token::{any, literal, take_until, take_while};
use winnow::{Parser, Result as WResult};

use schema_org::field::{
    ItemListItemListElementFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum,
};
use schema_org::{AtType, Energy, Mass, NutritionInformation, Recipe};

use crate::helpers::{to_is_based_on, to_yield};
use crate::{
    Error, Result,
    apps::{
        helpers::{Ingredient, Instruction, read_file},
        mealmaster,
        recipya::at_context,
    },
};

#[derive(Default)]
struct RecipeComponents<'a> {
    category: &'a str,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    image: Option<&'a str>,
    is_based_on: &'a str,
    nutrition: Option<&'a str>,
    title: &'a str,
    r#yield: i64,
}

struct NutritionComponents<'a> {
    calories: &'a str,
    carbohydrate: Option<&'a str>,
    cholesterol: Option<&'a str>,
    fat: Option<&'a str>,
    protein: Option<&'a str>,
    sodium: Option<&'a str>,
}

impl NutritionComponents<'_> {
    const fn is_empty(&self) -> bool {
        self.calories.is_empty()
            && self.carbohydrate.is_none()
            && self.cholesterol.is_none()
            && self.fat.is_none()
            && self.protein.is_none()
            && self.sodium.is_none()
    }
}

impl From<NutritionComponents<'_>> for NutritionInformation {
    fn from(n: NutritionComponents<'_>) -> Self {
        let to_mass = |opt: Option<&str>| {
            opt.map(|s| {
                if s == "0 g" {
                    vec![]
                } else {
                    vec![Mass::new(s)]
                }
            })
            .unwrap_or_default()
        };

        Self {
            calories: vec![Energy::new(if n.calories.ends_with("cal") {
                n.calories.into()
            } else {
                format!("{} kcal", n.calories)
            })],
            carbohydrate_content: to_mass(n.carbohydrate),
            cholesterol_content: to_mass(n.cholesterol),
            context: at_context(),
            fat_content: to_mass(n.fat),
            protein_content: to_mass(n.protein),
            sodium_content: to_mass(n.sodium),
            r#type: AtType::NutritionInformation.to_opt(),
            ..Default::default()
        }
    }
}

impl TryFrom<RecipeComponents<'_>> for Recipe {
    type Error = String;

    fn try_from(r: RecipeComponents<'_>) -> std::result::Result<Self, Self::Error> {
        if r.category == "Reference Text" || r.ingredients.is_empty() {
            return Err("skipped".into());
        }

        Ok(Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            name: vec![r.title.into()],
            is_based_on: to_is_based_on(r.is_based_on),
            image: r.image.map_or(Vec::new(), |s| {
                vec![RecipeImageFieldEnum::URL(s.split_whitespace().join(""))]
            }),
            nutrition: r.nutrition.map_or(Vec::new(), |s| {
                let s = s.replace('\n', "");
                let parts = s
                    .split(',')
                    .map(|s| s.split(": ").map(str::trim).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                let n = NutritionComponents {
                    calories: parts
                        .iter()
                        .find(|s| s[0] == "Calories")
                        .map_or("", |s| s[1]),
                    carbohydrate: parts.iter().find(|s| s[0] == "Carbohydrates").map(|s| s[1]),
                    cholesterol: parts.iter().find(|s| s[0] == "Cholesterol").map(|s| s[1]),
                    fat: parts.iter().find(|s| s[0] == "Fat").map(|s| s[1]),
                    protein: parts.iter().find(|s| s[0] == "Protein").map(|s| s[1]),
                    sodium: parts.iter().find(|s| s[0] == "Sodium").map(|s| s[1]),
                };

                if n.is_empty() { vec![] } else { vec![n.into()] }
            }),
            recipe_category: vec![r.category.into()],
            recipe_ingredient: r.ingredients.into_iter().fold(
                Vec::new(),
                |mut acc, item| match item {
                    Ingredient::Line(s)
                        if let Some(RecipeRecipeIngredientFieldEnum::ItemList(list)) =
                            acc.last_mut() =>
                    {
                        list.item_list_element
                            .push(ItemListItemListElementFieldEnum::Text(s.to_string()));
                        if let Some(i) = list.number_of_items.first_mut() {
                            *i += 1;
                        }
                        acc
                    }
                    Ingredient::Line(s) => {
                        acc.push(RecipeRecipeIngredientFieldEnum::Text(s.to_string()));
                        acc
                    }
                    Ingredient::Section(s) => {
                        acc.push(RecipeRecipeIngredientFieldEnum::new_section(
                            s.as_ref(),
                            &[],
                        ));
                        acc
                    }
                },
            ),
            recipe_instructions: r
                .instructions
                .into_iter()
                .fold(Vec::new(), |mut acc, item| match item {
                    Instruction::Line(s)
                        if let Some(RecipeRecipeInstructionsFieldEnum::ItemList(list)) =
                            acc.last_mut() =>
                    {
                        list.item_list_element
                            .push(ItemListItemListElementFieldEnum::Text(
                                s.split_whitespace().join(" "),
                            ));
                        if let Some(i) = list.number_of_items.first_mut() {
                            *i += 1;
                        }
                        acc
                    }
                    Instruction::Line(s) => {
                        acc.push(RecipeRecipeInstructionsFieldEnum::Text(
                            s.split_whitespace().join(" "),
                        ));
                        acc
                    }
                    Instruction::Section(s) => {
                        acc.push(RecipeRecipeInstructionsFieldEnum::new_section(
                            s.as_ref(),
                            Vec::<String>::new(),
                        ));
                        acc
                    }
                }),
            recipe_yield: to_yield(r.r#yield),
            ..Default::default()
        })
    }
}

/// Parses a native `Home Cookin` file.
pub fn parse_hc<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    Ok(parse_hc_helper(&mut content.as_str())?
        .into_iter()
        .filter_map(|r| r.try_into().ok())
        .collect())
}

fn parse_hc_helper<'s>(input: &mut &'s str) -> Result<Vec<RecipeComponents<'s>>> {
    repeat(1.., parse_recipe_hc)
        .parse_next(input)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn parse_recipe_hc<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        category: parse_header,
        title: parse_title,
        _: parse_equals,
        ingredients: parse_ingredients,
        instructions: parse_instructions,
        nutrition: opt(parse_nutrition),
        image: opt(parse_image),
        r#yield: parse_servings,
        is_based_on: parse_is_based_on,
    }}
    .parse_next(input)
}

fn parse_header<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(
        literal("Home Cookin Chapter: "),
        till_line_ending,
        multispace1,
    )
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, (line_ending, space0)).parse_next(input)
}

fn parse_equals<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(take_while(1.., '='), (line_ending, space0)).parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    repeat_till(
        0..,
        terminated(till_line_ending, (line_ending, space0)),
        (space0, literal('.'), line_ending),
    )
    .map(|(lines, _): (Vec<&str>, _)| {
        lines
            .into_iter()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.ends_with(':') {
                    Ingredient::Section(Cow::Borrowed(s.trim_end_matches(':')))
                } else {
                    Ingredient::Line(Cow::Borrowed(s))
                }
            })
            .collect()
    })
    .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    let text: &str = repeat_till::<_, _, (), _, _, _, _>(
        1..,
        any,
        alt((
            peek(literal("Calories: ")).map(|_| ()),
            peek(literal("Photo: ")).map(|_| ()),
            peek(literal("Servings: ")).map(|_| ()),
        )),
    )
    .take()
    .parse_next(input)?;

    Ok(text
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            if s.ends_with(':') {
                Instruction::Section(Cow::Borrowed(s.trim_end_matches(':')))
            } else {
                Instruction::Line(Cow::Borrowed(s))
            }
        })
        .collect())
}

fn parse_nutrition<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(
        peek(literal("Calories: ")),
        take_until(1.., "\n\n"),
        multispace1,
    )
    .parse_next(input)
}

fn parse_image<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(
        (literal("Photo: "), till_line_ending, line_ending, space0),
        take_until(1.., "\n\n"),
        multispace1,
    )
    .parse_next(input)
}

fn parse_servings<'s>(input: &mut &'s str) -> WResult<i64> {
    delimited(literal("Servings: "), digit1, multispace1)
        .parse_to()
        .parse_next(input)
}

fn parse_is_based_on<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(
        literal("Exported from "),
        take_until(1.., " (www.mountainsoftware.com)"),
        (literal(" (www.mountainsoftware.com)"), multispace1),
    )
    .parse_next(input)
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
            .filter_map(|r| r.try_into().ok())
            .collect(),
    };

    Ok(recipes)
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> Result<Vec<RecipeComponents<'s>>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use super::*;

        #[test]
        fn test_hm_hc_ok() -> Result<()> {
            let buf = Cursor::new(files::hc());

            let got = parse_hc(buf)?;

            let expected = results::hc();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }
    }

    mod files {
        pub fn hc<'a>() -> &'a str {
            r"Home Cookin Chapter: Pork and Lamb

        Baked Ham and Kraut Rolls
        =========================
        6 ounces thinly sliced ham
        8 ounce can sauerkraut, drained
        2 tablespoons sliced green onion
        1/2 teaspoon caraway seed
        1/4 cup mayonnaise
        2 tablespoons milk
        2 teaspoons mustard
        .
        Finely chop 2 slices of the ham.

        Combine chopped ham and all remaining ingredients.

        Place a little of the sauerkraut mixture on each remaining ham
        slice. Roll up each slice from one side.

        Microwave on high till heated through.


        Photo: 12891
        /9j/4AAQSkZJRgABAQEAYABgAAD/2wBDAAgGBgcGBQgHBwcJCQgKDBQNDAsLDBkSEw8UHRofHh0a
        HBwgJC4nICIsIxwcKDcpLDAxNDQ0Hyc5PTgyPC4zNDL/2wBDAQkJCQwLDBgNDRgyIRwhMjIyMjIy
        MjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjL/wAARCADcAR0DASIA
        AhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQA
        AAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3
        ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWm
        p6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEA
        AwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSEx
        BhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElK
        U1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3
        uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwDyhA8r
        lUGWHLE9APUmteysed7Fuecnq3+AqxaWKqFLIVQcqpPJ929T/Kr+O5rknU6I74U7asSMBFAXge1O
        3N60nanhOQOcfSsTURCWwQSR9alGe9IFA6Cp4osuc4460m7DIWTcnoDxWfNaTSTxRtG7wltxdTjG
        K3htRegAA5NRwSG+8wW0sYVDtaRgSM+gHc0lNrUpU3LQcXjt4cu4VVHJJxUkP77kfInUMwxn6DrT
        odLt45zJM0k8w5DzD7v+6BwP50kkhEu/Zznvms3JdDup4f8AmLSW9rIvlOJZM/eKvt/lWXN4PWeb
        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
        8xFO3VHO2f/Z

        Servings: 2

        Exported from Home Cookin 9.96 (www.mountainsoftware.com)


        Home Cookin Chapter: Pork and Lamb
        Blood Alcohol Levels
        ====================
        .
        DO NOT DRIVE UNDER THE INFLUENCE:

        Your driving ability is related to your Blood Alcohol
        Concentration.  Alcohol is a drug that affects your judgement and
        slows your reactions. When you have been drinking -- Don't Gamble!
        Call a Cab, Call a sober friend!

        Blood Alcohol Concentration Guide:

        Alcohol is burned up by your body at .015% per hour.

        If your BAC is .025% it takes 1.7 hours to reach .000%
        If your BAC is .050% it takes 3.3 hours to reach .000%
        If your BAC is .075% it takes 5.0 hours to reach .000%
        If your BAC is .100% it takes 6.7 hours to reach .000%
        If your BAC is .125% it takes 8.3 hours to reach .000%

        Percent of Alcohol in Bloodstream:

        .100% is legally drunk in most states.
        Crash risk quadruples at .08% which is now the legal limit in many
        states.

        If you weigh 100 pounds:  2 drinks = .058%, 3 drinks = .088%, 4
        drinks = .117%, 5 drinks = .146%
        If you weigh 120 pounds:  3 drinks = .073%, 4 drinks = .097%, 5
        drinks = .121%, 6 drinks = .145%
        If you weigh 140 pounds:  3 drinks = .063%, 4 drinks = .083%, 5
        drinks = .104%, 6 drinks = .125%
        If you weigh 160 pounds:  4 drinks = .073%, 5 drinks = .091%, 6
        drinks = .109%, 7 drinks = .128%
        If you weigh 180 pounds:  4 drinks = .065%, 5 drinks = .081%, 6
        drinks = .097%, 7 drinks = .113%
        If you weigh 200 pounds:  5 drinks = .073%, 6 drinks = .087%, 7
        drinks = .102%, 8 drinks = .117%
        If you weigh 220 pounds:  5 drinks = .067%, 6 drinks = .080%, 7
        drinks = .093%, 8 drinks = .106%
        If you weigh 240 pounds:  6 drinks = .073%, 7 drinks = .085%, 8
        drinks = .097%, 9 drinks = .109%

        One Drink Equals: 1 oz. of 80 proof Alcohol
        One Drink Equals: 2 oz. of 20% wine
        One Drink Equals: 3 oz. of 12% wine
        One Drink Equals: 12 oz. Bottle of Beer

        Source: Washington State Liquor Control Board

        Servings: 0

        Exported from Home Cookin 9.96 (www.mountainsoftware.com)

        Home Cookin Chapter: Desserts

        Coconut Cream Trifle
        ====================
        Cake:

        2 cups cake flour
        1 cup sugar
        2 teaspoons baking powder
        1 cup sour milk
        3 eggs
        1/4 cup vegetable oil
        2 teaspoons vanilla extract

        Coconut Cream:

        3 cups milk
        1 cup fine unsweetened coconut
        6 tablespoons flour
        1 cup sugar
        pinch of salt
        1 egg
        2 tablespoons butter
        1 teaspoon vanilla extract
        1/2 teaspoon coconut extract
        1/2 teaspoon almond extract

        Whipped Cream:

        2 cups whipping cream
        6 tablespoons sugar
        2 teaspoons vanilla extract

        1-1/2 ounces coconut rum
        .
        Cake:

        Mix flour, sugar, and baking powder. Add sour milk, eggs, oil, and
        vanilla. Bake in a 9x13 greased and floured baking pan at 325°F
        for 30 minutes. Cool completely and cut into cubes.

        Coconut Cream:

        Scald milk and coconut in the microwave. Meanwhile, in a saucepan
        combine flour, sugar, and salt. Over medium heat, slowly add the
        scalded milk whisking constantly.

        Continue to cook over medium heat until mixture begins to slightly
        thicken. At this point remove from heat and pour about a half cup
        of this mixture onto a slightly beaten extra large egg whisking
        constantly. Pour the egg mixture immediately back into the pot,
        continuing to constantly stir. Cook for an additional minute or
        two until pudding consistency and remove from the flame.

        Stir in butter and extracts. Cool completely.

        Vanilla Whipped Cream:

        Combine whipping cream, sugar, and vanila. Whip to firm peaks.

        To assemble your trifle divide your cake and filling into 3 equal
        portions. In a large serving bowl, place a layer of cake cubes and
        sprinkle with rum.

        Repeat for the remaining layers and top with Vanilla Whipped Cream
        and toasted coconut to garnish.

        Calories: 252, Cholesterol: 77mg, Fat: 11g, Protein: 25g, Sodium:
        304mg, Carbohydrates: 11g, Potassium: 520mg

        Servings: 4

        Exported from Home Cookin 9.96 (www.mountainsoftware.com)
"
        }
    }

    mod results {
        use schema_org::{
            Energy, Mass, NutritionInformation,
            field::{RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum},
        };

        use super::*;
        use crate::helpers::{to_is_based_on, to_yield};

        pub fn hc() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    name: vec!["Baked Ham and Kraut Rolls".into()],
                    recipe_category: vec!["Pork and Lamb".into()],
                    image: vec![
                    RecipeImageFieldEnum::URL("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".into()),
                    ],
                    recipe_yield: to_yield(2),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("6 ounces thinly sliced ham".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 ounce can sauerkraut, drained".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons sliced green onion".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon caraway seed".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 cup mayonnaise".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons milk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 teaspoons mustard".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Finely chop 2 slices of the ham.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Combine chopped ham and all remaining ingredients.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place a little of the sauerkraut mixture on each remaining ham slice. Roll up each slice from one side.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Microwave on high till heated through.".into(),
                        ),
                    ],
                    is_based_on: to_is_based_on("Home Cookin 9.96"),
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    name: vec!["Coconut Cream Trifle".into()],
                    nutrition: vec![NutritionInformation {
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        calories: vec![Energy::new("252 kcal")],
                        carbohydrate_content: vec![Mass::new("11g")],
                        cholesterol_content: vec![Mass::new("77mg")],
                        fat_content: vec![Mass::new("11g")],
                        protein_content: vec![Mass::new("25g")],
                        sodium_content: vec![Mass::new("304mg")],
                        ..Default::default()
                    }],
                    recipe_category: vec!["Desserts".into()],
                    recipe_yield: to_yield(4),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Cake", &[
                            "2 cups cake flour",
                            "1 cup sugar",
                            "2 teaspoons baking powder",
                            "1 cup sour milk",
                            "3 eggs",
                            "1/4 cup vegetable oil",
                            "2 teaspoons vanilla extract",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Coconut Cream", &[
                            "3 cups milk",
                            "1 cup fine unsweetened coconut",
                            "6 tablespoons flour",
                            "1 cup sugar",
                            "pinch of salt",
                            "1 egg",
                            "2 tablespoons butter",
                            "1 teaspoon vanilla extract",
                            "1/2 teaspoon coconut extract",
                            "1/2 teaspoon almond extract",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Whipped Cream", &[
                            "2 cups whipping cream",
                            "6 tablespoons sugar",
                            "2 teaspoons vanilla extract",
                            "1-1/2 ounces coconut rum",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::new_section("Cake", vec![
                            "Mix flour, sugar, and baking powder. Add sour milk, eggs, oil, and vanilla. Bake in a 9x13 greased and floured baking pan at 325°F for 30 minutes. Cool completely and cut into cubes."
                        ]),
                        RecipeRecipeInstructionsFieldEnum::new_section("Coconut Cream", vec![
                            "Scald milk and coconut in the microwave. Meanwhile, in a saucepan combine flour, sugar, and salt. Over medium heat, slowly add the scalded milk whisking constantly.",
                            "Continue to cook over medium heat until mixture begins to slightly thicken. At this point remove from heat and pour about a half cup of this mixture onto a slightly beaten extra large egg whisking constantly. Pour the egg mixture immediately back into the pot, continuing to constantly stir. Cook for an additional minute or two until pudding consistency and remove from the flame.",
                            "Stir in butter and extracts. Cool completely.",
                        ]),
                        RecipeRecipeInstructionsFieldEnum::new_section("Vanilla Whipped Cream", vec![
                            "Combine whipping cream, sugar, and vanila. Whip to firm peaks.",
                            "To assemble your trifle divide your cake and filling into 3 equal portions. In a large serving bowl, place a layer of cake cubes and sprinkle with rum.",
                            "Repeat for the remaining layers and top with Vanilla Whipped Cream and toasted coconut to garnish.",
                        ]),
                    ],
                    is_based_on: to_is_based_on("Home Cookin 9.96"),
                    ..Default::default()
                },
            ]
        }
    }
}
