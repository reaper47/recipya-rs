use std::borrow::Cow;
use std::io::{Read, Seek};

use scraper::{Html, Selector};
use winnow::Result as WResult;
use winnow::ascii::{line_ending, multispace0, multispace1, space0, till_line_ending};
use winnow::combinator::{alt, delimited, eof, not, opt, peek, seq, terminated};
use winnow::stream::AsChar;
use winnow::token::literal;
use winnow::{Parser, combinator::repeat};

use schema_org::field::{
    ItemListItemListElementFieldEnum, RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum,
    RecipeImageFieldEnum, RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum,
};
use schema_org::{AtType, Comment, DurationOrText, Recipe, at_context};

use crate::apps::helpers::{Ingredient, Parsers, ToSections, parse_archive_helper, read_file};
use crate::{Error, Result};

enum Instruction<'a> {
    Line(Cow<'a, str>),
    Section(Cow<'a, str>),
    Tip(Cow<'a, str>),
}

#[derive(Default)]
struct RecipeComponents<'a> {
    author: Option<&'a str>,
    description: Option<&'a str>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    image: Option<&'a str>,
    cook_time: Option<&'a str>,
    prep_time: Option<&'a str>,
    servings: Option<&'a str>,
    title: &'a str,
    r#yield: Option<&'a str>,
}

impl From<RecipeComponents<'_>> for Recipe {
    #[allow(clippy::too_many_lines)]
    fn from(r: RecipeComponents) -> Self {
        let mut comments = Vec::new();

        let recipe_instructions = r
            .instructions
            .into_iter()
            .fold(Vec::new(), |mut acc, item| match item {
                Instruction::Line(s)
                    if let Some(RecipeRecipeInstructionsFieldEnum::ItemList(list)) =
                        acc.last_mut() =>
                {
                    list.item_list_element
                        .push(ItemListItemListElementFieldEnum::Text(s.replace("Â", "")));
                    if let Some(i) = list.number_of_items.first_mut() {
                        *i += 1;
                    }
                    acc
                }
                Instruction::Line(s) => {
                    acc.push(RecipeRecipeInstructionsFieldEnum::Text(s.replace("Â", "")));
                    acc
                }
                Instruction::Section(s) => {
                    acc.push(RecipeRecipeInstructionsFieldEnum::new_section(
                        &s.replace("Â", ""),
                        Vec::<String>::new(),
                    ));
                    acc
                }
                Instruction::Tip(s) => {
                    comments.push(Comment {
                        text: vec![s.to_string()],
                        ..Default::default()
                    });
                    acc
                }
            });

        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            author: r
                .author
                .map(|s| vec![RecipeAuthorFieldEnum::new_person(s)])
                .unwrap_or_default(),
            description: r
                .description
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    vec![RecipeDescriptionFieldEnum::Text(
                        s.replace("Â", "")
                            .replace("’", "'")
                            .replace("Ã©", "é")
                            .trim()
                            .into(),
                    )]
                })
                .unwrap_or_default(),
            comment: comments,
            cook_time: r
                .cook_time
                .map(|s| vec![DurationOrText::Text(s.into())])
                .unwrap_or_default(),
            image: r
                .image
                .map(|s| vec![RecipeImageFieldEnum::URL(s.to_string())])
                .unwrap_or_default(),
            name: vec![r.title.into()],
            prep_time: r
                .prep_time
                .map(|s| s.split(',').next().unwrap_or_default())
                .map(|s| vec![DurationOrText::Text(s.into())])
                .unwrap_or_default(),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions,
            recipe_yield: [r.servings, r.r#yield]
                .into_iter()
                .find_map(|s| s.map(|o| vec![RecipeRecipeYieldFieldEnum::Text(o.into())]))
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Parses a `Cook'n` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    Ok(parse_txt_helper(
        &mut read_file(r)?
            .replace("\r\n", "\n")
            .replace('\r', "\n")
            .as_ref(),
    )?
    .into_iter()
    .map(Recipe::from)
    .collect())
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> Result<Vec<RecipeComponents<'s>>> {
    repeat(1.., parse_recipe_txt)
        .parse_next(input)
        .map_err(|err| Error::Parse(err.to_string()))
}

fn parse_recipe_txt<'s>(input: &mut &'s str) -> WResult<RecipeComponents<'s>> {
    seq! {RecipeComponents {
        _: parse_header,
        title: parse_title,
        author: opt(parse_author),
        description: opt(parse_description),
        servings: opt(parse_servings),
        prep_time: opt(parse_prep_time),
        cook_time: opt(parse_cook_time),
        r#yield: opt(parse_yield),
        ingredients: parse_ingredients,
        instructions: parse_instructions,
        _: parse_footer,
        ..Default::default()
    }}
    .parse_next(input)
}

fn parse_header<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(literal("@@@@@"), (line_ending, space0)).parse_next(input)
}

fn parse_title<'s>(input: &mut &'s str) -> WResult<&'s str> {
    terminated(
        till_line_ending,
        ((line_ending, space0), literal('|'), (line_ending, space0)),
    )
    .parse_next(input)
}

fn parse_author<'s>(input: &mut &'s str) -> WResult<&'s str> {
    alt((
        delimited(
            (literal("from the kitchen of:"), multispace0),
            till_line_ending,
            multispace0,
        ),
        delimited(literal("By "), till_line_ending, multispace1),
    ))
    .parse_next(input)
}

fn parse_description<'s>(input: &mut &'s str) -> WResult<&'s str> {
    let start = *input;

    loop {
        if peek(not(is_metadata_line)).parse_next(input).is_err() {
            break;
        }

        terminated(till_line_ending, multispace0).parse_next(input)?;
    }

    let consumed_len = start.len() - input.len();
    Ok(&start[..consumed_len])
}

fn is_metadata_line<'s>(input: &mut &'s str) -> WResult<&'s str> {
    alt((
        literal("Serves:"),
        literal("Prep Time:"),
        literal("Cook Time:"),
        literal("Yield:"),
    ))
    .parse_next(input)
}

fn parse_servings<'s>(input: &mut &'s str) -> WResult<&'s str> {
    parse_metaline(input, "Serves: ")
}

