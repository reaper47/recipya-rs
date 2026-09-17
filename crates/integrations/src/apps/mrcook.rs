use std::{
    borrow::Cow,
    io::{Read, Seek},
};

use itertools::Itertools;
use serde::{Deserialize, Deserializer};
use tracing::error;

use schema_org::{
    AggregateRating, AtType, Comment, DurationOrText, Energy, Mass, NutritionInformation, Recipe,
    VideoObject, at_context,
    field::{
        RecipeImageFieldEnum, RecipeIsBasedOnUrlFieldEnum, RecipeVideoFieldEnum,
        RecipeYieldFieldEnum,
    },
};

use crate::{
    Result,
    apps::helpers::{
        Ingredient, Instruction, Parsers, ToSections, parse_archive_helper, read_file,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonRoot {
    pub meta: Meta,
    pub recipe: Recipe,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub version: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
struct CsvRecord {
    pub title: String,
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub source: Option<String>,
    #[serde(rename = "videoUrl")]
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub video_url: Option<String>,
    #[serde(rename = "cookTime")]
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub cook_time: Option<String>,
    #[serde(rename = "prepTime")]
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub prep_time: Option<String>,
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub rating: Option<String>,
    #[serde(rename = "ratingCount")]
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub rating_count: Option<String>,
    pub ingredients: String,
    pub instructions: String,
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub notes: Option<String>,
    pub servings: String,
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub cuisine: Option<String>,
    #[serde(deserialize_with = "deserialize_null_as_none")]
    pub image: Option<String>,
    pub calories: Option<String>,
    pub carbohydrates: Option<String>,
    pub protein: Option<String>,
    pub fat: Option<String>,
    pub fiber: Option<String>,
    pub sugar: Option<String>,
    pub sodium: Option<String>,
}

fn deserialize_null_as_none<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "null" => Ok(None),
        _ => Ok(Some(s)),
    }
}

impl From<CsvRecord> for Recipe {
    fn from(r: CsvRecord) -> Self {
        let mut nut = NutritionInformation::default();
        if let Some(v) = r.calories {
            nut.calories = vec![Energy::new(v)];
        }
        if let Some(v) = r.carbohydrates {
            nut.carbohydrate_content = vec![Mass::new(v)];
        }
        if let Some(v) = r.protein {
            nut.protein_content = vec![Mass::new(v)];
        }
        if let Some(v) = r.fat {
            nut.fat_content = vec![Mass::new(v)];
        }
        if let Some(v) = r.fiber {
            nut.fiber_content = vec![Mass::new(v)];
        }
        if let Some(v) = r.sugar {
            nut.sugar_content = vec![Mass::new(v)];
        }
        if let Some(v) = r.sodium {
            nut.sodium_content = vec![Mass::new(v)];
        }

        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            aggregate_rating: r.rating.map_or(Vec::new(), |rating| {
                vec![AggregateRating::new(
                    rating.parse().unwrap_or(3.0),
                    r.rating_count.map_or(1, |s| s.parse().unwrap_or(1)),
                )]
            }),
            comment: r.notes.map_or(Vec::new(), |c| {
                let s = c.trim();
                if s.is_empty() {
                    vec![]
                } else {
                    vec![Comment {
                        text: vec![c],
                        ..Default::default()
                    }]
                }
            }),
            cook_time: r.cook_time.map_or(Vec::new(), |s| {
                vec![DurationOrText::Text(format!("{s} min"))]
            }),
            prep_time: r.prep_time.map_or(Vec::new(), |s| {
                vec![DurationOrText::Text(format!("{s} min"))]
            }),
            image: r
                .image
                .map_or(Vec::new(), |s| vec![RecipeImageFieldEnum::URL(s)]),
            recipe_cuisine: r.cuisine.map_or(Vec::new(), |s| vec![s]),
            recipe_ingredient: r
                .ingredients
                .split("\\n")
                .map(|s| Ingredient::Line(Cow::Borrowed(s.trim())))
                .collect_vec()
                .to_sections(),
            recipe_instructions: r
                .instructions
                .split("\\n\\n")
                .map(|s| Instruction::Line(Cow::Borrowed(s.trim())))
                .collect_vec()
                .to_sections(),
            recipe_yield: vec![RecipeYieldFieldEnum::Text(r.servings)],
            name: vec![r.title],
            is_based_on_url: r
                .source
                .map_or(Vec::new(), |s| vec![RecipeIsBasedOnUrlFieldEnum::URL(s)]),
            video: r.video_url.map_or(Vec::new(), |u| {
                vec![RecipeVideoFieldEnum::VideoObject(
                    VideoObject {
                        r#type: AtType::VideoObject.to_opt(),
                        context: at_context(),
                        url: vec![u],
                        ..Default::default()
                    }
                    .into(),
                )]
            }),
            ..Default::default()
        }
    }
}

