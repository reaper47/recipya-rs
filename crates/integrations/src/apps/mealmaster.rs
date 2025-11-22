//! Parses the MealMaster file format.
//!
//! The following versions are supported:
//!     - Meal-Master v6.14
//!     - Meal-Master v6.20
//!     - Meal-Master v7.01
//!     - Meal-Master v7.04
//!     - Meal-Master v7.07
//!     - Meal-Master v8.00
//!     - Meal-Master v8.01
//!     - Meal-Master v8.02
//!     - Meal-Master v8.05
//!     - Meal-Master v8.06
//!     - COOKmate
//!     - Now You're Cooking! v4.72 (Meal-Master Export Format)

use std::borrow::Cow;
use std::io::{Read, Seek};

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_until, take_while_m_n, take_while1};
use nom::bytes::take_while;
use nom::character::complete::{char, line_ending, multispace0, multispace1, space0, space1};
use nom::combinator::{map, map_res, opt, peek, recognize, verify};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use url::Url;

use schema_org::Recipe;
use schema_org::field::{
    RecipeAuthorFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum,
};

use super::helpers::{Ingredient, Instruction, ToSections, is_vchar_or_space, read_file};
use crate::Result;
use crate::helpers::{to_is_based_on, to_yield};

struct MealMasterRecipe {
    author: Option<String>,
    title: String,
    category: Option<String>,
    keywords: Vec<String>,
    yield_: i16,
    ingredients: Vec<RecipeRecipeIngredientFieldEnum>,
    instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    source: String,
}

#[allow(dead_code)]
struct RecipeComponents<'a> {
    author: Option<&'a str>,
    header: (&'a str, &'a str),
    title: &'a str,
    categories: Vec<&'a str>,
    tags: Option<Vec<&'a str>>,
    servings: i16,
    ingredients: Vec<Ingredient<'a>>,
    ingredient_notes: Option<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
}

impl From<RecipeComponents<'_>> for MealMasterRecipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        let items = r.categories.split_first();

        let mut instructions = r.instructions.to_sections();
        if let Some(Ingredient::Line(ref s)) = r.ingredient_notes {
            let line = s.split_whitespace().collect::<Vec<_>>().join(" ");
            let line = format!("*{}", line.trim());
            instructions.push(RecipeRecipeInstructionsFieldEnum::new_section(
                "Notes",
                split_by_asterisks(&line)
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            ));
        }

        Self {
            author: r.author.map(String::from),
            title: r.title.to_string(),
            category: items.map(|(a, _b)| a.to_string()),
            keywords: items
                .map(|(_a, b)| b.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default(),
            yield_: r.servings,
            ingredients: r.ingredients.to_sections(),
            instructions,
            source: format!("{} {}", r.header.0, r.header.1.trim_end_matches('-').trim()),
        }
    }
}

fn split_by_asterisks(input: &str) -> Vec<String> {
    input
        .chars()
        .fold(String::new(), |mut acc, ch| {
            if ch == '*' {
                if !acc.ends_with('|') {
                    acc.push('|');
                }
            } else {
                acc.push(ch);
            }
            acc
        })
        .split('|')
        .filter(|s| !s.is_empty())
        .map(|s| format!("- {}\n", s.trim()))
        .collect()
}

impl From<MealMasterRecipe> for Recipe {
    fn from(r: MealMasterRecipe) -> Self {
        Self {
            author: vec![RecipeAuthorFieldEnum::new_person(
                &r.author.unwrap_or_default(),
            )],
            is_based_on: to_is_based_on(r.source.trim()),
            keywords: r
                .keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: Some(r.title)
                .filter(|s| !s.is_empty())
                .map(|s| vec![s])
                .unwrap_or_default(),
            recipe_category: vec![r.category.unwrap_or_default()],
            recipe_ingredient: r.ingredients,
            recipe_instructions: r.instructions,
            recipe_yield: to_yield(r.yield_ as i64),
            url: Url::parse(&r.source)
                .ok()
                .map(|u| vec![u.to_string()])
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Parses a MealMaster recipe from the file's content.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?.replace("\n \n", "\n\n");

    Ok(parse_meal_master_recipe(&content)?
        .into_iter()
        .map(Recipe::from)
        .collect())
}

fn parse_meal_master_recipe(input: &str) -> Result<Vec<MealMasterRecipe>> {
    let res = many1(alt((
        map(
            preceded(take_while_m_n(1, 10, is_vchar_or_space), line_ending),
            |_| None,
        ),
        map(recipe, |r| Some(r.into())),
    )))
    .parse(input)
    .map(|(_, recipes)| recipes.into_iter().flatten().collect::<Vec<_>>())?;

    Ok(res)
}

fn recipe(input: &str) -> IResult<&str, RecipeComponents<'_>> {
    map(
        (
            many0(line_ending),
            header,
            title,
            categories,
            opt(tags),
            servings,
            opt(author),
            ingredients,
            opt(ingredient_notes),
            instructions,
            footer,
        ),
        |(
            _,
            (header_tag, header_rest),
            title,
            categories,
            tags,
            servings,
            author,
            ingredients,
            ingredient_notes,
            instructions,
            _,
        )| {
            RecipeComponents {
                author,
                header: (header_tag, header_rest),
                title,
                categories,
                tags,
                servings,
                ingredients,
                ingredient_notes,
                instructions,
            }
        },
    )
    .parse(input)
}

fn header(input: &str) -> IResult<&str, (&str, &str)> {
    let meal_master = "Meal-Master";
    let now_youre_cooking = "Now You're Cooking!";
    let cookmate = "Cookmate";

    map(
        (
            separator,
            alt((
                take_until(now_youre_cooking),
                take_until(cookmate),
                take_until(meal_master),
            )),
            alt((tag(now_youre_cooking), tag(cookmate), tag(meal_master))),
            take_while(is_vchar_or_space),
            many1(eol),
        ),
        |(_, _, tag, rest, _)| (tag, rest),
    )
    .parse(input)
}

fn title(input: &str) -> IResult<&str, &str> {
    map(
        (
            opt(char(' ')),
            opt((eol, char(' '))),
            space0,
            tag("Title: "),
            take_while_m_n(0, 60, is_vchar_or_space),
            eol,
        ),
        |(_, _, _, _, s, _)| s,
    )
    .parse(input)
}

fn categories(input: &str) -> IResult<&str, Vec<&str>> {
    alt((
        preceded(
            (opt(char(' ')), opt(tag("       ")), tag("Categories: ")),
            terminated(categlist, many1(eol)),
        ),
        preceded(
            (opt(char(' ')), opt(tag("       ")), tag("Categories: ")),
            terminated(categlist_spaces, many1(eol)),
        ),
    ))
    .parse(input)
}

fn tags(input: &str) -> IResult<&str, Vec<&str>> {
    alt((
        preceded(
            (tag("Tags:"), alt((space0, line_ending))),
            terminated(categlist, many1(eol)),
        ),
        map((tag("Tags:"), take_until("\n"), many1(line_ending)), |_| {
            Vec::new()
        }),
    ))
    .parse(input)
}

fn categlist(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        char(','),
        preceded(
            space0,
            take_while_m_n(1, 80, |c: char| is_vchar_or_space(c) && c != ','),
        ),
    )
    .parse(input)
}

fn categlist_spaces(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        char(' '),
        preceded(
            space0,
            take_while_m_n(1, 11, |c: char| c.is_ascii_graphic() && c != ' '),
        ),
    )
    .parse(input)
}

fn servings(input: &str) -> IResult<&str, i16> {
    alt((
        map_res(
            (
                space0,
                alt((tag("Servings: "), tag("Yield: "))),
                opt(char(' ')),
                take_while_m_n(1, 4, |c: char| c.is_ascii_digit()),
                opt((char(' '), take_until("\n"))),
                many1(eol),
                opt((space0, eol)),
            ),
            |(_, _, _, digits, _, _, _)| digits.parse(),
        ),
        map(
            (
                space0,
                alt((tag("Servings: "), tag("Yield: "))),
                many1(eol),
                opt((space0, eol)),
            ),
            |_| 2,
        ),
    ))
    .parse(input)
}

fn author(input: &str) -> IResult<&str, &str> {
    map(
        (
            space0,
            tag("Contributor: "),
            take_while_m_n(0, 60, is_vchar_or_space),
            line_ending,
        ),
        |(_, _, s, _)| s,
    )
    .parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    alt((twocolumn, onecolumn)).parse(input)
}

fn onecolumn(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    many0(alt((
        map(section, |s| Ingredient::Section(Cow::Borrowed(s))),
        terminated(ingredone, eol),
        map((space0, eol), |_| Ingredient::Line(Cow::Borrowed(""))),
    )))
    .parse(input)
}

fn twocolumn(input: &str) -> IResult<&str, Vec<Ingredient<'_>>> {
    many1(alt((
        map(section, |s| vec![Ingredient::Section(Cow::Borrowed(s))]),
        map(
            (ingredtwo, char(' '), ingredone, many0(line_ending)),
            |(ing1, _, ing2, _)| vec![ing1, ing2],
        ),
        map(terminated(ingredone, many0(eol)), |ing| vec![ing]),
    )))
    .parse(input)
    .map(|(rest, nested)| (rest, nested.into_iter().flatten().collect()))
}

fn ingredone(input: &str) -> IResult<&str, Ingredient<'_>> {
    map(
        recognize((
            amount,
            char(' '),
            unit,
            space1,
            take_while_m_n(1, 90, is_vchar_or_space),
        )),
        |s| Ingredient::Line(Cow::Borrowed(s)),
    )
    .parse(input)
}

fn ingredtwo(input: &str) -> IResult<&str, Ingredient<'_>> {
    map(
        recognize((
            amount,
            char(' '),
            unit,
            char(' '),
            take_while_m_n(29, 29, is_vchar_or_space),
        )),
        |s| Ingredient::Line(Cow::Borrowed(s)),
    )
    .parse(input)
}

fn amount(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 7, |c: char| is_vchar_or_space(c) || c == '.' || c == '/').parse(input)
}

fn unit(input: &str) -> IResult<&str, &str> {
    alt((units1, units2, units3, units4)).parse(input)
}

fn units1(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("x "),
        tag_no_case("sm"),
        tag_no_case("md"),
        tag_no_case("lg"),
        tag_no_case("cn"),
        tag_no_case("pk"),
        tag_no_case("pn"),
        tag_no_case("dr"),
        tag_no_case("ds"),
        tag_no_case("ct"),
        tag_no_case("bn"),
    ))
    .parse(input)
}

fn units2(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("ea"),
        tag_no_case("t "),
        tag_no_case("ts"),
        tag_no_case("T "),
        tag_no_case("tb"),
        tag_no_case("fl"),
        tag_no_case("c "),
        tag_no_case("pt"),
        tag_no_case("qt"),
        tag_no_case("ga"),
        tag_no_case("oz"),
        tag_no_case("lb"),
    ))
    .parse(input)
}

