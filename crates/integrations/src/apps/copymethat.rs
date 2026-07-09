use std::borrow::Cow;
use std::io::{Read, Seek};

use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use winnow::Result as WResult;
use winnow::ascii::{digit1, line_ending, multispace0, multispace1, space1, till_line_ending};
use winnow::combinator::{alt, delimited, not, opt, peek, preceded, repeat, terminated};
use winnow::token::{literal, rest};
use winnow::{Parser, combinator::seq};

use schema_org::field::{
    ItemListItemListElementFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
    RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    RecipeRecipeYieldFieldEnum,
};
use schema_org::{
    AggregateRating, AtType, Comment, DurationOrText, Energy, Mass, NutritionInformation, Recipe,
    at_context,
};

use crate::apps::helpers::{Parsers, ToSections, parse_archive_helper};
use crate::{
    Error, Result,
    apps::helpers::{Ingredient, Instruction, read_file},
};

#[derive(Default, Deserialize)]
struct RecipeYaml {
    name: String,
    description: Option<String>,
    servings: Option<String>,
    source: Option<String>,
    rating: Option<f32>,
    image: Option<String>,
    prep_time: Option<String>,
    cook_time: Option<String>,
    notes: Option<String>,
    images: Option<Vec<String>>,
    keywords: Option<String>,
    tags: Option<Vec<String>>,
    nutrition: Option<String>,
    ingredients: Vec<String>,
    directions: Vec<String>,
}

impl From<RecipeYaml> for Recipe {
    #[allow(clippy::too_many_lines)]
    fn from(r: RecipeYaml) -> Self {
        let mut images = r.images.unwrap_or_default();
        images.extend_from_slice(r.image.map(|s| vec![s]).unwrap_or_default().as_slice());

        let mut keywords = r.tags.unwrap_or_default();
        keywords.extend_from_slice(
            r.keywords
                .as_deref()
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        keywords = keywords.into_iter().unique().collect();

        let (cat, keywords) = match keywords.as_slice() {
            [first, rest @ ..] => (
                Some(first.clone()).filter(|s| !s.trim().is_empty()),
                rest.to_vec(),
            ),
            [] => (None, vec![]),
        };

        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            aggregate_rating: r
                .rating
                .map(|n| vec![AggregateRating::new(n, 1)])
                .unwrap_or_default(),
            comment: r.notes.map(|n| vec![Comment::new(n)]).unwrap_or_default(),
            description: r
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
                .unwrap_or_default(),
            image: images.into_iter().map(RecipeImageFieldEnum::URL).collect(),
            keywords: keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect::<Vec<_>>(),
            name: vec![r.name],
            prep_time: r
                .prep_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            cook_time: r
                .cook_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            nutrition: r
                .nutrition
                .filter(|n| !n.trim().is_empty())
                .map(|n| {
                    let lines = n.lines().map(ToString::to_string).collect::<Vec<_>>();

                    let extract = |prefix: &str| {
                        lines
                            .iter()
                            .find(|l| l.starts_with(prefix))
                            .map(|s| s.trim_start_matches(prefix).trim().to_string())
                            .filter(|s| !s.starts_with("0.00"))
                    };

                    let extract_mass = |prefix: &str| {
                        extract(prefix)
                            .map(|l| vec![Mass::new(l)])
                            .unwrap_or_default()
                    };

                    let nut = NutritionInformation {
                        calories: extract("Calories:")
                            .map(|l| vec![Energy::new(l.trim().to_string())])
                            .unwrap_or_default(),
                        carbohydrate_content: extract_mass("Carbs:"),
                        cholesterol_content: extract_mass("Cholesterol:"),
                        context: at_context(),
                        fat_content: extract_mass("Total fat:"),
                        fiber_content: extract_mass("Fiber:"),
                        protein_content: extract_mass("Protein:"),
                        saturated_fat_content: extract_mass("Saturated fat:"),
                        serving_size: extract("Servings:")
                            .map(|l| vec![l.trim().to_string()])
                            .unwrap_or_default(),
                        sodium_content: extract_mass("Sodium:"),
                        sugar_content: extract_mass("Sugars:"),
                        r#type: AtType::NutritionInformation.to_opt(),
                        trans_fat_content: extract_mass("Trans fat:"),
                        unsaturated_fat_content: extract_mass("Unsaturated fat:"),
                    };
                    if nut.is_empty() { vec![] } else { vec![nut] }
                })
                .unwrap_or_default(),
            recipe_category: cat.map(|c| vec![c]).unwrap_or_default(),
            recipe_ingredient: r
                .ingredients
                .into_iter()
                .filter_map(|s| {
                    if s.trim().is_empty() {
                        None
                    } else if s.ends_with(':') {
                        Some(Ingredient::Section(Cow::Owned(
                            s.trim_end_matches(':').to_string(),
                        )))
                    } else {
                        Some(Ingredient::Line(Cow::Owned(s)))
                    }
                })
                .fold(Vec::new(), |mut acc, items| match items {
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
                }),
            recipe_instructions: r
                .directions
                .into_iter()
                .filter_map(|s| {
                    if s.trim().is_empty() {
                        None
                    } else if s.ends_with(':') {
                        Some(Instruction::Section(Cow::Owned(
                            s.trim_end_matches(':').to_string(),
                        )))
                    } else {
                        Some(Instruction::Line(Cow::Owned(s)))
                    }
                })
                .fold(Vec::new(), |mut acc, items| match items {
                    Instruction::Line(s)
                        if let Some(RecipeRecipeInstructionsFieldEnum::ItemList(list)) =
                            acc.last_mut() =>
                    {
                        list.item_list_element
                            .push(ItemListItemListElementFieldEnum::Text(s.to_string()));
                        if let Some(i) = list.number_of_items.first_mut() {
                            *i += 1;
                        }
                        acc
                    }
                    Instruction::Line(s) => {
                        acc.push(RecipeRecipeInstructionsFieldEnum::Text(s.to_string()));
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
            recipe_yield: r
                .servings
                .map(|s| vec![RecipeRecipeYieldFieldEnum::Text(s)])
                .unwrap_or_default(),
            url: r.source.map(|u| vec![u]).unwrap_or_default(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct RecipeComponents<'a> {
    description: Option<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    notes: Option<&'a str>,
    rating: Option<f32>,
    servings: Option<&'a str>,
    title: &'a str,
    url: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            aggregate_rating: r
                .rating
                .map(|n| vec![AggregateRating::new(n, 1)])
                .unwrap_or_default(),
            comment: r
                .notes
                .map(|n| {
                    vec![Comment::new(
                        n.lines()
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>()
                            .join("\n\n"),
                    )]
                })
                .unwrap_or_default(),
            description: r
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s.into())])
                .unwrap_or_default(),
            name: vec![r.title.into()],
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            recipe_yield: r
                .servings
                .map(|s| vec![RecipeRecipeYieldFieldEnum::Text(s.to_string())])
                .unwrap_or_default(),
            url: r.url.map(|u| vec![u.to_string()]).unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Parses a `Copy Me That` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper(
        r,
        &Parsers {
            html: Some(parse_html),
            txt: Some(parse_txt),
            yaml: Some(parse_yaml),
            ..Default::default()
        },
    )
}

/// Parses a `Cook'n` text file.
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
        url: opt(parse_url),
        description: opt(parse_description),
        rating: opt(parse_rating),
        servings: opt(parse_servings),
        ingredients: parse_ingredients,
        instructions: parse_instructions,
        notes: opt(parse_notes),
    }}
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, multispace1).parse_next(input)
}

