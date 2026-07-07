use std::{
    borrow::Cow,
    io::{Read, Seek},
};

use schema_org::{
    DurationOrText, Recipe,
    field::{RecipeDescriptionFieldEnum, RecipeIsBasedOnUrlFieldEnum, RecipeRecipeYieldFieldEnum},
};
use winnow::{
    Parser, Result as WResult,
    ascii::{line_ending, multispace0, multispace1, space0, till_line_ending},
    combinator::{alt, delimited, eof, not, opt, peek, repeat, repeat_till, seq, terminated},
    token::{literal, take_until},
};
use zip::ZipArchive;

use crate::{
    Error, Result,
    apps::helpers::{
        Ingredient, Instruction, Parsers, ToSections, extract_archive_contents, read_file,
    },
};

#[derive(Default)]
struct RecipeComponents<'a> {
    description: Option<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    prep_time: Option<&'a str>,
    cook_time: Option<&'a str>,
    title: &'a str,
    url: Option<&'a str>,
    r#yield: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            description: r.description.map_or(Vec::new(), |s| {
                vec![RecipeDescriptionFieldEnum::Text(s.into())]
            }),
            name: vec![r.title.into()],
            is_based_on_url: r.url.map_or(Vec::new(), |s| {
                vec![RecipeIsBasedOnUrlFieldEnum::URL(s.into())]
            }),
            recipe_yield: r.r#yield.map_or(Vec::new(), |s| {
                vec![RecipeRecipeYieldFieldEnum::Text(s.into())]
            }),
            prep_time: r
                .prep_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.into())]),
            cook_time: r
                .cook_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.into())]),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            ..Default::default()
        }
    }
}

/// Parses a `Le Collectionneur de Recettes` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let archive = ZipArchive::new(r)?;

    let (recipes, _) = extract_archive_contents(
        archive,
        &Parsers {
            html: Some(parse_html),
            txt: Some(parse_txt),
            ..Default::default()
        },
    )?;

    Ok(recipes)
}

/// Parses an `Le Collectionneur de Recettes` HTML file.
pub fn parse_html<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    todo!()
}

/// Parses an `Le Collectionneur de Recettes` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let recipes = parse_txt_helper(&mut content.as_str())?;
    Ok(recipes.into_iter().map(Into::into).collect())
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> Result<Vec<RecipeComponents<'s>>> {
    repeat(1.., parse_recipe_txt)
        .parse_next(input)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn parse_recipe_txt<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        title: parse_title,
        url: opt(parse_url),
        description: opt(parse_description),
        r#yield: opt(parse_servings),
        prep_time: opt(parse_prep),
        cook_time: opt(parse_cook),
        ingredients: parse_ingredients,
        instructions: parse_instructions,
    }}
    .parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, multispace0).parse_next(input)
}

fn parse_url<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(till_line_ending, multispace0)
        .verify(|s: &str| s.starts_with("http"))
        .parse_next(input)
}

fn parse_description<'s>(input: &mut &'s str) -> WResult<&'s str> {
    peek(not(alt((
        (multispace0, literal("For: ")),
        (multispace0, literal("Preparation: ")),
        (multispace0, literal("Cooking: ")),
    ))))
    .parse_next(input)?;

    terminated(till_line_ending, multispace0).parse_next(input)
}

fn parse_servings<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(literal("For: "), till_line_ending, multispace0).parse_next(input)
}

fn parse_prep<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(literal("Preparation: "), till_line_ending, multispace0).parse_next(input)
}