fn units3(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("ml"),
        tag_no_case("cb"),
        tag_no_case("cl"),
        tag_no_case("dl"),
        tag_no_case("l "),
        tag_no_case("mg"),
        tag_no_case("cg"),
        tag_no_case("dg"),
        tag_no_case("g "),
        tag_no_case("kg"),
    ))
    .parse(input)
}

fn units4(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("st"),
        tag_no_case("cv"),
        tag_no_case("sp"),
        tag_no_case("sl"),
        tag_no_case("sk"),
        tag_no_case("sks"),
        tag_no_case("ta"),
        tag_no_case("lh"),
        tag_no_case("hd"),
        tag_no_case("bx"),
        tag_no_case("lf"),
        tag_no_case("  "),
    ))
    .parse(input)
}

fn ingredient_notes(input: &str) -> IResult<&str, Ingredient<'_>> {
    let delim = "*----------------------------------------------------------------------*";
    let delim2 = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++";

    let take_until_either = alt((take_until(delim), take_until(delim2)));

    let closing_delim = alt((
        (multispace0, tag(delim), line_ending),
        (multispace0, tag(delim2), line_ending),
    ));

    map(
        preceded(
            peek(delimited(
                (multispace1, tag("*"), space1),
                alt((take_until(delim), take_until(delim2))),
                alt((
                    (multispace0, tag(delim), line_ending),
                    (multispace0, tag(delim2), line_ending),
                )),
            )),
            delimited(
                (multispace1, tag("*"), space1),
                take_until_either,
                closing_delim,
            ),
        ),
        |s| Ingredient::Line(Cow::Borrowed(s)),
    )
    .parse(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction<'_>>> {
    many0(alt((
        map((multispace0, section), |(_, s)| {
            Instruction::Section(Cow::Borrowed(s))
        }),
        map(instruction, |s| Instruction::Line(Cow::Borrowed(s))),
    )))
    .parse(input)
}

fn instruction(input: &str) -> IResult<&str, &str> {
    terminated(
        verify(
            take_until_earliest_of(&["\n-----", "\n\n"]),
            |line: &str| {
                !line.trim_start().starts_with("MMMMM") && !line.trim_start().starts_with("-----")
            },
        ),
        many1(line_ending),
    )
    .parse(input)
}

fn take_until_earliest_of(patterns: &[&str]) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let mut earliest_pos = input.len();
        let mut found_pattern = None;

        for &pattern in patterns {
            if let Some(pos) = input.find(pattern)
                && pos < earliest_pos
            {
                earliest_pos = pos;
                found_pattern = Some(pattern);
            }
        }

        if let Some(_pattern) = found_pattern {
            Ok((&input[earliest_pos..], &input[..earliest_pos]))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TakeUntil,
            )))
        }
    }
}

fn section(input: &str) -> IResult<&str, &str> {
    preceded(
        separator,
        terminated(delimited(dashes, not_dash, dashes), line_ending),
    )
    .parse(input)
}

fn dashes(input: &str) -> IResult<&str, &str> {
    recognize(many1(char('-'))).parse(input)
}

fn not_dash(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c != '-').parse(input) // Take until we hit a dash again
}
fn footer(input: &str) -> IResult<&str, &str> {
    terminated(separator, multispace0).parse(input)
}

fn separator(input: &str) -> IResult<&str, &str> {
    recognize(alt((
        tag("MMMMM"),
        tag("-----"),
        tag("-------------"),
        tag("-----------------------------------------------------------------------------"),
    )))
    .parse(input)
}

