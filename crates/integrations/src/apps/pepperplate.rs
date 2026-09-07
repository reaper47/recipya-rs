use std::borrow::Cow;
use std::io::{Read, Seek};

use itertools::Itertools;
use support::strings::SplitFirstOwned;
use winnow::Result as WResult;
use winnow::ascii::{line_ending, multispace0, multispace1, till_line_ending};
use winnow::combinator::{delimited, opt, preceded, separated};
use winnow::token::{literal, rest, take_till, take_until};
use winnow::{Parser, combinator::seq};

use schema_org::{
    AtType, Comment, DurationOrText, Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeYieldFieldEnum,
    },
};

use crate::apps::helpers::{Ingredient, Instruction, ToSections, parse_archive_helper, read_file};
use crate::helpers::to_is_based_on;
use crate::{Error, Result, apps::helpers::Parsers};

#[derive(Default)]
struct RecipeComponents<'a> {
    categories: Option<Vec<&'a str>>,
    description: Option<&'a str>,
    image: Option<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    notes: Option<Vec<&'a str>>,
    source: Option<&'a str>,
    title: &'a str,
    prep_time: Option<&'a str>,
    total_time: Option<&'a str>,
    r#yield: &'a str,
    url: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for Recipe {
    #[allow(clippy::too_many_lines)]
    fn from(r: RecipeComponents) -> Self {
        let (cat, keywords) = r.categories.unwrap_or_default().split_first_owned();

        let notes = r.notes.unwrap_or_default();

        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            comment_count: if notes.is_empty() {
                vec![]
            } else {
                vec![i32::try_from(notes.len()).unwrap_or_default()]
            },
            comment: notes
                .into_iter()
                .map(|s| Comment {
                    text: vec![s.into()],
                    ..Default::default()
                })
                .collect(),
            description: r.description.map_or(Vec::new(), |s| {
                if s.trim().is_empty() {
                    vec![]
                } else {
                    vec![RecipeDescriptionFieldEnum::Text(s.into())]
                }
            }),
            keywords: keywords
                .into_iter()
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into()))
                .collect(),
            name: vec![r.title.into()],
            image: r
                .image
                .map_or(Vec::new(), |s| vec![RecipeImageFieldEnum::URL(s.into())]),
            is_based_on: to_is_based_on(r.source.unwrap_or_default()),
            prep_time: r
                .prep_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.into())]),
            total_time: r
                .total_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.into())]),
            recipe_category: cat.map_or(Vec::new(), |s| vec![s.into()]),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            recipe_yield: vec![RecipeYieldFieldEnum::Text(r.r#yield.into())],
            url: r.url.map_or(Vec::new(), |s| vec![s.into()]),
            ..Default::default()
        }
    }
}

/// Parses a `Pepperplate` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper(
        r,
        &Parsers {
            txt: Some(parse_txt),
            ..Default::default()
        },
    )
}

/// Parses a `Pepperplate` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    Ok(vec![
        parse_txt_helper(&mut content.as_str())
            .map_err(|err| Error::Parse(err.to_string()))?
            .into(),
    ])
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_title,
        description: opt(parse_description),
        source: opt(parse_source),
        url: opt(parse_url),
        r#yield: parse_yield,
        prep_time: opt(parse_prep),
        total_time: opt(parse_total),
        categories: opt(parse_categories),
        image: opt(parse_image),
        ingredients: parse_ingredients,
        instructions: parse_instructions,
        notes: opt(parse_notes),
    }}
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Title:")
}

fn parse_description<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Description:")
}

fn parse_source<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Source:")
}

fn parse_url<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Original URL:")
}

fn parse_yield<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Yield:")
}

fn parse_prep<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Active:")
}

fn parse_total<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Total:")
}

fn parse_categories<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    delimited(
        literal("Categories:"),
        separated(1.., take_till(1.., [',', '\r', '\n']).map(str::trim), ", "),
        multispace0,
    )
    .parse_next(input)
}

fn parse_image<'s>(input: &mut &'s str) -> WResult<&'s str> {
    extract_line(input, "Image:")
}

