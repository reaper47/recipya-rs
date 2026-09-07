//! Parses the REZCONV file format.
//!
//! The following versions are supported:
//!     - 'Kalorio V4.03' nach REZKONV
//!     - Recipe via Cookmate [REZKONV Export Format]

use std::borrow::Cow;
use std::io::{Read, Seek};

use itertools::Itertools;
use support::strings::SplitFirstOwned;
use winnow::Result as WResult;
use winnow::ascii::{alphanumeric1, dec_int, line_ending, multispace0, space0, space1};
use winnow::combinator::{
    alt, delimited, opt, peek, preceded, repeat, repeat_till, separated, seq, terminated,
};
use winnow::prelude::*;
use winnow::token::{literal, take_till, take_until, take_while};

use schema_org::field::{RecipeAuthorFieldEnum, RecipeKeywordsFieldEnum};
use schema_org::{AtType, DurationOrText, Energy, Mass, NutritionInformation, Recipe, at_context};

use crate::apps::helpers::{Ingredient, Instruction, ToSections, read_file};
use crate::helpers::{to_is_based_on, to_yield};
use crate::{Error, Result};

#[derive(Default)]
struct RecipeComponents<'a> {
    software_version: &'a str,
    title: &'a str,
    category: Vec<&'a str>,
    r#yield: i16,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    prep_time: Option<&'a str>,
    nutrition: Option<NutritionComponents<'a>>,
    keywords: Vec<&'a str>,
    author: Option<&'a str>,
    #[allow(unused)]
    erfasst: Option<&'a str>,
}

struct NutritionComponents<'a> {
    calories: Option<&'a str>,
    carbohydrate_content: Option<&'a str>,
    cholesterol_content: Option<&'a str>,
    fat_content: Option<&'a str>,
    fiber_content: Option<&'a str>,
    protein_content: Option<&'a str>,
    saturated_fat_content: Option<&'a str>,
    sodium_content: Option<&'a str>,
    sugar_content: Option<&'a str>,
    trans_fat_content: Option<&'a str>,
    unsaturated_fat_content: Option<&'a str>,
}

impl NutritionComponents<'_> {
    const fn is_empty(&self) -> bool {
        self.calories.is_none()
            && self.carbohydrate_content.is_none()
            && self.cholesterol_content.is_none()
            && self.fat_content.is_none()
            && self.fiber_content.is_none()
            && self.protein_content.is_none()
            && self.saturated_fat_content.is_none()
            && self.sodium_content.is_none()
            && self.sugar_content.is_none()
            && self.trans_fat_content.is_none()
            && self.unsaturated_fat_content.is_none()
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
            calories: n.calories.map(|s| vec![Energy::new(s)]).unwrap_or_default(),
            carbohydrate_content: to_mass(n.carbohydrate_content),
            cholesterol_content: to_mass(n.cholesterol_content),
            context: at_context(),
            fat_content: to_mass(n.fat_content),
            fiber_content: to_mass(n.fiber_content),
            protein_content: to_mass(n.protein_content),
            saturated_fat_content: to_mass(n.saturated_fat_content),
            serving_size: vec![],
            sodium_content: to_mass(n.sodium_content),
            sugar_content: to_mass(n.sugar_content),
            r#type: AtType::NutritionInformation.to_opt(),
            trans_fat_content: to_mass(n.trans_fat_content),
            unsaturated_fat_content: to_mass(n.unsaturated_fat_content),
        }
    }
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        let (category, keywords) = match r
            .category
            .into_iter()
            .filter(|&s| !s.trim().is_empty())
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .split_first_owned()
        {
            (Some(category), v) => (Some(category), v),
            (None, _) => r
                .keywords
                .into_iter()
                .map(|s| s.trim().to_string())
                .collect_vec()
                .split_first_owned(),
        };

        Self {
            author: r
                .author
                .map(|s| vec![RecipeAuthorFieldEnum::new_person(s)])
                .unwrap_or_default(),
            is_based_on: to_is_based_on(
                r.software_version
                    .replace("(unreg.) ", "")
                    .trim_start_matches("Recipe via")
                    .trim(),
            ),
            keywords: keywords
                .into_iter()
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.trim().into()))
                .collect(),
            name: vec![r.title.into()],
            nutrition: r
                .nutrition
                .map(|n| if n.is_empty() { vec![] } else { vec![n.into()] })
                .unwrap_or_default(),
            prep_time: r
                .prep_time
                .map(|t| vec![DurationOrText::Text(t.into())])
                .unwrap_or_default(),
            recipe_category: category.map_or(Vec::new(), |s| vec![s]),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            recipe_yield: to_yield(i64::from(r.r#yield)),
            ..Default::default()
        }
    }
}

/// Parses the recipes in a REZKONV file.
pub fn parse<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let mut input = content.as_str();

    Ok(parse_recipes(&mut input)
        .map_err(|err| Error::Parse(err.to_string()))?
        .into_iter()
        .map(Recipe::from)
        .collect())
}

fn parse_recipes<'s>(input: &mut &'s str) -> WResult<Vec<RecipeComponents<'s>>> {
    repeat(1.., parse_recipe).parse_next(input)
}

fn parse_recipe<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        software_version: parse_header,
        title: parse_title,
        category: parse_category,
        r#yield: parse_yield,
        ingredients: parse_ingredients,
        _: parse_quelle,
        instructions: parse_instructions,
        prep_time: opt(parse_prep_time),
        nutrition: opt(terminated(parse_nutrition, multispace0)),
        keywords: parse_keywords,
        author: parse_author,
        _: parse_erfasst,
        _: repeat::<_, _, (), _, _>(0.., line_ending),
        _: (literal("====="), multispace0),
        _: repeat::<_, _, (), _, _>(0.., line_ending),
        ..Default::default()
    }}
    .parse_next(input)
}

fn parse_header<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(
        take_while(5..=10, '='),
        preceded(' ', take_till(1.., |c| c == '\n' || c == '=')),
        (opt(take_while(5..=10, '=')), line_ending, line_ending),
    )
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    preceded(
        (space1, "Titel:", space0),
        terminated(take_until(1.., "\n"), line_ending),
    )
    .parse_next(input)
}

