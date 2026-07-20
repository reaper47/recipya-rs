use std::fmt::Write;
use std::io::{Cursor, Read, Seek};

use itertools::Itertools;
use schema_org::at_context;
use serde::{Deserialize, Serialize};

use schema_org::field::RecipeRecipeInstructionsFieldEnum;
use schema_org::{
    AtType, Recipe,
    field::{RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum},
};

use crate::helpers::to_yield;
use crate::{Error, Result};

#[derive(Serialize, Deserialize)]
pub struct RecipeML {
    #[serde(rename = "@version")]
    pub version: String,
    pub recipe: Vec<RecipeXML>,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeXML {
    pub head: Head,
    #[serde(default)]
    pub ingredients: Ingredients,
    pub directions: Directions,
}

#[derive(Serialize, Deserialize)]
pub struct Head {
    pub title: String,
    #[serde(default)]
    pub categories: Vec<Categories>,
    #[serde(rename = "yield")]
    pub r#yield: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Categories {
    pub cat: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Ingredients {
    #[serde(rename = "ing-div", default)]
    pub ing_div: Vec<IngDiv>,
}

#[derive(Serialize, Deserialize)]
pub struct IngDiv {
    pub ing: Vec<Ing>,
    pub title: Option<IngDivTitle>,
}

#[derive(Serialize, Deserialize)]
pub struct Ing {
    pub amt: Amt,
    pub item: String,
}

#[derive(Serialize, Deserialize)]
pub struct Amt {
    pub qty: String,
    pub unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct IngDivTitle {
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Directions {
    pub step: Vec<String>,
}

impl From<RecipeXML> for Recipe {
    fn from(r: RecipeXML) -> Self {
        let (category, keywords) = match r
            .head
            .categories
            .into_iter()
            .map(|c| c.cat)
            .collect::<Vec<_>>()
            .as_slice()
        {
            [first, rest @ ..] => (Some(first.clone()), Some(rest.to_vec())),
            [] => (None, None),
        };

        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            name: vec![r.head.title],
            recipe_category: category.map_or(Vec::new(), |s| vec![s]),
            keywords: keywords.map_or(Vec::new(), |v| {
                v.into_iter()
                    .map(RecipeKeywordsFieldEnum::TextOrURL)
                    .collect()
            }),
            recipe_yield: to_yield(
                r.head
                    .r#yield
                    .unwrap_or_default()
                    .parse()
                    .ok()
                    .unwrap_or_default(),
            ),
            recipe_ingredient: r
                .ingredients
                .ing_div
                .into_iter()
                .flat_map(|div| {
                    let ingredients = div
                        .ing
                        .iter()
                        .map(|ing| {
                            let mut s = String::new();
                            if !ing.amt.qty.trim().is_empty() {
                                write!(s, "{} ", ing.amt.qty.trim()).unwrap();
                            }
                            if !ing.amt.unit.trim().is_empty() {
                                write!(s, "{} ", ing.amt.unit.trim()).unwrap();
                            }
                            write!(s, "{}", ing.item.trim()).unwrap();
                            s
                        })
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();

                    match div.title {
                        Some(title) if title.text.is_some() => {
                            vec![RecipeRecipeIngredientFieldEnum::new_section(
                                &title.text.unwrap(),
                                ingredients
                                    .iter()
                                    .map(String::as_str)
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                            )]
                        }
                        _ => ingredients
                            .into_iter()
                            .map(RecipeRecipeIngredientFieldEnum::Text)
                            .collect(),
                    }
                })
                .collect(),
            recipe_instructions: r
                .directions
                .step
                .into_iter()
                .filter(|s| !s.is_empty())
                .filter_map(|s| {
                    if s.trim().is_empty() {
                        None
                    } else {
                        Some(RecipeRecipeInstructionsFieldEnum::Text(
                            s.trim().lines().map(str::trim).join(" "),
                        ))
                    }
                })
                .collect(),
            ..Default::default()
        }
    }
}

/// Parses a `RecipeML` XML file.
pub fn parse_xml<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;