fn parse_cook<'s>(input: &mut &'s str) -> WResult<&'s str> {
    delimited(literal("Cooking: "), till_line_ending, multispace0).parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    terminated(
        alt((
            take_until(1.., "\n\n\n").map(|s: &str| {
                s.split("\n\n")
                    .flat_map(|block| {
                        block.lines().map(|s| {
                            let s = s.replacen("            ", "", 1);
                            if s.starts_with(' ') {
                                Ingredient::Line(Cow::Owned(s))
                            } else {
                                Ingredient::Section(Cow::Owned(s))
                            }
                        })
                    })
                    .collect()
            }),
            repeat_till(
                0..,
                terminated(till_line_ending, (line_ending, space0)),
                (space0, line_ending),
            )
            .map(|(lines, _): (Vec<&str>, _)| {
                lines
                    .into_iter()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| Ingredient::Line(Cow::Borrowed(s)))
                    .collect()
            }),
        )),
        multispace1,
    )
    .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    repeat_till(
        1..,
        (till_line_ending, opt(line_ending)).map(|(s, _): (&'s str, _)| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else if s.starts_with(|c: char| c.is_ascii_digit()) {
                let rest = s.trim_start_matches(|c: char| c.is_ascii_digit());
                let rest = rest.strip_prefix(')').unwrap_or(rest).trim_start();
                Some(Instruction::Line(Cow::Borrowed(rest)))
            } else {
                Some(Instruction::Section(Cow::Borrowed(s)))
            }
        }),
        eof,
    )
    .map(|(lines, _): (Vec<Option<Instruction<'s>>>, _)| lines.into_iter().flatten().collect())
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use std::io::Cursor;

        use super::*;

        #[test]
        fn test_collect_html_ok() -> Result<()> {
            let buf = Cursor::new(files::html());

            let got = parse_html(buf)?;

            let expected = results::recipe();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }

        #[test]
        fn test_collect_html2_ok() -> Result<()> {
            let buf = Cursor::new(files::html2());

            let got = parse_html(buf)?;

            let expected = results::recipe2();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }

        #[test]
        fn test_collect_txt_ok() -> Result<()> {
            let buf = Cursor::new(files::txt());

            let got = parse_txt(buf)?;

            let expected = results::recipe();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }

        #[test]
        fn test_collect_txt2_ok() -> Result<()> {
            let buf = Cursor::new(files::txt2());

            let got = parse_txt(buf)?;

            let expected = results::recipe2();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }
    }

    mod files {
        pub fn html<'a>() -> &'a str {
            r#"<html>
             <head>
              <meta charset="UTF-8">
              <title></title>
             </head>
             <body>
              <div id="rcpcontainer">
               <h3>Sautéed chicken with maple</h3>
               <div id="presentation"></div>
               <div style="clear:both">
                <table>
                 <tbody>
                  <tr>
                   <td id="inglist" style="width: 40.0%;">
                    <div id="metainfo">
                     <ul>
                      <li> <span class="metalabel">For: </span> <span class="metavalue">4 portions</span> </li>
                      <li> <span class="metalabel">Preparation: </span> <span class="metavalue">15 minutes</span> </li>
                      <li> <span class="metalabel">Cooking: </span> <span class="metavalue">20 minutes</span> </li>
                     </ul>
                    </div>
                    <ul>
                     <li>600 g (about 1 ½ lb.) chicken, cut into strips</li>
                     <li>1 bag (500 g / 18 oz.) frozen vegetables (Asian mix)</li>
                     <li>60 ml (¼ cup) maple syrup</li>
                     <li>250 ml (1 cup) your favourite store-bought Asian sauce</li>
                     <li>Salt and freshly ground pepper, to taste</li>
                    </ul> </td>
                   <td id="inslist">
                    <div id="mainImg">
                     <img src="Saut%C3%A9ed%20chicken%20with%20maple_1.jpg">
                    </div>
                    <ul style="clear:both">
                     <li> Sauté the chicken in oil for 3 minutes over high heat. Add the remaining ingredients. </li>
                     <li> Simmer uncovered for 15 minutes, stirring occasionally. </li>
                     <li> Season with salt and pepper. </li>
                    </ul> </td>
                  </tr>
                 </tbody>
                </table>
               </div>
              </div>
             </body>
            </html>"#
        }

        pub fn html2<'a>() -> &'a str {
            r#"<html>
             <head>
              <meta charset="UTF-8">
              <title></title>
             </head>
             <body>
              <div id="rcpcontainer">
               <h3>Barbequed Curried Chicken Burgers with Yogurt Sauce</h3>
               <div id="presentation">
                Succulent on toasted wholegrain buns, piled with vegetable toppings.
               </div>
               <div style="clear:both">
                <table>
                 <tbody>
                  <tr>
                   <td id="inglist" style="width: 40.0%;">
                    <div id="metainfo">
                     <ul>
                      <li> <span class="metalabel">For: </span> <span class="metavalue">4</span> </li>
                      <li> <span class="metalabel">Preparation: </span> <span class="metavalue">15 minutes</span> </li>
                      <li> <span class="metalabel">Cooking: </span> <span class="metavalue">12-15 minutes</span> </li>
                     </ul>
                    </div>
                    <ul>
                     <li class="ingredientGroup">Yogurt Sauce
                      <ul>
                       <li>1 clove garlic, finely chopped</li>
                       <li>1/4 tsp (1 mL) fresh ginger, grated</li>
                       <li>1 1/2 tbsp (22 mL) maple syrup</li>
                       <li>Juice and finely grated zest of one lime</li>
                       <li>1 1/4 cup (310 mL) plain Greek yogurt</li>
                      </ul> </li>
                     <li class="ingredientGroup">Chicken Burgers
                      <ul>
                       <li>1 lb (450 g) ground chicken</li>
                       <li>1 French shallot or</li>
                       <li>green onion, chopped</li>
                       <li>1 clove garlic, finely chopped</li>
                       <li>1/2 tsp (2.5 mL) ground turmeric</li>
                       <li>2 tbsp (30 mL) curry powder</li>
                       <li>1/4 cup (60 mL) fresh coriander (cilantro), chopped</li>
                       <li>A small drizzle of olive oil</li>
                      </ul> </li>
                    </ul> </td>
                   <td id="inslist">
                    <div id="mainImg">
                     <img src="Barbequed%20Curried%20Chicken%20Burgers%20with%20Yogurt%20Sauce_1.jpg">
                    </div>
                    <ul style="clear:both">
                     <li class="instructionGroup">Sauce
                      <ul>
                       <li> Stir all sauce ingredients together in a bowl. Adjust seasoning to taste and set aside. </li>
                      </ul> </li>
                     <li class="instructionGroup">Patties
                      <ul>
                       <li> In another bowl, combine chicken with remaining ingredients. Lightly oil hands and form 4 patties. Oil patties and season surface. Barbecue 12 to 15 minutes or until chicken is cooked. </li>
                      </ul> </li>
                    </ul>
                    <ul id="mynotes">
                     <li><b>My notes</b></li>
                     <li>The chicken burger patty should be moist and fat. Try to buy the greenest veggies you can find.</li>
                    </ul> </td>
                  </tr>
                 </tbody>
                </table>
               </div>
              </div>
             </body>
            </html>"#
        }

        pub fn txt<'a>() -> &'a str {
            r"Sautéed chicken with maple
            http://ilovemaple.ca/recipes/sauteed-chicken-maple

            For: 4 portions
            Preparation: 15 minutes
            Cooking: 20 minutes

            600 g (about 1 ½ lb.) chicken, cut into strips
            1 bag (500 g / 18 oz.) frozen vegetables (Asian mix)
            60 ml (¼ cup) maple syrup
            250 ml (1 cup) your favourite store-bought Asian sauce
            Salt and freshly ground pepper, to taste

            1) Sauté the chicken in oil for 3 minutes over high heat. Add the remaining ingredients.

            2) Simmer uncovered for 15 minutes, stirring occasionally.

            3) Season with salt and pepper.

