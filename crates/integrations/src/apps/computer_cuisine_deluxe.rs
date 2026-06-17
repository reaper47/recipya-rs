use std::io::{Read, Seek};

use encoding_rs::WINDOWS_1252;
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

use schema_org::{
    AggregateRating, AtType, DurationOrText, Energy, Mass, NutritionInformation, Recipe,
    VideoObject,
    field::{
        RecipeAuthorFieldEnum, RecipeIsBasedOnUrlFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum, RecipeVideoFieldEnum,
    },
};

use crate::{Result, apps::recipya::at_context};

pub type Recipes = Vec<CuisineRecipe>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CuisineRecipe {
    #[allow(unused)]
    #[serde(rename = "calcium")]
    calcium: StringOrNumber,
    calories: StringOrNumber,
    cholesterol: StringOrNumber,
    #[serde(rename = "Cooking Time")]
    cooking_time: String,
    #[serde(rename = "Dietary Fiber")]
    dietary_fiber: StringOrNumber,
    #[serde(rename = "Ingredients 3 Final")]
    ingredients_3_final: String,
    #[allow(unused)]
    iron: StringOrNumber,
    #[serde(rename = "Item Name")]
    title: String,
    method: String,
    #[serde(rename = "Preparation Time")]
    preparation_time: String,
    protein: StringOrNumber,
    #[serde(rename = "Recipe From Name")]
    recipe_from_name: String,
    #[serde(rename = "Recipe From Website")]
    recipe_from_website: String,
    #[serde(rename = "Recipe Notes")]
    recipe_notes: String,
    #[serde(rename = "Recipe Rating")]
    recipe_rating: RecipeRating,
    #[serde(rename = "Saturated Fat")]
    saturated_fat: StringOrNumber,
    serves: StringOrNumber,
    sodium: StringOrNumber,
    style: String,
    sugars: StringOrNumber,
    #[serde(rename = "Total Carbohydrate")]
    total_carbohydrate: StringOrNumber,
    #[serde(rename = "Total Fat")]
    total_fat: StringOrNumber,
    #[serde(rename = "Type")]
    root_type: String,
    #[serde(rename = "Video Reference Link 1")]
    video_reference_link_1: String,
    #[serde(rename = "Video Reference Link 2")]
    video_reference_link_2: String,
    #[serde(rename = "Video Reference Link 3")]
    video_reference_link_3: String,
    #[serde(rename = "Video Reference Link 4")]
    video_reference_link_4: String,
    #[allow(unused)]
    #[serde(rename = "Vitamin A")]
    vitamin_a: StringOrNumber,
    #[allow(unused)]
    #[serde(rename = "Vitamin C")]
    vitamin_c: StringOrNumber,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    Integer(i64),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub enum RecipeRating {
    Average,
    #[serde(rename = "")]
    Empty,
    Good,
    Highest,
    Poor,
    #[serde(rename = "Very Good")]
    VeryGood,
}

impl RecipeRating {
    const fn to_int(&self) -> i8 {
        match self {
            Self::Average => 2,
            Self::Empty => 0,
            Self::Good => 3,
            Self::Highest => 5,
            Self::Poor => 1,
            Self::VeryGood => 4,
        }
    }
}

#[allow(clippy::too_many_lines)]
/// Parses a `ChefTap` recipe in the text format from the file's content.
pub fn parse_csv<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let mut raw = Vec::new();
    r.read_to_end(&mut raw)?;

    let (decoded, _, has_err) = WINDOWS_1252.decode(&raw);
    if has_err {
        warn!(
            "Computer Cuisine Deluxe parser: Some bytes could not be cleanly decoded as Windows-1252"
        );
    }

    let mut reader = csv::Reader::from_reader(decoded.as_bytes());
    let mut recipes = Vec::new();

    for result in reader.deserialize() {
        let mut nut = NutritionInformation::default();

        let recipe: CuisineRecipe = match result {
            Ok(r) => r,
            Err(err) => {
                error!("Failed to parse Computer Cuisine Deluxe entry: {err}");
                continue;
            }
        };

        if let StringOrNumber::Integer(n) = recipe.calories
            && n > 0
        {
            nut.calories = vec![Energy::new(format!("{n} kcal"))];
        }

        if let StringOrNumber::Integer(n) = recipe.cholesterol
            && n > 0
        {
            nut.cholesterol_content = vec![Mass::new(format!("{n} mg"))];
        }

        if let StringOrNumber::Integer(n) = recipe.dietary_fiber
            && n > 0
        {
            nut.fiber_content = vec![Mass::new(format!("{n} g"))];
        }

        if let StringOrNumber::Integer(n) = recipe.protein
            && n > 0
        {
            nut.protein_content = vec![Mass::new(format!("{n} g"))];
        }

        if let StringOrNumber::Integer(n) = recipe.saturated_fat
            && n > 0
        {
            nut.saturated_fat_content = vec![Mass::new(format!("{n} g"))];
        }

        if let StringOrNumber::Integer(n) = recipe.sodium
            && n > 0
        {
            nut.sodium_content = vec![Mass::new(format!("{n} mg"))];
        }

        if let StringOrNumber::Integer(n) = recipe.sugars
            && n > 0
        {
            nut.sugar_content = vec![Mass::new(format!("{n} g"))];
        }

        if let StringOrNumber::Integer(n) = recipe.total_carbohydrate
            && n > 0
        {
            nut.carbohydrate_content = vec![Mass::new(format!("{n} g"))];
        }

        if let StringOrNumber::Integer(n) = recipe.total_fat
            && n > 0
        {
            nut.fat_content = vec![Mass::new(format!("{n} g"))];
        }

        let mut recipe_instructions = recipe
            .method
            .split('\x0B')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.into()))
            .collect::<Vec<_>>();

        if !recipe.recipe_notes.is_empty() {
            recipe_instructions.push(RecipeRecipeInstructionsFieldEnum::Text(format!(
                "Notes: {}",
                recipe.recipe_notes.trim()
            )));
        }

        let num_servings = match recipe.serves {
            StringOrNumber::Integer(n) => n,
            StringOrNumber::String(_) => 4,
        };

        let video_links = vec![
            recipe.video_reference_link_1,
            recipe.video_reference_link_2,
            recipe.video_reference_link_3,
            recipe.video_reference_link_4,
        ]
        .into_iter()
        .filter(|s| !s.trim().is_empty());

        let url = &recipe.recipe_from_website;

        recipes.push(Recipe {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            aggregate_rating: Some(recipe.recipe_rating.to_int())
                .filter(|r| r > &0)
                .map(|r| vec![AggregateRating::new(f32::from(r), 1)])
                .unwrap_or_default(),
            author: if recipe.recipe_from_name.is_empty() {
                vec![]
            } else {
                vec![RecipeAuthorFieldEnum::new_person(&recipe.recipe_from_name)]
            },
            cook_time: if recipe.cooking_time.to_lowercase() == "none" {
                vec![]
            } else {
                vec![DurationOrText::Text(recipe.cooking_time.replace('&', ""))]
            },
            is_based_on_url: if url.is_empty() {
                vec![]
            } else {
                vec![RecipeIsBasedOnUrlFieldEnum::URL(url.into())]
            },
            nutrition: if nut.is_empty() { vec![] } else { vec![nut] },
            name: vec![recipe.title],
            prep_time: if recipe.preparation_time.to_lowercase() == "none" {
                vec![]
            } else {
                vec![DurationOrText::Text(
                    recipe.preparation_time.replace('&', ""),
                )]
            },
            recipe_cuisine: if recipe.style.is_empty() {
                vec![]
            } else {
                vec![recipe.style]
            },
            recipe_category: if recipe.root_type.is_empty() {
                vec![]
            } else {
                vec![recipe.root_type]
            },
            recipe_ingredient: recipe
                .ingredients_3_final
                .split('\x0B')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.into()))
                .collect::<Vec<_>>(),
            recipe_instructions,
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Number(f64::from(
                i32::try_from(num_servings).unwrap_or(i32::MAX),
            ))],
            url: if url.is_empty() {
                vec![]
            } else {
                vec![url.into()]
            },
            video: video_links
                .into_iter()
                .map(|s| {
                    RecipeVideoFieldEnum::VideoObject(
                        VideoObject {
                            r#type: AtType::VideoObject.to_opt(),
                            context: at_context(),
                            url: vec![s],
                            ..Default::default()
                        }
                        .into(),
                    )
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        });
    }

    Ok(recipes)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use schema_org::field::AggregateRatingRatingValueFieldEnum;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_ccd_recipes_csv_ok() -> Result<()> {
        let file = csv_file1();
        let buf = Cursor::new(file);

        let got = parse_csv(buf)?;

        assert_eq!(got.len(), 3);
        pretty_assertions::assert_eq!(
            got,
            vec![
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                nutrition: vec![
                    NutritionInformation {
                        calories: vec![Energy::new("225 kcal")],
                        carbohydrate_content: vec![Mass::new("29 g")],
                        cholesterol_content: vec![Mass::new("27 mg")],
                        fat_content: vec![Mass::new("4 g")],
                        protein_content: vec![Mass::new("20 g")],
                        saturated_fat_content: vec![Mass::new("3 g")],
                        sodium_content: vec![Mass::new("147 mg")],
                        sugar_content: vec![Mass::new("2 g")],
                        ..Default::default()
                    },
                ],
                recipe_yield: vec![
                    RecipeRecipeYieldFieldEnum::Number(8.0),
                ],
                recipe_cuisine: vec!["American".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text(
                        "3 tablespoons butter".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "3/4 cup chopped onion".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1/2 cup chopped celery".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 teaspoon garlic powder".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 cups diced potatoes".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 carrots, diced".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 cups chicken broth".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 teaspoon salt".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 teaspoon ground black pepper".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 teaspoon dried dill weed".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 (16 ounce) cans salmon".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 (12 fluid ounce) can evaporated milk".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 (15 ounce) can creamed corn".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1/2 pound Cheddar cheese, shredded".into(),
                    ),
                ],
                suitable_for_diet: vec![],
                cook_time: vec![
                    DurationOrText::Text("30 mins.".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "1. Melt butter in a large pot over medium heat. Sauté onion, celery, and garlic powder until onions are tender. Stir in potatoes, carrots, broth, salt, pepper, and dill. Bring to a boil, and reduce heat. Cover, and simmer 20 minutes.".into(),
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "2. Stir in salmon, evaporated milk, corn, and cheese. Cook until heated through.".into(),
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Notes: A fantastic recipe!".into(),
                    ),
                ],
                recipe_category: vec!["Soups".into()],
                prep_time: vec![
                    DurationOrText::Text("15 mins.".into()),
                ],
                is_based_on_url: vec![
                    RecipeIsBasedOnUrlFieldEnum::URL(
                        "http://www.inakasoftware.com/".into(),
                    ),
                ],
                aggregate_rating: vec![
                    AggregateRating {
                        r#type: AtType::AggregateRating.to_opt(),
                        context: at_context(),
                        rating_count: vec![1],
                        rating_value: vec![
                            AggregateRatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                video: vec![
                    RecipeVideoFieldEnum::VideoObject(
                        VideoObject {
                            r#type: AtType::VideoObject.to_opt(),
                            context: at_context(),
                            url: vec![
                                "https://www.youtube.com/embed/2YbTARjGU1Q".into(),
                            ],
                            ..Default::default()
                        }.into(),
                    ),
                    RecipeVideoFieldEnum::VideoObject(
                        VideoObject {
                            r#type: AtType::VideoObject.to_opt(),
                            context: at_context(),
                            url: vec![
                                "https://www.youtube.com/embed/osQsujIxzgo".into(),
                            ],
                            ..Default::default()
                        }.into(),
                    ),
                    RecipeVideoFieldEnum::VideoObject(
                        VideoObject {
                            r#type: AtType::VideoObject.to_opt(),
                            context: at_context(),
                            url: vec![
                                "https://www.youtube.com/embed/R993HG7B7jw".into(),
                            ],
                            ..Default::default()
                        }.into(),
                    ),
                    RecipeVideoFieldEnum::VideoObject(
                        VideoObject {
                            r#type: AtType::VideoObject.to_opt(),
                            context: at_context(),
                            url: vec![
                                "https://www.youtube.com/embed/EYw5y806RxM".into(),
                            ],
                            ..Default::default()
                        }.into(),
                    ),
                ],
                author: vec![RecipeAuthorFieldEnum::new_person("Mike McGee")],
                name: vec![
                    "Alaskan Salmon Chowder".into(),
                ],
                url: vec!["http://www.inakasoftware.com/".into()],
                ..Default::default()
            },
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                recipe_yield: vec![
                    RecipeRecipeYieldFieldEnum::Number(10.0),
                ],
                recipe_cuisine: vec!["Italian".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 pint whipping cream".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 cup parmesan cheese".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 stick of butter.".into(),
                    ),
                ],
                cook_time: vec![
                    DurationOrText::Text("10 mins.".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Melt the butter and slowly add cheese cream and cooked pasta. You need to add the cheese slowly so it doesnt form clumps. I have modified this recipe by taking out all the butter and using whipping cream and milk instead of all whipping cream. It doesnt taste as good but its not nearly as bad. Sprinkle with nutmeg to serve".into(),
                    ),
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Notes: This recipe was included with the purchase of Computer Cuisine Deluxe. \u{b}\u{b}If you would like to remove all of the included recipes before you have entered any of your own, simply select \"Delete All Recipes...\" from the Recipes menu.".into(),
                    ),
                ],
                recipe_category: vec!["Sauces".into()],
                prep_time: vec![
                    DurationOrText::Text("10 mins.".into()),
                ],
                is_based_on_url: vec![
                    RecipeIsBasedOnUrlFieldEnum::URL(
                        "http://www.inakasoftware.com/".into(),
                    ),
                ],
                aggregate_rating: vec![
                    AggregateRating {
                        r#type: AtType::AggregateRating.to_opt(),
                        context: at_context(),
                        review_count: vec![],
                        item_reviewed: vec![],
                        rating_count: vec![
                            1,
                        ],
                        rating_value: vec![
                            AggregateRatingRatingValueFieldEnum::Number(4.0),
                        ],
                        ..Default::default()
                    },
                ],
                author: vec![RecipeAuthorFieldEnum::new_person("Mike McGee")],
                name: vec![
                    "Alfredo Sauce".into(),
                ],
                url: vec!["http://www.inakasoftware.com/".into()],
                ..Default::default()
            },
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                recipe_yield: vec![
                    RecipeRecipeYieldFieldEnum::Number(4.0),
                ],
                recipe_cuisine: vec![
                    "American".into(),
                ],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1/4 cups Salad oil".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "2 TBL. Soy sauce".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1/4 cups Bourbon, sherry, or wine".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 t Garlic powder".into(),
                    ),
                    RecipeRecipeIngredientFieldEnum::Text(
                        "1 t Pepper, freshly ground".into(),
                    ),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text(
                        "Combine all ingredients and pour over meat. Marinate in refrigerator. Also use to baste meat as you cook it. Good on red meat, fish or chicken.".into(),
                    ),
                ],
                recipe_category: vec![
                    "Barbeque".into(),
                ],
                prep_time: vec![
                    DurationOrText::Text("10 mins.".into()),
                ],
                is_based_on_url: vec![
                    RecipeIsBasedOnUrlFieldEnum::URL(
                        "http://www.inakasoftware.com/".into(),
                    ),
                ],
                aggregate_rating: vec![
                    AggregateRating {
                        r#type: AtType::AggregateRating.to_opt(),
                        context: at_context(),
                        rating_count: vec![1],
                        rating_value: vec![
                            AggregateRatingRatingValueFieldEnum::Number(4.0),
                        ],
                        ..Default::default()
                    },
                ],
                author: vec![RecipeAuthorFieldEnum::new_person("Mike McGee")],
                name: vec![
                    "All Purpose Barbeque Sauce".into(),
                ],
                url: vec!["http://www.inakasoftware.com/".into()],
                ..Default::default()
            },
            ]
        );
        Ok(())
    }

    fn csv_file1<'a>() -> &'a str {
        r#"calcium,Calorie Counter,Calories,Calories from Fat,Cholesterol,Cholesterol Level,Cooking Time,Date Last Prepared,Dietary Fiber,Favorite Recipe,Ingredients 3 Final,Iron,Item Name,Method,My Recipe,Photo Included,Preparation Time,Protein,Recipe From Date,Recipe From Email,Recipe From Name,Recipe From Pages,Recipe From Publication,Recipe From Source,Recipe From Website,Recipe Notes,Recipe Rating,Saturated Fat,Serves,Sodium,Source Detailed Information,Style,Sugars,Total Carbohydrate,Total Fat,Type,Video Reference Link 1,Video Reference Link 2,Video Reference Link 3,Video Reference Link 4,Vitamin A,Vitamin C,Recipe Difficulty
        "36","Medium","225","50","27","Medium","30 mins.","7/2/2010","0","Yes","3 tablespoons butter3/4 cup chopped onion1/2 cup chopped celery1 teaspoon garlic powder2 cups diced potatoes2 carrots, diced2 cups chicken broth1 teaspoon salt1 teaspoon ground black pepper1 teaspoon dried dill weed2 (16 ounce) cans salmon1 (12 fluid ounce) can evaporated milk1 (15 ounce) can creamed corn1/2 pound Cheddar cheese, shredded","0","Alaskan Salmon Chowder","1. Melt butter in a large pot over medium heat. Sauté onion, celery, and garlic powder until onions are tender. Stir in potatoes, carrots, broth, salt, pepper, and dill. Bring to a boil, and reduce heat. Cover, and simmer 20 minutes.2. Stir in salmon, evaporated milk, corn, and cheese. Cook until heated through.","No","Yes","15 mins.","20","Food Network","mike@inakasoftware.com","Mike McGee","June 2008","","Internet","http://www.inakasoftware.com/","A fantastic recipe! ","Highest","3","8","147","","American","2","29","4","Soups","https://www.youtube.com/embed/2YbTARjGU1Q","https://www.youtube.com/embed/osQsujIxzgo","https://www.youtube.com/embed/R993HG7B7jw","https://www.youtube.com/embed/EYw5y806RxM","0","0","Average"
        "","Medium","","","","Medium","10 mins.","","","No","1 pint whipping cream1 cup parmesan cheese1 stick of butter.","","Alfredo Sauce","Melt the butter and slowly add cheese cream and cooked pasta. You need to add the cheese slowly so it doesnt form clumps. I have modified this recipe by taking out all the butter and using whipping cream and milk instead of all whipping cream. It doesnt taste as good but its not nearly as bad. Sprinkle with nutmeg to serve","No","No","10 mins.","","","mike@inakasoftware.com","Mike McGee","","","Internet","http://www.inakasoftware.com/","This recipe was included with the purchase of Computer Cuisine Deluxe. If you would like to remove all of the included recipes before you have entered any of your own, simply select ""Delete All Recipes..."" from the Recipes menu.","Very Good","","10","","This recipe was included with the purchase of Computer Cuisine Deluxe. If you would like to remove all of the included recipes before you have entered any of your own, simply select ""Delete All Recipes..."" from the Recipes menu.","Italian","","","","Sauces","","","","","","","Easy"
        "","Low","","","","None","None","","","No","1/4 cups Salad oil2 TBL. Soy sauce1/4 cups Bourbon, sherry, or wine1 t Garlic powder1 t Pepper, freshly ground","","All Purpose Barbeque Sauce","Combine all ingredients and pour over meat. Marinate in refrigerator. Also use to baste meat as you cook it. Good on red meat, fish or chicken.","No","No","10 mins.","","","mike@inakasoftware.com","Mike McGee","","","Internet","http://www.inakasoftware.com/","","Very Good","","4","","This recipe was included with the purchase of Computer Cuisine Deluxe. If you would like to remove all of the included recipes before you have entered any of your own, simply select ""Delete All Recipes..."" from the Recipes menu.","American","","","","Barbeque","","","","","","","Average""#
    }
}