    let root: RecipeML = quick_xml::de::from_reader(Cursor::new(buf))
        .map_err(|err| Error::Parse(err.to_string()))?;

    Ok(root
        .recipe
        .into_iter()
        .filter(|r| !r.ingredients.ing_div.is_empty())
        .map(Recipe::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use super::*;

        #[test]
        fn test_hc_xml_ok() -> Result<()> {
            let buf = Cursor::new(files::xml());

            let got = parse_xml(buf)?;

            let expected = results::xml();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }
    }

    mod files {
        #[allow(clippy::too_many_lines)]
        pub fn xml<'a>() -> &'a str {
            r#"<?xml version="1.0" ?><recipeml version="0.5"><recipe><head><title>Baked Ham and Kraut Rolls</title><categories><cat>Pork and Lamb</cat></categories><yield>2</yield></head><ingredients><ing-div><ing><amt><qty>6</qty><unit>ounces</unit></amt><item>thinly sliced ham</item></ing><ing><amt><qty>8</qty><unit>ounces</unit></amt><item>can sauerkraut, drained</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>sliced green onion</item></ing><ing><amt><qty>1/2</qty><unit>teaspoon</unit></amt><item>caraway seed</item></ing><ing><amt><qty>1/4</qty><unit>cup</unit></amt><item>mayonnaise</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>milk</item></ing><ing><amt><qty>2</qty><unit>teaspoons</unit></amt><item>mustard</item></ing></ing-div></ingredients><directions><step>
            Finely chop 2 slices of the ham.
            </step><step>
            Combine chopped ham and all remaining ingredients.
            </step><step>
            Place a little of the sauerkraut mixture on each remaining ham
            slice. Roll up each slice from one side.
            </step><step>
            Microwave on high till heated through.
            </step><step></step></directions></recipe><recipe><head><title>Barbecued Pork</title><categories><cat>Pork and Lamb</cat></categories><yield>4</yield></head><ingredients><ing-div><ing><amt><qty>1/4</qty><unit>cup</unit></amt><item>catsup</item></ing><ing><amt><qty>1/3</qty><unit>cup</unit></amt><item>water</item></ing><ing><amt><qty>1</qty><unit>tablespoon</unit></amt><item>cornstarch</item></ing><ing><amt><qty>1</qty><unit>tablespoon</unit></amt><item>brown sugar</item></ing><ing><amt><qty>1</qty><unit>tablespoon</unit></amt><item>chili powder</item></ing><ing><amt><qty>1</qty><unit>tablespoon</unit></amt><item>Worcestershire sauce</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>vinegar</item></ing><ing><amt><qty>1/4</qty><unit>teaspoon</unit></amt><item>pepper</item></ing><ing><amt><qty>1/2</qty><unit>cup</unit></amt><item>onion, chopped</item></ing><ing><amt><qty>1/2</qty><unit>cup</unit></amt><item>carrot, chopped</item></ing><ing><amt><qty>1/2</qty><unit>cup</unit></amt><item>celery, chopped</item></ing><ing><amt><qty>1</qty><unit>pound</unit></amt><item>pork, cut in bite size strips</item></ing><ing><amt><qty></qty><unit></unit></amt><item>Pita Bread</item></ing></ing-div></ingredients><directions><step>
            Mix first 8 ingredients, set aside. Stir fry pork in a wok 2-3
            minutes. Add onion, carrot, and celery. Stir fry until vegetables
            are tender.
            </step><step>
            Stir in sauce, cook and stir till mixture thickens and boils.
            </step><step>
            Serve in Pita Bread
            </step><step>
            Calories: 252, Cholesterol: 77mg, Fat: 11g, Protein: 25g, Sodium:
            304mg, Carbohydrates: 11g, Potassium: 520mg
            </step><step></step></directions></recipe><recipe><head><title>Barbeque Beef Sandwiches</title><categories><cat>Beef</cat></categories><yield>8</yield></head><ingredients><ing-div><ing><amt><qty>2</qty><unit>pounds</unit></amt><item>beef Round Steak</item></ing><ing><amt><qty>16</qty><unit>ounces</unit></amt><item>can cut tomatoes</item></ing><ing><amt><qty>1</qty><unit>large</unit></amt><item>onion, chopped</item></ing><ing><amt><qty>1</qty><unit>large</unit></amt><item>carrot, chopped</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>Worcestershire sauce</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>vinegar</item></ing><ing><amt><qty>1</qty><unit>tablespoon</unit></amt><item>brown sugar</item></ing><ing><amt><qty>2</qty><unit>teaspoons</unit></amt><item>chili powder</item></ing><ing><amt><qty>1</qty><unit>teaspoon</unit></amt><item>dried oregano</item></ing><ing><amt><qty>1</qty><unit></unit></amt><item>Garlic clove, minced</item></ing><ing><amt><qty>8</qty><unit></unit></amt><item>Hamburger buns</item></ing></ing-div></ingredients><directions><step>
            Trim fat from meat and cut into 4 to 6 large pieces. In a large
            skillet, in oil, brown meat on both sides. Add undrained tomatoes,
            onion, carrot, worcestershire sauce, vinegar, brown sugar, chili
            powder, oregano, and garlic. Bring to boil; reduce heat. Cover and
            simmer for 2 to 2-1/2 hours. Remove meat from sauce and shred
            meat, using two forks. Simmer sauce, uncovered, for 5 to 10
            minutes while shredding meat. Mix shredded meat with sauce. Serve
            on Hamburger buns.
            </step><step>
            Calories: 278, Cholesterol: 54 mg, Fat: 8 grams, Protein: 22 gram,
            Sodium: 300 mg, Carbohydrates: 29 gram, Potassium: 502 mg
            </step><step></step></directions></recipe><recipe><head><title>Blood Alcohol Levels</title><categories><cat>Reference Text</cat></categories></head><ingredients></ingredients><directions><step>
            DO NOT DRIVE UNDER THE INFLUENCE:

            Your driving ability is related to your Blood Alcohol Concentration.  Alcohol
            is a drug that affects your judgement and slows your reactions. When you have
            been drinking -- Don&apos;t Gamble! Call a Cab, Call a sober friend!

            Blood Alcohol Concentration Guide:

            Alcohol is burned up by your body at .015% per hour.

            If your BAC is .025% it takes 1.7 hours to reach .000%
            If your BAC is .050% it takes 3.3 hours to reach .000%
            If your BAC is .075% it takes 5.0 hours to reach .000%
            If your BAC is .100% it takes 6.7 hours to reach .000%
            If your BAC is .125% it takes 8.3 hours to reach .000%

            Percent of Alcohol in Bloodstream:

            .100% is legally drunk in most states.
            Crash risk quadruples at .08% which is now the legal limit in many states.

            If you weigh 100 pounds:  2 drinks = .058%, 3 drinks = .088%, 4 drinks =
            .117%, 5 drinks = .146%
            If you weigh 120 pounds:  3 drinks = .073%, 4 drinks = .097%, 5 drinks =
            .121%, 6 drinks = .145%
            If you weigh 140 pounds:  3 drinks = .063%, 4 drinks = .083%, 5 drinks =
            .104%, 6 drinks = .125%
            If you weigh 160 pounds:  4 drinks = .073%, 5 drinks = .091%, 6 drinks =
            .109%, 7 drinks = .128%
            If you weigh 180 pounds:  4 drinks = .065%, 5 drinks = .081%, 6 drinks =
            .097%, 7 drinks = .113%
            If you weigh 200 pounds:  5 drinks = .073%, 6 drinks = .087%, 7 drinks =
            .102%, 8 drinks = .117%
            If you weigh 220 pounds:  5 drinks = .067%, 6 drinks = .080%, 7 drinks =
            .093%, 8 drinks = .106%
            If you weigh 240 pounds:  6 drinks = .073%, 7 drinks = .085%, 8 drinks =
            .097%, 9 drinks = .109%

            One Drink Equals: 1 oz. of 80 proof Alcohol
            One Drink Equals: 2 oz. of 20% wine
            One Drink Equals: 3 oz. of 12% wine
            One Drink Equals: 12 oz. Bottle of Beer

            Source: Washington State Liquor Control Board
            </step></directions></recipe><recipe><head><title>Brandied Beef</title><categories><cat>Beef</cat></categories><yield>6</yield></head><ingredients><ing-div><ing><amt><qty>1</qty><unit>teaspoon</unit></amt><item>salt</item></ing><ing><amt><qty>1</qty><unit>teaspoon</unit></amt><item>paprika</item></ing><ing><amt><qty>1/2</qty><unit>teaspoon</unit></amt><item>dried basill</item></ing><ing><amt><qty>1/4</qty><unit>teaspoon</unit></amt><item>dried thyme</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>flour</item></ing><ing><amt><qty>2</qty><unit>pounds</unit></amt><item>beef stew meat</item></ing><ing><amt><qty>2</qty><unit>tablespoons</unit></amt><item>peanut oil</item></ing><ing><amt><qty>1</qty><unit>clove</unit></amt><item>garlic, peeled and crushed</item></ing><ing><amt><qty>1</qty><unit>large</unit></amt><item>onion, peeled and sliced</item></ing><ing><amt><qty>1/2</qty><unit>cup</unit></amt><item>beef stock</item></ing><ing><amt><qty>1/2</qty><unit>cup</unit></amt><item>brandy</item></ing><ing><amt><qty>2</qty><unit>cups</unit></amt><item>fresh mushrooms, sliced</item></ing><ing><amt><qty></qty><unit></unit></amt><item>Hot cooked buttered egg noodles</item></ing></ing-div></ingredients><directions><step>
            Mix first 5 ingredients. Lightly dredge meat in flour mixture.
            Heat oil in uncovered work at 375&apos;. Brown meat in batches,
            removing to plate. Add garlic and onion; stir-fry 30 seconds.
            Return meat to wok with stock and brandy. Reduce heat to &quot;simmer&quot;;
            cover; simmer 1 hour. Add mushrooms; stir; cook 10 minutes,
            covered. Serve immediately with buttered egg noodles. Preparation
            Time: 1 hour, 45 minutes.
            </step><step></step></directions></recipe><recipe><head><title>Measurement Equivalents</title><categories><cat>Reference Text</cat></categories></head><ingredients></ingredients><directions><step>
            1 Dash:
            1/8 teaspoon, or 2 to 3 drops

            1 Teaspoon:
            5 ml

            1 Tablespoon:
            3 Teaspoons, or 15 ml

            1 Ounce:
            2 Tablespoons, or 30 ml

            1/4 Cup:
            4 Tablespoons, or 2 ounces, or 60 ml

            1/3 Cup:
            5-1/3 Tablespoons

            1/2 Cup:
            8 Tablespoons, or 4 ounces, or 120 ml

            1 Cup:
            16 Tablespoons, or 8 Ounces, or 1/2 Pint, or 240 ml

            1/2 Pint:
            1 Cup, or 8 ounces, or 240 ml

            1 Pint:
            2 Cups, or 16 Ounces, or 480 ml

            1 Quart:
            4 Cups, or 32 Ounces, or 2 Pints, or 960 ml

            1 Gallon:
            16 Cups, or 128 Ounces, or 4 Quarts

            1 Peck:
            2 Gallons, or 32 Cups, or 8 Quarts

            1 Bushel:
            8 Gallons, or 128 Cups, or 32 Quarts

            1 Ounce (weight):
            28 Grams

            1 Pound (weight):
            16 Ounces, or 454 Grams

            </step></directions></recipe></recipeml>
"#
        }
    }

    mod results {
        use crate::helpers::to_yield;

        use super::*;

        #[allow(clippy::too_many_lines)]
        pub fn xml() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    name: vec!["Baked Ham and Kraut Rolls".into()],
                    recipe_category: vec!["Pork and Lamb".into()],
                    recipe_yield: to_yield(2),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("6 ounces thinly sliced ham".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 ounces can sauerkraut, drained".into()),
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
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    name: vec!["Barbecued Pork".into()],
                    recipe_category: vec!["Pork and Lamb".into()],
                    recipe_yield: to_yield(4),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1/4 cup catsup".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/3 cup water".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon cornstarch".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon brown sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon chili powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon Worcestershire sauce".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons vinegar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup onion, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup carrot, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup celery, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 pound pork, cut in bite size strips".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Pita Bread".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mix first 8 ingredients, set aside. Stir fry pork in a wok 2-3 minutes. Add onion, carrot, and celery. Stir fry until vegetables are tender.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Stir in sauce, cook and stir till mixture thickens and boils.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Serve in Pita Bread".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Calories: 252, Cholesterol: 77mg, Fat: 11g, Protein: 25g, Sodium: 304mg, Carbohydrates: 11g, Potassium: 520mg".into(),
                        ),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    name: vec!["Barbeque Beef Sandwiches".into()],
                    recipe_category: vec!["Beef".into()],
                    recipe_yield: to_yield(8),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("2 pounds beef Round Steak".into()),
                        RecipeRecipeIngredientFieldEnum::Text("16 ounces can cut tomatoes".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 large onion, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 large carrot, chopped".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons Worcestershire sauce".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons vinegar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon brown sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 teaspoons chili powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 teaspoon dried oregano".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 Garlic clove, minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("8 Hamburger buns".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Trim fat from meat and cut into 4 to 6 large pieces. In a large skillet, in oil, brown meat on both sides. Add undrained tomatoes, onion, carrot, worcestershire sauce, vinegar, brown sugar, chili powder, oregano, and garlic. Bring to boil; reduce heat. Cover and simmer for 2 to 2-1/2 hours. Remove meat from sauce and shred meat, using two forks. Simmer sauce, uncovered, for 5 to 10 minutes while shredding meat. Mix shredded meat with sauce. Serve on Hamburger buns.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Calories: 278, Cholesterol: 54 mg, Fat: 8 grams, Protein: 22 gram, Sodium: 300 mg, Carbohydrates: 29 gram, Potassium: 502 mg".into(),
                        ),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    name: vec!["Brandied Beef".into()],
                    recipe_category: vec!["Beef".into()],
                    recipe_yield: to_yield(6),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 teaspoon salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 teaspoon paprika".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon dried basill".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon dried thyme".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons flour".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 pounds beef stew meat".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 tablespoons peanut oil".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 clove garlic, peeled and crushed".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 large onion, peeled and sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup beef stock".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/2 cup brandy".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 cups fresh mushrooms, sliced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("Hot cooked buttered egg noodles".into()),
                    ],
                    recipe_instructions: vec![RecipeRecipeInstructionsFieldEnum::Text(
                        "Mix first 5 ingredients. Lightly dredge meat in flour mixture. Heat oil in uncovered work at 375'. Brown meat in batches, removing to plate. Add garlic and onion; stir-fry 30 seconds. Return meat to wok with stock and brandy. Reduce heat to \"simmer\"; cover; simmer 1 hour. Add mushrooms; stir; cook 10 minutes, covered. Serve immediately with buttered egg noodles. Preparation Time: 1 hour, 45 minutes.".into(),
                    )],
                    ..Default::default()
                }
            ]
        }
    }
}