"
        }

        pub fn txt2<'a>() -> &'a str {
            r"Barbequed Curried Chicken Burgers with Yogurt Sauce
            http://www.dairygoodness.ca/recipes/barbequed-curried-chicken-burgers-with-yogurt-sauce

            Succulent on toasted wholegrain buns, piled with vegetable toppings.

            For: 4
            Preparation: 15 minutes
            Cooking: 12-15 minutes

            Yogurt Sauce
                1 clove garlic, finely chopped
                1/4	tsp	(1 mL) fresh ginger, grated
                1 1/2 tbsp (22 mL) maple syrup
                Juice and finely grated zest of one lime
                1 1/4 cup (310 mL) plain Greek yogurt

            Chicken Burgers
                1 lb (450 g) ground chicken
                1 French shallot or
                green onion, chopped
                1 clove garlic, finely chopped
                1/2	tsp	(2.5 mL) ground turmeric
                2 tbsp	(30 mL) curry powder
                1/4	cup	(60 mL) fresh coriander (cilantro), chopped
                A small drizzle of olive oil


            Sauce

                1) Stir all sauce ingredients together in a bowl. Adjust seasoning to taste and set aside.

            Patties

                2) In another bowl, combine chicken with remaining ingredients. Lightly oil hands and form 4 patties. Oil patties and season surface. Barbecue 12 to 15 minutes or until chicken is cooked.