fn parse_category<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    preceded(
        (space0, "Kategorien:", space0),
        terminated(
            separated(0.., take_till(0.., |c| c == ',' || c == '\n'), ","),
            line_ending,
        ),
    )
    .map(|cats: Vec<&str>| {
        cats.into_iter()
            .filter(|cat| !cat.trim().is_empty())
            .collect()
    })
    .parse_next(input)
}

fn parse_yield(input: &mut &str) -> WResult<i16> {
    preceded(
        (space1, "Menge:", space0),
        terminated(
            dec_int,
            (
                take_till(0.., |c| c == '\n'),
                (line_ending, opt(line_ending), opt(line_ending)),
            ),
        ),
    )
    .parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    alt((parse_ingredient_list, parse_ingredient_block)).parse_next(input)
}

fn parse_ingredient_list<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    terminated(
        repeat_till(
            1..,
            parse_ingredient,
            peek(alt((
                (line_ending, line_ending),
                (
                    line_ending,
                    "============================= QUELLE ================================",
                ),
                (line_ending, alphanumeric1),
            ))),
        )
        .map(|(ingredients, _)| ingredients),
        alt((
            (line_ending, take_while(6.., '=')),
            (line_ending, line_ending),
            line_ending.map(|s| (s, "")),
        )),
    )
    .parse_next(input)
}

fn parse_ingredient<'s>(input: &mut &'s str) -> WResult<Ingredient<'s>> {
    let main_line = if input.starts_with("\n====") {
        delimited(line_ending, take_till(1.., |c| c == '\n'), line_ending).parse_next(input)?
    } else {
        terminated(take_till(1.., |c| c == '\n'), line_ending).parse_next(input)?
    };

    let continuations: Vec<&str> = repeat(
        0..,
        preceded(
            ((space0, opt(line_ending)), peek('-')),
            terminated(take_till(1.., |c| c == '\n'), line_ending),
        ),
    )
    .parse_next(input)?;

    if continuations.is_empty() {
        if main_line.starts_with('=') {
            Ok(Ingredient::Section(Cow::Borrowed(
                main_line.trim_matches('=').trim(),
            )))
        } else {
            Ok(Ingredient::Line(Cow::Borrowed(main_line)))
        }
    } else {
        let mut full_text = main_line.to_string();
        for s in continuations {
            full_text.push(' ');
            full_text.push_str(s.trim());
        }
        Ok(Ingredient::Line(Cow::Owned(full_text)))
    }
}

fn parse_ingredient_block<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    terminated(
        take_until(0.., "\n\n\n"),
        (line_ending, line_ending, line_ending),
    )
    .map(|block: &str| {
        block
            .lines()
            .filter_map(|line| {
                let cleaned_line = line.trim();
                if cleaned_line.is_empty() {
                    None
                } else {
                    let s = line.split_whitespace().collect::<Vec<_>>().join(" ");
                    Some(Ingredient::Line(Cow::Owned(s)))
                }
            })
            .fold(Vec::new(), |mut acc, item| {
                match &item {
                    Ingredient::Line(line) if line.trim().starts_with('-') => {
                        if let Some(Ingredient::Line(last_line)) = acc.last_mut() {
                            last_line.to_mut().push(' ');
                            last_line.to_mut().push_str(&line[1..]);
                        } else {
                            acc.push(item.clone());
                        }
                    }
                    Ingredient::Section(_) | Ingredient::Line(_) => {
                        acc.push(item.clone());
                    }
                }

                acc
            })
    })
    .parse_next(input)
}