fn parse_prep_time<'s>(input: &mut &'s str) -> WResult<&'s str> {
    parse_metaline(input, "Prep Time: ")
}

fn parse_cook_time<'s>(input: &mut &'s str) -> WResult<&'s str> {
    parse_metaline(input, "Cook Time: ")
}

fn parse_yield<'s>(input: &mut &'s str) -> WResult<&'s str> {
    parse_metaline(input, "Yield: ")
}

fn parse_metaline<'s>(input: &mut &'s str, prefix: &'s str) -> WResult<&'s str> {
    delimited(literal(prefix), till_line_ending, multispace0).parse_next(input)
}

fn parse_ingredients<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    delimited(
        (literal('|'), multispace0),
        parse_ingredient_lines,
        (literal('|'), multispace0),
    )
    .parse_next(input)
}

fn parse_ingredient_lines<'s>(input: &mut &'s str) -> WResult<Vec<Ingredient<'s>>> {
    repeat(1.., parse_ingredient_line).parse_next(input)
}

fn parse_ingredient_line<'s>(input: &mut &'s str) -> WResult<Ingredient<'s>> {
    delimited(not(literal('|')), till_line_ending, (line_ending, space0))
        .map(|s: &str| {
            let without_leading_digits = s.trim_start_matches(|c: char| c.is_ascii_digit()).trim();
            if without_leading_digits
                .chars()
                .all(|c: char| c.is_uppercase() || c.is_whitespace())
            {
                Ingredient::Section(Cow::Borrowed(without_leading_digits))
            } else {
                Ingredient::Line(Cow::Borrowed(s))
            }
        })
        .parse_next(input)
}

fn parse_instructions<'s>(input: &mut &'s str) -> WResult<Vec<Instruction<'s>>> {
    repeat(1.., parse_instruction_line).parse_next(input)
}

fn parse_instruction_line<'s>(input: &mut &'s str) -> WResult<Instruction<'s>> {
    delimited(
        not(alt((literal("_____"), eof))),
        till_line_ending,
        multispace0,
    )
    .map(|s: &str| {
        s.strip_prefix("TIP: ").map_or_else(
            || {
                if s.chars()
                    .all(|c: char| c.is_uppercase() || c.is_whitespace())
                {
                    Instruction::Section(Cow::Borrowed(s))
                } else {
                    Instruction::Line(Cow::Borrowed(s))
                }
            },
            |rest| Instruction::Tip(Cow::Borrowed(rest)),
        )
    })
    .parse_next(input)
}

fn parse_footer<'s>(input: &mut &'s str) -> WResult<&'s str> {
    alt((terminated(literal("_____"), multispace0), eof)).parse_next(input)
}

/// Parses a `Cook'n` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper(
        r,
        &Parsers {
            html: Some(parse_html),
            ..Default::default()
        },
    )
}