"
        }
    }

    mod results {
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        };

        use super::*;

        pub fn recipe() -> Vec<Recipe> {
            vec![Recipe {
                name: vec!["Sautéed chicken with maple".into()],
                is_based_on_url: vec![RecipeIsBasedOnUrlFieldEnum::URL(
                    "http://ilovemaple.ca/recipes/sauteed-chicken-maple".into(),
                )],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4 portions".into())],
                prep_time: vec![DurationOrText::Text("15 minutes".into())],
                cook_time: vec![DurationOrText::Text("20 minutes".into())],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text(
                        "600 g (about 1 ½ lb.) chicken, cut into strips".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 bag (500 g / 18 oz.) frozen vegetables (Asian mix)".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("60 ml (¼ cup) maple syrup".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "250 ml (1 cup) your favourite store-bought Asian sauce".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "Salt and freshly ground pepper, to taste".into(),
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Sauté the chicken in oil for 3 minutes over high heat. Add the remaining ingredients.".into(),
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Simmer uncovered for 15 minutes, stirring occasionally.".into(),
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text("Season with salt and pepper.".into()),
                ],
                ..Default::default()
            }]
        }

        pub fn recipe2() -> Vec<Recipe> {
            vec![Recipe {
                name: vec!["Barbequed Curried Chicken Burgers with Yogurt Sauce".into()],
                is_based_on_url: vec![RecipeIsBasedOnUrlFieldEnum::URL(
                    "http://www.dairygoodness.ca/recipes/barbequed-curried-chicken-burgers-with-yogurt-sauce".into(),
                )],
                description: vec![RecipeDescriptionFieldEnum::Text("Succulent on toasted wholegrain buns, piled with vegetable toppings.".into())],
                recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4".into())],
                prep_time: vec![DurationOrText::Text("15 minutes".into())],
                cook_time: vec![DurationOrText::Text("12-15 minutes".into())],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::new_section("Yogurt Sauce", &[
                        "1 clove garlic, finely chopped",
                        "1/4 tsp (1 mL) fresh ginger, grated",
                        "1 1/2 tbsp (22 mL) maple syrup",
                        "Juice and finely grated zest of one lime",
                        "1 1/4 cup (310 mL) plain Greek yogurt",
                    ]),
                    RecipeRecipeIngredientFieldEnum::new_section("Chicken Burgers", &[
                        "1 lb (450 g) ground chicken",
                        "1 French shallot or",
                        "green onion, chopped",
                        "1 clove garlic, finely chopped",
                        "1/2 tsp (2.5 mL) ground turmeric",
                        "2 tbsp (30 mL) curry powder",
                        "1/4 cup (60 mL) fresh coriander (cilantro), chopped",
                        "A small drizzle of olive oil",
                    ])
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::new_section("Sauce", vec![
                        "Stir all sauce ingredients together in a bowl. Adjust seasoning to taste and set aside."
                    ]),
                    RecipeRecipeInstructionsFieldEnum::new_section("Patties", vec![
                        "In another bowl, combine chicken with remaining ingredients. Lightly oil hands and form 4 patties. Oil patties and season surface. Barbecue 12 to 15 minutes or until chicken is cooked."
                    ]),
                ],
                ..Default::default()
            }]
        }
    }
}