fn extract_line<'s>(input: &mut &'s str, prefix: &'s str) -> WResult<&'s str> {
    delimited(literal(prefix), till_line_ending, multispace1)
        .map(str::trim)
        .parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    preceded(
        (literal("Ingredients:"), line_ending),
        take_until(1.., "Instructions:"),
    )
    .map(|blocks: &str| {
        blocks
            .split("\n\n")
            .flat_map(|block| {
                block
                    .split('\t')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        if s.ends_with(":]") {
                            let s = s.trim_end_matches(":]").trim_start_matches('[');
                            Ingredient::Section(Cow::Borrowed(s))
                        } else {
                            Ingredient::Line(Cow::Borrowed(s))
                        }
                    })
                    .collect_vec()
            })
            .collect()
    })
    .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    preceded(
        (literal("Instructions:"), line_ending),
        take_until(1.., "\n\n"),
    )
    .map(|blocks: &str| {
        blocks
            .split('\t')
            .map(|s| Instruction::Line(Cow::Borrowed(s.trim())))
            .collect()
    })
    .parse_next(input)
}

fn parse_notes<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    preceded((multispace0, literal("Notes:")), rest)
        .map(|block: &str| {
            block
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
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
        fn test_pepperplate_txt_recipe1() -> Result<()> {
            let buf = Cursor::new(files::txt1());

            let got = parse_txt(buf)?;

            assert_eq!(got.len(), 1);
            pretty_assertions::assert_eq!(got[0], results::recipe1());
            Ok(())
        }

        #[test]
        fn test_pepperplate_txt_recipe2() -> Result<()> {
            let buf = Cursor::new(files::txt2());

            let got = parse_txt(buf)?;

            assert_eq!(got.len(), 1);
            pretty_assertions::assert_eq!(got[0], results::recipe2());
            Ok(())
        }
    }

    mod files {
        pub fn txt1<'a>() -> &'a str {
            r"Title: Pasta with Simple Cherry Tomato Sauce
            Description:
            Source: Alexandra's Kitchen
            Original URL: https://alexandracooks.com/2020/09/16/pasta-with-simple-cherry-tomato-sauce/
            Yield: Serves 4
            Active: 15 minutes
            Total: 45 minutes
            Image: https://cdn2.pepperplate.com/recipes/0f4a2ddce6ed4eabb35bbe76e4e7056e.jpg
            Ingredients:
	1/4 cup olive oil
	1 small shallot, thinly sliced
	4 garlic cloves, thinly sliced
	kosher salt
	12 ounces pasta, such as orecchiette, see notes above
	1/3 cup pine nuts
	1/2 to 1 teaspoon crusted red pepper flakes (or less if serving to children)
	1 pound small, sweet tomatoes, halved, see notes above
	1/2 cup grated parmesan (a little over an ounce), plus more to taste
	4 ounces small mozzarella balls, see notes above
	1/2 cup fresh basil, thinly sliced or more to taste
	fresh cracked black pepper

            Instructions:
	Bring a large pot of water to a boil. Place the pine nuts in a small skillet and place over the lowest heat possible.
	Meanwhile place the olive oil, shallots, and garlic in a large skillet. Season with a pinch of salt. Turn the heat to high. As soon as you see the oil beginning to shimmer, give the shallots and garlic a stir, cover the lid, and turn the heat to low. Cook for roughly 10 minutes, or until the shallots and garlic are very soft.
	When the pasta water boils, add 1 tablespoon kosher salt. Boil the pasta till al dente, (11 to 12 minutes for the Barilla orecchiette, but check your package for the accurate time).
	Meanwhile, uncover the lid of the pan with the shallots and garlic. Add the crushed red pepper flakes and stir briefly. Raise the heat to medium, and add the tomatoes. Season with a pinch of salt. Cook for 3 to 5 minutes, stirring occasionally, until the tomatoes begin to break down. Add 1 cup of water and bring to a gentle simmer. Taste for seasoning. Add salt and pepper to taste.
	Before draining the pasta reserve a cup or so of the pasta cooking liquid. (You may not need the pasta cooking liquid, but reserve some just in case.) Drain the pasta. Do not rinse or shake. Immediately transfer drained pasta to the tomato sauce and stir to combine. Turn heat to low.
	Check on the pine nuts. Increase the heat to medium and stir constantly until the pine nuts are toasted - do not walk away from the skillet for a second. Add them to the pasta once toasted.
	Add the parmesan cheese to the pasta as well as fresh cracked pepper to taste. Taste. Season with salt to taste if necessary. Add some of the reserved pasta cooking liquid if the sauce has thickened too much - I have yet to need the reserved liquid, but I always reserve some should I need it. Add more parmesan to taste if you wish.
	When the sauce is tasting seasoned to your liking, add the mozzarella and basil, stir to combine, then serve immediately. Shave more parmesan over top if you wish. Crack pepper over top if you wish as well.

            Notes: I love a short pasta for this recipe (as opposed to spaghetti or linguini), and I particularly like the Barilla orecchiette. Conveniently, the boxes are 12 ounces, which is what you need for this recipe.
            I have only ever made this with cherry tomatoes, but I imagine any ripe, juicy tomatoes you have on hand, will work just fine.
            I like the pearl-sized mozzarella balls, packed in brine. The slightly larger-sized balls, ciliegine, work fine, too, as would simply dicing up a large ball of mozzarella.
            If making for children, consider omitting the crushed red pepper flakes or using just a pinch.

"
        }

        pub fn txt2<'a>() -> &'a str {
            r"Title: To Die For Blueberry Muffins
            Description: The beast of the century
            Source: Allrecipes.com
            Original URL: https://www.allrecipes.com/recipe/6865/to-die-for-blueberry-muffins/
            Yield: 8 large muffins
            Active: 15 mins
            Total: 35 mins
            Categories: dinner, meat, delicious
            Image: https://cdn2.pepperplate.com/recipes/8281aef3317e441aa1f81abe5ccde3bb.jpg
            Ingredients:
            [Muffins:]
	1 1/2 cups all-purpose flour
	3/4 cup white sugar
	2 teaspoons baking powder
	1/2 teaspoon salt
	1/3 cup vegetable oil
	1 large egg
	1/3 cup milk, or more as needed
	1 cup fresh blueberries

            [Crumb Topping:]
	1/2 cup white sugar
	1/3 cup all-purpose flour
	1/4 cup butter, cubed
	1 1/2 teaspoons ground cinnamon

            Instructions:
	Gather all ingredients.
	Preheat the oven to 400 degrees F (200 degrees C). Grease 8 muffin cups or line with paper liners.
	To make the muffins: Whisk flour, sugar, baking powder, and salt together in a large bowl.
	Pour oil into a small liquid measuring cup. Add egg and enough milk to reach the 1-cup mark; stir until combined.
	Pour into flour mixture and mix just until batter is combined. Fold in blueberries; set batter aside.
	To make the crumb topping: Combine sugar, flour, butter, and cinnamon in a small bowl. Mix with a fork until crumbly.
	Spoon batter into the prepared muffin cups, filling right to the top. Sprinkle with crumb topping.
	Bake in the preheated oven until a toothpick inserted in the center of a muffin comes out clean, 20 to 25 minutes. Enjoy!

            Notes: The eggs are beaten
"
        }
    }

    mod results {
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        };

        use crate::helpers::to_is_based_on;

        use super::*;

        pub fn recipe1() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Pasta with Simple Cherry Tomato Sauce".into()],
                is_based_on: to_is_based_on("Alexandra's Kitchen"),
                url: vec![
                    "https://alexandracooks.com/2020/09/16/pasta-with-simple-cherry-tomato-sauce/"
                        .into(),
                ],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("Serves 4".into())],
                prep_time: vec![DurationOrText::Text("15 minutes".into())],
                total_time: vec![DurationOrText::Text("45 minutes".into())],
                image: vec![RecipeImageFieldEnum::URL(
                    "https://cdn2.pepperplate.com/recipes/0f4a2ddce6ed4eabb35bbe76e4e7056e.jpg"
                        .into(),
                )],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1/4 cup olive oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 small shallot, thinly sliced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 garlic cloves, thinly sliced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("kosher salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("12 ounces pasta, such as orecchiette, see notes above".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/3 cup pine nuts".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 to 1 teaspoon crusted red pepper flakes (or less if serving to children)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 pound small, sweet tomatoes, halved, see notes above".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 cup grated parmesan (a little over an ounce), plus more to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("4 ounces small mozzarella balls, see notes above".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 cup fresh basil, thinly sliced or more to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("fresh cracked black pepper".into()),
                ],
                recipe_instructions: vec![
                   	RecipeRecipeInstructionsFieldEnum::Text("Bring a large pot of water to a boil. Place the pine nuts in a small skillet and place over the lowest heat possible.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Meanwhile place the olive oil, shallots, and garlic in a large skillet. Season with a pinch of salt. Turn the heat to high. As soon as you see the oil beginning to shimmer, give the shallots and garlic a stir, cover the lid, and turn the heat to low. Cook for roughly 10 minutes, or until the shallots and garlic are very soft.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("When the pasta water boils, add 1 tablespoon kosher salt. Boil the pasta till al dente, (11 to 12 minutes for the Barilla orecchiette, but check your package for the accurate time).".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Meanwhile, uncover the lid of the pan with the shallots and garlic. Add the crushed red pepper flakes and stir briefly. Raise the heat to medium, and add the tomatoes. Season with a pinch of salt. Cook for 3 to 5 minutes, stirring occasionally, until the tomatoes begin to break down. Add 1 cup of water and bring to a gentle simmer. Taste for seasoning. Add salt and pepper to taste.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Before draining the pasta reserve a cup or so of the pasta cooking liquid. (You may not need the pasta cooking liquid, but reserve some just in case.) Drain the pasta. Do not rinse or shake. Immediately transfer drained pasta to the tomato sauce and stir to combine. Turn heat to low.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Check on the pine nuts. Increase the heat to medium and stir constantly until the pine nuts are toasted - do not walk away from the skillet for a second. Add them to the pasta once toasted.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Add the parmesan cheese to the pasta as well as fresh cracked pepper to taste. Taste. Season with salt to taste if necessary. Add some of the reserved pasta cooking liquid if the sauce has thickened too much - I have yet to need the reserved liquid, but I always reserve some should I need it. Add more parmesan to taste if you wish.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("When the sauce is tasting seasoned to your liking, add the mozzarella and basil, stir to combine, then serve immediately. Shave more parmesan over top if you wish. Crack pepper over top if you wish as well.".into()),
                ],
                comment: vec![
                    Comment {
                        text: vec!["I love a short pasta for this recipe (as opposed to spaghetti or linguini), and I particularly like the Barilla orecchiette. Conveniently, the boxes are 12 ounces, which is what you need for this recipe.".into()],
                        ..Default::default()
                    },
                    Comment {
                        text: vec!["I have only ever made this with cherry tomatoes, but I imagine any ripe, juicy tomatoes you have on hand, will work just fine.".into()],
                        ..Default::default()
                    },
                    Comment {
                        text: vec!["I like the pearl-sized mozzarella balls, packed in brine. The slightly larger-sized balls, ciliegine, work fine, too, as would simply dicing up a large ball of mozzarella.".into()],
                        ..Default::default()
                    },
                    Comment {
                        text: vec!["If making for children, consider omitting the crushed red pepper flakes or using just a pinch.".into()],
                        ..Default::default()
                    }
                ],
                comment_count: vec![4],
                ..Default::default()
            }
        }

        pub fn recipe2() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["To Die For Blueberry Muffins".into()],
                description: vec![RecipeDescriptionFieldEnum::Text("The beast of the century".into())],
                is_based_on: to_is_based_on("Allrecipes.com"),
                url: vec![
                    "https://www.allrecipes.com/recipe/6865/to-die-for-blueberry-muffins/"
                        .into(),
                ],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("8 large muffins".into())],
                prep_time: vec![DurationOrText::Text("15 mins".into())],
                total_time: vec![DurationOrText::Text("35 mins".into())],
                recipe_category: vec!["dinner".into()],
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("meat".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("delicious".into()),
                ],
                image: vec![RecipeImageFieldEnum::URL(
                    "https://cdn2.pepperplate.com/recipes/8281aef3317e441aa1f81abe5ccde3bb.jpg"
                        .into(),
                )],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::new_section("Muffins", &[
                        "1 1/2 cups all-purpose flour",
                        "3/4 cup white sugar",
                        "2 teaspoons baking powder",
                        "1/2 teaspoon salt",
                        "1/3 cup vegetable oil",
                        "1 large egg",
                        "1/3 cup milk, or more as needed",
                        "1 cup fresh blueberries",
                    ]),
                    RecipeRecipeIngredientFieldEnum::new_section("Crumb Topping", &[
                        "1/2 cup white sugar",
                        "1/3 cup all-purpose flour",
                        "1/4 cup butter, cubed",
                        "1 1/2 teaspoons ground cinnamon",
                    ]),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Gather all ingredients.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 400 degrees F (200 degrees C). Grease 8 muffin cups or line with paper liners.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("To make the muffins: Whisk flour, sugar, baking powder, and salt together in a large bowl.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Pour oil into a small liquid measuring cup. Add egg and enough milk to reach the 1-cup mark; stir until combined.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Pour into flour mixture and mix just until batter is combined. Fold in blueberries; set batter aside.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("To make the crumb topping: Combine sugar, flour, butter, and cinnamon in a small bowl. Mix with a fork until crumbly.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Spoon batter into the prepared muffin cups, filling right to the top. Sprinkle with crumb topping.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Bake in the preheated oven until a toothpick inserted in the center of a muffin comes out clean, 20 to 25 minutes. Enjoy!".into()),
                ],
                comment: vec![
                    Comment {
                        text: vec!["The eggs are beaten".into()],
                        ..Default::default()
                    },
                ],
                comment_count: vec![1],
                ..Default::default()
            }
        }
    }
}