/// Parses a `Mr. Cook` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper(
        r,
        &Parsers {
            csv: Some(parse_csv),
            json: Some(parse_json),
            ..Default::default()
        },
    )
}

/// Parses a `Mr. Cook` CSV file.
pub fn parse_csv<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut recipes = Vec::new();

    for result in reader.deserialize() {
        let recipe: CsvRecord = match result {
            Ok(r) => r,
            Err(err) => {
                error!(?err, "Failed to parse Mr. Cook CSV entry");
                continue;
            }
        };
        recipes.push(recipe);
    }

    Ok(recipes.into_iter().map(Into::into).collect_vec())
}

fn parse_json<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let root: JsonRoot = serde_json::from_reader(r)?;
    Ok(vec![root.recipe])
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use std::io::Cursor;

        use super::*;

        #[test]
        fn test_mrcook_csv_ok() -> Result<()> {
            let buf = Cursor::new(files::csv());

            let got = parse_csv(buf)?;

            assert_eq!(got.len(), 2);
            pretty_assertions::assert_eq!(got, vec![results::recipe1(), results::recipe2_csv()]);
            Ok(())
        }

        #[test]
        fn test_mrcook_json_ok() -> Result<()> {
            let buf = Cursor::new(files::json());

            let mut got = parse_json(buf)?;

            assert_eq!(got.len(), 1);
            got[0].date_modified.clear();
            got[0].date_published.clear();
            pretty_assertions::assert_eq!(got, vec![results::recipe2_json()]);
            Ok(())
        }
    }

    mod files {
        pub fn csv<'a>() -> &'a str {
            r#"id,createdAt,updatedAt,title,source,videoUrl,cookTime,prepTime,rating,ratingCount,ingredients,instructions,notes,servings,totalTime,difficulty,language,cuisine,image,calories,carbohydrates,protein,fat,fiber,sugar,sodium
            "019eda7f-4957-7118-b18a-6e832ead4a34","06/18/2026, 07:30 AM","06/18/2026, 07:30 AM","All-American Trifle","https://www.allrecipes.com/recipe/218361/all-american-trifle/",null,null,30,null,null,"3 pounds fresh strawberries, hulled and sliced\n0.25 cup white sugar\n1 quart heavy cream\n1 (6 ounce) container lemon yogurt\n1 (3.3 ounce) package instant white chocolate pudding mix\n2 tablespoons coconut-flavored rum, or to taste, divided\n2 (16 ounce) prepared pound cakes, cubed\n2 pints fresh blueberries, or as needed","Gather all ingredients.\n\nSprinkle strawberries with sugar in a bowl; stir to distribute sugar, and set aside.\n\nChill a large metal mixing bowl and beaters from an electric mixer.\n\nPour cream into the chilled mixing bowl, and add lemon yogurt, pudding mix, and about 1 tablespoon of coconut rum; beat until fluffy with an electric mixer set on medium speed.\n\nSpread a layer of pound cake cubes into the bottom of a glass trifle dish, and sprinkle cubes with remaining tablespoon coconut rum. Cover pound cake with a layer of strawberries; sprinkle blueberries over strawberries. Spread a thick layer of whipped cream over the berries.\n\nRepeat the layers several times, ending with a layer of strawberries sprinkled with blueberries and reserving about 1 cup of whipped cream; top the trifle with dollops of whipped cream to serve. Refrigerate leftovers.",,"15 servings",30,null,"en","American","https://images.mrcook.app/recipe-image/019eda7f-4957-7118-b18a-6e832ead4a34?cacheKey=VGh1LCAxOCBKdW4gMjAyNiAxMTozMDozOCBHTVQ=",,,,,,,
            "019eda7f-bc63-7abf-a53e-a17e6cabc82a","06/18/2026, 07:31 AM","06/18/2026, 07:32 AM","Strawberry Shortcake Watermelon Pizza","https://www.allrecipes.com/recipe/265401/strawberry-shortcake-watermelon-pizza/",null,15,20,null,null,"8 ounces fresh strawberries, hulled and sliced\n1 tablespoon sugar\n1 center-cut slice of watermelon, about 1 inch thick\n1 cup whipped cream, or to taste","Mix strawberries and sugar together in a medium bowl until the strawberries are thoroughly coated. Allow the strawberries to macerate for 10 to 20 minutes.\n\nCut the round watermelon slice into 8 wedges, and keep them together. Pat dry with a paper towel or clean dish towel.\n\nSpray or spoon the whipped cream onto the center of the watermelon round; don't be afraid to pile it on, as the cream will sink under the weight of the strawberries.\n\nSpoon the macerated strawberries and their juices over the whipped cream and serve immediately.",,"8 servings",65,"MEDIUM","en","Ukrainian","https://images.mrcook.app/recipe-image/019eda7f-bc63-7abf-a53e-a17e6cabc82a?cacheKey=VGh1LCAxOCBKdW4gMjAyNiAxMTozMjozMyBHTVQ=",,,,,,,"#
        }

        pub fn json<'a>() -> &'a str {
            r#"{"meta":{"version":"0.2","date":"2026-06-18T11:35:18.816Z"},"recipe":{"@context":"https://schema.org","@type":"Recipe","name":"Strawberry Shortcake Watermelon Pizza","url":"https://www.mrcook.app/recipes/019eda7f-bc63-7abf-a53e-a17e6cabc82a","description":"For a fun take on a classic summer entertaining dessert, try this simple strawberry shortcake watermelon pizza with store-bought or homemade whipped cream.","datePublished":"2026-06-18T11:31:04.000Z","dateModified":"2026-06-18T11:32:33.000Z","author":{"@type":"Person","name":"Mr. Cook"},"prepTime":"PT20M","cookTime":"PT15M","totalTime":"PT1H5M","recipeYield":"8 servings","image":["https://images.mrcook.app/recipe-image/019eda7f-bc63-7abf-a53e-a17e6cabc82a?cacheKey=VGh1LCAxOCBKdW4gMjAyNiAxMTozMjozMyBHTVQ="],"recipeIngredient":["8 ounces fresh strawberries, hulled and sliced","1 tablespoon sugar","1 center-cut slice of watermelon, about 1 inch thick","1 cup whipped cream, or to taste"],"recipeInstructions":[{"@type":"HowToStep","text":"Mix strawberries and sugar together in a medium bowl until the strawberries are thoroughly coated. Allow the strawberries to macerate for 10 to 20 minutes."},{"@type":"HowToStep","text":"Cut the round watermelon slice into 8 wedges, and keep them together. Pat dry with a paper towel or clean dish towel."},{"@type":"HowToStep","text":"Spray or spoon the whipped cream onto the center of the watermelon round; don't be afraid to pile it on, as the cream will sink under the weight of the strawberries."},{"@type":"HowToStep","text":"Spoon the macerated strawberries and their juices over the whipped cream and serve immediately."}],"recipeCuisine":"Ukrainian","nutrition":{"@type":"NutritionInformation","calories":"169 calories","carbohydrateContent":"17 g","proteinContent":"2 g","fatContent":"11 g","sugarContent":"14 g"}}}"#
        }
    }

    mod results {
        use schema_org::{
            AtType, HowToStep, at_context,
            field::{
                RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeRecipeIngredientFieldEnum,
                RecipeRecipeInstructionsFieldEnum,
            },
        };

        use super::*;

        pub fn recipe1() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["All-American Trifle".into()],
                is_based_on_url: vec![RecipeIsBasedOnUrlFieldEnum::URL(
                    "https://www.allrecipes.com/recipe/218361/all-american-trifle/".into(),
                )],
                image: vec![RecipeImageFieldEnum::URL("https://images.mrcook.app/recipe-image/019eda7f-4957-7118-b18a-6e832ead4a34?cacheKey=VGh1LCAxOCBKdW4gMjAyNiAxMTozMDozOCBHTVQ=".into())],
                prep_time: vec![DurationOrText::Text("30 min".into())],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text(
                        "3 pounds fresh strawberries, hulled and sliced".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text("0.25 cup white sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 quart heavy cream".into()),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 (6 ounce) container lemon yogurt".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 (3.3 ounce) package instant white chocolate pudding mix".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 tablespoons coconut-flavored rum, or to taste, divided".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 (16 ounce) prepared pound cakes, cubed".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 pints fresh blueberries, or as needed".into(),
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Gather all ingredients.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Sprinkle strawberries with sugar in a bowl; stir to distribute sugar, and set aside.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Chill a large metal mixing bowl and beaters from an electric mixer.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Pour cream into the chilled mixing bowl, and add lemon yogurt, pudding mix, and about 1 tablespoon of coconut rum; beat until fluffy with an electric mixer set on medium speed.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Spread a layer of pound cake cubes into the bottom of a glass trifle dish, and sprinkle cubes with remaining tablespoon coconut rum. Cover pound cake with a layer of strawberries; sprinkle blueberries over strawberries. Spread a thick layer of whipped cream over the berries.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Repeat the layers several times, ending with a layer of strawberries sprinkled with blueberries and reserving about 1 cup of whipped cream; top the trifle with dollops of whipped cream to serve. Refrigerate leftovers.".into()),
                ],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("15 servings".into())],
                recipe_cuisine: vec!["American".into()],
                ..Default::default()
            }
        }

        pub fn recipe2_csv() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Strawberry Shortcake Watermelon Pizza".into()],
                is_based_on_url: vec![RecipeIsBasedOnUrlFieldEnum::URL(
                    "https://www.allrecipes.com/recipe/265401/strawberry-shortcake-watermelon-pizza/".into(),
                )],
                image: vec![RecipeImageFieldEnum::URL("https://images.mrcook.app/recipe-image/019eda7f-bc63-7abf-a53e-a17e6cabc82a?cacheKey=VGh1LCAxOCBKdW4gMjAyNiAxMTozMjozMyBHTVQ=".into())],
                cook_time: vec![DurationOrText::Text("15 min".into())],
                prep_time: vec![DurationOrText::Text("20 min".into())],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("8 ounces fresh strawberries, hulled and sliced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tablespoon sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 center-cut slice of watermelon, about 1 inch thick".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup whipped cream, or to taste".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Mix strawberries and sugar together in a medium bowl until the strawberries are thoroughly coated. Allow the strawberries to macerate for 10 to 20 minutes.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Cut the round watermelon slice into 8 wedges, and keep them together. Pat dry with a paper towel or clean dish towel.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Spray or spoon the whipped cream onto the center of the watermelon round; don't be afraid to pile it on, as the cream will sink under the weight of the strawberries.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Spoon the macerated strawberries and their juices over the whipped cream and serve immediately.".into()),
                ],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("8 servings".into())],
                recipe_cuisine: vec!["Ukrainian".into()],
                ..Default::default()
            }
        }

        pub fn recipe2_json() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                author: vec![RecipeAuthorFieldEnum::new_org("Mr. Cook")],
                description: vec![RecipeDescriptionFieldEnum::Text("For a fun take on a classic summer entertaining dessert, try this simple strawberry shortcake watermelon pizza with store-bought or homemade whipped cream.".into())],
                name: vec!["Strawberry Shortcake Watermelon Pizza".into()],
                url: vec!["https://www.mrcook.app/recipes/019eda7f-bc63-7abf-a53e-a17e6cabc82a".into()],
                image: vec![RecipeImageFieldEnum::URL("https://images.mrcook.app/recipe-image/019eda7f-bc63-7abf-a53e-a17e6cabc82a?cacheKey=VGh1LCAxOCBKdW4gMjAyNiAxMTozMjozMyBHTVQ=".into())],
                cook_time: vec![DurationOrText::Text("PT15M".into())],
                prep_time: vec![DurationOrText::Text("PT20M".into())],
                total_time: vec![DurationOrText::Text("PT1H5M".into())],
                nutrition: vec![
                    NutritionInformation {
                        calories: vec![Energy::new("169 calories")],
                        carbohydrate_content: vec![Mass::new("17 g")],
                        fat_content: vec![Mass::new("11 g")],
                        protein_content: vec![Mass::new("2 g")],
                        sugar_content: vec![Mass::new("14 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    },
                ],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("8 ounces fresh strawberries, hulled and sliced".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tablespoon sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 center-cut slice of watermelon, about 1 inch thick".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup whipped cream, or to taste".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Mix strawberries and sugar together in a medium bowl until the strawberries are thoroughly coated. Allow the strawberries to macerate for 10 to 20 minutes.".into()],
                        ..Default::default()
                    }.into()),
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Cut the round watermelon slice into 8 wedges, and keep them together. Pat dry with a paper towel or clean dish towel.".into()],
                        ..Default::default()
                    }.into()),
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Spray or spoon the whipped cream onto the center of the watermelon round; don't be afraid to pile it on, as the cream will sink under the weight of the strawberries.".into()],
                        ..Default::default()
                    }.into()),
                    RecipeRecipeInstructionsFieldEnum::HowToStep(HowToStep {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec!["Spoon the macerated strawberries and their juices over the whipped cream and serve immediately.".into()],
                        ..Default::default()
                    }.into()),
                ],
                recipe_yield: vec![RecipeYieldFieldEnum::Text("8 servings".into())],
                recipe_cuisine: vec!["Ukrainian".into()],
                ..Default::default()
            }
        }
    }
}