fn eol(input: &str) -> IResult<&str, &str> {
    recognize(alt((tag("\r\n"), tag("\n")))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    use files::*;
    use results::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_recipes {
        use super::*;

        #[test]
        fn test_recipe_unspecified_version() -> Result<()> {
            let file = recipe_unspecified_version_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_unspecified_version()]);
            Ok(())
        }

        #[test]
        fn test_recipes_unspecified_version() -> Result<()> {
            let file = recipes_unspecified_version_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, recipes_unspecified_version());
            Ok(())
        }

        #[test]
        fn test_v6_14_ok() -> Result<()> {
            let file = recipe_v6_14_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v6_14()]);
            Ok(())
        }

        #[test]
        fn test_v6_20_ok() -> Result<()> {
            let file = recipe_v6_20_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v6_20()]);
            Ok(())
        }

        #[test]
        fn test_v7_01_ok() -> Result<()> {
            let file = recipe_v7_01_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v7_01()]);
            Ok(())
        }

        #[test]
        fn test_v7_04_ok() -> Result<()> {
            let file = recipe_v7_04_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v7_04()]);
            Ok(())
        }

        #[test]
        fn test_v7_07_ok() -> Result<()> {
            let file = recipe_v7_07_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v7_07()]);
            Ok(())
        }

        #[test]
        fn test_v8_00_ok() -> Result<()> {
            let file = recipe_v8_00_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v8_00()]);
            Ok(())
        }

        #[test]
        fn test_v8_01_ok() -> Result<()> {
            let file = recipe_v8_01_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v8_01()]);
            Ok(())
        }

        #[test]
        fn test_v8_02_ok() -> Result<()> {
            let file = recipe_v8_02_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v8_02()]);
            Ok(())
        }

        #[test]
        fn test_v8_05_ok() -> Result<()> {
            let file = recipe_v8_05_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v8_05()]);
            Ok(())
        }

        #[test]
        fn test_v8_06_ok() -> Result<()> {
            let file = recipe_v8_06_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v8_06()]);
            Ok(())
        }

        #[test]
        fn test_now_youre_cooking_v4_72() -> Result<()> {
            let file = now_youre_cooking_v4_72_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![now_youre_cooking_v4_72()]);
            Ok(())
        }

        #[test]
        fn test_cookmate_ok() -> Result<()> {
            let file = recipe_cookmate_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_cookmate()]);
            Ok(())
        }

        #[test]
        fn test_multiple_recipes_ok() -> Result<()> {
            let mut file = String::new();
            file.push_str(recipe_v8_01_file());
            file.push_str(recipe_v8_05_file());
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, vec![recipe_v8_01(), recipe_v8_05()]);
            Ok(())
        }
    }

    mod files {
        pub fn recipe_unspecified_version_file<'a>() -> &'a str {
            r##"------------- Recipe Extracted from Meal-Master (tm) Database --------------

     Title: West Haven Chocolate Cake
Categories: Chocolate Cakes Fruits Desserts
  Servings: 16

      8 oz Dates; Pitted, Chopped
      1 t  Baking Soda
      1 c  ;Boiling Water
  1 3/4 c  Flour; Unbleached, Sifted
      2 T  Cocoa; Baking
    1/2 t  Salt
      1 c  Shortening; Vegetable
      1 c  Sugar
      2 ea Eggs; Large
      6 oz Semisweet Chocolate Chips
    1/2 c  Walnuts; Chopped

  Combine dates, baking soda, and boiling water in a small bowl.  Cool to
  room temperature.  Sift together the flour, cocoa, and salt; set aside.
  Cream the shortening and sugar together in a mixing bowl until light and
  fluffy, using an electric mixer at medium speed.  Add eggs, one at a time,
  beating well after each addition.  Blend in date mixture.  Then stir in
  dry ingredients.  Pour into a greased 13 x 9 x 2-inch baking pan.  Bake in
  preheated 350 degree F. oven for 35 minutes or until cake tests done.
  Cool in pan on rack.  Cut into squares and serve with a scoop of vanilla
  ice cream on top.

-----------------------------------------------------------------------------"##
        }

        pub fn recipes_unspecified_version_file<'a>() -> &'a str {
            r##"
----- Meal-Master -----------------------

Title: Apfelkuchen
Categories: dessert
Yield: 1

    2/3 c  butter;softened
    2/3 c  sugar
      2 ts vanilla sugar
      1 pn salt
      2 dr almond extract
      2    eggs
      1 c  flour
      1 ts baking powder
    1/2 c  ground almonds
      8    apples
      1    lemon of juice
    1/3 c  currant
    1/4 c  slivered almond
      1 ts icing sugar;for dusting

Preheat oven to 350°F.

Beat the butter with a wooden spoon or mixer until creamy. Slowly add the sugar, together with the vanilla sugar, salt and almond extract, alternating with the eggs. Mix the dough until it is very foamy and the sugar has been thoroughly dissolved. Sift in the flour with the baking powder, add the ground almonds, and mix everything together quickly.

Carefully coat a springform with butter or margarine. Pour the batter in and smooth down the surface. Peel the apples, cut in half lengthwise, and remove the cores. Cut into the outer surface a few times with a knife, but donï¿œt cut all the way through. Brush apples with the lemon juice. Place the apples close together onto the batter, core side down. Soak the currants for a short time in hot water, wash thoroughly and rub dry in a cloth. Sprinkle the currants, together with the slivered almonds over the apples. Bake the cake at 350°F for 75-80 minutes or until golden brown. Remove from the oven and let cool before dusting with icing sugar.

Serve with whipped cream.
-----
----- Meal-Master -----------------------

Title: Apple & Mango Curried Chicken Salad
Categories: salad
Yield: 1

    1/2 lb  boneless skinless chicken b
           - reast
      6 tb low low-fat yogurt ;eqty  plus 2 tbsptablespoo
           - ns
      2 tb low low-fat mayonnaise;tablespoons
      2 tb salsa
      1 tb lime juice;freshly squeezed
      1 ts curry powder
      1    mango ;substitute mandarin orange
           -  slices, if desired,ripe
      1    apple ;if you would like more tan
           - g, use a less-sweet apple such as Granny Smith,a small Fiji apple
      1 sk celery;medium sized
      4 oz fresh mozzarella cheese
      6 sp cilantro
      1 c  greens ;field greens, about two sm
           - all handfuls

Cook the chicken breast in an oven preheated to 350 degrees for 20-30 minutes (until no longer pink in the center).
2
As the chicken is baking, combine the yogurt, mayonnaise, salsa, lime juice and curry powder in a medium bowl. Stir to combine.
3
Peel and de-seed the mango then chop into half-inch chunks (I haven?t tried this salad with mandarin orange slices, but I think they would make a nice substitution if mango is unavailable).
4
Remove the apple?s core and chop into one-inch chunks. Chop the celery into 1/4-1/2 inch chunks. Cut the fresh mozzarella into one-inch chunks.
5
Add the mango, apple, celery and mozzarella to the bowl. Toss to combine.
6
Finely chop the cilantro. Add the cilantro and field greens to the bowl and stir to combine.
7
Shred the cooked chicken after it has cooled (cutting the chicken into small cubes would work as well, but shredded chicken adds a nice texture to the dish). Add to the bowl and toss.
8
Serves four as a small dish or serves two as a large dish.
-----
----- Meal-Master -----------------------

Title: Apple And Sausage Stuffing
Categories: casserole
Yield: 1

      1 lb sweet Italian sausage
    1/4 c  butter
      2 c  celery;chopped
      5 c  onion;chopped
  5 1/2 c  herbed cubed stuffing mix
      8 c  tart green apples;diced cored
      1 tb dried rubbed sage
      2 ts dried thyme
    1/2 ts ground allspice

Saute sausage in heavy large skillet over medium-high heat until cooked through, crumbling sausage with back of spoon, about 10 minutes. Using slotted spoon, transfer sausage to large bowl. Add butter, onions and celery to skillet saute until onions are tender, about 15 minutes. Add apples saute until apples are tender but still hold shape, about 10 minutes. Add sage, thyme and allspice saute 1 minute. Add to sausage. Stir in stuffing mix. Season with salt and pepper. (Can be made 1 day ahead. Cover chill.)

Preheat oven to 325°F. Generously butter 5x10-inch glass baking dish. Fill center of crown roast of pork with stuffing. Transfer remaining stuffing to prepared dish. Cover with foil and bake until heated through, about 40 minutes.
-----
----- Meal-Master -----------------------

Title: Artichoke And Basil Dip
Categories: appetizers,Dip
Yield: 2 cups

      4 oz artichoke hearts;(1/2 of a 14 oz can)
      1 tb olive oil
      1 tb shallot;minced
      2 ts Garlic;minced
      1    bay leaf
      2 tb fresh thyme,(2 to 3 sprigs)
      1 c  cottage cheese
    1/4 c  fresh basil;finely chopped
           Salt and pepper

1. Heat olive oil in saute pan over medium heat. Add the shallots, garlic, bay leaf and fresh thyme. Saute, stirring until the shallots and garlic are soft. Add the artichoke hearts and stir gently to coat them with the seasonings. Remove from heat and remove the bay leaf.

2. Transfer the contents of the saute pan to a blender or food processor fitted with a metal blade. Add the cottage cheese. Process until smooth.

3. Transfer to a mixing bowl or serving bowl. Stir in the basil and season with salt and black pepper.

4. Garnish with basil leaves or thyme sprigs and serve immediately or cover and chill until ready to serve. Tip, This dip can be made up to one day ahead, through step

 3. Keep covered and chilled. Stir before serving, adjust seasoning and garnish with fresh herbs.





-----
----- Meal-Master -----------------------

Title: Apple Crisp
Categories: dessert
Yield: 8 servings

           APPLE MIXTURE---
     10 c  apples;peeled and sliced
    1/4 c  lemon juice
      1 tb lemon zest
    3/4 c  sugar
    1/2 c  Golden raisins
           MIXTURE--
  1 1/2 st butter
  1 1/4 c  all-purpose flour
  1 1/2 c  light brown sugar
  1 1/2 c  oats
      1 tb lemon zest
      1 tb ground cinnamon
      1 ts ground nutmeg
      1 ts ground cardamom
           vegetable oil

1. Preheat the oven to 350°F.

2. Place the apples in a large, shallow baking dish and toss with the lemon juice and sugar. Combine all of the ingredients for the apple mixture in a bowl.

3. In another bowl, cut the butter into the flour and stir in the remaining topping ingredients.

4. Cover with the oat mixture. Bake until the top is nicely browned and the apples are tender, about 45 minutes. Serve warm or at room temperature.


-----
----- Meal-Master -----------------------

Title: Apple Cider-Braised Chicken
Categories: meat,Chicken,holiday,Yom Kipper
Yield: 4 Serves
Contributor: Leah Koenig

      4 lb chicken;quartered
            Kosher salt and black peppe
           - r;freshly ground
      2 tb avocado oil
      3    apples
      6 sp fresh thyme ;plus 1 tbsp finely chopped
           -  fresh thyme leaves
      6 md shallots;thinly sliced
      1 c  apple cider
    1/2 c  apple cider vinegar
      2 c  chicken or vegetable broth

Sprinkle the chicken pieces with salt and pepper.  Heat 2 tablespoons of the olive oil in a large pan set over medium-high heat until shimmering.  Working in batches, brown the chicken pieces, starting skin-side down and flipping once, until browned on both sides, about 10 minutes per batch.  Add up to 2 tablespoons more oil, if needed.  Transfer the chicken to a large oven proof baking dish and top with the apples and thyme sprigs.  Preheat oven to 375F.

Meanwhile, set the pan you cooked the chicken in over medium heat.  Add the shallots, season with salt and pepper, and cook, stirring occasionally, until browned, about 5 minutes.  Add the chopped thyme and cook, stirring often, for 1 minute.s  Stir in the apple cider and cider vinegar, scraping up any browned bits at the bottom of the pan.  Raise the heat to high and cook until the liquid has reduced by half, 4 to 5 minutes.  Stir in the broth and bring to a boil, then carefully pour the braising liquid over the chicken and apples.  Cover the baking dish with aluminum foil. Braise in the oven until the chicken is fork-tender, 45-55 minutes.  Use tongs or a slotted spoon to transfer the chicken and apples to a serving platter, and let rest.

Meanwhile, if desired, make a sauce by transferring 1 1/2 cups of the braising liquid to a saucepan set over high heat.  Bring to a boil and cook, stirring often, until the liquid reduces by two-thirds, 10 to 15 minutes.  Spoon the sauce over the chicken and serve warm.
-----
----- Meal-Master -----------------------

Title: Apple Galette
Categories: nina original,culinaria,Jewish,pastry,Tarts
Yield: 2
Contributor: Nina MacDowall

      1    recipe Pate Brisee
      4    Granny Smith apples ;peeled, cored and sliced 1
           - /8 inch
      3 tb butter;softened
      3 tb butter;melted
      3 tb sugar
      2 oz apricot jam

1. Roll the chilled brisée into a rectangle to fit a 1/4 sheet pan about 1/8 inch thick. Be careful to work quickly and not over-work the dough or allow it to become warm.

2. Fold one inch of the dough over to form a border.

3. Return to refrigerator to chill for another 30-60 minutes.

4. Peel apples, cut in half and core. Slice in even 1/8 inch slices.

5. Fan apples slightly and place on parchment on baking sheet. Place 1/2 T softened butter on each apple and bake at 400°F for 15 minutes.

6. Remove from oven and cool on parchment to allow juices to return to apples.

7. Remove the rolled brisée from the refrigerator and brush a thin coat of butter over the brisée.

8. Sprinkle with 1 tbs of the sugar.

9. Arrange the apple slices over the brisée.

10. Brush the apples and dough border with the remaining butter and sprinkle with the remaining sugar.

11. Bake at 425°F until the galette begins to color, about 10-15 minutes.

12. Reduce the oven temperature to 375°F and continue to cook until the pastry is golden brown, about 20 minutes longer.

13. Slightly warm the apricot glaze and brush on warm tart when it is finished baking.
-----
----- Meal-Master -----------------------

Title: Apple-ginger Turnovers
Categories: fruit,nina original,pastry,Puff,culinaria,Puff
Yield: 4 large or 6 small turnovers
Contributor: Nina MacDowall

      2    Granny Smith apple;small dice
      1 Tb lemon juice
    1/2 ts Chinese five-spice powder
      2 Tb brown sugar
      4 ts crystallized ginger;finely minced
      1 Tb unsalted butter
           PUFF PASTRY
           EGG WASH

1. In a medium bowl, coat the apples with lemon juice.
2. Add brown sugar, five-spice powder and ginger.
3. Melt butter in skillet and add apple mixture.
4. Cook for several minutes until apples start to soften but are still slightly firm. Set aside.
5. Roll puff pastry to 1/4 inch thickness and cut into 3 or 4 inch squares.
6. Brush egg wash around perimeter.
7. Place small amount of filling on center and fold over the diagonal.
8. Crimp edges to seal and make a small slit in the top of each turnover to vent.
9. Chill for 30 minutes (or freeze.)
10. Bake at 400°F for 20 minutes or until medium golden brown.
-----
----- Meal-Master -----------------------

Title: 3-minute Italian Dressing
Categories: dressing
Yield: 4

    1/4 c  white vinegar
    1/4 c  lemon juice
      2 ts sugar
      1 ts dry mustard
      1 ts kosher salt
    1/2 ts red pepper flakes
    1/4 ts black pepper
      4 cv garlic
      1 c   neutral oil or extra-virgin
           -  olive oil
    1/3 c  Parmesan cheese;either fresh or from a can
    3/4 ts Italian seasoning;add more if needed

Place vinegar, lemon juice, sugar, mustard, salt, red pepper flakes, black pepper, and garlic in the jar of a blender and blend until smooth. While the blender is running, add the oil in a steady stream. Remove the blender jar from the blender and mix in the cheese and Italian seasoning by hand. Transfer to a storage or serving container and refrigerate for at least 1 hour before serving.
-----"##
        }

        pub fn recipe_v6_14_file<'a>() -> &'a str {
            r##"------------- Recipe Extracted from Meal-Master (tm) v6.14 ------------------

     Title: Poppin' Fresh Barbe Cups
Categories: Breads Cheese Main dish Meats Sandwiches
  Servings:  6

    3/4 lb Ground Beef; Lean
      1 tb Onion; Minced
      2 tb Brown Sugar
     12 ea Biscuits; *
    1/2 c  Barbecue Sauce; **
    3/4 c  Cheddar; Sharp, Shredded

  *    Use 1 8-oz tube of store bought biscuits, or your favorite 12 biscuit
       recipe.
  **   Use store bought sauce or your favorite recipe.
  ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

  In a skillet brown the ground beef and then drain off the excess fat.  Add
  the bbq sauce, onion and brown sugar and set aside.  Separate the biscuit
  dough into 12 pieces and place one in each of 12 ungreased muffin cups,
  pressing the dough up the sides to the edge of the cup.  Spoon the mixture
  into the cups and sprinkle with the shredded Cheddar Cheese.  Bake in a
  preheated 400 degrees F. oven for 12 minutes.  Serve hot.

  VARIATIONS:

  Use 1 13-oz can of chili beans in place of the meat mixture (or 1 13-oz
  can of baked beans, and frankfurters or hot dogs that have been cut into
  pieces) in place of the meat mixture.  You can also add green bell pepper
  or a hot pepper to the above recipe with good results.

-----------------------------------------------------------------------------"##
        }

        pub fn recipe_v6_20_file<'a>() -> &'a str {
            r##"----- Recipe in Meal-Master v6.2 Importable Format

     Title: Magic Pan Orange Almond Salad
Categories: Salads
  Servings: 6

    1/4 c  Almonds; slivered
      2 ea Onions; green, chopped
      1 ea Lettuce; romaine
      1 c  Mandarin oranges; drained 1
    1/2 c  Mushrooms; sliced (optional
      1 x  Dressing:
      1 ts Sugar
    1/2 ts Tarragon; dried
    1/3 c  Oil; vegetable
      1 x  Salt & pepper
    1/8 ts Tabasco sauce
      1 ea Egg yolk

     Shaking constantly, toast almonds in skillet over low heat till
  golden brown (about 5 minutes). Was and dry lettuce. Tear into bite
  size pieces. Place with green onions and mandarin oranges in large
  salad bowl.     Dressing: Combine all ingredients but egg and vinegar,
  add in thin stream and process till well blended. MAKES : 1 cup
  Just before serving, toss well. Leftover dressing keeps up to 1 week
  in fridge. from Best Recipes Under the Sun
-----"##
        }

        pub fn recipe_v7_01_file<'a>() -> &'a str {
            r##"MMMMM----- Recipe via Meal-Master (tm) v7.01

     Title: Old Style Enchiladas
Categories: Chili
  Servings:  4

      2 lb Hamburger
     16 oz Can tomatoes
      2    Lg. onions, chopped separate
     16 oz Can kidney beans, drained
      4 tb Chili powder (adjust to tast
      1 t  Sugar
      1 t  Salt and pepper to taste
      1    Pkg. corn tortillas
      1 lb Cheese, grated

  1-2 c Oil for cooking corn tortillas Brown hamburger and 1 chopped onion
  (add 2 cloves garlic, chopped, if desired) and drain. Add tomatoes,
  crushed, kidney beans and spices. Simmer. Heat oil, and cook tortillas to
  desired degree. (Soft seems to work best) Drain on paper towels. Put 1
  tortilla on plate, spoon "sauce" over it and sprinkle some of raw onion and
  cheese on sauce. Put on another tortilla and repeat untill large enough for
  you. Stop with layer of onion and cheese.

MMMMM"##
        }

        pub fn recipe_v7_04_file<'a>() -> &'a str {
            r##"---------- Recipe via Meal-Master (tm) v7.04

      Title: Apple Pork Chops
 Categories: Meats, French can, Benoit
   Servings:  1

      6    Pork chops
           Pork chop fat or oil
      2 ts Butter
           -salt and pepper to taste
      3    Apples-unpeeled with cores
      1 ts Sugar
           Cinnamon

  Cook the chops using melted fat trimmed from the meat and 1 tsp butter.
  (Note those concerned about their fat intake may chose to use corn oil or
  some other vegetable oil rather than the pork fat). Season to taste and set
  on hot platter. Keep warm. Slice the apples 1/2" thick and add to the pan
  with 1 tsp butter, the sugar and a few pinches of cinnamon or cloves. Cook
  over medium heat for about 10 minutes, turning once or twice until some of
  apples are browned. Arrange them around the chops and serve. Serves: 4-6

  To quote Mme. Benoit, "The apples keep the chops moist and tender. I
  sometimes use 6 to 7 apples, then I use 1 Tablespoon sugar. Serve very
  hot."
  Source" _The Canadiana Cookbook_ by Mme. Jehane Benoit

-----
"##
        }

        pub fn recipe_v7_07_file<'a>() -> &'a str {
            r##"MMMMM----- Recipe via Meal-Master (tm) v7.07

      Title: Zucchini Date Cake
 Categories: Cakes
   Servings: 10

    1/2 lb Zucchini
      1 c  Chopped dried dates
      2 ts Grated orange zest
      2 c  Flour
      2 ts Baking powder
  1 1/2 ts Soda
      2    Egg whites
      2    Eggs
      1 tb Vanilla
  1 1/4 c  Sugar
      1 c  Plain, non-fat yogurt
    1/4 c  Almonds (opt.)
    1/2 ts Salt
           Cinnamon Orange Icing
      1 c  Powdered sugar
      1 ts Ground cinnamon
      2 tb Orange juice
      1 tb Orange curacao
           Orange Glaze (opt.)
      3 tb Orange juice
      2 tb Sugar

  If dates are dry, soak to make them moist.  Chop squash in processor.  Add
  dates and orange zest.  Blend well.  Sift flour with baking powder, soda
  and salt.  Blend dry ingredients.  Beat egg. Add yogurt, sugar and vanilla.
  Add alternately dry mix and egg mix alternately to zucchini.  Pour batter
  into lightly greased and floured bundt pan.  Bake at 350 about 45 minutes,
  or til tests done.  Cool on wire rack 10 minutes.  Unmold onto serving
  platter.  If using glaze, peirce top and sides with with toothpicks.  Spoon
  glaze over cake, allowing to soak in until cake is moist but not wet.  Cool
  completely.  Drizzle icing over cake and sprinkle with almonds

  Icing:  combine powdered sugar, cinnamon, orange juice and liqueur in bowl.
  Mix til smooth.  Use immediately.

  Orange Glaze: Stir together til smooth.

  Note: Avoid using dry old dates. For slightly softer cake texture, add 2
  Tbsp melted butter to batter before folding into squash.  This will add 2
  grams fat per serving. From: The Spectator....Aug 12/92 There is yellow
  squash hidden in this cake, but you'd never guess it. It's delicate,
  faintly sweet flavor blends right in. You can use crook neck, straight neck
  or even yellow zucchini, but be sure to select young squach with soft, thin
  skins. (why not green zucchini?) This cake, with its accent of chopped
  dates, needs only a simple icing to dress it up To add extra moistness and
  a sweet citrus flavour, poke all over the warm cake and pour optional glaze
  over the cake til it soaks in.

  Adapted from a rich recipe with sour cream and pecans created by the late
  Bert Greene.

MMMMM
 "##
        }

        pub fn recipe_v8_00_file<'a>() -> &'a str {
            r##"---------- Recipe via Meal-Master (tm) v8.00

      Title: Chicken Avocado Melt
 Categories: Poultry, Main dish
      Yield: 2 servings

      2    Chicken breast halves
      1 tb Cornstarch
    1/2 ts Cumin, ground
    1/2 ts Garlic salt
    1/2    Egg; lightly beaten
    1/2 tb Water
      3 tb Cornmeal
  1 1/2 tb Oil
    1/2    Avocado; peeled, sliced
    3/4 c  Cheese, monterey jack;
           -shredded
    1/4 c  Sour cream; divided
    1/8 c  Onion, green, tops only
    1/8    Pepper, red bell; chopped
           Tomatoes, cherry
           Parsley sprigs

  Skin and bone chicken breasts. On hard surface, with meat mallet or similar
  flattening utensil, pound chicken to 1/4 in thickness. In shallow dish, mix
  together cornstarch, cumin and garlic salt. Add chicken on piece at a time,
  dredging to coat. In a small bowl, mix egg and water. Place cornmeal in
  another bowl. Dip chicken, first in egg and then in cornmeal turning to
  coat. In large frying pan, place oil and heat to medium temp.; add chicken
  and cook 2 minutes on each side. Remove chicken to shallow baking pan;
  place avocado slices over chicken and sprinkle with cheese. Bake in 350øF
  oven about 15 minutes or until fork can be inserted in chicken with ease
  and cheese melts. Top chicken with sour cream, dividing equally; sprinkle
  with chopped green onion and red pepper.

-----
 "##
        }

        pub fn recipe_v8_01_file<'a>() -> &'a str {
            r##"---------- Recipe via Meal-Master (tm) v8.01

      Title: Cannoli
 Categories: Italian, Desserts
      Yield: 16 servings

----------------------------------FILLING----------------------------------
  1 1/2 c  Whole-milk ricotta cheese;      1 1/2 c  Milk chocolate;
           - well drained                           - coarsely chopped
      3 tb Sugar                             1/4 c  Pistachio nuts;
  1 1/2 ts Cinnamon                                 - coarsely chopped

-----------------------------------DOUGH-----------------------------------
      1 c  All-purpose flour                        - or dry white wine
      1 tb Sugar                               2 c  Vegetable oil
      1 tb Butter or lard                           Colored sprinkles
      4 tb To 5 Tbl sweet Marsala wine

  In a bowl, combine all the filling ingredients and mix well. Refreigerate,
  covered, until ready to fill the cannoli shells.

  To make the dough, place the flour in a bowl or food processor. Add the
  butter or lard and sugar and mix with a fork, or pulse, until the mixture
  resembles coarse meal. Slowly add the 1/4 cup of wine and shape the mixture
  into a ball; add a little more wine if the dough appears too dry. It should
  be soft but not sticky. Knead the dough on a floured surface until smooth,
  about 10 minutes.  Wrap the dough and refrigerate for 45 minutes.

  Place the chilled dough on a floured work surface. Divide the dough in
  half.  Work with 1 piece of dough at a time; keep the remaining dough
  refrigerated. Roll the dough out to a very thin long rectangle about 14
  inches long and 3 inches wide, either by hand or using a pasta machine set
  to the finest setting.  Cut the dough into 3-inch squares. Place a cannoli
  form diagnoally across 1 square. Roll the dough up around the form so the
  points meet in the center.  Seal the points with a little water. Continue
  making cylinders until all the dough is used.

  In an electric skillet, heat the vegetable oil to 375F. Fry the cannoli 3
  or 4 at a time, turning them as they brown and blister, until golden brown
  on all sides. Drain them on brown paper. When they are cool enough to
  handle, carefully slide the cannoli off the forms.

  To serve, use a long iced tea spoon or a pastry bag without a tip to fill
  the cannoli with the ricotta cheese mixture. Dip the ends into colored
  sprinkles, arrange them on a tray, and sprinkle confectioner's sugar over
  fill the cannoli just before serving - any sooner will make the shells
  soggy.
  the tops.  Serve at once.

  NOTE:  If you prefer, you can fry the cannoli in a deep fryer. Be sure to

  This recipe from CIAO ITALIA by Mary Ann Esposito

-----
"##
        }

        pub fn recipe_v8_02_file<'a>() -> &'a str {
            r##"---------- Recipe via Meal-Master (tm) v8.02

      Title: Ziti with Asparagus Peas & Lemon Cream
 Categories: Vegetables
      Yield: 4 servings

    1/2 c  Shelled fresh peas (about
           -1/2 lb unshelled)
    1/2 lb Asparagus, trimmed, peeled
           -and cut on the diagonal
           -into 1-1/2" pieces
      8 tb Unsalted butter
  1 1/4 c  Heavy cream
      1    Juice of lemon
      1 ts Finely grated lemon rind
           Salt
           Freshly ground white pepper
    1/2 lb Ziti or tubular pasta
      3    Hearts of Bibb or butter
           -lettuce, separated into
           -leaves
    1/2 c  Freshly grated Parmesan
           -cheese

  (From "Perla Meyers' Art of Seasonal Cooking," Simon and Schuster).

  Place the peas in a vegetable steamer, set over simmering water, and steam,
  covered, for 3-5 minutes or until just tender. Remove and run under cold
  water to stop further cooking. Drain and set aside.

  Add the asparagus to the vegetable steamer. Cover and steam for 3-5 minutes
  or until just tender. Remove and run under cold water to stop further
  cooking. Drain and set aside.

  Add asparagus to the vegetable steamer. Cover and steam for 3-5 minutes or
  until just tender. Run under cold water, drain and reserve.

  In a large heavy skillet, melt the butter over medium heat and whisk in the
  cream. Bring to a boil, reduce the heat, and simmer until reduced by
  one-third. Add the lemon juice and rind. Season with salt and white pepper
  and keep warm.

  Bring plenty of salted water to a boil in a large casserole. Add the ziti
  and cook until just tender, al dente. Add 2 cups of cold water to stop
  further cooking. Drain thoroughly, and add to the lemon cream together with
  the peas, asparagus and Bibb lettuce and simmer until the sauce lightly
  coats the pasta and the lettuce has just wilted. Add the Parmesan and toss
  gently. Taste and correct the seasoning and serve at once.

  Nutritional analysis per serving: 642.3 calories; 52.6 grams total fat;
  (33.7 grams saturated fat); 10.9 grams protein; 26.5 grams carbohydrates;
  179 milligrams cholesterol; 230.5 milligrams sodium.

-----
"##
        }

        pub fn recipe_v8_05_file<'a>() -> &'a str {
            r##"---------- Recipe via Meal-Master (tm) v8.05

      Title: South of the Border Stew
 Categories: Main dish, Stew, Beef
      Yield: 6 servings

    1/4 c  butter                              1 ts salt
      2 lb boneless round steak, cubed       1/4 ts oregano
      5 ea medium zucchini, sliced thin      1/4 ts cumin
      3 c  corn                                1 c  cheddar cheese, shredded
      4 oz green chilies, chopped            1/4 c  chopped cilantro
      2 ea cloves garlic, minced

  In a large skillet, melt butter.  Brown meat, a few pieces at a time.
  Remove from skillet as they brown.  Saute zucchini in skillet 7-10
  minutes.  Return meat and add corn, chilies, garlic, salt, oregano and
  cumin.  Simmer, stirring occassionally, about 12-15 minutes or until
  meat is tender.  Stir in cheese until melted.  Garnish with chopped
  cilantro and serve.  Serves 6

-----
 "##
        }

        pub fn recipe_v8_06_file<'a>() -> &'a str {
            r##"MMMMM----- Recipe via Meal-Master (tm) v8.06

      Title: Yellow Rice & Shrimp Casserole
 Categories: Seafood, Casseroles, Ethnic, Vegetables
      Yield: 6 Servings

   0.50 c  Olive oil
   1.00 sm Onion; chopped
   1.00 sm Green pepper; chopped
   1.00    Garlic clove; minced
   1.00    Parsley sprig
   1.00 lg Ripe tomato
           - peeled, seeded & chopped
   1.00    Bay leaf
   0.25 ts Nutmeg
   0.25 ts Cumin
   0.25 ts Thyme
   1.00 pn Saffron; toasted
   1.00 lb Shrimp, raw
           - shelled, deveined
   1.00 c  -Hot water
   0.25 c  Dry white wine
   1.00 tb Lemon juice
   1.00 tb Salt
   0.50 ts Hot sauce
   2.00 c  Long grain white rice
   2.50 c  -Water
   0.50 c  Beer
           Cooked peas
           Pimiento strips
           Parsley bouquets

  Use a 3-quart casserole with lid.  An earthenware casserole is
  preferable, especially if you wish to add a touch of Spain to a
  dinner party. However, I know that good earthenware is hard to find
  today. I have 2 casseroles that I've had for 15 years.

  Heat oil in casserole.  Saute onion and pepper until transparent.  Add
  garlic, parsley, tomato, bay leaf, nutmeg, cumin and thyme.  Mix well,
  cover, and cook over low heat until mushy (about 15 minutes).  The
  saffron should be toasting on the lid in the little brown paper.

  Add the shrimp to the saute and cook until it turns pink.  Dissolve
  the saffron in the 1 cup hot water.  Combine with wine, lemon juice,
  salt and hot sauce.  Pour into casserole, stir to mix, and cook
  covered 10 minutes more.  Now add the rice and the 2 1/2 cups of
  water. Distribute ingredients well in casserole.  Bring to a quick
  boil, STIR ONCE, and place in preheated 325 degree F. oven for only
  20 minutes - NI UN MINUTO MAS! Remove from oven, uncover, and garnish
  with peas, pimientos, and parsley. Pour beer over all.  Cover again
  and allow to stand 15 minutes longer, before serving.

  Source: Clarita's Cocina - by Clarita Garcia  (ISBN: 0-942084-74-8)
  Typos provided by: Karen Mintzias

MMMMM
 "##
        }

        pub fn recipe_cookmate_file<'a>() -> &'a str {
            r##"----- Recipe via Cookmate [Meal-Master Export Format] -----

      Title: Baklava with Cooky Filling
Categories: Greek, Desserts
Tags:
      Yield: 36

           Karen Mintzias
      2    c Sweet butter 1 ts

      1    c Confectioners' sugar 4 c
           -Flour
      1    Egg yolk
      4    c Granulated sugar 1 c
           -Honey
      4    c Water Lemon juice to
           -taste
      1 lb Toasted blanched almonds *

    1/2    c Zwieback crumbs 1 lb

      4 tb Granulated sugar 1 1/2 c



*Note: Either toasted blanched almonds, or walnut meats, or half of each,
(finely chopped) may be used.
Cooky Filling: Cream the 2 cups sweet butter until light. Gradually beat
in the confectioners' sugar and continue to beat until mixture is fluffy.
Add egg yolk and vanilla or almond extract and blend well. Work in about 4
cups flour to make a medium-soft dough. Set aside and make syrup.
Syrup: In a saucepan combine the granulated sugar and water. Bring to a
boil and boil for 15 minutes or until syrup is slightly thick. Add honey
and again bring to the boiling point. Add lemon juice to taste, and cool.
Mix almonds or walnuts, or a combination of the two, with zwieback, 4 T
granulated sugar, and cinnamon. Brush 2 sheets of phyllo pastry evenly
with butter and sprinkle with nut mixture. Place 2 buttered sheets of
phyllo on top and sprinkle with nut mixture. Shape a portion of cooky
filling into a 1/2-inch-thick roll and place the roll along one edge of the
pastry sheets. Roll up loosely, cut into 2-inch slices, and place slices
in a buttered cake pan. (Continue in this method until all phyllo pastry
and/or cooky filling is used.) Brush tops of each slice with butter and
bake in a 350 F oven for 20 minutes or until lightly browned. Dip the hot
baklava slices, one at a time, in cold syrup, allowing each piece to remain
in the syrup for a few minutes.
From: "The Art of Greek Cookery" by The Women of St. Paul's Greek Orthodox
Church (Hempstead, NY)
Typed for you by Karen Mintzias


-----
"##
        }
    }

    mod results {
        use super::*;
        use recipe_schema::Recipe;
        use schema_org::Recipe;

        pub fn recipe_unspecified_version() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) Database".into()),
                name: vec!["West Haven Chocolate Cake".into()],
                recipe_category: RecipeCategory::Text("Chocolate Cakes Fruits Desserts".into()),
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("8 oz Dates; Pitted, Chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 t Baking Soda".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c ;Boiling Water".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 3/4 c Flour; Unbleached, Sifted".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 T Cocoa; Baking".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 t Salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Shortening; Vegetable".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 ea Eggs; Large".into()),
                    RecipeRecipeIngredientFieldEnum::Text("6 oz Semisweet Chocolate Chips".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 c Walnuts; Chopped".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Combine dates, baking soda, and boiling water in a small bowl. Cool to room temperature. Sift together the flour, cocoa, and salt; set aside. Cream the shortening and sugar together in a mixing bowl until light and fluffy, using an electric mixer at medium speed. Add eggs, one at a time, beating well after each addition. Blend in date mixture. Then stir in dry ingredients. Pour into a greased 13 x 9 x 2-inch baking pan. Bake in preheated 350 degree F. oven for 35 minutes or until cake tests done. Cool in pan on rack. Cut into squares and serve with a scoop of vanilla ice cream on top.".into(),
                    )],
                recipe_yield: to_yield(16),
                ..Default::default()
            }
        }

        pub fn recipes_unspecified_version() -> Vec<Recipe> {
            vec![
                Recipe {
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    name: vec!["Apfelkuchen".into()],
                    recipe_category: RecipeCategory::Text("dessert".into()),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2/3 c butter;softened".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2/3 c sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 ts vanilla sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 pn salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 dr almond extract".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c flour".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts baking powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 c ground almonds".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 apples".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lemon of juice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/3 c currant".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 c slivered almond".into()),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 ts icing sugar;for dusting".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat oven to 350°F.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Beat the butter with a wooden spoon or mixer until creamy. Slowly add the sugar, together with the vanilla sugar, salt and almond extract, alternating with the eggs. Mix the dough until it is very foamy and the sugar has been thoroughly dissolved. Sift in the flour with the baking powder, add the ground almonds, and mix everything together quickly.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Carefully coat a springform with butter or margarine. Pour the batter in and smooth down the surface. Peel the apples, cut in half lengthwise, and remove the cores. Cut into the outer surface a few times with a knife, but donï¿œt cut all the way through. Brush apples with the lemon juice. Place the apples close together onto the batter, core side down. Soak the currants for a short time in hot water, wash thoroughly and rub dry in a cloth. Sprinkle the currants, together with the slivered almonds over the apples. Bake the cake at 350°F for 75-80 minutes or until golden brown. Remove from the oven and let cool before dusting with icing sugar.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Serve with whipped cream.".into()),
                    ],
                    recipe_yield: to_yield(1),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    name: vec!["Apple & Mango Curried Chicken Salad".into()],
                    recipe_category: vec!["salad".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1/2 lb boneless skinless chicken b".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- reast".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 tb low low-fat yogurt ;eqty plus 2 tbsptablespoo".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- ns".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tb low low-fat mayonnaise;tablespoons".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tb salsa".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tb lime juice;freshly squeezed".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts curry powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 mango ;substitute mandarin orange".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- slices, if desired,ripe".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 apple ;if you would like more tan".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- g, use a less-sweet apple such as Granny Smith,a small Fiji apple".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 sk celery;medium sized".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 oz fresh mozzarella cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 sp cilantro".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c greens ;field greens, about two sm".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- all handfuls".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                        "Cook the chicken breast in an oven preheated to 350 degrees for 20-30 minutes (until no longer pink in the center). 2 As the chicken is baking, combine the yogurt, mayonnaise, salsa, lime juice and curry powder in a medium bowl. Stir to combine. 3 Peel and de-seed the mango then chop into half-inch chunks (I haven?t tried this salad with mandarin orange slices, but I think they would make a nice substitution if mango is unavailable). 4 Remove the apple?s core and chop into one-inch chunks. Chop the celery into 1/4-1/2 inch chunks. Cut the fresh mozzarella into one-inch chunks. 5 Add the mango, apple, celery and mozzarella to the bowl. Toss to combine. 6 Finely chop the cilantro. Add the cilantro and field greens to the bowl and stir to combine. 7 Shred the cooked chicken after it has cooled (cutting the chicken into small cubes would work as well, but shredded chicken adds a nice texture to the dish). Add to the bowl and toss. 8 Serves four as a small dish or serves two as a large dish.".into(),
                    )],
                    recipe_yield: to_yield(1),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    name: vec!["Apple And Sausage Stuffing".into()],
                    recipe_category: vec!["casserole".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 lb sweet Italian sausage".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 c butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 c celery;chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("5 c onion;chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("5 1/2 c herbed cubed stuffing mix".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 c tart green apples;diced cored".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tb dried rubbed sage".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 ts dried thyme".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 ts ground allspice".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Saute sausage in heavy large skillet over medium-high heat until cooked through, crumbling sausage with back of spoon, about 10 minutes. Using slotted spoon, transfer sausage to large bowl. Add butter, onions and celery to skillet saute until onions are tender, about 15 minutes. Add apples saute until apples are tender but still hold shape, about 10 minutes. Add sage, thyme and allspice saute 1 minute. Add to sausage. Stir in stuffing mix. Season with salt and pepper. (Can be made 1 day ahead. Cover chill.)".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Preheat oven to 325°F. Generously butter 5x10-inch glass baking dish. Fill center of crown roast of pork with stuffing. Transfer remaining stuffing to prepared dish. Cover with foil and bake until heated through, about 40 minutes.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(1),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    keywords: to_defined_text(["Dip"].join(",")),
                    name: vec!["Artichoke And Basil Dip".into()],
                    recipe_category: vec!["appetizers".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 oz artichoke hearts;(1/2 of a 14 oz can)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tb olive oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tb shallot;minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 ts Garlic;minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 bay leaf".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tb fresh thyme,(2 to 3 sprigs)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c cottage cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 c fresh basil;finely chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salt and pepper".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Heat olive oil in saute pan over medium heat. Add the shallots, garlic, bay leaf and fresh thyme. Saute, stirring until the shallots and garlic are soft. Add the artichoke hearts and stir gently to coat them with the seasonings. Remove from heat and remove the bay leaf.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Transfer the contents of the saute pan to a blender or food processor fitted with a metal blade. Add the cottage cheese. Process until smooth.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Transfer to a mixing bowl or serving bowl. Stir in the basil and season with salt and black pepper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Garnish with basil leaves or thyme sprigs and serve immediately or cover and chill until ready to serve. Tip, This dip can be made up to one day ahead, through step".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Keep covered and chilled. Stir before serving, adjust seasoning and garnish with fresh herbs.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    name: vec!["Apple Crisp".into()],
                    recipe_category: vec!["dessert".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("APPLE MIXTURE", vec![
                            "10 c apples;peeled and sliced",
                            "1/4 c lemon juice",
                            "1 tb lemon zest",
                            "3/4 c sugar",
                            "1/2 c Golden raisins",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("MIXTURE", vec![
                            "1 1/2 st butter",
                            "1 1/4 c all-purpose flour",
                            "1 1/2 c light brown sugar",
                            "1 1/2 c oats",
                            "1 tb lemon zest",
                            "1 tb ground cinnamon",
                            "1 ts ground nutmeg",
                            "1 ts ground cardamom",
                            "vegetable oil",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 350°F.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place the apples in a large, shallow baking dish and toss with the lemon juice and sugar. Combine all of the ingredients for the apple mixture in a bowl.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "In another bowl, cut the butter into the flour and stir in the remaining topping ingredients.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cover with the oat mixture. Bake until the top is nicely browned and the apples are tender, about 45 minutes. Serve warm or at room temperature.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(8),
                    ..Default::default()
                },
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Leah Koenig")],
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Chicken".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("holiday".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Yom Kipper".into()),
                    ],
                    name: vec!["Apple Cider-Braised Chicken".into()],
                    recipe_category: vec!["meat".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 lb chicken;quartered".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Kosher salt and black peppe".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- r;freshly ground".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tb avocado oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 apples".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 sp fresh thyme ;plus 1 tbsp finely chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- fresh thyme leaves".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 md shallots;thinly sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c apple cider".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 c apple cider vinegar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 c chicken or vegetable broth".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Sprinkle the chicken pieces with salt and pepper. Heat 2 tablespoons of the olive oil in a large pan set over medium-high heat until shimmering. Working in batches, brown the chicken pieces, starting skin-side down and flipping once, until browned on both sides, about 10 minutes per batch. Add up to 2 tablespoons more oil, if needed. Transfer the chicken to a large oven proof baking dish and top with the apples and thyme sprigs. Preheat oven to 375F.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Meanwhile, set the pan you cooked the chicken in over medium heat. Add the shallots, season with salt and pepper, and cook, stirring occasionally, until browned, about 5 minutes. Add the chopped thyme and cook, stirring often, for 1 minute.s Stir in the apple cider and cider vinegar, scraping up any browned bits at the bottom of the pan. Raise the heat to high and cook until the liquid has reduced by half, 4 to 5 minutes. Stir in the broth and bring to a boil, then carefully pour the braising liquid over the chicken and apples. Cover the baking dish with aluminum foil. Braise in the oven until the chicken is fork-tender, 45-55 minutes. Use tongs or a slotted spoon to transfer the chicken and apples to a serving platter, and let rest.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Meanwhile, if desired, make a sauce by transferring 1 1/2 cups of the braising liquid to a saucepan set over high heat. Bring to a boil and cook, stirring often, until the liquid reduces by two-thirds, 10 to 15 minutes. Spoon the sauce over the chicken and serve warm.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Nina MacDowall".into())],
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("culinaria".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Jewish".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("pastry".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Tarts".into()),
                    ],
                    name: vec!["Apple Galette".into()],
                    recipe_category: vec!["nina original".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 recipe Pate Brisee".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 Granny Smith apples ;peeled, cored and sliced 1".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- /8 inch".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 tb butter;softened".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 tb butter;melted".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 tb sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 oz apricot jam".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Roll the chilled brisée into a rectangle to fit a 1/4 sheet pan about 1/8 inch thick. Be careful to work quickly and not over-work the dough or allow it to become warm.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text("Fold one inch of the dough over to form a border.".into()),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Return to refrigerator to chill for another 30-60 minutes.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Peel apples, cut in half and core. Slice in even 1/8 inch slices.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Fan apples slightly and place on parchment on baking sheet. Place 1/2 T softened butter on each apple and bake at 400°F for 15 minutes.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Remove from oven and cool on parchment to allow juices to return to apples.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Remove the rolled brisée from the refrigerator and brush a thin coat of butter over the brisée.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text("Sprinkle with 1 tbs of the sugar.".into()),
                        RecipeRecipeInstructionFieldEnum::Text("Arrange the apple slices over the brisée.".into()),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Brush the apples and dough border with the remaining butter and sprinkle with the remaining sugar.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Bake at 425°F until the galette begins to color, about 10-15 minutes.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Reduce the oven temperature to 375°F and continue to cook until the pastry is golden brown, about 20 minutes longer.".into(),
                        ),
                        RecipeRecipeInstructionFieldEnum::Text(
                            "Slightly warm the apricot glaze and brush on warm tart when it is finished baking.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Nina MacDowall".into())],
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("nina original".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("pastry".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Puff".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("culinaria".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Puff".into()),
                    ],
                    name: vec!["Apple-ginger Turnovers".into()],
                    recipe_category: vec!["fruit".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2 Granny Smith apple;small dice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Tb lemon juice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 ts Chinese five-spice powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Tb brown sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 ts crystallized ginger;finely minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Tb unsalted butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("PUFF PASTRY".into()),
                        RecipeRecipeIngredientFieldEnum::Text("EGG WASH".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                        "In a medium bowl, coat the apples with lemon juice. 2. Add brown sugar, five-spice powder and ginger. 3. Melt butter in skillet and add apple mixture. 4. Cook for several minutes until apples start to soften but are still slightly firm. Set aside. 5. Roll puff pastry to 1/4 inch thickness and cut into 3 or 4 inch squares. 6. Brush egg wash around perimeter. 7. Place small amount of filling on center and fold over the diagonal. 8. Crimp edges to seal and make a small slit in the top of each turnover to vent. 9. Chill for 30 minutes (or freeze.) 10. Bake at 400°F for 20 minutes or until medium golden brown.".into(),
                    )],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Meal-Master".into()),
                    name: vec!["3-minute Italian Dressing".into()],
                    recipe_category: vec!["dressing".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1/4 c white vinegar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 c lemon juice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 ts sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts dry mustard".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts kosher salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 ts red pepper flakes".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 ts black pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 cv garlic".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c neutral oil or extra-virgin".into()),
                        RecipeRecipeIngredientFieldEnum::Text("- olive oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/3 c Parmesan cheese;either fresh or from a can".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3/4 ts Italian seasoning;add more if needed".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                        "Place vinegar, lemon juice, sugar, mustard, salt, red pepper flakes, black pepper, and garlic in the jar of a blender and blend until smooth. While the blender is running, add the oil in a steady stream. Remove the blender jar from the blender and mix in the cheese and Italian seasoning by hand. Transfer to a storage or serving container and refrigerate for at least 1 hour before serving.".into(),
                    )],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
            ]
        }

        pub fn recipe_v6_14() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v6.14".into()),
                name: vec!["Poppin' Fresh Barbe Cups".into()],
                recipe_category: vec!["Breads Cheese Main dish Meats Sandwiches".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("3/4 lb Ground Beef; Lean".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tb Onion; Minced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tb Brown Sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("12 ea Biscuits; *".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 c Barbecue Sauce; **".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3/4 c Cheddar; Sharp, Shredded".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text(
                        "In a skillet brown the ground beef and then drain off the excess fat. Add the bbq sauce, onion and brown sugar and set aside. Separate the biscuit dough into 12 pieces and place one in each of 12 ungreased muffin cups, pressing the dough up the sides to the edge of the cup. Spoon the mixture into the cups and sprinkle with the shredded Cheddar Cheese. Bake in a preheated 400 degrees F. oven for 12 minutes. Serve hot.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text("VARIATIONS:".into()),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Use 1 13-oz can of chili beans in place of the meat mixture (or 1 13-oz can of baked beans, and frankfurters or hot dogs that have been cut into pieces) in place of the meat mixture. You can also add green bell pepper or a hot pepper to the above recipe with good results.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::new_section("Notes", vec![
                        RecipeRecipeInstructionFieldEnum::Text("- Use 1 8-oz tube of store bought biscuits, or your favorite 12 biscuit recipe.\n"),
                        RecipeRecipeInstructionFieldEnum::Text("- Use store bought sauce or your favorite recipe.\n"),
                    ]),
                ],
                recipe_yield: to_yield(6),
                ..Default::default()
            }
        }

        pub fn recipe_v6_20() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master v6.2 Importable Format".into()),
                name: vec!["Magic Pan Orange Almond Salad".into()],
                recipe_category: vec!["Salads".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1/4 c Almonds; slivered".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 ea Onions; green, chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ea Lettuce; romaine".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Mandarin oranges; drained 1".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1/2 c Mushrooms; sliced (optional".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("1 x Dressing:".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts Sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 ts Tarragon; dried".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/3 c Oil; vegetable".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 x Salt & pepper".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/8 ts Tabasco sauce".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ea Egg yolk".into()),
                ],
                recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                    "Shaking constantly, toast almonds in skillet over low heat till golden brown (about 5 minutes). Was and dry lettuce. Tear into bite size pieces. Place with green onions and mandarin oranges in large salad bowl. Dressing: Combine all ingredients but egg and vinegar, add in thin stream and process till well blended. MAKES : 1 cup Just before serving, toss well. Leftover dressing keeps up to 1 week in fridge. from Best Recipes Under the Sun".into(),
                )],
                recipe_yield: to_yield(6),
                ..Default::default()
            }
        }

        pub fn recipe_v7_01() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v7.01".into()),
                name: vec!["Old Style Enchiladas".into()],
                recipe_category: vec!["Chili".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("2 lb Hamburger".into()),
                    RecipeRecipeIngredientFieldEnum::Text("16 oz Can tomatoes".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 Lg. onions, chopped separate".into()),
                    RecipeRecipeIngredientFieldEnum::Text("16 oz Can kidney beans, drained".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 tb Chili powder (adjust to tast".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 t Sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 t Salt and pepper to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 Pkg. corn tortillas".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 lb Cheese, grated".into()),
                ],
                recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                    "1-2 c Oil for cooking corn tortillas Brown hamburger and 1 chopped onion (add 2 cloves garlic, chopped, if desired) and drain. Add tomatoes, crushed, kidney beans and spices. Simmer. Heat oil, and cook tortillas to desired degree. (Soft seems to work best) Drain on paper towels. Put 1 tortilla on plate, spoon \"sauce\" over it and sprinkle some of raw onion and cheese on sauce. Put on another tortilla and repeat untill large enough for you. Stop with layer of onion and cheese.".into(),
                )],
                recipe_yield: to_yield(4),
                ..Default::default()
            }
        }

        pub fn recipe_v7_04() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v7.04".into()),
                keywords: ["French can", "Benoit"]
                    .into_iter()
                    .map(RecipeKeywordsFieldEnum::TextOrURL)
                    .collect(),
                name: vec!["Apple Pork Chops".into()],
                recipe_category: vec!["Meats".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("6 Pork chops".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Pork chop fat or oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 ts Butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-salt and pepper to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 Apples-unpeeled with cores".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts Sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Cinnamon".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Cook the chops using melted fat trimmed from the meat and 1 tsp butter. (Note those concerned about their fat intake may chose to use corn oil or some other vegetable oil rather than the pork fat). Season to taste and set on hot platter. Keep warm. Slice the apples 1/2\" thick and add to the pan with 1 tsp butter, the sugar and a few pinches of cinnamon or cloves. Cook over medium heat for about 10 minutes, turning once or twice until some of apples are browned. Arrange them around the chops and serve. Serves: 4-6".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "To quote Mme. Benoit, \"The apples keep the chops moist and tender. I sometimes use 6 to 7 apples, then I use 1 Tablespoon sugar. Serve very hot.\" Source\" _The Canadiana Cookbook_ by Mme. Jehane Benoit".into(),
                    ),
                ],
                recipe_yield: to_yield(1),
                ..Default::default()
            }
        }

        pub fn recipe_v7_07() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v7.07".into()),
                name: vec!["Zucchini Date Cake".into()],
                recipe_category: vec!["Cakes".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1/2 lb Zucchini".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Chopped dried dates".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 ts Grated orange zest".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 c Flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 ts Baking powder".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 1/2 ts Soda".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 Egg whites".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 Eggs".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tb Vanilla".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 1/4 c Sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Plain, non-fat yogurt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/4 c Almonds (opt.)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 ts Salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Cinnamon Orange Icing".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Powdered sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts Ground cinnamon".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tb Orange juice".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tb Orange curacao".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Orange Glaze (opt.)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 tb Orange juice".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tb Sugar".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text(
                        "If dates are dry, soak to make them moist. Chop squash in processor. Add dates and orange zest. Blend well. Sift flour with baking powder, soda and salt. Blend dry ingredients. Beat egg. Add yogurt, sugar and vanilla. Add alternately dry mix and egg mix alternately to zucchini. Pour batter into lightly greased and floured bundt pan. Bake at 350 about 45 minutes, or til tests done. Cool on wire rack 10 minutes. Unmold onto serving platter. If using glaze, peirce top and sides with with toothpicks. Spoon glaze over cake, allowing to soak in until cake is moist but not wet. Cool completely. Drizzle icing over cake and sprinkle with almonds".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Icing: combine powdered sugar, cinnamon, orange juice and liqueur in bowl. Mix til smooth. Use immediately.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text("Orange Glaze: Stir together til smooth.".into()),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Note: Avoid using dry old dates. For slightly softer cake texture, add 2 Tbsp melted butter to batter before folding into squash. This will add 2 grams fat per serving. From: The Spectator....Aug 12/92 There is yellow squash hidden in this cake, but you'd never guess it. It's delicate, faintly sweet flavor blends right in. You can use crook neck, straight neck or even yellow zucchini, but be sure to select young squach with soft, thin skins. (why not green zucchini?) This cake, with its accent of chopped dates, needs only a simple icing to dress it up To add extra moistness and a sweet citrus flavour, poke all over the warm cake and pour optional glaze over the cake til it soaks in.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Adapted from a rich recipe with sour cream and pecans created by the late Bert Greene.".into(),
                    ),
                ],
                recipe_yield: to_yield(10),
                ..Default::default()
            }
        }

        pub fn recipe_v8_00() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v8.00".into()),
                keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Main dish".into())],
                name: vec!["Chicken Avocado Melt".into()],
                recipe_category: vec!["Poultry".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("2 Chicken breast halves".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tb Cornstarch".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 ts Cumin, ground".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 ts Garlic salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 Egg; lightly beaten".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 tb Water".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 tb Cornmeal".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 1/2 tb Oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 Avocado; peeled, sliced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3/4 c Cheese, monterey jack; 1/4 c Sour cream; divided".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-shredded".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/8 c Onion, green, tops only".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/8 Pepper, red bell; chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Tomatoes, cherry".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Parsley sprigs".into()),
                ],
                recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                    "Skin and bone chicken breasts. On hard surface, with meat mallet or similar flattening utensil, pound chicken to 1/4 in thickness. In shallow dish, mix together cornstarch, cumin and garlic salt. Add chicken on piece at a time, dredging to coat. In a small bowl, mix egg and water. Place cornmeal in another bowl. Dip chicken, first in egg and then in cornmeal turning to coat. In large frying pan, place oil and heat to medium temp.; add chicken and cook 2 minutes on each side. Remove chicken to shallow baking pan; place avocado slices over chicken and sprinkle with cheese. Bake in 350øF oven about 15 minutes or until fork can be inserted in chicken with ease and cheese melts. Top chicken with sour cream, dividing equally; sprinkle with chopped green onion and red pepper.".into(),
                )],
                recipe_yield: to_yield(2),
                ..Default::default()
            }
        }

        pub fn recipe_v8_01() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v8.01".into()),
                keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Desserts".into())],
                name: vec!["Cannoli".into()],
                recipe_category: vec!["Italian".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::new_section("FILLING", vec![
                        "1 1/2 c Whole-milk ricotta cheese; - well drained",
                        "1 1/2 c Milk chocolate; - coarsely chopped",
                        "3 tb Sugar",
                        "1/4 c Pistachio nuts; - coarsely chopped",
                        "1 1/2 ts Cinnamon",
                    ]),
                    RecipeRecipeIngredientFieldEnum::new_section("DOUGH", vec![
                        "1 c All-purpose flour",
                        "- or dry white wine",
                        "1 tb Sugar",
                        "2 c Vegetable oil",
                        "1 tb Butter or lard",
                        "Colored sprinkles",
                        "4 tb To 5 Tbl sweet Marsala wine",
                    ]),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text(
                        "In a bowl, combine all the filling ingredients and mix well. Refreigerate, covered, until ready to fill the cannoli shells.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "To make the dough, place the flour in a bowl or food processor. Add the butter or lard and sugar and mix with a fork, or pulse, until the mixture resembles coarse meal. Slowly add the 1/4 cup of wine and shape the mixture into a ball; add a little more wine if the dough appears too dry. It should be soft but not sticky. Knead the dough on a floured surface until smooth, about 10 minutes. Wrap the dough and refrigerate for 45 minutes.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Place the chilled dough on a floured work surface. Divide the dough in half. Work with 1 piece of dough at a time; keep the remaining dough refrigerated. Roll the dough out to a very thin long rectangle about 14 inches long and 3 inches wide, either by hand or using a pasta machine set to the finest setting. Cut the dough into 3-inch squares. Place a cannoli form diagnoally across 1 square. Roll the dough up around the form so the points meet in the center. Seal the points with a little water. Continue making cylinders until all the dough is used.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "In an electric skillet, heat the vegetable oil to 375F. Fry the cannoli 3 or 4 at a time, turning them as they brown and blister, until golden brown on all sides. Drain them on brown paper. When they are cool enough to handle, carefully slide the cannoli off the forms.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "To serve, use a long iced tea spoon or a pastry bag without a tip to fill the cannoli with the ricotta cheese mixture. Dip the ends into colored sprinkles, arrange them on a tray, and sprinkle confectioner's sugar over fill the cannoli just before serving - any sooner will make the shells soggy. the tops. Serve at once.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "NOTE: If you prefer, you can fry the cannoli in a deep fryer. Be sure to".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text("This recipe from CIAO ITALIA by Mary Ann Esposito".into()),
                ],
                recipe_yield: to_yield(16),
                ..Default::default()
            }
        }

        pub fn recipe_v8_02() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v8.02".into()),
                name: vec!["Ziti with Asparagus Peas & Lemon Cream".into()],
                recipe_category: vec!["Vegetables".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1/2 c Shelled fresh peas (about".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-1/2 lb unshelled)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 lb Asparagus, trimmed, peeled".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-and cut on the diagonal".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-into 1-1/2\" pieces".into()),
                    RecipeRecipeIngredientFieldEnum::Text("8 tb Unsalted butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 1/4 c Heavy cream".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 Juice of lemon".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts Finely grated lemon rind".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Freshly ground white pepper".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 lb Ziti or tubular pasta".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 Hearts of Bibb or butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-lettuce, separated into".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-leaves".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 c Freshly grated Parmesan".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-cheese".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text(
                        "(From \"Perla Meyers' Art of Seasonal Cooking,\" Simon and Schuster).".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Place the peas in a vegetable steamer, set over simmering water, and steam, covered, for 3-5 minutes or until just tender. Remove and run under cold water to stop further cooking. Drain and set aside.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Add the asparagus to the vegetable steamer. Cover and steam for 3-5 minutes or until just tender. Remove and run under cold water to stop further cooking. Drain and set aside.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Add asparagus to the vegetable steamer. Cover and steam for 3-5 minutes or until just tender. Run under cold water, drain and reserve.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "In a large heavy skillet, melt the butter over medium heat and whisk in the cream. Bring to a boil, reduce the heat, and simmer until reduced by one-third. Add the lemon juice and rind. Season with salt and white pepper and keep warm.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Bring plenty of salted water to a boil in a large casserole. Add the ziti and cook until just tender, al dente. Add 2 cups of cold water to stop further cooking. Drain thoroughly, and add to the lemon cream together with the peas, asparagus and Bibb lettuce and simmer until the sauce lightly coats the pasta and the lettuce has just wilted. Add the Parmesan and toss gently. Taste and correct the seasoning and serve at once.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Nutritional analysis per serving: 642.3 calories; 52.6 grams total fat; (33.7 grams saturated fat); 10.9 grams protein; 26.5 grams carbohydrates; 179 milligrams cholesterol; 230.5 milligrams sodium.".into(),
                    ),
                ],
                recipe_yield: to_yield(4),
                ..Default::default()
            }
        }

        pub fn recipe_v8_05() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v8.05".into()),
                keywords: ["Stew", "Beef"]
                    .into_iter()
                    .map(RecipeKeywordsFieldEnum::TextOrURL)
                    .collect(),
                name: vec!["South of the Border Stew".into()],
                recipe_category: vec!["Main dish".into()],
                recipe_ingredient: Some(vec![
                    RecipeRecipeIngredientFieldEnum::Text("1/4 c butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 lb boneless round steak, cubed".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/4 ts oregano".into()),
                    RecipeRecipeIngredientFieldEnum::Text("5 ea medium zucchini, sliced thin".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/4 ts cumin".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 c corn".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c cheddar cheese, shredded".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 oz green chilies, chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/4 c chopped cilantro".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 ea cloves garlic, minced".into()),
                ]),
                recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                    "In a large skillet, melt butter. Brown meat, a few pieces at a time. Remove from skillet as they brown. Saute zucchini in skillet 7-10 minutes. Return meat and add corn, chilies, garlic, salt, oregano and cumin. Simmer, stirring occassionally, about 12-15 minutes or until meat is tender. Stir in cheese until melted. Garnish with chopped cilantro and serve. Serves 6".into(),
                )],
                recipe_yield: to_yield(6),
                ..Default::default()
            }
        }

        pub fn recipe_v8_06() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Meal-Master (tm) v8.06".into()),
                keywords: ["Casseroles", "Ethnic", "Vegetables"].into_iter().map(RecipeKeywordsFieldEnum::TextOrURL).collect(),
                name: vec!["Yellow Rice & Shrimp Casserole".into()],
                recipe_category: vec!["Seafood".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("0.50 c Olive oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 sm Onion; chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 sm Green pepper; chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 Garlic clove; minced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 Parsley sprig".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 lg Ripe tomato".into()),
                    RecipeRecipeIngredientFieldEnum::Text("- peeled, seeded & chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 Bay leaf".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 ts Nutmeg".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 ts Cumin".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 ts Thyme".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 pn Saffron; toasted".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 lb Shrimp, raw".into()),
                    RecipeRecipeIngredientFieldEnum::Text("- shelled, deveined".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 c -Hot water".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 c Dry white wine".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 tb Lemon juice".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1.00 tb Salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.50 ts Hot sauce".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2.00 c Long grain white rice".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2.50 c -Water".into()),
                    RecipeRecipeIngredientFieldEnum::Text("0.50 c Beer".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Cooked peas".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Pimiento strips".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Parsley bouquets".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Use a 3-quart casserole with lid. An earthenware casserole is preferable, especially if you wish to add a touch of Spain to a dinner party. However, I know that good earthenware is hard to find today. I have 2 casseroles that I've had for 15 years.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Heat oil in casserole. Saute onion and pepper until transparent. Add garlic, parsley, tomato, bay leaf, nutmeg, cumin and thyme. Mix well, cover, and cook over low heat until mushy (about 15 minutes). The saffron should be toasting on the lid in the little brown paper.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Add the shrimp to the saute and cook until it turns pink. Dissolve the saffron in the 1 cup hot water. Combine with wine, lemon juice, salt and hot sauce. Pour into casserole, stir to mix, and cook covered 10 minutes more. Now add the rice and the 2 1/2 cups of water. Distribute ingredients well in casserole. Bring to a quick boil, STIR ONCE, and place in preheated 325 degree F. oven for only 20 minutes - NI UN MINUTO MAS! Remove from oven, uncover, and garnish with peas, pimientos, and parsley. Pour beer over all. Cover again and allow to stand 15 minutes longer, before serving.".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "Source: Clarita's Cocina - by Clarita Garcia (ISBN: 0-942084-74-8) Typos provided by: Karen Mintzias".into(),
                    ),
                ],
                recipe_yield: to_yield(6),
                ..Default::default()
            }
        }

        pub fn now_youre_cooking_v4_72_file<'a>() -> &'a str {
            r##"----- Now You're Cooking! v4.72 [Meal-Master Export Format]

      Title: Biscotti Di Greve ( Orange Almond Biscotti)
 Categories: cookies, italian
      Yield: 48 servings

      2 c  flour; unbleached, all purp
      1 c  sugar
      1 ts baking soda salt
      2    eggs, large
      1    egg yolk, large
      1 ts vanilla
      1 tb orange zest; freshly grated
  1 1/2 c  almonds, whole; toasted
           -lightly & chopped

----------------------------------EGG WASH----------------------------------
      1    egg, large; beaten with
           -water

From the bakery in Greve, in Chianti, Italy.

In the bowl of an electric mixer, fitted with a paddle attachment, blend
the flour, the sugar, the baking soda and the salt until the mixture is
combined well. In a small bowl whisk together the whole eggs, the yolk,
thevanilla and the zest, add the mixture to the flour mixture, beating
until adough is formed and stir in the almonds.
Turn the dough out onto a lightly floured surface, knead it several
times  and halve it. Working on a large buttered and floured baking sheet,
with   floured hands form each piece of dough into a flattish log 12 inches
long  and 2 inches wide, arrange the logs at least 3 inches apart on the
sheet,  and brush them with the egg wash. Bake the logs in the middle of a
preheated 300F for 50 minutes and them cool on the baking rack for
10      minutes.
On a cutting board, cut the logs crosswise on the diagonal into 1/2
inch   thick slices, arrange the biscotti, cut sides down, on the baking
sheet andbake them, in the 300F oven for 15 minutes on each side. Transfer
the      biscotti to racks to cool and store them in airtight containers.
MAKES:    about 48 BISCOTTI

SOURCE: Gourmet, December 1992

-----
"##
        }

        pub fn now_youre_cooking_v4_72() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on(
                    "Now You're Cooking! v4.72 [Meal-Master Export Format]".into(),
                ),
                keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("italian".into())],
                name: vec!["Biscotti Di Greve ( Orange Almond Biscotti)".into()],
                recipe_category: vec!["cookies".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("2 c flour; unbleached, all purp".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts baking soda salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 eggs, large".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 egg yolk, large".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ts vanilla".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 tb orange zest; freshly grated".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("1 1/2 c almonds, whole; toasted".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-lightly & chopped".into()),
                    RecipeRecipeIngredientFieldEnum::new_section(
                        "EGG WASH",
                        vec!["1 egg, large; beaten with", "-water"],
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionFieldEnum::Text("From the bakery in Greve, in Chianti, Italy.".into()),
                    RecipeRecipeInstructionFieldEnum::Text(
                        "In the bowl of an electric mixer, fitted with a paddle attachment, blend the flour, the sugar, the baking soda and the salt until the mixture is combined well. In a small bowl whisk together the whole eggs, the yolk, thevanilla and the zest, add the mixture to the flour mixture, beating until adough is formed and stir in the almonds. Turn the dough out onto a lightly floured surface, knead it several times and halve it. Working on a large buttered and floured baking sheet, with floured hands form each piece of dough into a flattish log 12 inches long and 2 inches wide, arrange the logs at least 3 inches apart on the sheet, and brush them with the egg wash. Bake the logs in the middle of a preheated 300F for 50 minutes and them cool on the baking rack for 10 minutes. On a cutting board, cut the logs crosswise on the diagonal into 1/2 inch thick slices, arrange the biscotti, cut sides down, on the baking sheet andbake them, in the 300F oven for 15 minutes on each side. Transfer the biscotti to racks to cool and store them in airtight containers. MAKES: about 48 BISCOTTI".into(),
                    ),
                    RecipeRecipeInstructionFieldEnum::Text("SOURCE: Gourmet, December 1992".into()),
                ],
                recipe_yield: to_yield(48),
                ..Default::default()
            }
        }

        pub fn recipe_cookmate() -> Recipe {
            Recipe {
                is_based_on: to_is_based_on("Cookmate [Meal-Master Export Format]".into()),
                keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Desserts".into())],
                name: vec!["Baklava with Cooky Filling".into()],
                recipe_category: vec!["Greek".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("<section></section>".into()),
                    RecipeRecipeIngredientFieldEnum::Text("Karen Mintzias".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 c Sweet butter 1 ts".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 c Confectioners' sugar 4 c".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-Flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 Egg yolk".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 c Granulated sugar 1 c".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-Honey".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 c Water Lemon juice to".into()),
                    RecipeRecipeIngredientFieldEnum::Text("-taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 lb Toasted blanched almonds *".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 c Zwieback crumbs 1 lb".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 tb Granulated sugar 1 1/2 c".into()),
                ],
                recipe_instructions: vec![RecipeRecipeInstructionFieldEnum::Text(
                    "*Note: Either toasted blanched almonds, or walnut meats, or half of each, (finely chopped) may be used. Cooky Filling: Cream the 2 cups sweet butter until light. Gradually beat in the confectioners' sugar and continue to beat until mixture is fluffy. Add egg yolk and vanilla or almond extract and blend well. Work in about 4 cups flour to make a medium-soft dough. Set aside and make syrup. Syrup: In a saucepan combine the granulated sugar and water. Bring to a boil and boil for 15 minutes or until syrup is slightly thick. Add honey and again bring to the boiling point. Add lemon juice to taste, and cool. Mix almonds or walnuts, or a combination of the two, with zwieback, 4 T granulated sugar, and cinnamon. Brush 2 sheets of phyllo pastry evenly with butter and sprinkle with nut mixture. Place 2 buttered sheets of phyllo on top and sprinkle with nut mixture. Shape a portion of cooky filling into a 1/2-inch-thick roll and place the roll along one edge of the pastry sheets. Roll up loosely, cut into 2-inch slices, and place slices in a buttered cake pan. (Continue in this method until all phyllo pastry and/or cooky filling is used.) Brush tops of each slice with butter and bake in a 350 F oven for 20 minutes or until lightly browned. Dip the hot baklava slices, one at a time, in cold syrup, allowing each piece to remain in the syrup for a few minutes. From: \"The Art of Greek Cookery\" by The Women of St. Paul's Greek Orthodox Church (Hempstead, NY) Typed for you by Karen Mintzias".into(),
                )],
                recipe_yield: to_yield(36),
                ..Default::default()
            }
        }
    }
}