fn parse_url<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(literal("Adapted from "), till_line_ending, multispace1).parse_next(input)
}

fn parse_description<'s>(input: &mut &'s str) -> WResult<&'s str> {
    peek(not(alt((
        (multispace0, literal("Adapted from")),
        (multispace0, literal("Servings")),
        (multispace0, literal("INGREDIENTS")),
    ))))
    .parse_next(input)?;

    terminated(till_line_ending, multispace0).parse_next(input)
}

fn parse_rating(input: &mut &str) -> WResult<f32> {
    delimited(literal("Rated "), digit1, (till_line_ending, multispace0))
        .parse_next(input)
        .map(|s| s.parse().unwrap_or_default())
}

fn parse_servings<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(literal("Servings: "), till_line_ending, multispace1).parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    preceded(
        (literal("INGREDIENTS"), multispace1),
        repeat(1.., parse_ingredient),
    )
    .parse_next(input)
}

fn parse_ingredient<'s>(input: &mut &'s str) -> WResult<Ingredient<'s>> {
    peek(not((multispace0, literal("STEPS")))).parse_next(input)?;

    terminated(till_line_ending, (line_ending, multispace0))
        .map(|s| Ingredient::Line(Cow::Borrowed(s)))
        .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    preceded(
        (literal("STEPS"), multispace1),
        repeat(1.., parse_instruction),
    )
    .parse_next(input)
}

fn parse_instruction<'s>(input: &mut &'s str) -> WResult<Instruction<'s>> {
    peek(not((multispace0, literal("STEPS")))).parse_next(input)?;

    alt((parse_instruction_line, parse_instruction_section)).parse_next(input)
}

fn parse_instruction_section<'s>(input: &mut &'s str) -> WResult<Instruction<'s>> {
    terminated(till_line_ending, multispace1)
        .verify_map(|s: &str| {
            s.strip_suffix(":")
                .map(|s| Instruction::Section(Cow::Borrowed(s)))
        })
        .parse_next(input)
}

fn parse_instruction_line<'s>(input: &mut &'s str) -> WResult<Instruction<'s>> {
    delimited(
        (digit1, literal(')'), space1),
        till_line_ending,
        (line_ending, multispace0),
    )
    .map(|s| Instruction::Line(Cow::Borrowed(s)))
    .parse_next(input)
}

fn parse_notes<'s>(input: &mut &'s str) -> WResult<&'s str> {
    preceded((literal("NOTES"), multispace1), rest).parse_next(input)
}

/// Parses a YAML recipe file into a [`Recipe`] struct.
pub fn parse_yaml<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let recipe: RecipeYaml =
        serde_yaml_ng::from_reader(r).map_err(|err| Error::Parse(err.to_string()))?;

    Ok(vec![recipe.into()])
}

