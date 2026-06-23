use std::borrow::Cow;
use std::io::{Read, Seek};
use std::path::Path;

use winnow::Result as WResult;
use winnow::ascii::{digit1, line_ending, multispace0, multispace1, space1, till_line_ending};
use winnow::combinator::{alt, delimited, not, opt, peek, preceded, repeat, terminated};
use winnow::token::{literal, rest};
use winnow::{Parser, combinator::seq};

use schema_org::field::{
    ItemListItemListElementFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
    RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum,
};
use schema_org::{AggregateRating, AtType, Comment, Recipe, at_context};
use zip::ZipArchive;

use crate::apps::helpers::{Parsers, extract_archive_contents, update_recipe_image_paths};
use crate::{
    Error, Result,
    apps::helpers::{Ingredient, Instruction, read_file},
};

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
                    vec![Comment {
                        text: vec![
                            n.lines()
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty())
                                .collect::<Vec<_>>()
                                .join("\n\n"),
                        ],
                        ..Default::default()
                    }]
                })
                .unwrap_or_default(),
            description: r
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s.into())])
                .unwrap_or_default(),
            name: vec![r.title.into()],
            recipe_ingredient: r
                .ingredients
                .into_iter()
                .map(|ing| match ing {
                    Ingredient::Line(cow) | Ingredient::Section(cow) => {
                        RecipeRecipeIngredientFieldEnum::Text(cow.to_string())
                    }
                })
                .collect(),
            recipe_instructions: r
                .instructions
                .into_iter()
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
                            &s.to_string(),
                            Vec::<String>::new(),
                        ));
                        acc
                    }
                }),
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
    let archive = ZipArchive::new(r)?;

    let (mut recipes, images) = extract_archive_contents(
        archive,
        Parsers {
            txt: Some(parse_txt),
            ..Default::default()
        },
    )?;

    for recipe in &mut recipes {
        for image in &mut recipe.image {
            if let RecipeImageFieldEnum::URL(u) = image
                && let Some(file_name) = Path::new(u.as_str()).file_name()
                && let Some(path) = images.get(file_name.to_string_lossy().as_ref() as &str)
            {
                *u = path.to_string_lossy().into_owned();
            }
        }
    }

    update_recipe_image_paths(&mut recipes, &images);
    Ok(recipes)
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
        ..Default::default()
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

fn parse_rating<'s>(input: &mut &'s str) -> WResult<f32> {
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
    }

    mod results {
        use schema_org::{
            AggregateRating, Comment,
            field::{
                RecipeDescriptionFieldEnum, RecipeRecipeIngredientFieldEnum,
                RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum,
            },
        };

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
    }
}