fn parse_quelle<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt((
        " QUELLE ================================",
        line_ending,
        take_until(1.., "\n\n"),
        (line_ending, line_ending),
    ))
    .map(|result| result.map(|(_, _, name, _): (_, _, &str, _)| name.trim()))
    .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    let cut_at = input.find("=====").unwrap_or(input.len());

    if input[..cut_at].contains(":Stichworte:") {
        take_until(1.., ":Stichworte:")
            .map(|s: &str| {
                s.split("\n\n")
                    .filter_map(|l| {
                        if l.trim().is_empty() {
                            None
                        } else {
                            Some(Instruction::Line(Cow::Borrowed(l.trim())))
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .parse_next(input)
    } else {
        alt((
            take_until(1.., "\n\n\n").map(|s: &str| {
                s.split("\n\n")
                    .filter_map(|l| {
                        let s = l.trim();
                        if s.is_empty() {
                            None
                        } else {
                            Some(Instruction::Line(Cow::Borrowed(s)))
                        }
                    })
                    .collect::<Vec<_>>()
            }),
            separated(1.., take_until(1.., "\n\n"), "\n\n").map(|blocks: Vec<&str>| {
                blocks
                    .into_iter()
                    .map(|s| Instruction::Line(Cow::Borrowed(s.trim())))
                    .collect::<Vec<_>>()
            }),
        ))
        .parse_next(input)
    }
}

fn parse_prep_time<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(
        (line_ending, line_ending, line_ending, space0),
        take_while(2..=4, |c: char| {
            c.is_ascii_digit() || c.is_ascii_alphabetic()
        }),
        line_ending,
    )
    .parse_next(input)
}

fn parse_nutrition<'s>(input: &mut &'s str) -> WResult<NutritionComponents<'s>> {
    seq! {NutritionComponents {
        calories: opt(delimited((space0, "calories : "), take_while(1.., |c| c != '\n'), line_ending)),
        carbohydrate_content: opt(delimited((space0, "carbohydrateContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        cholesterol_content: opt(delimited((space0, "cholesterolContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        fat_content: opt(delimited((space0, "fatContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        fiber_content: opt(delimited((space0, "fiberContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        protein_content: opt(delimited((space0, "proteinContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        saturated_fat_content: opt(delimited((space0, "saturatedFatContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        sodium_content: opt(delimited((space0, "sodiumContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        sugar_content: opt(delimited((space0, "sugarContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        trans_fat_content: opt(delimited((space0, "transFatContent : "), take_while(1.., |c| c != '\n'), line_ending)),
        unsaturated_fat_content: opt(delimited((space0, "unsaturatedFatContent : "), take_while(1.., |c| c != '\n'), line_ending)),
    }}
    .parse_next(input)
}

fn parse_keywords<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    parse_metadata(input, ":Stichworte: ")
}

fn parse_author<'s>(input: &mut &'s str) -> WResult<Option<&'s str>> {
    opt(terminated(
        preceded(literal(":Erfasser/Name:"), take_while(0.., |c| c != '\n')),
        line_ending,
    ))
    .map(|s: Option<&str>| s.map(|s: &str| s.trim()))
    .parse_next(input)
}

fn parse_erfasst<'s>(input: &mut &'s str) -> WResult<Vec<&'s str>> {
    parse_metadata(input, ":Erfass")
}

fn parse_metadata<'s>(input: &mut &'s str, line: &'s str) -> WResult<Vec<&'s str>> {
    repeat(
        0..,
        terminated(
            preceded(literal(line), take_while(0.., |c| c != '\n')),
            line_ending,
        ),
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_recipes {
        use super::*;

        use std::io::Cursor;

        type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

        #[test]
        fn test_cookmate() -> Result<()> {
            let file = files::cookmate();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::cookmate());
            Ok(())
        }

        #[test]
        fn test_cookmate2() -> Result<()> {
            let file = files::cookmate2();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::cookmate2());
            Ok(())
        }

        #[test]
        fn test_kalorio_v4_03() -> Result<()> {
            let file = files::kalorio_v4_03();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::kalorio_v4_03());
            Ok(())
        }
    }

    mod files {
        #[allow(clippy::too_many_lines)]
        pub fn cookmate<'a>() -> &'a str {
            r"===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Aunt Julia's Paella
Kategorien: Pork/ham, Poultry, Fish/sea, Spanish
      Menge: 6

      1    Chicken, cut up (Or 4
           -thighs 1 3/4 oz Jar

           -and legs) 2 ts Capers,

           Salt and pepper to thaste
           -4 oz Jar pimento-stiffed
           -green
      1 lb Lean pork, cut into 1-inch
           --olives
    1/2 lb Calamari (squid), cleaned
      1    md Onion, minced -and
           -sliced
      2    Toes garlic, minced 5 c
           -Water
           Cut into 1 1/2 inch
           -julliene 4 Chicken

      1 ts Saffron threads
    1/2 lg Bell pepper 2 1/2 c Uncle

      1 lg Carrot -uncooked
      1    Stalk celery 3 Hard boiled

      1    c Frozen green peas 1/2 lb
           -Unpeeled shrimp (heads
           -on)
  1 1/2 lb Peeled shrimp Oil for
           -frying


{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }
In a large electric skillet or paella pan, brown the chicken pieces (that
have been seasoned with salt and pepper) in a little oil. Remove from the
pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.
Remove from the pan. To the pan drippings (add a little more oil if
necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry
for 2 minutes.
Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.
Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the
bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.
Gently stir the rice into the skillet mixture. Slowly pour in enough of
the bouillon mixture to cover the rice and chicken pieces. Cover and cook
over low heat for about 20 minutes. Uncover and decoaratively arrange the
egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary
to keep the rice moist.
Cover and steam for another 10 minutes until the shrimp are cooked and the
rice is tender. (Paella should be moist but not wet!) Place the pan on a
hot pad on the serving table and let everyone help themselves.
Serve with a mixed green salad, red ripe tomatoes and some French bread.
Also mix up a pitcher of Sangria and enjoy!
Serves: 12.
[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]
Posted by Fred Peters





=====

===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Ausgezogenes Mehlmus
Kategorien: German
      Menge: 4

     80 g  Flour (3/4 cup) 8 Egg

      1 l  Milk (approx. 1 qt) -peaks
    120 g  Sugar (1/2 cup plus 1/2

      1 pn Salt
      8    Egg yolks, whisked to a 50

           -froth


From Central Swabia.
From grandmother's more thrifty times; rarely encountered today.
Combine the flour and a little milk, and stir until smooth. Gradually add
the remainder of the milk, the sugar and salt. Bring to a boil. Remove the
pot from the heat, add the grated lemon peel. Carefully fold in the egg
yolk froth and beaten egg whites. Pour the mixture into a buttered
casserole dish and bake at medium heat for 20 minutes.
Serves 4.
From: D'SCHWAEBISCH' KUCHE' by Aegidius Kolb and Leonhard Lidel, Allgaeuer
Zeitungsverlag, Kempten. 1976. (Translation/Conversion: Karin Brewer)
Posted by: Karin Brewer, Cooking Echo, 8/92





=====

===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Austrian Bread Dumplings
Kategorien: Ethnic, Breads
      Menge: 6

      4 oz Dry bread, diced Salt and
           -pepper
    1/2 oz (1 Tbsp) butter or lard 1

      1    Egg -(parsley, chervil,
    1/2    c Milk -marjoram) -
      3 oz (3/4 cup) flour


Tbsp chopped fresh herbs (parsley, chervil, marjoram) - optional, but
a great improvement
You will need a frying pan, a large and a small bowl, and a saucepan of
water or soup. Fry the diced bread lightly in the fat in a frying pan.
Meanwhile, mix the egg and the milk in a small bowl. Tip the contents of
the frying pan into a large bowl, and pour the egg and milk over all. Stir
in the flour, and season with salt and pepper. Add the herbs, if using.
You may need more milk to make a soft dough. Allow it to stand for 1/2 an
hour.
Dip your hand into cold water and roll the mixture into a dozen small
balls. Put a pot of salted water on to boil, if there isn't a simmering
soup pot waiting. Drop little balls of dough into the boiling salted water
or the soup. Poach them for 10 to 15 minutes, until they are light and
firm and well risen.
Yield: 12 dumplings Time: 1 hour
Notes: You may include chopped fried bacon or cubed pork cracklings in the
mixture. Leaving out flour will result in a lighter dumpling.
From: THE OLD WORLD KITCHEN - THE RICH TRADITION OF EUROPEAN PEASANT
COOKING by Elisabeth Luard, ISBN 0-553-05219-5 Posted by: Karin Brewer,
Cooking Echo, 7/92





=====

===== Recipe via Cookmate [REZKONV Export Format] =====

      Titel: Authentic Italian Bread
Kategorien: Italian, Breads
      Menge: 2

      1 ts Active dry yeast or 1/3

           -small cake (6 grams)
           -fresh 2/3 c Milk at room
           -temperature
      1    c (135 grams) unbleached
      1    Scant tsp. malt syrup



This takes 2 days but is worth the wait. Makes 2 round loaves Starter
Stir the yeast and malt into the water; let stand until foamy, about 10
minutes. Stir in the milk and beat in the flour with a rubber spatula or
wooden spoon about 100 strokes until smooth. Cover with plastic wrap and
let stand until bubbly, at least 4 hours but preferably overnight.
Dough 2 cups water, at room temperature 6 1/4 cups (860 grams) unbleached
all-purpose flour 1 T. salt Cornmeal
Mix the starter and the water in a mixer until the starter is well broken
up. Add the flour and salt and mix for 2 to 3 minutes at low speed. The
dough will be smooth but won't pull away from the side of the bowl. Change
to the dough hook and knead at medium speed, scraping down the side of the
bowl as necessary, until the dough is elastic but slightly sticky, 3 to 4
minutes. Finish kneading by hand on a floured work surface.
First rise Place in a well-oiled bowl, cover tightly with plastic wrap, and
let rise until doubled, about 1 1/2 hours. The dough is ready when it is
very bubbled and blistered.
Shaping and second rise Cut the dough in half on a floured surface and
shape into 2 round loaves. Place on an oiled cookie sheet sprinkled with
cornmeal. Cover and let rise till doubled, about 1 hour.
Baking Preheat oven to 400 degrees F. Bake about 1 hour and cool on racks.
To get a really good crust spray the loaves with water 3 times in the first
minutes of baking.





=====

"
        }

        pub fn cookmate2<'a>() -> &'a str {
            r"===== Recipe via Cookmate [REZKONV Export Format] =====

                  Titel: Simple White Cake
            Kategorien:
                  Menge: 12

                  1 c  white sugar
                0.5 c  unsalted butter
                  2    large eggs
                  2 ts vanilla extract
                1.5 c  all-purpose flour
               1.75 ts baking powder
                1/4 ts table salt
                0.5 c  milk


            Gather all ingredients. Preheat the oven to 350 degrees F (175 degrees C). Grease and flour a 9-inch square cake pan.
            Beat sugar and butter together in a mixing bowl with an electric mixer until lighter in color and fluffy, 3 to 4 minutes. Add eggs, one at a time, beating briefly after each addition, 30 seconds total. Mix in vanilla, about 15 seconds.
            Whisk flour, baking powder, and salt in a separate bowl. With mixer on low speed, add flour mixture to butter mixture in 3 batches, alternating with milk, beginning and ending with flour. Mix just until combined stopping to scrape down sides if needed, about 2 minutes.
            Spread cake batter into the prepared pan.
            Bake cake in the preheated oven until a toothpick inserted into the center comes out clean, about 30 minutes.
            Remove cake from the oven and let cool in pan on a wire rack for 10 minutes. Invert cake onto wire rack; remove pan and let cake cool completely before frosting. Enjoy!


            10m
            calories : 209 kcal
            carbohydrateContent : 29 g
            cholesterolContent : 52 mg
            fatContent : 9 g
            fiberContent : 0 g
            proteinContent : 3 g
            saturatedFatContent : 5 g
            sodiumContent : 142 mg
            sugarContent : 17 g
            unsaturatedFatContent : 0 g

            =====

            ===== Recipe via Cookmate [REZKONV Export Format] =====

                  Titel: The Best Chicken Fried Steak
            Kategorien: Starter, Main course
                  Menge: 4

                  4    (1/2 pound) beef cube
                       -steaks
               2.25 c  all-purpose flour, divided
                  2 ts baking powder
                  1 ts baking soda
                  1 ts black pepper
               0.75 ts salt
                1.5 c  buttermilk
                  1    large egg
                  1 tb hot pepper sauce (e.g.
                       -Tabasco?)
                  2    cloves garlic, minced
                  3 c  vegetable shortening for
                       -frying
                  4 c  milk
                       kosher salt and ground



            Place steaks between two sheets of heavy plastic on a solid, level surface; firmly pound with a meat mallet to a ¼-inch thickness.
            Place 2 cups flour in a shallow bowl.
            Combine baking powder, baking soda, 1 teaspoon pepper, and ¾ teaspoon salt in a separate shallow bowl; stir in buttermilk, egg, Tabasco, and garlic to combine.
            Heat shortening in a deep cast-iron skillet to 325 degrees F (165 degrees C). Place a wire rack over a sheet of parchment paper.
            Meanwhile, dredge 1 steak in flour to coat; shake off excess. Dip into buttermilk batter; lift up so excess batter drips back into the bowl. Dredge in flour again to coat both sides completely. Place breaded steak on the prepared wire rack. Repeat with remaining steaks.
            Fry steaks, in batches if necessary, until evenly golden brown, 3 to 5 minutes per side. Transfer steaks to a paper towel-lined plate to drain. Cover with foil to keep warm.
            Drain fat from the skillet, reserving ¼ cup and as much solid remnants as possible.
            Place skillet over medium-low heat. Add reserved ¼ cup oil; whisk in remaining ¼ cup flour. Scrape the brown bits of food off the bottom of the skillet with a spatula.
            Stir in milk; increase heat to medium and bring gravy to a simmer. Cook, stirring often, until thick, 6 to 7 minutes. Season gravy with salt and black pepper.
            Transfer steaks to a platter; pour gravy over top.


            20m
            calories : 832 kcal
            carbohydrateContent : 71 g
            cholesterolContent : 206 mg
            fatContent : 29 g
            fiberContent : 2 g
            proteinContent : 68 g
            saturatedFatContent : 12 g
            sodiumContent : 1273 mg
            unsaturatedFatContent : 0 g

            =====

            ===== Recipe via Cookmate [REZKONV Export Format] =====

                  Titel: To Die For Fettuccine Alfredo
            Kategorien: Dessert
                  Menge: 6

                 24 oz dry fettuccine pasta
                  1 c  butter
               0.75 pt heavy cream
                  1 ds garlic salt
                       salt and pepper to taste
               0.75 c  grated Romano cheese
                0.5 c  grated Parmesan cheese


            Gather all ingredients.
            Fill a large pot with lightly salted water and bring to a rolling boil. Cook fettuccine at a boil until tender yet firm to the bite, about 8 minutes. Drain.
            Heat butter and cream in a large saucepan over low heat until butter melted; add garlic salt, salt, and black pepper.
            Increase the heat to medium; stir in Romano and Parmesan cheeses until melted and sauce has thickened.
            Add cooked pasta to sauce; toss until thoroughly coated. Serve immediately.


            15m
            calories : 964 kcal
            carbohydrateContent : 84 g
            cholesterolContent : 184 mg
            fatContent : 61 g
            fiberContent : 4 g
            proteinContent : 24 g
            saturatedFatContent : 37 g
            sodiumContent : 582 mg
            sugarContent : 4 g
            unsaturatedFatContent : 0 g

            =====


"
        }

        #[allow(clippy::too_many_lines)]
        pub fn kalorio_v4_03<'a>() -> &'a str {
            r#"========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Ananas-Käsekuchen
Kategorien: , ,
     Menge: 1 Kuchen

    150     Gramm  Kokoszwieback
     60     Gramm  Butter
    600     Gramm  Doppelrahmfrischkäse
    150     Gramm  Saure Sahne
    120     Gramm  Zucker
      4            Eier
      1            Limette
      1      Essl  Mehl
      1     klein. Ananas
                   - a 1 kg
      2      Essl. Rum
    200        ml  Ananassaft
      3      Essl. Vanille-Puddingpulver
      1  Spritzer  Zitrone
     20     Gramm  Kokosraspel

============================= QUELLE ================================
           essen & trinken 1/2002
           Gepostet von: Petra Holzapfel
           Überarbeitet für Kalorio
           - Erfasst am 9.2.2 von
           - Petra Holzapfel

Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und
unter die Brösel mischen. Den Boden einer Springform (24 cm
Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf
verteilen und gut andrücken. Kalt stellen.

Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des
Handrührers glattrühren. Nacheinander die Eier dazugeben. Die
Limettenschale fein abreiben, die Limette auspressen. Limettensaft
und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der
Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste
von
unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC
reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf
einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt
stellen.

3. Die Ananas grosszügig schälen, vierteln und den Strunk entfernen.
Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben
schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft
zum Kochen bringen. Ananasscheiben dazugeben und aufkochen.
Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer
glattrühren
und das Kompott damitbinden. Das Kompott auskühlen lassen und die
Hälfte auf dem
Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.

Das restliche Ananaskompott extra zum Kuchen servieren.

:Stichworte: Ananas
:Stichworte: Käse
:Stichworte: Kuchen
:Erfasser/Name: Petra Holzapfel
:Erfasst am: 9.2.2


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Flammenkuchen von Marc
Kategorien:
     Menge: 2 Backbleche


========================== Für den Brotteig: ========================
      6      Essl. Mehl
    1/2      Teel. ; Salz
      1     Prise  ; Pfeffer
    1/2            Würfel Hefe
    1/4      Litr. ; Wasser
                   - (lauwarm)

============================ Für den Belag ==========================
    250     Gramm  Quark
                   - 40% Fett
      1        Be  Saure Sahne
    1/2      Teel. ; Salz
      1     Prise  ; Pfeffer
      1            Zitrone

========================== Für die Garnitur =========================
      5      groß. Zwiebel
    125     Gramm  Durchwachsener Speck
    125     Gramm  Reibkäse
                   - (optional)

============================= QUELLE ================================
           Von meinem Kollegen Marc Hurstel
           - Erfasst am 8.3.1999 von
           - Jochen Herz

Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel
geben. Die Hefe im Wasser auflösen und anschließend über das Mehl
gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den
Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl
beimischen).
Den Teig aus der Schüssel herrausnehmen und mit den Händen auf
der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird.
Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig
nicht kleben bleibt ! Den Teig während der Zubereitung
der Sauce und Garnitur ruhen lassen.

Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in
einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren
Geschmack bekommt, wird der Saft einer Zitrone untergerührt.

Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm
dick) schneiden. Den Speck in kleine Würfel schneiden.
Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn
wie möglich ausrollen (je dünner, umso schmackhafter !). Die
Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn
auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die
Speckwürfel. Eventuell auch Käse dazu.
(Traditioneller Flammenkuchen ist ohne Käse !)

Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der
Teig an den Rändern goldbraun wird)
Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach
ein schönes elsäßisches Bier (Fischer Ambré).
Und jetzt: "A güata !" wie man bei uns sagt

:Stichworte: Frankreich
:Stichworte: Elsass
:Stichworte: Hefe
:Erfasser/Name: Jochen Herz
:Erfasst am: 8.3.1999


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Möhren-Mandel-Muffins
Kategorien: , , ,
     Menge: 12 Muffins

      4            Eiweiß
      4            Eigelb
     50     Gramm  Zucker
    200     Gramm  geraspelte Möhren
    200     Gramm  gemahlene Mandeln
      1      Teel. Kirschwasser
    1/2      Teel. Zimt
    1/2            Zitrone (Schale davon)
     25     Gramm  Stärkemehl
    150     Gramm  Puderzucker
                   Zitronensaft

============================= QUELLE ================================
           - Erfasst am 18.8.2006 von
           - Jochen 'Nunz' Herz

Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig
rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die
Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale
und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee
vorsichtig unter die Teigmasse heben.

Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die
Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf
mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C).
Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die
fertigen Muffins damit bestreichen.

:Stichworte: Muffins
:Stichworte: Karotten
:Erfasser/Name: Jochen 'Nunz' Herz
:Erfasst am: 18.8.2006


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Zwiebelkuchen
Kategorien: ,
     Menge: 4 Portionen


================================ Teig ===============================
    250     Gramm  Mehl
    125     Gramm  Margarine
      1     Prise  Salz
      1            Eier

=============================== Füllung =============================
      6      groß. Zwiebeln
     75     Gramm  Butter
      1        Be  Joghurt
      1        Be  Saure Sahne
                   - oder Schmand
      3            Eier
                   Kümmel
                   - gemahlen
                   Pfeffer
                   Paprika
                   - edelsüss
      1      Teel. Speisestärke

Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig
1-2 Stunden kalt stellen.
Teig in 28er Kuchenform auswellen und hohen Rand kneten.

Zwiebel schälen und in Streifen schneiden und in 75 g Butter und
eventuell etwas Fondor eine halbe Stunde dünsten.
Kalt stellen.

Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer
verrühren. und die Zwiebeln untermischen.
Die Füllung auf den Teigboden und den rand herunterdrücken damit er
nicht übersteht.

Oben mit Kümmel oder Käse nach Belieben verfeindern und im
vorgeheizten Backofen bei 220 Grad 30 Minuten backen.

:Stichworte: Backen
:Stichworte: Zwiebeln
:Erfasser/Name: Jochen 'Nunz' Herz


=====
"#
        }
    }

    mod results {
        use super::*;

        use schema_org::Recipe;
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        };

        pub fn cookmate() -> Vec<Recipe> {
            vec![
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    keywords: ["Poultry", "Fish/sea", "Spanish"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    name: vec!["Aunt Julia's Paella".into()],
                    recipe_category: vec!["Pork/ham".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar and legs) 2 ts Capers,".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Salt and pepper to thaste 4 oz Jar pimento-stiffed green".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lb Lean pork, cut into 1-inch -olives".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 lb Calamari (squid), cleaned".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 md Onion, minced -and sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Toes garlic, minced 5 c Water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Cut into 1 1/2 inch julliene 4 Chicken".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts Saffron threads".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 lg Bell pepper 2 1/2 c Uncle".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 lg Carrot -uncooked".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Stalk celery 3 Hard boiled".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 lb Peeled shrimp Oil for frying".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant } In a large electric skillet or paella pan, brown the chicken pieces (that have been seasoned with salt and pepper) in a little oil. Remove from the pan. Add the pork cubes to the drippinfs and brown for about 5 minutes. Remove from the pan. To the pan drippings (add a little more oil if necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry for 2 minutes. Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork. Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the bouillon cubes and saffron. Let it stand for 5 minutes until dissolved. Gently stir the rice into the skillet mixture. Slowly pour in enough of the bouillon mixture to cover the rice and chicken pieces. Cover and cook over low heat for about 20 minutes. Uncover and decoaratively arrange the egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary to keep the rice moist. Cover and steam for another 10 minutes until the shrimp are cooked and the rice is tender. (Paella should be moist but not wet!) Place the pan on a hot pad on the serving table and let everyone help themselves. Serve with a mixed green salad, red ripe tomatoes and some French bread. Also mix up a pitcher of Sangria and enjoy! Serves: 12. [ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ] Posted by Fred Peters".into(),
                    )],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    name: vec!["Ausgezogenes Mehlmus".into()],
                    recipe_category: vec!["German".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("80 g Flour (3/4 cup) 8 Egg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 l Milk (approx. 1 qt) -peaks".into()),
                        RecipeRecipeIngredientFieldEnum::Text("120 g Sugar (1/2 cup plus 1/2".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 pn Salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 Egg yolks, whisked to a 50 froth".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "From Central Swabia. From grandmother's more thrifty times; rarely encountered today. Combine the flour and a little milk, and stir until smooth. Gradually add the remainder of the milk, the sugar and salt. Bring to a boil. Remove the pot from the heat, add the grated lemon peel. Carefully fold in the egg yolk froth and beaten egg whites. Pour the mixture into a buttered casserole dish and bake at medium heat for 20 minutes. Serves 4. From: D'SCHWAEBISCH' KUCHE' by Aegidius Kolb and Leonhard Lidel, Allgaeuer Zeitungsverlag, Kempten. 1976. (Translation/Conversion: Karin Brewer) Posted by: Karin Brewer, Cooking Echo, 8/92".into(),
                    )],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Breads".into())],
                    name: vec!["Austrian Bread Dumplings".into()],
                    recipe_category: vec!["Ethnic".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 oz Dry bread, diced Salt and pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 oz (1 Tbsp) butter or lard 1".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Egg -(parsley, chervil,".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 c Milk -marjoram)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 oz (3/4 cup) flour".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "Tbsp chopped fresh herbs (parsley, chervil, marjoram) - optional, but a great improvement You will need a frying pan, a large and a small bowl, and a saucepan of water or soup. Fry the diced bread lightly in the fat in a frying pan. Meanwhile, mix the egg and the milk in a small bowl. Tip the contents of the frying pan into a large bowl, and pour the egg and milk over all. Stir in the flour, and season with salt and pepper. Add the herbs, if using. You may need more milk to make a soft dough. Allow it to stand for 1/2 an hour. Dip your hand into cold water and roll the mixture into a dozen small balls. Put a pot of salted water on to boil, if there isn't a simmering soup pot waiting. Drop little balls of dough into the boiling salted water or the soup. Poach them for 10 to 15 minutes, until they are light and firm and well risen. Yield: 12 dumplings Time: 1 hour Notes: You may include chopped fried bacon or cubed pork cracklings in the mixture. Leaving out flour will result in a lighter dumpling. From: THE OLD WORLD KITCHEN - THE RICH TRADITION OF EUROPEAN PEASANT COOKING by Elisabeth Luard, ISBN 0-553-05219-5 Posted by: Karin Brewer, Cooking Echo, 7/92".into(),
                    )],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                Recipe {
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Breads".into())],
                    name: vec!["Authentic Italian Bread".into()],
                    recipe_category: vec!["Italian".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 ts Active dry yeast or 1/3 small cake (6 grams) fresh 2/3 c Milk at room temperature".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c (135 grams) unbleached".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Scant tsp. malt syrup".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "This takes 2 days but is worth the wait. Makes 2 round loaves Starter Stir the yeast and malt into the water; let stand until foamy, about 10 minutes. Stir in the milk and beat in the flour with a rubber spatula or wooden spoon about 100 strokes until smooth. Cover with plastic wrap and let stand until bubbly, at least 4 hours but preferably overnight. Dough 2 cups water, at room temperature 6 1/4 cups (860 grams) unbleached all-purpose flour 1 T. salt Cornmeal Mix the starter and the water in a mixer until the starter is well broken up. Add the flour and salt and mix for 2 to 3 minutes at low speed. The dough will be smooth but won't pull away from the side of the bowl. Change to the dough hook and knead at medium speed, scraping down the side of the bowl as necessary, until the dough is elastic but slightly sticky, 3 to 4 minutes. Finish kneading by hand on a floured work surface. First rise Place in a well-oiled bowl, cover tightly with plastic wrap, and let rise until doubled, about 1 1/2 hours. The dough is ready when it is very bubbled and blistered. Shaping and second rise Cut the dough in half on a floured surface and shape into 2 round loaves. Place on an oiled cookie sheet sprinkled with cornmeal. Cover and let rise till doubled, about 1 hour. Baking Preheat oven to 400 degrees F. Bake about 1 hour and cool on racks. To get a really good crust spray the loaves with water 3 times in the first minutes of baking.".into(),
                    )],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
            ]
        }

        #[allow(clippy::too_many_lines)]
        pub fn cookmate2() -> Vec<Recipe> {
            vec![
                Recipe {
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("209 kcal")],
                        carbohydrate_content: vec![Mass::new("29 g")],
                        cholesterol_content: vec![Mass::new("52 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("9 g")],
                        protein_content: vec![Mass::new("3 g")],
                        saturated_fat_content: vec![Mass::new("5 g")],
                        sodium_content: vec![Mass::new("142 mg")],
                        sugar_content: vec![Mass::new("17 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    recipe_yield: to_yield(12),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 c white sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.5 c unsalted butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 large eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 ts vanilla extract".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1.5 c all-purpose flour".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1.75 ts baking powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 ts table salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.5 c milk".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "Gather all ingredients. Preheat the oven to 350 degrees F (175 degrees C). Grease and flour a 9-inch square cake pan. Beat sugar and butter together in a mixing bowl with an electric mixer until lighter in color and fluffy, 3 to 4 minutes. Add eggs, one at a time, beating briefly after each addition, 30 seconds total. Mix in vanilla, about 15 seconds. Whisk flour, baking powder, and salt in a separate bowl. With mixer on low speed, add flour mixture to butter mixture in 3 batches, alternating with milk, beginning and ending with flour. Mix just until combined stopping to scrape down sides if needed, about 2 minutes. Spread cake batter into the prepared pan. Bake cake in the preheated oven until a toothpick inserted into the center comes out clean, about 30 minutes. Remove cake from the oven and let cool in pan on a wire rack for 10 minutes. Invert cake onto wire rack; remove pan and let cake cool completely before frosting. Enjoy!".into(),
                    )],
                    prep_time: vec![DurationOrText::Text("10m".into())],
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    name: vec!["Simple White Cake".into()],
                    ..Default::default()
                },
                Recipe {
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("832 kcal")],
                        carbohydrate_content: vec![Mass::new("71 g")],
                        cholesterol_content: vec![Mass::new("206 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("29 g")],
                        fiber_content: vec![Mass::new("2 g")],
                        protein_content: vec![Mass::new("68 g")],
                        saturated_fat_content: vec![Mass::new("12 g")],
                        sodium_content: vec![Mass::new("1273 mg")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    recipe_yield: to_yield(4),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 (1/2 pound) beef cube -steaks".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2.25 c all-purpose flour, divided".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 ts baking powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts baking soda".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ts black pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.75 ts salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1.5 c buttermilk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 large egg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tb hot pepper sauce (e.g. -Tabasco?)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 cloves garlic, minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 c vegetable shortening for -frying".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 c milk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("kosher salt and ground".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "Place steaks between two sheets of heavy plastic on a solid, level surface; firmly pound with a meat mallet to a ¼-inch thickness. Place 2 cups flour in a shallow bowl. Combine baking powder, baking soda, 1 teaspoon pepper, and ¾ teaspoon salt in a separate shallow bowl; stir in buttermilk, egg, Tabasco, and garlic to combine. Heat shortening in a deep cast-iron skillet to 325 degrees F (165 degrees C). Place a wire rack over a sheet of parchment paper. Meanwhile, dredge 1 steak in flour to coat; shake off excess. Dip into buttermilk batter; lift up so excess batter drips back into the bowl. Dredge in flour again to coat both sides completely. Place breaded steak on the prepared wire rack. Repeat with remaining steaks. Fry steaks, in batches if necessary, until evenly golden brown, 3 to 5 minutes per side. Transfer steaks to a paper towel-lined plate to drain. Cover with foil to keep warm. Drain fat from the skillet, reserving ¼ cup and as much solid remnants as possible. Place skillet over medium-low heat. Add reserved ¼ cup oil; whisk in remaining ¼ cup flour. Scrape the brown bits of food off the bottom of the skillet with a spatula. Stir in milk; increase heat to medium and bring gravy to a simmer. Cook, stirring often, until thick, 6 to 7 minutes. Season gravy with salt and black pepper. Transfer steaks to a platter; pour gravy over top.".into(),
                    )],
                    recipe_category: vec!["Starter".into()],
                    prep_time: vec![DurationOrText::Text("20m".into())],
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Main course".into())],
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    name: vec!["The Best Chicken Fried Steak".into()],
                    ..Default::default()
                },
                Recipe {
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("964 kcal")],
                        carbohydrate_content: vec![Mass::new("84 g")],
                        cholesterol_content: vec![Mass::new("184 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("61 g")],
                        fiber_content: vec![Mass::new("4 g")],
                        protein_content: vec![Mass::new("24 g")],
                        saturated_fat_content: vec![Mass::new("37 g")],
                        sodium_content: vec![Mass::new("582 mg")],
                        sugar_content: vec![Mass::new("4 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    recipe_yield: to_yield(6),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("24 oz dry fettuccine pasta".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 c butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.75 pt heavy cream".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 ds garlic salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("salt and pepper to taste".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.75 c grated Romano cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.5 c grated Parmesan cheese".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "Gather all ingredients. Fill a large pot with lightly salted water and bring to a rolling boil. Cook fettuccine at a boil until tender yet firm to the bite, about 8 minutes. Drain. Heat butter and cream in a large saucepan over low heat until butter melted; add garlic salt, salt, and black pepper. Increase the heat to medium; stir in Romano and Parmesan cheeses until melted and sauce has thickened. Add cooked pasta to sauce; toss until thoroughly coated. Serve immediately.".into(),
                    )],
                    recipe_category: vec!["Dessert".into()],
                    prep_time: vec![DurationOrText::Text("15m".into())],
                    is_based_on: to_is_based_on("Cookmate [REZKONV Export Format]"),
                    name: vec!["To Die For Fettuccine Alfredo".into()],
                    ..Default::default()
                },
            ]
        }

        #[allow(clippy::too_many_lines)]
        pub fn kalorio_v4_03() -> Vec<Recipe> {
            vec![
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Petra Holzapfel")],
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV"),
                    keywords: ["Käse", "Kuchen"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    name: vec!["Ananas-Käsekuchen".into()],
                    recipe_category: vec!["Ananas".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("150 Gramm Kokoszwieback".into()),
                        RecipeRecipeIngredientFieldEnum::Text("60 Gramm Butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("600 Gramm Doppelrahmfrischkäse".into()),
                        RecipeRecipeIngredientFieldEnum::Text("150 Gramm Saure Sahne".into()),
                        RecipeRecipeIngredientFieldEnum::Text("120 Gramm Zucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 Eier".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Limette".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Essl Mehl".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 klein. Ananas - a 1 kg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 Essl. Rum".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 ml Ananassaft".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 Essl. Vanille-Puddingpulver".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Spritzer Zitrone".into()),
                        RecipeRecipeIngredientFieldEnum::Text("20 Gramm Kokosraspel".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und unter die Brösel mischen. Den Boden einer Springform (24 cm Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf verteilen und gut andrücken. Kalt stellen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des Handrührers glattrühren. Nacheinander die Eier dazugeben. Die Limettenschale fein abreiben, die Limette auspressen. Limettensaft und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste von unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt stellen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Die Ananas grosszügig schälen, vierteln und den Strunk entfernen. Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft zum Kochen bringen. Ananasscheiben dazugeben und aufkochen. Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer glattrühren und das Kompott damitbinden. Das Kompott auskühlen lassen und die Hälfte auf dem Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Das restliche Ananaskompott extra zum Kuchen servieren.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(1),
                    ..Default::default()
                },
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Jochen Herz")],
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV"),
                    keywords: ["Elsass", "Hefe"].into_iter().map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.into())).collect(),
                    name: vec!["Flammenkuchen von Marc".into()],
                    recipe_category: vec!["Frankreich".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Für den Brotteig:", &[
                            "6 Essl. Mehl",
                            "1/2 Teel. ; Salz",
                            "1 Prise ; Pfeffer",
                            "1/2 Würfel Hefe",
                            "1/4 Litr. ; Wasser - (lauwarm)",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Für den Belag", &[
                            "250 Gramm Quark - 40% Fett",
                            "1 Be Saure Sahne",
                            "1/2 Teel. ; Salz",
                            "1 Prise ; Pfeffer",
                            "1 Zitrone",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Für die Garnitur", &[
                            "5 groß. Zwiebel",
                            "125 Gramm Durchwachsener Speck",
                            "125 Gramm Reibkäse - (optional)",
                        ]),

                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel geben. Die Hefe im Wasser auflösen und anschließend über das Mehl gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl beimischen). Den Teig aus der Schüssel herrausnehmen und mit den Händen auf der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird. Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig nicht kleben bleibt ! Den Teig während der Zubereitung der Sauce und Garnitur ruhen lassen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren Geschmack bekommt, wird der Saft einer Zitrone untergerührt.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm dick) schneiden. Den Speck in kleine Würfel schneiden. Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn wie möglich ausrollen (je dünner, umso schmackhafter !). Die Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die Speckwürfel. Eventuell auch Käse dazu. (Traditioneller Flammenkuchen ist ohne Käse !)".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            r#"Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der Teig an den Rändern goldbraun wird) Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach ein schönes elsäßisches Bier (Fischer Ambré). Und jetzt: "A güata !" wie man bei uns sagt"#.into(),
                        ),
                    ],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Jochen 'Nunz' Herz")],
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV"),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Karotten".into())],
                    name: vec!["Möhren-Mandel-Muffins".into()],
                    recipe_category: vec!["Muffins".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 Eiweiß".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 Eigelb".into()),
                        RecipeRecipeIngredientFieldEnum::Text("50 Gramm Zucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 Gramm geraspelte Möhren".into()),
                        RecipeRecipeIngredientFieldEnum::Text("200 Gramm gemahlene Mandeln".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Teel. Kirschwasser".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Teel. Zimt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 Zitrone (Schale davon)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("25 Gramm Stärkemehl".into()),
                        RecipeRecipeIngredientFieldEnum::Text("150 Gramm Puderzucker".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Zitronensaft".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee vorsichtig unter die Teigmasse heben.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C). Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die fertigen Muffins damit bestreichen.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(12),
                    ..Default::default()
                },
                Recipe {
                    author: vec![RecipeAuthorFieldEnum::new_person("Jochen 'Nunz' Herz")],
                    is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV"),
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Zwiebeln".into())],
                    name: vec!["Zwiebelkuchen".into()],
                    recipe_category: vec!["Backen".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Teig", &[
                            "250 Gramm Mehl",
                            "125 Gramm Margarine",
                            "1 Prise Salz",
                            "1 Eier",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Füllung", &[
                            "6 groß. Zwiebeln",
                            "75 Gramm Butter",
                            "1 Be Joghurt",
                            "1 Be Saure Sahne - oder Schmand",
                            "3 Eier",
                            "Kümmel - gemahlen",
                            "Pfeffer",
                            "Paprika - edelsüss",
                            "1 Teel. Speisestärke",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig 1-2 Stunden kalt stellen. Teig in 28er Kuchenform auswellen und hohen Rand kneten.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Zwiebel schälen und in Streifen schneiden und in 75 g Butter und eventuell etwas Fondor eine halbe Stunde dünsten. Kalt stellen.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer verrühren. und die Zwiebeln untermischen. Die Füllung auf den Teigboden und den rand herunterdrücken damit er nicht übersteht.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Oben mit Kümmel oder Käse nach Belieben verfeindern und im vorgeheizten Backofen bei 220 Grad 30 Minuten backen.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
            ]
        }
    }
}