/// Parses an HTML recipe file into a [`Recipe`] struct.
///
/// # Panics
///
/// Panics if the HTML is not valid or does not contain the expected elements.
pub fn parse_html<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;

    let sel_name = Selector::parse("#name").unwrap();
    let sel_desc = Selector::parse("#description").unwrap();
    let sel_img = Selector::parse("img.recipeImage").unwrap();
    let sel_rating = Selector::parse("#ratingValue").unwrap();
    let sel_yield = Selector::parse("#recipeYield").unwrap();
    let sel_source = Selector::parse("#original_link").unwrap();
    let sel_notes = Selector::parse("#recipeNotes li.recipeNote").unwrap();
    let sel_inredient = Selector::parse("#recipeIngredients li.recipeIngredient").unwrap();
    let sel_ins = Selector::parse("#recipeInstructions").unwrap();

    let txt = |el: ElementRef<'_>, sel: &Selector| {
        el.select(sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .map(|s| s.trim().to_string())
    };

    Ok(Html::parse_document(&buf)
        .select(&Selector::parse("div.recipe").unwrap())
        .map(|el| {
            RecipeYaml {
                name: txt(el, &sel_name).unwrap_or_default(),
                description: txt(el, &sel_desc),
                servings: txt(el, &sel_yield),
                source: el
                    .select(&sel_source)
                    .next()
                    .map(|el| el.attr("href").unwrap_or_default().to_string()),
                rating: txt(el, &sel_rating).and_then(|s| s.parse::<f32>().ok()),
                image: el.select(&sel_img).next().map(|el| {
                    el.attr("src")
                        .unwrap_or_default()
                        .trim_start_matches("images/")
                        .to_string()
                }),
                notes: {
                    let notes = el
                        .select(&sel_notes)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|el| {
                            let str = el.text().collect::<String>();
                            str.trim().to_string()
                        })
                        .collect::<Vec<_>>()
                        .join("\n\n");

                    if notes.is_empty() { None } else { Some(notes) }
                },
                ingredients: el
                    .select(&sel_inredient)
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|el| el.text().collect::<String>())
                    .collect::<Vec<_>>(),
                directions: el
                    .select(&sel_ins)
                    .next()
                    .map(|el| {
                        el.children()
                            .filter_map(ElementRef::wrap)
                            .map(|el| {
                                let txt = el.text().collect::<String>();
                                txt.trim().to_string()
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                ..Default::default()
            }
            .into()
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_recipes {
        use std::io::Cursor;

        use super::*;

        #[test]
        fn test_cmt_txt1_ok() -> Result<()> {
            let buf = Cursor::new(files::txt1());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, vec![results::txt1()]);
            Ok(())
        }

        #[test]
        fn test_cmt_txt2_ok() -> Result<()> {
            let buf = Cursor::new(files::txt2());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, vec![results::txt2()]);
            Ok(())
        }

        #[test]
        fn test_cmt_txt3_ok() -> Result<()> {
            let buf = Cursor::new(files::txt3());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got, vec![results::txt3()]);
            Ok(())
        }

        #[test]
        fn test_cmt_yaml_ok() -> Result<()> {
            let buf = Cursor::new(files::yaml());

            let got = parse_yaml(buf)?;

            pretty_assertions::assert_eq!(got, vec![results::yaml()]);
            Ok(())
        }

        #[test]
        fn test_cmt_html_ok() -> Result<()> {
            let buf = Cursor::new(files::html());

            let got = parse_html(buf)?;

            pretty_assertions::assert_eq!(got.len(), 3);
            let mut expected = results::html();
            expected[0].image = vec![RecipeImageFieldEnum::URL(
                "the_best_blueberry_pie_modjr.jpg".into(),
            )];
            expected[1].image = vec![RecipeImageFieldEnum::URL(
                "zucchini_cornbread_nd0jw.jpg".into(),
            )];
            expected[2].image = vec![RecipeImageFieldEnum::URL(
                "beef_wellington_4gaej.jpg".into(),
            )];
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }
    }

    mod files {
        pub fn txt1<'a>() -> &'a str {
            r"Zucchini Cornbread

            Adapted from https://southern-bytes.com/zucchini-cornbread-recipe/

            Servings: 9 SLICES

            INGREDIENTS

            ¾ cup yellow cornmeal
            1 ¼ cup all-purpose flour
            2 teaspoons baking powder
            ½ teaspoon baking soda
            ½ teaspoon kosher salt
            ½ cup butter melted and cooled
            ⅓ cup sugar
            2 large eggs
            ⅔ cup buttermilk
            1 cup zucchini grated and very tightly packed

            STEPS

            1) Grate your zucchini if you have not already done so and preheat the oven to 400°F.

            2) Drop about a tablespoon of butter into an 8×8 or 9×9 square baking pan or a skillet and set it in the oven as it preheats.

            3) In a large bowl, combine the flour, cornmeal, baking powder, baking soda, and salt.

            4) In another bowl, whisk the melted butter and sugar together.

            5) Combine the eggs and buttermilk, then combine this with the butter and sugar.

            6) Fold the wet ingredients into the dry ingredients, and stir until just combined.

            7) Stir in the shredded zucchini and fold gently to combine.

            8) Remove the warm baking dish from the oven and turn it so the butter runs up the sides of the pan.

            9) Pour the batter into the prepared dish and return it to the hot oven. (If the dish is glass, set it on a trivet or towel while you pour – not a cold surface or the glass can explode.)

            10) Bake for about 30 minutes, depending on the size of the pan* that you use, until the top is golden brown and a toothpick inserted into the center comes out clean. (Without wet batter on it – crumbs are okay.)

            11) Allow the cornbread to cool slightly before you slice it, then serve with honey butter.


            NOTES

            Baking Times

            25-35 minutes for a 9×9 dish, a pie pan, or a small skillet (10-inch)

            35-40 minutes for an 8×8 dish

            Buttermilk Substitute

            Add 1 tablespoon of fresh lemon juice or white vinegar to a liquid measuring cup. Fill with milk to the ⅔ line. Let sit for 10 minutes, then stir before using.

            Preheat the baking dish for extra crispy edges on your cornbread.

            Try not to overmix your cornbread or it will be tough.

            Grate zucchini with a food processor to make it easier.

            Freeze extra grated zucchini to use for later. (I like to freeze 1-2 cups of zucchini at a time.)

            Variations

            Make it Savory – Exclude the sugar and add ½ cup shredded cheddar cheese, ½ cup fresh corn, and 1-2 diced jalapeños.

            Use Different Types of Squash – This recipe works great with yellow squash!

            Storage

            Once cooled, store in an airtight container or freeze in small portions.

            "
        }

        pub fn txt2<'a>() -> &'a str {
            r"The best blueberry pie

            There is nothing like it

            Rated 4/5

            Servings: 4

            INGREDIENTS

            1 cup of blueberries
            50g of milk
            lemon juice

            STEPS

            1) Mix all ingredients together

            2) Preheat oven to 350f

            3) Cook for 1 hour


            NOTES

            Nothing in paritcular

            "
        }

        pub fn txt3<'a>() -> &'a str {
            r"Beef Wellington

            Adapted from https://marketgrow.com/beef-wellington/

            The best

            Servings: 5

            INGREDIENTS

            1 (2-pound) beef tenderloin, trimmed
            Salt and black pepper, to taste
            2 tablespoons olive oil
            2 tablespoons Dijon mustard
            1 tablespoon unsalted butter
            1 pound mushrooms (such as cremini or button), finely chopped
            2 cloves garlic, minced
            1 tablespoon fresh thyme leaves, chopped
            1/4 cup dry white wine
            8-10 slices prosciutto
            1 sheet puff pastry, thawed
            1 egg, beaten
            1 tablespoon all-purpose flour (for dusting)

            STEPS

            Sear the beef:

            1) Preheat your oven to 400°F (200°C). Season the beef tenderloin generously with salt and pepper. Heat olive oil in a large skillet over high heat. Sear the beef on all sides until browned, about 2-3 minutes per side. Remove from the skillet and brush with Dijon mustard while warm. Set aside to cool.

            Prepare the mushroom duxelles:

            1) In the same skillet, melt butter over medium heat. Add the finely chopped mushrooms, garlic, and thyme. Cook for about 10 minutes, stirring occasionally, until the mushrooms release their moisture and become dry. Add white wine and cook until it has evaporated. Season with salt and pepper. Remove from heat and let cool completely.

            Assemble the Wellington:

            1) Lay a large piece of plastic wrap on a flat surface. Arrange the prosciutto slices, slightly overlapping, to form a rectangle large enough to wrap around the beef. Spread the cooled mushroom duxelles evenly over the prosciutto. Place the beef on top and carefully roll it up tightly using the plastic wrap, twisting the ends to seal. Refrigerate for 15-20 minutes to set.

            Wrap in puff pastry:

            1) Roll out the puff pastry on a lightly floured surface to a size that will fully encase the beef. Unwrap the beef from the plastic wrap and place it in the center of the pastry. Brush the edges of the pastry with beaten egg. Fold over the pastry, trimming any excess, and seal the edges. Place seam-side down on a baking sheet. Brush the top with more beaten egg.

            Bake the Wellington:

            1) Bake in the preheated oven for 25-30 minutes, or until the pastry is golden brown and the internal temperature of the beef reaches your desired doneness (135°F/57°C for medium-rare). Remove from the oven and let rest for 10 minutes before slicing.

            Serve:

            1) Slice the Beef Wellington and serve warm, accompanied by your choice of sides such as roasted vegetables or mashed potatoes.

            2) Enjoy your Beef Wellington as a luxurious and impressive dish that’s perfect for any celebration or special dinner, delivering a blend of flavors and textures that are sure to impress.

            "
        }

        pub fn yaml<'a>() -> &'a str {
            r#"name: Berry Acai Bowl
description: Perfect healthy breakfast for easy mornings. Acai bowls are
    essentially thick smoothie bowls loaded with toppings - yum! Thanks to their
    high antioxidant content, acai berries have many potential health benefits.
    They're loaded with powerful plant compounds that act as antioxidants and
    could have benefits for your brain, heart and overall health.
servings: 1 Bowl
source: CookBook App
rating: 5
image: https://media.cookbookmanager.com/61/R9P2vnN9QyjBrmZvErppJ0IE3thtNPJWg7MWLlCrcOakJKBnS46XzxOl421ii7qh.png
prep_time: PT5M
cook_time: PT5M
notes: Keep in mind that using brown sugar will result in a slightly different
    taste and colour compared to using caster sugar. Brown sugar has a deeper,
    more caramel-like flavour, while caster sugar has a cleaner, more neutral
    sweetness.
on_favorites: yes
favorite: yes
cook_count: 0
images:
    - https://media.cookbookmanager.com/61/93CUAnLdaRci4QBdLuDl7iHgnqBqplFNjp85goax9Roq8jGoChqqCjlisrlFOC5O.png
keywords: Breakfast, Lunch, Vegetarian, Smoothies, Healthy
tags:
    - Breakfast
    - Lunch
    - Vegetarian
    - Smoothies
    - Healthy
nutrition: |-
    Servings: 1.00
    Calories: 511.65 kcal
    Carbs: 102.84 g
    Protein: 8.76 g
    Total fat: 11.82 g
    Saturated fat: 2.39 g
    Unsaturated fat: 7.58 g
    Trans fat: 0.03 g
    Sugars: 53.61 g
    Fiber: 16.26 g
    Cholesterol: 0.00 mg
    Sodium: 150.92 mg
ingredients:
    - ""
    - 1 cup milk
    - 2 tbsp baking powder
    - "Smoothie:"
    - 2 tsp acai powder
    - 1 handful blueberries
    - 1 medium banana
    - 0.75 cup almond milk, or milk of your choice - adjust recipe as required
    - "Topping:"
    - 4 strawberries, sliced
    - 1 sprinkle coconut flakes
    - sprinkle chia seeds
    - 1 medium banana, sliced
    - 1 small handful raspberries
    - 1 handful blueberries
    - 0.5 kiwi, sliced
    - 1 handful granola
directions:
    - ""
    - Put all the smoothie ingredients into a blender
    - If the mixture is having trouble blending, add more almond milk or water
    - Blend for 2-3 minutes or until the smoothie has no lumps. Pour the smoothie
      mixture into a bowl
    - Top with desired ingredients
    - Serve and enjoy!
exportedBy: |-
    Shared from CookBook
    https://cookbookmanager.com
"#
        }

        pub fn html<'a>() -> &'a str {
            r#"<!DOCTYPE html><head><meta http-equiv="Content-Type" content="text/html;charset=UTF-8"><meta name="escaped characters" content="&lt; &gt; &quot; &amp; &#39;"><meta name="copymethat_class_id_version" content="2"></head><html><style type="text/css">.recipe{margin-top:20px;border-top:solid 3px #000}#name{font-weight:700;font-size:130%;margin:10px 0}#link{margin-bottom:10px}.recipeImage{width:125px}#categories,#description,#extra_info,#recipeIngredients,#recipeInstructions,#recipeNotes,#servings{margin:10px 0}#recipeIngredient_header,#recipeInstructions_header,#recipeNotes_header{font-weight:700}.instruction_subheader,.recipeIngredient_subheader{font-weight:700;margin:5px 0}.recipeIngredient_spacer{height:10px}</style><body><div class="recipe"><div id="name">The best blueberry pie</div><img class="recipeImage" src="images/the_best_blueberry_pie_modjr.jpg" width="120px"><div id="description">There is nothing like it</div><div id="extra_info"><span id="rating">Rated<span id="ratingValue">4</span>/5</span></div><div id="servings">Servings:<a id="recipeYield">4</a></div><div id="recipeIngredient_header">Ingredients</div><ul id="recipeIngredients"><li class="recipeIngredient">1 cup of blueberries</li><li class="recipeIngredient">50g of milk</li><li class="recipeIngredient">lemon juice</li></ul><div id="recipeInstructions_header">Steps</div><ol id="recipeInstructions"><li class="instruction" value="1">Mix all ingredients together</li><li class="instruction" value="2">Preheat oven to 350f</li><li class="instruction" value="3">Cook for 1 hour</li></ol><div id="recipeNotes_header">Notes</div><ul id="recipeNotes"><li class="recipeNote">Nothing in paritcular</li></ul></div><div class="recipe"><div id="name">Zucchini Cornbread</div><div id="link">Adapted from<a id="original_link" href="https://southern-bytes.com/zucchini-cornbread-recipe/">https://southern-bytes.com/zucchini-cornbread-recipe/</a></div><img class="recipeImage" src="images/zucchini_cornbread_nd0jw.jpg" width="120px"><div id="servings">Servings:<a id="recipeYield">9 SLICES</a></div><div id="recipeIngredient_header">Ingredients</div><ul id="recipeIngredients"><li class="recipeIngredient">¾ cup yellow cornmeal</li><li class="recipeIngredient">1 ¼ cup all-purpose flour</li><li class="recipeIngredient">2 teaspoons baking powder</li><li class="recipeIngredient">½ teaspoon baking soda</li><li class="recipeIngredient">½ teaspoon kosher salt</li><li class="recipeIngredient">½ cup butter melted and cooled</li><li class="recipeIngredient">⅓ cup sugar</li><li class="recipeIngredient">2 large eggs</li><li class="recipeIngredient">⅔ cup buttermilk</li><li class="recipeIngredient">1 cup zucchini grated and very tightly packed</li></ul><div id="recipeInstructions_header">Steps</div><ol id="recipeInstructions"><li class="instruction" value="1">Grate your zucchini if you have not already done so and preheat the oven to 400°F.</li><li class="instruction" value="2">Drop about a tablespoon of butter into an 8×8 or 9×9 square baking pan or a skillet and set it in the oven as it preheats.</li><li class="instruction" value="3">In a large bowl, combine the flour, cornmeal, baking powder, baking soda, and salt.</li><li class="instruction" value="4">In another bowl, whisk the melted butter and sugar together.</li><li class="instruction" value="5">Combine the eggs and buttermilk, then combine this with the butter and sugar.</li><li class="instruction" value="6">Fold the wet ingredients into the dry ingredients, and stir until just combined.</li><li class="instruction" value="7">Stir in the shredded zucchini and fold gently to combine.</li><li class="instruction" value="8">Remove the warm baking dish from the oven and turn it so the butter runs up the sides of the pan.</li><li class="instruction" value="9">Pour the batter into the prepared dish and return it to the hot oven. (If the dish is glass, set it on a trivet or towel while you pour – not a cold surface or the glass can explode.)</li><li class="instruction" value="10">Bake for about 30 minutes, depending on the size of the pan* that you use, until the top is golden brown and a toothpick inserted into the center comes out clean. (Without wet batter on it – crumbs are okay.)</li><li class="instruction" value="11">Allow the cornbread to cool slightly before you slice it, then serve with honey butter.</li></ol><div id="recipeNotes_header">Notes</div><ul id="recipeNotes"><li class="recipeNote">Baking Times</li><li class="recipeNote">25-35 minutes for a 9×9 dish, a pie pan, or a small skillet (10-inch)</li><li class="recipeNote">35-40 minutes for an 8×8 dish</li><li class="recipeNote">Buttermilk Substitute</li><li class="recipeNote">Add 1 tablespoon of fresh lemon juice or white vinegar to a liquid measuring cup. Fill with milk to the ⅔ line. Let sit for 10 minutes, then stir before using.</li><li class="recipeNote">Preheat the baking dish for extra crispy edges on your cornbread.</li><li class="recipeNote">Try not to overmix your cornbread or it will be tough.</li><li class="recipeNote">Grate zucchini with a food processor to make it easier.</li><li class="recipeNote">Freeze extra grated zucchini to use for later. (I like to freeze 1-2 cups of zucchini at a time.)</li><li class="recipeNote">Variations</li><li class="recipeNote">Make it Savory – Exclude the sugar and add ½ cup shredded cheddar cheese, ½ cup fresh corn, and 1-2 diced jalapeños.</li><li class="recipeNote">Use Different Types of Squash – This recipe works great with yellow squash!</li><li class="recipeNote">Storage</li><li class="recipeNote">Once cooled, store in an airtight container or freeze in small portions.</li></ul></div><div class="recipe"><div id="name">Beef Wellington</div><div id="link">Adapted from<a id="original_link" href="https://marketgrow.com/beef-wellington/">https://marketgrow.com/beef-wellington/</a></div><img class="recipeImage" src="images/beef_wellington_4gaej.jpg" width="120px"><div id="description">The best</div><div id="servings">Servings:<a id="recipeYield">5</a></div><div id="recipeIngredient_header">Ingredients</div><ul id="recipeIngredients"><li class="recipeIngredient">1 (2-pound) beef tenderloin, trimmed</li><li class="recipeIngredient">Salt and black pepper, to taste</li><li class="recipeIngredient">2 tablespoons olive oil</li><li class="recipeIngredient">2 tablespoons Dijon mustard</li><li class="recipeIngredient">1 tablespoon unsalted butter</li><li class="recipeIngredient">1 pound mushrooms (such as cremini or button), finely chopped</li><li class="recipeIngredient">2 cloves garlic, minced</li><li class="recipeIngredient">1 tablespoon fresh thyme leaves, chopped</li><li class="recipeIngredient">1/4 cup dry white wine</li><li class="recipeIngredient">8-10 slices prosciutto</li><li class="recipeIngredient">1 sheet puff pastry, thawed</li><li class="recipeIngredient">1 egg, beaten</li><li class="recipeIngredient">1 tablespoon all-purpose flour (for dusting)</li></ul><div id="recipeInstructions_header">Steps</div><ol id="recipeInstructions"><div class="instruction instruction_subheader">Sear the beef:</div><li class="instruction" value="1">Preheat your oven to 400°F (200°C). Season the beef tenderloin generously with salt and pepper. Heat olive oil in a large skillet over high heat. Sear the beef on all sides until browned, about 2-3 minutes per side. Remove from the skillet and brush with Dijon mustard while warm. Set aside to cool.</li><div class="instruction instruction_subheader">Prepare the mushroom duxelles:</div><li class="instruction" value="1">In the same skillet, melt butter over medium heat. Add the finely chopped mushrooms, garlic, and thyme. Cook for about 10 minutes, stirring occasionally, until the mushrooms release their moisture and become dry. Add white wine and cook until it has evaporated. Season with salt and pepper. Remove from heat and let cool completely.</li><div class="instruction instruction_subheader">Assemble the Wellington:</div><li class="instruction" value="1">Lay a large piece of plastic wrap on a flat surface. Arrange the prosciutto slices, slightly overlapping, to form a rectangle large enough to wrap around the beef. Spread the cooled mushroom duxelles evenly over the prosciutto. Place the beef on top and carefully roll it up tightly using the plastic wrap, twisting the ends to seal. Refrigerate for 15-20 minutes to set.</li><div class="instruction instruction_subheader">Wrap in puff pastry:</div><li class="instruction" value="1">Roll out the puff pastry on a lightly floured surface to a size that will fully encase the beef. Unwrap the beef from the plastic wrap and place it in the center of the pastry. Brush the edges of the pastry with beaten egg. Fold over the pastry, trimming any excess, and seal the edges. Place seam-side down on a baking sheet. Brush the top with more beaten egg.</li><div class="instruction instruction_subheader">Bake the Wellington:</div><li class="instruction" value="1">Bake in the preheated oven for 25-30 minutes, or until the pastry is golden brown and the internal temperature of the beef reaches your desired doneness (135°F/57°C for medium-rare). Remove from the oven and let rest for 10 minutes before slicing.</li><div class="instruction instruction_subheader">Serve:</div><li class="instruction" value="1">Slice the Beef Wellington and serve warm, accompanied by your choice of sides such as roasted vegetables or mashed potatoes.</li><li class="instruction" value="2">Enjoy your Beef Wellington as a luxurious and impressive dish that’s perfect for any celebration or special dinner, delivering a blend of flavors and textures that are sure to impress.</li></ol></div></body></html>"#
        }
    }

    mod results {
        use super::*;

        pub fn txt1() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Zucchini Cornbread".into()],
                comment:vec![Comment {
                    text: vec!["Baking Times\n\n25-35 minutes for a 9×9 dish, a pie pan, or a small skillet (10-inch)\n\n35-40 minutes for an 8×8 dish\n\nButtermilk Substitute\n\nAdd 1 tablespoon of fresh lemon juice or white vinegar to a liquid measuring cup. Fill with milk to the ⅔ line. Let sit for 10 minutes, then stir before using.\n\nPreheat the baking dish for extra crispy edges on your cornbread.\n\nTry not to overmix your cornbread or it will be tough.\n\nGrate zucchini with a food processor to make it easier.\n\nFreeze extra grated zucchini to use for later. (I like to freeze 1-2 cups of zucchini at a time.)\n\nVariations\n\nMake it Savory – Exclude the sugar and add ½ cup shredded cheddar cheese, ½ cup fresh corn, and 1-2 diced jalapeños.\n\nUse Different Types of Squash – This recipe works great with yellow squash!\n\nStorage\n\nOnce cooled, store in an airtight container or freeze in small portions.".into()],
                    ..Default::default()
                }],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("¾ cup yellow cornmeal".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 ¼ cup all-purpose flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 teaspoons baking powder".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ teaspoon baking soda".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ teaspoon kosher salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ cup butter melted and cooled".into()),
                    RecipeRecipeIngredientFieldEnum::Text("⅓ cup sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 large eggs".into()),
                    RecipeRecipeIngredientFieldEnum::Text("⅔ cup buttermilk".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 cup zucchini grated and very tightly packed".into(),
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Grate your zucchini if you have not already done so and preheat the oven to 400°F.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Drop about a tablespoon of butter into an 8×8 or 9×9 square baking pan or a skillet and set it in the oven as it preheats.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("In a large bowl, combine the flour, cornmeal, baking powder, baking soda, and salt.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("In another bowl, whisk the melted butter and sugar together.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Combine the eggs and buttermilk, then combine this with the butter and sugar.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Fold the wet ingredients into the dry ingredients, and stir until just combined.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Stir in the shredded zucchini and fold gently to combine.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Remove the warm baking dish from the oven and turn it so the butter runs up the sides of the pan.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Pour the batter into the prepared dish and return it to the hot oven. (If the dish is glass, set it on a trivet or towel while you pour – not a cold surface or the glass can explode.)".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Bake for about 30 minutes, depending on the size of the pan* that you use, until the top is golden brown and a toothpick inserted into the center comes out clean. (Without wet batter on it – crumbs are okay.)".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Allow the cornbread to cool slightly before you slice it, then serve with honey butter.".into()),
                ],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("9 SLICES".into())],
                url: vec!["https://southern-bytes.com/zucchini-cornbread-recipe/".into()],
                ..Default::default()
            }
        }

        pub fn txt2() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["The best blueberry pie".into()],
                aggregate_rating: vec![AggregateRating::new(4.0, 1)],
                comment: vec![Comment {
                    text: vec!["Nothing in paritcular".into()],
                    ..Default::default()
                }],
                description: vec![RecipeDescriptionFieldEnum::Text(
                    "There is nothing like it".into(),
                )],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1 cup of blueberries".into()),
                    RecipeRecipeIngredientFieldEnum::Text("50g of milk".into()),
                    RecipeRecipeIngredientFieldEnum::Text("lemon juice".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Mix all ingredients together".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Preheat oven to 350f".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Cook for 1 hour".into()),
                ],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4".into())],
                ..Default::default()
            }
        }

        pub fn txt3() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                description: vec![RecipeDescriptionFieldEnum::Text("The best".into())],
                name: vec!["Beef Wellington".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 (2-pound) beef tenderloin, trimmed".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("Salt and black pepper, to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tablespoons olive oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tablespoons Dijon mustard".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tablespoon unsalted butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 pound mushrooms (such as cremini or button), finely chopped".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("2 cloves garlic, minced".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 tablespoon fresh thyme leaves, chopped".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("1/4 cup dry white wine".into()),
                    RecipeRecipeIngredientFieldEnum::Text("8-10 slices prosciutto".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 sheet puff pastry, thawed".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 egg, beaten".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 tablespoon all-purpose flour (for dusting)".into(),
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::new_section(
                        "Sear the beef",
                        vec![
                            "Preheat your oven to 400°F (200°C). Season the beef tenderloin generously with salt and pepper. Heat olive oil in a large skillet over high heat. Sear the beef on all sides until browned, about 2-3 minutes per side. Remove from the skillet and brush with Dijon mustard while warm. Set aside to cool.",
                        ],
                    ),
                    RecipeRecipeInstructionsFieldEnum::new_section(
                        "Prepare the mushroom duxelles",
                        vec![
                            "In the same skillet, melt butter over medium heat. Add the finely chopped mushrooms, garlic, and thyme. Cook for about 10 minutes, stirring occasionally, until the mushrooms release their moisture and become dry. Add white wine and cook until it has evaporated. Season with salt and pepper. Remove from heat and let cool completely.",
                        ],
                    ),
                    RecipeRecipeInstructionsFieldEnum::new_section(
                        "Assemble the Wellington",
                        vec![
                            "Lay a large piece of plastic wrap on a flat surface. Arrange the prosciutto slices, slightly overlapping, to form a rectangle large enough to wrap around the beef. Spread the cooled mushroom duxelles evenly over the prosciutto. Place the beef on top and carefully roll it up tightly using the plastic wrap, twisting the ends to seal. Refrigerate for 15-20 minutes to set.",
                        ],
                    ),
                    RecipeRecipeInstructionsFieldEnum::new_section(
                        "Wrap in puff pastry",
                        vec![
                            "Roll out the puff pastry on a lightly floured surface to a size that will fully encase the beef. Unwrap the beef from the plastic wrap and place it in the center of the pastry. Brush the edges of the pastry with beaten egg. Fold over the pastry, trimming any excess, and seal the edges. Place seam-side down on a baking sheet. Brush the top with more beaten egg.",
                        ],
                    ),
                    RecipeRecipeInstructionsFieldEnum::new_section(
                        "Bake the Wellington",
                        vec![
                            "Bake in the preheated oven for 25-30 minutes, or until the pastry is golden brown and the internal temperature of the beef reaches your desired doneness (135°F/57°C for medium-rare). Remove from the oven and let rest for 10 minutes before slicing.",
                        ],
                    ),
                    RecipeRecipeInstructionsFieldEnum::new_section(
                        "Serve",
                        vec![
                            "Slice the Beef Wellington and serve warm, accompanied by your choice of sides such as roasted vegetables or mashed potatoes.",
                            "Enjoy your Beef Wellington as a luxurious and impressive dish that’s perfect for any celebration or special dinner, delivering a blend of flavors and textures that are sure to impress.",
                        ],
                    ),
                ],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("5".into())],
                url: vec!["https://marketgrow.com/beef-wellington/".into()],
                ..Default::default()
            }
        }

        pub fn yaml() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Berry Acai Bowl".into()],
                aggregate_rating: vec![AggregateRating::new(5.0, 1)],
                comment: vec![Comment {
                    text: vec!["Keep in mind that using brown sugar will result in a slightly different taste and colour compared to using caster sugar. Brown sugar has a deeper, more caramel-like flavour, while caster sugar has a cleaner, more neutral sweetness.".into()],
                    ..Default::default()
                }],
                description: vec![RecipeDescriptionFieldEnum::Text(
                    "Perfect healthy breakfast for easy mornings. Acai bowls are essentially thick smoothie bowls loaded with toppings - yum! Thanks to their high antioxidant content, acai berries have many potential health benefits. They're loaded with powerful plant compounds that act as antioxidants and could have benefits for your brain, heart and overall health.".into(),
                )],
                image: vec![
                    RecipeImageFieldEnum::URL("https://media.cookbookmanager.com/61/93CUAnLdaRci4QBdLuDl7iHgnqBqplFNjp85goax9Roq8jGoChqqCjlisrlFOC5O.png".into()),
                    RecipeImageFieldEnum::URL("https://media.cookbookmanager.com/61/R9P2vnN9QyjBrmZvErppJ0IE3thtNPJWg7MWLlCrcOakJKBnS46XzxOl421ii7qh.png".into()),
                ],
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("Lunch".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Vegetarian".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Smoothies".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Healthy".into()),
                ],
                prep_time: vec![DurationOrText::Text("PT5M".into())],
                cook_time: vec![DurationOrText::Text("PT5M".into())],
                nutrition: vec![NutritionInformation {
                    calories: vec![Energy::new("511.65 kcal")],
                    carbohydrate_content: vec![Mass::new("102.84 g")],
                    cholesterol_content: vec![],
                    context: at_context(),
                    fat_content: vec![Mass::new("11.82 g")],
                    fiber_content: vec![Mass::new("16.26 g")],
                    protein_content: vec![Mass::new("8.76 g")],
                    saturated_fat_content: vec![Mass::new("2.39 g")],
                    serving_size: vec!["1.00".into()],
                    sodium_content: vec![Mass::new("150.92 mg")],
                    sugar_content: vec![Mass::new("53.61 g")],
                    r#type: AtType::NutritionInformation.to_opt(),
                    trans_fat_content: vec![Mass::new("0.03 g")],
                    unsaturated_fat_content: vec![Mass::new("7.58 g")],
                }],
                recipe_category: vec!["Breakfast".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("1 cup milk".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 tbsp baking powder".into()),
                    RecipeRecipeIngredientFieldEnum::new_section("Smoothie", &[
                        "2 tsp acai powder",
                        "1 handful blueberries",
                        "1 medium banana",
                        "0.75 cup almond milk, or milk of your choice - adjust recipe as required",
                    ]),
                    RecipeRecipeIngredientFieldEnum::new_section("Topping", &[
                        "4 strawberries, sliced",
                        "1 sprinkle coconut flakes",
                        "sprinkle chia seeds",
                        "1 medium banana, sliced",
                        "1 small handful raspberries",
                        "1 handful blueberries",
                        "0.5 kiwi, sliced",
                        "1 handful granola",
                    ]),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Put all the smoothie ingredients into a blender".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("If the mixture is having trouble blending, add more almond milk or water".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Blend for 2-3 minutes or until the smoothie has no lumps. Pour the smoothie mixture into a bowl".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Top with desired ingredients".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Serve and enjoy!".into()),
                ],
                url: vec!["CookBook App".into()],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("1 Bowl".into())],
                ..Default::default()
            }
        }

        pub fn html() -> Vec<Recipe> {
            vec![txt2(), txt1(), txt3()]
        }
    }
}