#[allow(clippy::too_many_lines)]
fn parse_html<R: Read>(mut r: R) -> Result<Vec<Recipe>> {
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;

    let doc = Html::parse_document(&buf);

    let txt = |sel: &str| {
        doc.select(&Selector::parse(sel).unwrap())
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
    };

    let summary = txt("span[itemprop='summary']");

    let parts = summary.split("\n\n").collect::<Vec<_>>();
    let mut author: Option<&str> = None;
    if !parts.is_empty() {
        if parts[0].starts_with("from the kitchen of:") {
            author = Some(parts[0].trim_start_matches("from the kitchen of:").trim());
        } else if parts[0].starts_with("By ") {
            author = Some(parts[0].trim_start_matches("By ").trim());
        }
    }

    let ing_quantity = Selector::parse("span.ingredient-quantity").unwrap();
    let ing_unit = Selector::parse("span.ingredient-unit").unwrap();
    let ing_prefix = Selector::parse("span.ingredient-prefix").unwrap();
    let ing_food = Selector::parse("span.ingredient-food").unwrap();
    let ing_suffix = Selector::parse("span.ingredient-suffix").unwrap();

    let recipe = RecipeComponents {
        author,
        description: if author.is_none() && parts.len() == 1 {
            Some(parts[0])
        } else if parts.len() > 1 {
            Some(&parts[1..].join(""))
        } else {
            None
        },
        image: doc
            .select(&Selector::parse("img[itemprop='image']").unwrap())
            .next()
            .and_then(|el| el.attr("src")),
        ingredients: doc
            .select(&Selector::parse("span[itemprop='ingredient']").unwrap())
            .map(|el| {
                let quantity = el
                    .select(&ing_quantity)
                    .next()
                    .map(|el| el.text().collect::<String>());
                let unit = el
                    .select(&ing_unit)
                    .next()
                    .map(|el| el.text().collect::<String>());
                let prefix = el
                    .select(&ing_prefix)
                    .next()
                    .map(|el| el.text().collect::<String>());
                let food = el
                    .select(&ing_food)
                    .next()
                    .map(|el| el.text().collect::<String>());
                let suffix = el
                    .select(&ing_suffix)
                    .next()
                    .map(|el| el.text().collect::<String>());

                if prefix
                    .as_deref()
                    .is_some_and(|p| p.chars().all(|c: char| c.is_space() || c.is_uppercase()))
                {
                    Ingredient::Section(Cow::Owned(prefix.unwrap_or_default()))
                } else {
                    Ingredient::Line(Cow::Owned(
                        [quantity, unit, prefix, food, suffix]
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>()
                            .join(" "),
                    ))
                }
            })
            .collect(),
        instructions: doc
            .select(&Selector::parse("div[itemprop='recipeInstructions']").unwrap())
            .next()
            .map(|el| {
                el.text()
                    .filter_map(|s| {
                        let s = s.trim();
                        if s.is_empty()
                            || s.starts_with("Recipe formatted with the Cook'n")
                            || s.starts_with("Recipe Software")
                            || s.contains("DVO Enter")
                        {
                            return None;
                        }
                        Some(s.strip_prefix("TIP:").map_or_else(
                            || {
                                if s.chars().all(|c| c.is_uppercase() || c.is_whitespace()) {
                                    Instruction::Section(Cow::Borrowed(s))
                                } else {
                                    Instruction::Line(Cow::Borrowed(s))
                                }
                            },
                            |tip| Instruction::Tip(Cow::Borrowed(tip.trim())),
                        ))
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        cook_time: doc
            .select(&Selector::parse("time[itemprop='cookTime']").unwrap())
            .next()
            .map(|el| el.text().next().unwrap_or_default()),
        prep_time: doc
            .select(&Selector::parse("time[itemprop='prepTime']").unwrap())
            .next()
            .map(|el| el.text().next().unwrap_or_default()),
        servings: doc
            .select(&Selector::parse("span.recipe-servingSize").unwrap())
            .next()
            .map(|el| el.text().next().unwrap_or_default()),
        title: doc
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|el| el.text().next().unwrap_or_default())
            .unwrap_or_default(),
        r#yield: doc
            .select(&Selector::parse("span[itemprop='recipeYield']").unwrap())
            .next()
            .map(|el| el.text().next().unwrap_or_default()),
    };

    Ok(vec![recipe.into()])
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use std::io::Cursor;

        use test_fixtures::open_test_file;

        use super::*;

        #[test]
        fn test_cookn_txt_ok() -> Result<()> {
            let buf = Cursor::new(files::txt());

            let got = parse_txt(buf)?;

            pretty_assertions::assert_eq!(got.len(), 11);
            pretty_assertions::assert_eq!(got, results::txt());
            Ok(())
        }

        #[test]
        fn test_cookn_archive_ok() -> Result<()> {
            let buf = open_test_file("integrations/cookn.zip");

            let mut got = parse_archive(buf)?;

            pretty_assertions::assert_eq!(got.len(), 3);
            let want = results::txt();
            for r in &mut got {
                r.image.clear();
            }
            for name in [
                "Apple Raisin Strata",
                "Blackberry Syrup",
                "Brats and Cinnamon Apple Topping",
            ] {
                pretty_assertions::assert_eq!(
                    got.iter().find(|r| r.name[0] == name).unwrap(),
                    want.iter().find(|r| r.name[0] == name).unwrap()
                );
            }
            Ok(())
        }

        #[test]
        fn test_html_ok() -> Result<()> {
            let buf = Cursor::new(files::html());

            let mut got = parse_html(buf)?;

            got[0].image = vec![];
            pretty_assertions::assert_eq!(
                got[0],
                results::txt()
                    .into_iter()
                    .find(|r| r.name[0] == "Apple Raisin Strata")
                    .unwrap()
            );
            Ok(())
        }
    }

    mod files {
        #[allow(clippy::too_many_lines)]
        pub fn html<'a>() -> &'a str {
            r#"	<!DOCTYPE html>
	<html lang="en">
		<head>
			<meta charset="UTF-8" />
			<title>Apple Raisin Strata</title>
		</head>
		<body>
			<div style="margin: 15px;">
				<!-- Recipe -->
				<div itemscope itemtype="http://schema.org/Recipe" >
					<h1 itemprop="name">Apple Raisin Strata</h1>
						<img itemprop="image" src="4.jpg" alt="" style="margin: 4px;" />
					<p style="font-style: italic;"><span itemprop="summary">from the kitchen of:
       Marion Albright

       The flavors of apples and raisins blend beautifully to create an elegant yet simple and fast breakfast. Serve this with a drizzle of maple syrup or caramel and a dollop of sweetened whipped cream as a lovely way to start the day.</span></p>
					<div style="font-size: 0.85em;">
						Prep time: <time datetime="" itemprop="prepTime">20 minutes, 2 hours refrigerated</time><br/>
						Cook time: <time datetime="" itemprop="cookTime">45 minutes</time><br/>

						Serving size: <span class="recipe-servingSize">12</span><br/>
							<span itemprop="nutrition" itemscope itemtype="http://schema.org/NutritionInformation">
								Calories per serving: <span itemprop="calories">494</span><br/>
							</span>
						<br/>
					</div>

					<b>Ingredients:</b>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1</span> <span class="ingredient-unit">(1-pound) loaf</span> <span class="ingredient-prefix">cinnamon</span>
						<span class="ingredient-food">raisin bread</span> <span class="ingredient-suffix">cubed</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1</span> <span class="ingredient-unit">(8-ounce) package</span>
						<span class="ingredient-food">cream cheese</span> <span class="ingredient-suffix">diced</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1 1/2</span> <span class="ingredient-unit">cups</span> <span class="ingredient-prefix">peeled and diced</span>
						<span class="ingredient-food">apples</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1 1/2</span> <span class="ingredient-unit">cups</span>
						<span class="ingredient-food">raisins</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">8</span>
						<span class="ingredient-food">eggs</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">2 1/2</span> <span class="ingredient-unit">cups</span>
						<span class="ingredient-food">half and half</span> <span class="ingredient-suffix">or cream</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">6</span> <span class="ingredient-unit">tablespoons</span>
						<span class="ingredient-food">butter</span> <span class="ingredient-suffix">melted</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1/3</span> <span class="ingredient-unit">cup</span>
						<span class="ingredient-food">maple syrup</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1</span>  <span class="ingredient-prefix">CARAMEL SAUCE</span>

					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1/2</span> <span class="ingredient-unit">cup</span>
						<span class="ingredient-food">brown sugar</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1/4</span> <span class="ingredient-unit">cup</span>
						<span class="ingredient-food">cream</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1/4</span> <span class="ingredient-unit">cup</span>
						<span class="ingredient-food">butter</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">1/8</span> <span class="ingredient-unit">teaspoon</span>
						<span class="ingredient-food">salt</span>
					</span>
					<br/>
					<span itemprop="ingredient" style="margin-left:7px;">
						<span class="ingredient-quantity">2</span> <span class="ingredient-unit">teaspoons</span>
						<span class="ingredient-food">vanilla extract</span>
					</span>
					<br/>
					<br/>
					<b>Directions:</b><br/>
					<div itemprop="recipeInstructions">
						Coat a 9x13-inch baking dish with cooking spray. Arrange half of the cubed raisin bread in the bottom of the dish. Sprinkle the cream cheese evenly over the bread and top with apples; sprinkle raisins evenly over all. Top with remaining bread cubes.
       <BR>
       <BR>In a large bowl, beat the eggs with the half-and-half, butter, and maple syrup. Pour this over the bread mixture. Cover with plastic wrap and press down so that all bread pieces are soaked. Refrigerate for at least 2 hours or overnight.
       <BR>
       <BR>Preheat oven to 325°F. Bake for 45 minutes. Let stand for 10 minutes before serving.
       <BR>
       <BR>CARAMEL SAUCE
       <BR>Mix the brown sugar, cream, butter, and salt in a saucepan over medium-low heat. Whisk while cooking for 5 minutes or until the sauce gets thick. Add vanilla and cook for 1 minute. Remove from heat, cool slightly, and pour into a jar.
       <BR>
       <BR>TIP: Freeze the cream cheese until just barely solid for easier dicing.
       <BR><P align=left><br>Recipe formatted with the Cook'n <a href="http://www.dvo.com/index.html?CID=export_to_html_feature">Recipe Software</a> from DVO Enterprises.
					</div>
				</div>
				<!-- /Recipe -->
			</div>

			<div style="width: 110px; height: 50px; margin: 50px auto;">
				<a href="http://www.dvo.com"><img id="cooknLogo" src="data:image/gif;base64,R0lGODlhAQABAIAAAP///////yH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==" alt="" border="0" height="40" width="88"/></a>
			</div>
		</body>
	</html>
"#
        }

        #[allow(clippy::too_many_lines)]
        pub fn txt<'a>() -> &'a str {
            r"@@@@@
                Apple Raisin Strata
                |
                from the kitchen of:

                Marion Albright



                The flavors of apples and raisins blend beautifully to create an elegant yet simple and fast breakfast. Serve this with a drizzle of maple syrup or caramel and a dollop of sweetened whipped cream as a lovely way to start the day.
                Serves: 12
                Prep Time: 20 minutes, 2 hours refrigerated
                Cook Time: 45 minutes
                |
                1 (1-pound) loaf cinnamon raisin bread cubed
                1 (8-ounce) package cream cheese diced
                1 1/2 cups peeled and diced apples
                1 1/2 cups raisins
                8 eggs
                2 1/2 cups half and half or cream
                6 tablespoons butter melted
                1/3 cup maple syrup
                1 CARAMEL SAUCE
                1/2 cup brown sugar
                1/4 cup cream
                1/4 cup butter
                1/8 teaspoon salt
                2 teaspoons vanilla extract
                |
                Coat a 9x13-inch baking dish with cooking spray. Arrange half of the cubed raisin bread in the bottom of the dish. Sprinkle the cream cheese evenly over the bread and top with apples; sprinkle raisins evenly over all. Top with remaining bread cubes.



                In a large bowl, beat the eggs with the half-and-half, butter, and maple syrup. Pour this over the bread mixture. Cover with plastic wrap and press down so that all bread pieces are soaked. Refrigerate for at least 2 hours or overnight.



                Preheat oven to 325°F. Bake for 45 minutes. Let stand for 10 minutes before serving.



                CARAMEL SAUCE

                Mix the brown sugar, cream, butter, and salt in a saucepan over medium-low heat. Whisk while cooking for 5 minutes or until the sauce gets thick. Add vanilla and cook for 1 minute. Remove from heat, cool slightly, and pour into a jar.



                TIP: Freeze the cream cheese until just barely solid for easier dicing.


                _____
                @@@@@
                Blackberry Syrup
                |
                By Annie Mays
                Cook Time: 35 minutes
                Yield: 3 cups
                |
                16 ounces frozen blackberries or 2 baskets fresh blackberries, divided
                1 cup sugar
                2 cups water
                1/2 cup water mixed with 2 tablespoons cornstarch
                |
                Reserve 1/2 cup of the berries. Put remaining berries, sugar, and 2 cups water in a pan and let simmer for 25 minutes. Add the ½ cup water and cornstarch mixture; stir constantly until contents thicken. This can be made ahead of time and reheated. When ready to serve, add remaining ½ cup berries to syrup; cook another 5 minutes or until berries are softened.
                _____
                @@@@@
                Brats and Cinnamon Apple Topping
                |
                By Bill Carter
                Serves: 6
                Prep Time: 20 minutes
                |
                1 package bratwurst
                1/2 cup apple juice
                6 cups peeled and thinly sliced apples
                1 tablespoon cinnamon
                1 cup brown sugar
                |
                Slice brats (if not pre-sliced) and brown in apple juice in a large frying pan. Add apples, cinnamon, and brown sugar. Cook together until meat is done and apples are soft. Serve hot over pancakes.



                TIP: Grill up a few extra brats to serve on the side for the dad who loves his meat for breakfast.


                _____
                @@@@@
                Farmer's Casserole
                |
                Gary and Pat Teske, retired innkeepers of The Thistle Inn, of Holland, Michigan, share their recipe for one of their guests favorite breakfasts.
                Serves: 6
                Cook Time: 45 minutes
                |
                3 cups frozen shredded hash brown potatoes
                3/4 cup Monterey Jack cheese with jalapeno peppers or shredded sharp Cheddar cheese
                1 cup diced fully cooked ham or Canadian bacon
                1/4 cup sliced green onions
                4 beaten eggs
                1 (12-ounce) can evaporated milk
                1/4 teaspoon pepper
                1/8 teaspoon salt
                |
                Heat oven to 350°F. Spray a 2-quart square baking dish with cooking spray. Arrange potatoes evenly in the bottom of the dish. Sprinkle with cheese, ham, and green onion.



                In a medium mixing bowl, combine eggs, milk, pepper, and salt. Pour egg mixture over potato mixture in the dish. (The dish may be covered and refrigerated at this point for several hours or overnight.)



                Bake uncovered for 40 to 45 minutes (or 55 to 60 minutes if made ahead and chilled) or until the center appears set. Let stand 5 minutes before serving.


                _____
                @@@@@
                Fluffiest Pancakes
                |
                Prep Time: 20 minutes
                Yield: 6-8 large pancakes
                |
                1 cup flour
                2 tablespoons sugar
                2 tablespoons baking powder
                1/2 teaspoon salt
                2 tablespoons canola oil
                1 large egg
                1 cup buttermilk
                2 tablespoons water
                |
                Sift together all dry ingredients in a large bowl and create a well in the middle. Add oil, egg, and buttermilk. Mix lightly by hand using a spoon. If the batter is too thick, add 2 tablespoons water. Note: Buttermilk can come in various consistencies, from thick to thin, which can affect the consistency of your batter. For large pancakes, pour ¼ cup batter per cake onto a preheated griddle. Flip pancakes when edges look set and bubbles in the middle begin to pop. Cook an additional minute or until golden brown.
                _____
                @@@@@
                Leek Gratine
                |
                A gratiné is quiche without the crust. It is traditionally made in a shallow-sided oval pan or dish, but a round or rectangular oven-safe dish may also be used. The gratiné dish allows the food to cook evenly while the top browns nicely. For an impressive and tasty quiche-like dish (without the stress of making a pie crust), gratiné is the way to go!
                Serves: 6
                Cook Time: 25 minutes
                |
                2 tablespoons butter divided
                6 slices bacon
                8 medium-sized leeks
                1/2 cup water
                4 eggs
                1 1/2 cups heavy cream
                1/4 teaspoon ground nutmeg
                1/2 teaspoon salt
                1/4 teaspoon freshly ground black pepper or to taste
                1/2 cup grated sharp cheddar cheese
                1/4 cup grated Parmesan cheese
                |
                Preheat oven to 375°F. Using 1 tablespoon of the butter, lightly butter a cooking dish or pan. Brown bacon in a 10-inch skillet. While bacon is cooking, wash and slice leeks into 1/2-inch rounds. When bacon has browned, remove most of the fat and then add leeks and water to the skillet. Cover and simmer over low heat for 20 minutes or until leeks are tender and have absorbed the water. Evaporate any remaining water over medium heat, uncovered. Be sure to stir leeks occasionally while cooking to prevent burning. Remove leeks and bacon to the bowl of a food processor or blender. Add eggs, cream, nutmeg, salt, and pepper. Process or blend for a few seconds at a time until the bacon is chopped. (Avoid over-processing; you want a chunky consistency, not puree.) Pour the mixture into the prepared pie plate. Sprinkle grated cheeses and pieces of the remaining tablespoon of butter on top. Bake for 25 minutes or until the custard is set.



                TIP: If the top begins to appear as if it will crack prior to completely baking through, spray or sprinkle several tablespoons of water on the walls of the hot oven in order to create steam, which will keep the top moist.


                _____
                @@@@@
                Mixed Berry Breakfast Smoothie
                |
                from the kitchen of:

                Marisa Fitzgerald; itsdinnertime2.blogspot.com



                This healthy and quick smoothie is refreshing and satisfying. Loaded with fiber and vitamins, this crowd-pleasing drink will lure even the sleepiest to the breakfast table.


                Prep Time: 5 minutes
                |
                1/2 cup skim milk
                1/2 cup water
                8 sucarlose packets (or 1 teaspoon stevia, if preferred)
                2 tablespoons orange juice concentrate
                1 1/2 cups frozen mixed berries
                2/3 teaspoon vanilla extract
                1/2 cup ice (optional)
                |
                Place all ingredients, except ice, into a blender and blend first on medium, then on high speed. Add ice if you prefer a thicker drink. If desired, add more vanilla to bring out the sweetness or increase the orange juice concentrate to give your smoothie a little extra zip.
                _____
                @@@@@
                Naturally Sweet Oatmeal
                |
                from the kitchen of:

                Susan; 5minutesformom.com



                This gluten-free and naturally sweetened oatmeal gets the day off to a perfect start. Blogger Susan says that amounts don't need to be exact, but suggests chopping the prunes first to create a sweeter tasting oatmeal.




                Prep Time: 5 minutes
                Cook Time: 1 1/2 minutes
                |
                1/2 cup oats
                2 tablespoons ground flaxseeds
                1 tablespoon ground chia seeds
                1/2 teaspoon cinnamon
                1 pinch salt (optional)
                3/4 cup water
                6 dried pitted prunes
                1/2 cup warmed milk (optional)
                |
                Mix dry ingredients in a medium bowl. Add water and prunes. Microwave on high for 1½ minutes. Stir and add warmed milk, if desired.



                _____
                @@@@@
                Vegetable and Bacon Quiche
                |
                from the kitchen of:

                Lori Hart; snowluvnferret.blogspot.com



                This versatile recipe works well with any vegetables on hand. Packed with protein, it cuts calories and fat by using a rice crust instead of traditional piecrust.


                Serves: 6
                Prep Time: 20 minutes
                Cook Time: 45 mintues
                |
                3 beaten eggs
                1/2 cup grated cheese of choice
                1 small zucchini sliced
                1 Roma tomato chopped
                5 slices bacon precooked and chopped
                1/4 teaspoon pepper or seasoning of choice
                1 asparagus spears for garnish
                1 RICE CRUST
                1 1/2 cups cooked rice (white or brown)
                1/4 cup grated cheese of choice
                1 beaten egg
                1/4 teaspoon dried dill
                1 clove garlic chopped
                |
                Place zucchini on warm crust; add bacon and tomatoes. Mix egg, cheese, and seasoning and pour over vegetables and bacon. If desired, top with asparagus spears. Bake for 40 to 45 minutes at 350°F.



                RICE CRUST

                Preheat oven to 350°F. Mix all ingredients together and press into an oiled 8-inch pie plate. Bake for 15 to 20 minutes.



                TIP: This quiche can be prepared the night before and kept covered in the refrigerator until ready to bake the next morning.

                _____
                @@@@@
                Cheesecake and Chocolate Stuffed Strawberries
                |
                This treat is as tasty as it is pretty to look at, and so easy to prepare! Weve all seen berries dipped in chocolate, but consider how clever Mom will think you are by stuffing them instead.
                Yield: 1-2 Pints
                |
                CHOCOLATE STUFFING
                1/2 cup sugar
                2 2/3 tablespoons milk
                2 1/2 tablespoons butter
                1/2 cup semisweet chocolate chips
                1/4 cup finely chopped pecans if desired
                CHEESECAKE STUFFING
                1 (8-ounce) package cream cheese softened
                3 tablespoons milk or cream
                1/2 teaspoon almond extract
                1 tablespoon powdered sugar
                |
                DIRECTIONS FOR CHOCOLATE STUFFING

                Put the first 3 ingredients in a small saucepan and bring to a boil, stirring constantly. Remove from heat and add chocolate chips. Beat with a wire whisk until chocolate mixture is creamy and cooled. Add chopped nuts, if desired.



                DIRECTIONS FOR CHEESECAKE STUFFING

                In a medium bowl, mix cream cheese, milk or cream, almond extract, and powdered sugar together until consistency is smooth. Fill an icing cone or small plastic zippered bag (cut about 1/4 inch off one corner of bag) with mixture.



                TO STUFF BERRIES

                Carefully core out fresh, firm berries. Create a hollow space of about ½ to ¾ inch, depending on the size of the berries. Cut a small piece off the berry tip, so the berries can stand on their own. Pipe stuffing into berries, and arrange nicely on a serving plate or tray.


                _____
                @@@@@
                Chocolate Fudge Dream Cake
                |
                By Megan and Jill Stapley



                This extra-special cake was first prepared for a potluck by 16-year-old Megan Stapley, of Pinetop, Arizona. She started with two 8-inch cake rounds sliced in half to make four layers. Each layer was mounded with Cloud Nine Frosting. This dessert won raves and applause from everyone who tried it and Megans baking career was launched.



                Megans mother, Jill, toyed with the idea of covering the sides of this already amazing cake with her grandmas fudge frosting. Not only did the fudge frosting addition make the cake more decadent, it added to the beauty of the cake. And thus was born the now famous Chocolate Fudge Dream Cake.


                Serves: 8
                Prep Time: 20 minutes
                Cook Time: 30 minutes
                |
                3/4 cup butter softened
                2 cups sugar
                3/4 cup Dutch baking cocoa
                2 large eggs
                1 tablespoon baking soda
                3/4 teaspoon salt
                2 teaspoons vanilla extract
                1 cup buttermilk
                1 cup hot water
                3 cups flour
                |
                Preheat oven to 350°F. Butter and flour two round 9-inch cake pans. Cream the butter and sugar. Then add the cocoa and eggs and mix well. Add the baking soda, salt, and vanilla. Alternately blend the buttermilk, flour, and hot water. The batter should be smooth. Bake for 30-35 minutes or until an inserted toothpick comes out clean. Allow cake to cool before frosting.



                TIP: After buttering your cake pans dust with sugar instead of flour. This gives this cake a nice sugary texture that makes it easy to frost. Another option is to dust the buttered pans with cocoa powder.



                TO FROST THE CAKE: Place the first cake layer on a decorative serving plate or cake stand and generously frost the top with the Cloud Nine Frosting. Repeat with the next two layers, generously frosting the top of each. Place the fourth layer on top and leave unfrosted.



                Smooth Chocolate Fudge Frosting around the sides of all four layers of cake. Finish frosting the cake by adding the remaining Cloud Nine Frosting to the top layer of cake, mounding it in the middle.

            "
        }
    }

    mod results {
        use schema_org::field::{
            RecipeDescriptionFieldEnum, RecipeRecipeIngredientFieldEnum,
            RecipeRecipeInstructionsFieldEnum, RecipeYieldFieldEnum,
        };

        use super::*;

        #[allow(clippy::too_many_lines)]
        pub fn txt() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Marion Albright")],
                    cook_time: vec![DurationOrText::Text("45 minutes".into())],
                    description: vec![RecipeDescriptionFieldEnum::Text("The flavors of apples and raisins blend beautifully to create an elegant yet simple and fast breakfast. Serve this with a drizzle of maple syrup or caramel and a dollop of sweetened whipped cream as a lovely way to start the day.".into())],
                    comment:vec![Comment {
                        text: vec!["Freeze the cream cheese until just barely solid for easier dicing.".into()],
                        ..Default::default()
                    }],
                    name: vec!["Apple Raisin Strata".into()],
                    prep_time: vec![DurationOrText::Text("20 minutes".into())],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 (1-pound) loaf cinnamon raisin bread cubed".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 (8-ounce) package cream cheese diced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 cups peeled and diced apples".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 cups raisins".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 1/2 cups half and half or cream".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 tablespoons butter melted".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/3 cup maple syrup".into()),
                        RecipeRecipeIngredientFieldEnum::new_section("CARAMEL SAUCE", &[
                            "1/2 cup brown sugar",
                            "1/4 cup cream",
                            "1/4 cup butter",
                            "1/8 teaspoon salt",
                            "2 teaspoons vanilla extract",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Coat a 9x13-inch baking dish with cooking spray. Arrange half of the cubed raisin bread in the bottom of the dish. Sprinkle the cream cheese evenly over the bread and top with apples; sprinkle raisins evenly over all. Top with remaining bread cubes.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("In a large bowl, beat the eggs with the half-and-half, butter, and maple syrup. Pour this over the bread mixture. Cover with plastic wrap and press down so that all bread pieces are soaked. Refrigerate for at least 2 hours or overnight.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat oven to 325°F. Bake for 45 minutes. Let stand for 10 minutes before serving.".into()),
                        RecipeRecipeInstructionsFieldEnum::new_section("CARAMEL SAUCE", vec![
                            "Mix the brown sugar, cream, butter, and salt in a saucepan over medium-low heat. Whisk while cooking for 5 minutes or until the sauce gets thick. Add vanilla and cook for 1 minute. Remove from heat, cool slightly, and pour into a jar."
                        ]),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("12".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Annie Mays")],
                    cook_time: vec![DurationOrText::Text("35 minutes".into())],
                    name: vec!["Blackberry Syrup".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("16 ounces frozen blackberries or 2 baskets fresh blackberries, divided".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 cups water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup water mixed with 2 tablespoons cornstarch".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Reserve 1/2 cup of the berries. Put remaining berries, sugar, and 2 cups water in a pan and let simmer for 25 minutes. Add the ½ cup water and cornstarch mixture; stir constantly until contents thicken. This can be made ahead of time and reheated. When ready to serve, add remaining ½ cup berries to syrup; cook another 5 minutes or until berries are softened.".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("3 cups".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Bill Carter")],
                    prep_time: vec![DurationOrText::Text("20 minutes".into())],
                    comment:vec![Comment {
                        text: vec!["Grill up a few extra brats to serve on the side for the dad who loves his meat for breakfast.".into()],
                        ..Default::default()
                    }],
                    name: vec!["Brats and Cinnamon Apple Topping".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 package bratwurst".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup apple juice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 cups peeled and thinly sliced apples".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon cinnamon".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup brown sugar".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Slice brats (if not pre-sliced) and brown in apple juice in a large frying pan. Add apples, cinnamon, and brown sugar. Cook together until meat is done and apples are soft. Serve hot over pancakes.".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("6".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: vec![DurationOrText::Text("45 minutes".into())],
                    description: vec![RecipeDescriptionFieldEnum::Text("Gary and Pat Teske, retired innkeepers of The Thistle Inn, of Holland, Michigan, share their recipe for one of their guests\u{92} favorite breakfasts.".into())],
                    name: vec!["Farmer's Casserole".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("3 cups frozen shredded hash brown potatoes".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3/4 cup Monterey Jack cheese with jalapeno peppers or shredded sharp Cheddar cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup diced fully cooked ham or Canadian bacon".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 cup sliced green onions".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 beaten eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 (12-ounce) can evaporated milk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/8 teaspoon salt".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Heat oven to 350°F. Spray a 2-quart square baking dish with cooking spray. Arrange potatoes evenly in the bottom of the dish. Sprinkle with cheese, ham, and green onion.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("In a medium mixing bowl, combine eggs, milk, pepper, and salt. Pour egg mixture over potato mixture in the dish. (The dish may be covered and refrigerated at this point for several hours or overnight.)".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Bake uncovered for 40 to 45 minutes (or 55 to 60 minutes if made ahead and chilled) or until the center appears set. Let stand 5 minutes before serving.".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("6".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    prep_time: vec![DurationOrText::Text("20 minutes".into())],
                    name: vec!["Fluffiest Pancakes".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 cup flour".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons baking powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons canola oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 large egg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup buttermilk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons water".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Sift together all dry ingredients in a large bowl and create a well in the middle. Add oil, egg, and buttermilk. Mix lightly by hand using a spoon. If the batter is too thick, add 2 tablespoons water. Note: Buttermilk can come in various consistencies, from thick to thin, which can affect the consistency of your batter. For large pancakes, pour ¼ cup batter per cake onto a preheated griddle. Flip pancakes when edges look set and bubbles in the middle begin to pop. Cook an additional minute or until golden brown.".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("6-8 large pancakes".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: vec![DurationOrText::Text("25 minutes".into())],
                    comment:vec![Comment {
                        text: vec!["If the top begins to appear as if it will crack prior to completely baking through, spray or sprinkle several tablespoons of water on the walls of the hot oven in order to create steam, which will keep the top moist.".into()],
                        ..Default::default()
                    }],
                    description: vec![RecipeDescriptionFieldEnum::Text("A gratiné is quiche without the crust. It is traditionally made in a shallow-sided oval pan or dish, but a round or rectangular oven-safe dish may also be used. The gratiné dish allows the food to cook evenly while the top browns nicely. For an impressive and tasty quiche-like dish (without the stress of making a pie crust), gratiné is the way to go!".into())],
                    name: vec!["Leek Gratine".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons butter divided".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 slices bacon".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 medium-sized leeks".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 cups heavy cream".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon ground nutmeg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon freshly ground black pepper or to taste".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup grated sharp cheddar cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 cup grated Parmesan cheese".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat oven to 375°F. Using 1 tablespoon of the butter, lightly butter a cooking dish or pan. Brown bacon in a 10-inch skillet. While bacon is cooking, wash and slice leeks into 1/2-inch rounds. When bacon has browned, remove most of the fat and then add leeks and water to the skillet. Cover and simmer over low heat for 20 minutes or until leeks are tender and have absorbed the water. Evaporate any remaining water over medium heat, uncovered. Be sure to stir leeks occasionally while cooking to prevent burning. Remove leeks and bacon to the bowl of a food processor or blender. Add eggs, cream, nutmeg, salt, and pepper. Process or blend for a few seconds at a time until the bacon is chopped. (Avoid over-processing; you want a chunky consistency, not puree.) Pour the mixture into the prepared pie plate. Sprinkle grated cheeses and pieces of the remaining tablespoon of butter on top. Bake for 25 minutes or until the custard is set.".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("6".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Marisa Fitzgerald; itsdinnertime2.blogspot.com")],
                    prep_time: vec![DurationOrText::Text("5 minutes".into())],
                    description: vec![RecipeDescriptionFieldEnum::Text("This healthy and quick smoothie is refreshing and satisfying. Loaded with fiber and vitamins, this crowd-pleasing drink will lure even the sleepiest to the breakfast table.".into())],
                    name: vec!["Mixed Berry Breakfast Smoothie".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup skim milk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 sucarlose packets (or 1 teaspoon stevia, if preferred)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons orange juice concentrate".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 1/2 cups frozen mixed berries".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2/3 teaspoon vanilla extract".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup ice (optional)".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Place all ingredients, except ice, into a blender and blend first on medium, then on high speed. Add ice if you prefer a thicker drink. If desired, add more vanilla to bring out the sweetness or increase the orange juice concentrate to give your smoothie a little extra zip.".into()),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Susan; 5minutesformom.com")],
                    cook_time: vec![DurationOrText::Text("1 1/2 minutes".into())],
                    prep_time: vec![DurationOrText::Text("5 minutes".into())],
                    description: vec![RecipeDescriptionFieldEnum::Text("This gluten-free and naturally sweetened oatmeal gets the day off to a perfect start. Blogger Susan says that amounts don't need to be exact, but suggests chopping the prunes first to create a sweeter tasting oatmeal.".into())],
                    name: vec!["Naturally Sweet Oatmeal".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup oats".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons ground flaxseeds".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon ground chia seeds".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon cinnamon".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 pinch salt (optional)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3/4 cup water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 dried pitted prunes".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup warmed milk (optional)".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Mix dry ingredients in a medium bowl. Add water and prunes. Microwave on high for 1½ minutes. Stir and add warmed milk, if desired.".into()),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Lori Hart; snowluvnferret.blogspot.com")],
                    comment:vec![Comment {
                        text: vec!["This quiche can be prepared the night before and kept covered in the refrigerator until ready to bake the next morning.".into()],
                        ..Default::default()
                    }],
                    cook_time: vec![DurationOrText::Text("45 mintues".into())],
                    prep_time: vec![DurationOrText::Text("20 minutes".into())],
                    description: vec![RecipeDescriptionFieldEnum::Text("This versatile recipe works well with any vegetables on hand. Packed with protein, it cuts calories and fat by using a rice crust instead of traditional piecrust.".into())],
                    name: vec!["Vegetable and Bacon Quiche".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("3 beaten eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup grated cheese of choice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 small zucchini sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Roma tomato chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("5 slices bacon precooked and chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon pepper or seasoning of choice".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 asparagus spears for garnish".into()),
                        RecipeRecipeIngredientFieldEnum::new_section("RICE CRUST", &[
                            "1 1/2 cups cooked rice (white or brown)",
                            "1/4 cup grated cheese of choice",
                            "1 beaten egg",
                            "1/4 teaspoon dried dill",
                            "1 clove garlic chopped",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Place zucchini on warm crust; add bacon and tomatoes. Mix egg, cheese, and seasoning and pour over vegetables and bacon. If desired, top with asparagus spears. Bake for 40 to 45 minutes at 350°F.".into()),
                        RecipeRecipeInstructionsFieldEnum::new_section("RICE CRUST", vec![
                            "Preheat oven to 350°F. Mix all ingredients together and press into an oiled 8-inch pie plate. Bake for 15 to 20 minutes."
                        ]),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("6".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    description: vec![RecipeDescriptionFieldEnum::Text("This treat is as tasty as it is pretty to look at, and so easy to prepare! We\u{92}ve all seen berries dipped in chocolate, but consider how clever Mom will think you are by stuffing them instead.".into())],
                    name: vec!["Cheesecake and Chocolate Stuffed Strawberries".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("CHOCOLATE STUFFING", &[
                            "1/2 cup sugar",
                            "2 2/3 tablespoons milk",
                            "2 1/2 tablespoons butter",
                            "1/2 cup semisweet chocolate chips",
                            "1/4 cup finely chopped pecans if desired",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("CHEESECAKE STUFFING", &[
                            "1 (8-ounce) package cream cheese softened",
                            "3 tablespoons milk or cream",
                            "1/2 teaspoon almond extract",
                            "1 tablespoon powdered sugar",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::new_section("DIRECTIONS FOR CHOCOLATE STUFFING", vec![
                            "Put the first 3 ingredients in a small saucepan and bring to a boil, stirring constantly. Remove from heat and add chocolate chips. Beat with a wire whisk until chocolate mixture is creamy and cooled. Add chopped nuts, if desired."
                        ]),
                        RecipeRecipeInstructionsFieldEnum::new_section("DIRECTIONS FOR CHEESECAKE STUFFING", vec![
                            "In a medium bowl, mix cream cheese, milk or cream, almond extract, and powdered sugar together until consistency is smooth. Fill an icing cone or small plastic zippered bag (cut about 1/4 inch off one corner of bag) with mixture."
                        ]),
                        RecipeRecipeInstructionsFieldEnum::new_section("TO STUFF BERRIES", vec![
                            "Carefully core out fresh, firm berries. Create a hollow space of about ½ to ¾ inch, depending on the size of the berries. Cut a small piece off the berry tip, so the berries can stand on their own. Pipe stuffing into berries, and arrange nicely on a serving plate or tray."
                        ]),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("1-2 Pints".into())],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    author: vec![RecipeAuthorFieldEnum::new_person("Megan and Jill Stapley")],
                    description: vec![
                        RecipeDescriptionFieldEnum::Text("This extra-special cake was first prepared for a potluck by 16-year-old Megan Stapley, of Pinetop, Arizona. She started with two 8-inch cake rounds sliced in half to make four layers. Each layer was mounded with Cloud Nine Frosting. This dessert won raves and applause from everyone who tried it and Megan\u{92}s baking career was launched.\n\n\n\n                Megan\u{92}s mother, Jill, toyed with the idea of covering the sides of this already amazing cake with her grandma\u{92}s fudge frosting. Not only did the fudge frosting addition make the cake more decadent, it added to the beauty of the cake. And thus was born the now famous Chocolate Fudge Dream Cake.".into())
                    ],
                    prep_time: vec![DurationOrText::Text("20 minutes".into())],
                    cook_time: vec![DurationOrText::Text("30 minutes".into())],
                    comment:vec![Comment {
                        text: vec!["After buttering your cake pans dust with sugar instead of flour. This gives this cake a nice sugary texture that makes it easy to frost. Another option is to dust the buttered pans with cocoa powder.".into()],
                        ..Default::default()
                    }],
                    name: vec!["Chocolate Fudge Dream Cake".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("3/4 cup butter softened".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 cups sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3/4 cup Dutch baking cocoa".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 large eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon baking soda".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3/4 teaspoon salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 teaspoons vanilla extract".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup buttermilk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup hot water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 cups flour".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Preheat oven to 350°F. Butter and flour two round 9-inch cake pans. Cream the butter and sugar. Then add the cocoa and eggs and mix well. Add the baking soda, salt, and vanilla. Alternately blend the buttermilk, flour, and hot water. The batter should be smooth. Bake for 30-35 minutes or until an inserted toothpick comes out clean. Allow cake to cool before frosting.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("TO FROST THE CAKE: Place the first cake layer on a decorative serving plate or cake stand and generously frost the top with the Cloud Nine Frosting. Repeat with the next two layers, generously frosting the top of each. Place the fourth layer on top and leave unfrosted.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Smooth Chocolate Fudge Frosting around the sides of all four layers of cake. Finish frosting the cake by adding the remaining Cloud Nine Frosting to the top layer of cake, mounding it in the middle.".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("8".into())],
                    ..Default::default()
                },

            ]
        }
    }
}
