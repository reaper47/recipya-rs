use std::{
    borrow::Cow,
    io::{Read, Seek},
    path::Path,
};

use scraper::{Html, Selector};
use zip::ZipArchive;

use schema_org::{
    AggregateRating, AtType, Comment, DurationOrText, Energy, Mass, NutritionInformation, Recipe,
    field::{RecipeImageFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeYieldFieldEnum},
};

use crate::{
    Result,
    apps::{
        helpers::{
            Ingredient, Instruction, Parsers, ToSections, extract_archive_contents,
            update_recipe_image_paths,
        },
        recipya::at_context,
    },
    helpers::to_is_based_on,
};

#[derive(Default)]
struct RecipeComponents<'a> {
    category: Option<Cow<'a, str>>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    images: Vec<Cow<'a, str>>,
    keywords: Vec<Cow<'a, str>>,
    notes: Option<Cow<'a, str>>,
    nutrition: Option<NutritionComponents<'a>>,
    prep_time: Option<Cow<'a, str>>,
    cook_time: Option<Cow<'a, str>>,
    rating: Option<f32>,
    title: Cow<'a, str>,
    source: Option<Cow<'a, str>>,
    r#yield: Option<Cow<'a, str>>,
}

impl From<RecipeComponents<'_>> for Recipe {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            aggregate_rating: r
                .rating
                .filter(|f| f.abs() > 0.0)
                .map_or(Vec::new(), |f| vec![AggregateRating::new(f, 1)]),
            comment: r
                .notes
                .map_or(Vec::new(), |s| vec![Comment::new(s.to_string())]),
            image: r
                .images
                .into_iter()
                .map(|img| RecipeImageFieldEnum::URL(img.to_string()))
                .collect(),
            is_based_on: to_is_based_on(&r.source.unwrap_or_default()),
            keywords: r
                .keywords
                .into_iter()
                .map(|k| RecipeKeywordsFieldEnum::TextOrURL(k.to_string()))
                .collect(),
            name: vec![r.title.to_string()],
            nutrition: r
                .nutrition
                .filter(|n| !n.is_empty())
                .map_or(Vec::new(), |n| vec![n.into()]),
            prep_time: r
                .prep_time
                .map_or(Vec::new(), |c| vec![DurationOrText::Text(c.to_string())]),
            cook_time: r
                .cook_time
                .map_or(Vec::new(), |c| vec![DurationOrText::Text(c.to_string())]),
            recipe_category: r.category.map_or(Vec::new(), |c| vec![c.to_string()]),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            recipe_yield: r.r#yield.map_or(Vec::new(), |s| {
                vec![RecipeRecipeYieldFieldEnum::Text(s.to_string())]
            }),
            ..Default::default()
        }
    }
}

struct NutritionComponents<'a> {
    serving_size: Option<&'a str>,
    calories: &'a str,
    carbohydrate: Option<&'a str>,
    cholesterol: Option<&'a str>,
    fat: Option<&'a str>,
    protein: Option<&'a str>,
    sodium: Option<&'a str>,
    sat_fat: Option<&'a str>,
    sugar: Option<&'a str>,
    fiber: Option<&'a str>,
}

impl NutritionComponents<'_> {
    const fn is_empty(&self) -> bool {
        self.calories.is_empty()
            && self.carbohydrate.is_none()
            && self.cholesterol.is_none()
            && self.fat.is_none()
            && self.protein.is_none()
            && self.sodium.is_none()
            && self.sat_fat.is_none()
            && self.sugar.is_none()
            && self.fiber.is_none()
    }
}

impl From<NutritionComponents<'_>> for NutritionInformation {
    fn from(n: NutritionComponents<'_>) -> Self {
        let to_mass = |opt: Option<String>| {
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
            calories: vec![Energy::new(if n.calories.ends_with("cal") {
                n.calories.into()
            } else {
                format!("{} kcal", n.calories)
            })],
            carbohydrate_content: to_mass(n.carbohydrate.map(|s| format!("{s} g"))),
            cholesterol_content: to_mass(n.cholesterol.map(|s| format!("{s} mg"))),
            context: at_context(),
            fat_content: to_mass(n.fat.map(|s| format!("{s} g"))),
            protein_content: to_mass(n.protein.map(|s| format!("{s} g"))),
            sodium_content: to_mass(n.sodium.map(|s| format!("{s} mg"))),
            saturated_fat_content: to_mass(n.sat_fat.map(|s| format!("{s} g"))),
            sugar_content: to_mass(n.sugar.map(|s| format!("{s} g"))),
            fiber_content: to_mass(n.fiber.map(|s| format!("{s} g"))),
            serving_size: n.serving_size.map_or(Vec::new(), |s| vec![s.into()]),
            r#type: AtType::NutritionInformation.to_opt(),
            ..Default::default()
        }
    }
}

// Parses a Recipe Keeper zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let archive = ZipArchive::new(r)?;

    let (mut recipes, images) = extract_archive_contents(
        archive,
        &Parsers {
            html: Some(parse_html),
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

#[allow(clippy::too_many_lines)]
fn parse_html<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;

    let doc = Html::parse_document(&buf);

    let sel_title = Selector::parse("h2[itemprop='name']").unwrap();
    let sel_rating = Selector::parse("meta[itemprop='recipeRating']").unwrap();
    let sel_course = Selector::parse("span[itemprop='recipeCourse']").unwrap();
    let sel_cat = Selector::parse("meta[itemprop='recipeCategory']").unwrap();
    let sel_img = Selector::parse("img.recipe-photo").unwrap();
    let sel_src = Selector::parse("span[itemprop='recipeSource']").unwrap();
    let sel_yield = Selector::parse("span[itemprop='recipeYield']").unwrap();
    let sel_prep = Selector::parse("meta[itemprop='prepTime']").unwrap();
    let sel_cook = Selector::parse("meta[itemprop='cookTime']").unwrap();
    let sel_ingredients = Selector::parse("div[itemprop='recipeIngredients'] > p").unwrap();
    let sel_instructions = Selector::parse("div[itemprop='recipeDirections'] > p").unwrap();
    let sel_b = Selector::parse("b").unwrap();
    let sel_notes = Selector::parse("div[itemprop='recipeNotes']").unwrap();

    let sel_nut_serving = Selector::parse("meta[itemprop='recipeNutServingSize']").unwrap();
    let sel_nut_cal = Selector::parse("meta[itemprop='recipeNutCalories']").unwrap();
    let sel_nut_fat = Selector::parse("meta[itemprop='recipeNutTotalFat']").unwrap();
    let sel_nut_sat_fat = Selector::parse("meta[itemprop='recipeNutSaturatedFat']").unwrap();
    let sel_nut_cho = Selector::parse("meta[itemprop='recipeNutCholesterol']").unwrap();
    let sel_nut_na = Selector::parse("meta[itemprop='recipeNutSodium']").unwrap();
    let sel_nut_carbs = Selector::parse("meta[itemprop='recipeNutTotalCarbohydrate']").unwrap();
    let sel_nut_fiber = Selector::parse("meta[itemprop='recipeNutDietaryFiber']").unwrap();
    let sel_nut_sugar = Selector::parse("meta[itemprop='recipeNutSugars']").unwrap();
    let sel_nut_protein = Selector::parse("meta[itemprop='recipeNutProtein']").unwrap();

    Ok(doc
        .select(&Selector::parse("div.recipe-details").unwrap())
        .map(|div| {
            let (main_category, mut keywords) = match div
                .select(&sel_course)
                .map(|el| el.text().collect::<String>())
                .collect::<Vec<_>>()
                .as_slice()
            {
                [first, rest @ ..] => (
                    Some(first.clone()).filter(|s| !s.trim().is_empty()),
                    rest.to_vec(),
                ),
                [] => (None, vec![]),
            };

            let other_keywords = div
                .select(&sel_cat)
                .filter_map(|el| el.attr("content"))
                .map(ToString::to_string)
                .collect::<Vec<_>>();

            keywords.extend_from_slice(other_keywords.as_slice());

            let extract_content = |sel: &Selector| {
                div.select(sel)
                    .next()
                    .map(|el| el.attr("content").unwrap_or_default())
            };

            RecipeComponents {
                category: main_category.map(Cow::Owned),
                ingredients: div
                    .select(&sel_ingredients)
                    .filter_map(|el| {
                        let text = el.text().collect::<String>();
                        if text.trim().is_empty() {
                            return None;
                        }

                        el.select(&sel_b).next().map_or(
                            Some(Ingredient::Line(Cow::Owned(text))),
                            |s| {
                                let text = s.text().collect::<String>();
                                Some(Ingredient::Section(Cow::Owned(text)))
                            },
                        )
                    })
                    .collect::<Vec<_>>(),
                instructions: div
                    .select(&sel_instructions)
                    .map(|el| el.text().collect::<String>())
                    .filter(|s| !s.is_empty())
                    .map(|s| Instruction::Line(Cow::Owned(s)))
                    .collect::<Vec<_>>(),
                images: div
                    .select(&sel_img)
                    .filter_map(|el| el.attr("src"))
                    .map(|s| Cow::Borrowed(s.trim_start_matches("images/")))
                    .collect(),
                keywords: keywords.into_iter().map(Cow::Owned).collect(),
                notes: div
                    .select(&sel_notes)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .filter(|s| !s.trim().is_empty())
                    .map(Cow::Owned),
                nutrition: Some(NutritionComponents {
                    serving_size: extract_content(&sel_nut_serving),
                    calories: extract_content(&sel_nut_cal).unwrap_or_default(),
                    carbohydrate: extract_content(&sel_nut_carbs),
                    cholesterol: extract_content(&sel_nut_cho),
                    fat: extract_content(&sel_nut_fat),
                    protein: extract_content(&sel_nut_protein),
                    sodium: extract_content(&sel_nut_na),
                    sat_fat: extract_content(&sel_nut_sat_fat),
                    sugar: extract_content(&sel_nut_sugar),
                    fiber: extract_content(&sel_nut_fiber),
                }),
                title: div
                    .select(&sel_title)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .map(Cow::Owned)
                    .unwrap_or_default(),
                r#yield: div
                    .select(&sel_yield)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .map(Cow::Owned),
                rating: div.select(&sel_rating).next().map(|el| {
                    el.attr("content")
                        .map(|s| s.parse().unwrap_or_default())
                        .filter(|f| f > &0.0)
                        .unwrap_or_default()
                }),
                source: div
                    .select(&sel_src)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .map(Cow::Owned),
                prep_time: div
                    .select(&sel_prep)
                    .next()
                    .map(|el| el.attr("content"))
                    .filter(Option::is_some)
                    .map(|s| Cow::Borrowed(s.unwrap_or_default())),
                cook_time: div
                    .select(&sel_cook)
                    .next()
                    .map(|el| el.attr("content"))
                    .filter(Option::is_some)
                    .map(|s| Cow::Borrowed(s.unwrap_or_default())),
            }
            .into()
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use std::io::Cursor;

        use super::*;

        #[test]
        fn test_recipekeeper_html_ok() -> Result<()> {
            let buf = Cursor::new(files::html());

            let got = parse_html(buf)?;

            let expected = results::html();
            pretty_assertions::assert_eq!(got.len(), expected.len());
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }
    }

    mod files {
        pub fn html<'a>() -> &'a str {
            r#"<!DOCTYPE html><html><head><style type=text/css>html{}body{font-family:sans-serif;font-size:14px;line-height:1.4;color:#333;background-color:#fff}h2,h3{font-weight:500;line-height:1.1;margin-top:20px;margin-bottom:10px}h2{font-size:24px}h3{font-size:14px}.recipe-details h2{color:#d24400}.recipe-details h3{color:#d24400}.recipe-ingredients p{margin:0}.recipe-notes p{margin:0}.recipe-photo{width:250px;height:250px;margin-top:20px;object-fit:cover}.recipe-photos-div{width:125px;height:125px;margin-right:5px;margin-bottom:5px;display:inline-block}.recipe-photos{width:100%;height:100%;object-fit:contain}</style></head><body><div class=recipe-details><meta content=a2489b23-050f-5c8a-8e5d-12fdb9fdeea3 itemprop=recipeId><meta content="" itemprop=recipeShareId><meta content=False itemprop=recipeIsFavourite><meta content=0 itemprop=recipeRating><table><tr><td><img src=images/a2489b23-050f-5c8a-8e5d-12fdb9fdeea3_0.jpg class="recipe-photo"/></td><td style=vertical-align:top><h2 itemprop=name>Blueberry Cheesecake</h2><div>Courses: <span itemprop=recipeCourse>Dessert</span></div><div>Categories: <span>Cake</span><meta content=Cake itemprop=recipeCategory></div><div>Collections: <span></span></div><div>Source: <span itemprop=recipeSource>Recipe Keeper</span></div><div>Serving size: <span itemprop=recipeYield>Serves 6-8</span></div><div>Preparation time: <span>10 mins</span><meta content=PT10M itemprop=prepTime></div><div>Cooking time: <span>1 hour </span><meta content=PT1H itemprop=cookTime></div></td></tr><tr><td style=vertical-align:top;width:250px><h3>Ingredients</h3><div class=recipe-ingredients itemprop=recipeIngredients><p><b>For the tart shell</b></p><p>1 1/2 cups all-purpose flour</p><p>1/2 cup icing sugar</p><p>2/3 cup softened butter</p><p>pinch of salt</p><p>1 egg yolk</p><p></p><p><b>For the lemon curd</b></p><p>6 lemons</p><p>6 large eggs</p><p>1 1/2 cups caster sugar</p><p>1 1/2 cups cream</p><p></p><p><b><i>To serve</i></b></p><p>Icing sugar</p><p>Cream</p><p></p></div></td><td style=vertical-align:top><h3>Directions</h3><div itemprop=recipeDirections><p>1. Melt the butter and pour onto the cracker crumbs and mix well. Press into the bottom of a 9-inch spring form pan. Bake at 325&#176;F until the crust is set, about 10-12 minutes. Allow to cool.</p><p></p><p>2. In a large bowl beat the cream cheese with the flour, caster sugar, eggs, soured cream and vanilla extract with an electric mixer until light and fluffy.</p><p></p><p>3. Pour the mixture into the pan and bake for 35-40 minutes until set. Remove from the oven and leave to cool.</p><p></p><p>4. Heat half the blueberries in a pan with 2 tablespoons icing sugar and stir gently until juicy. Squash the blueberries with a fork then continue to cook for a few minutes. Add the remaining blueberries, remove from the heat and allow to cool.</p><p></p><p>5. Pour the blueberries over the cheesecake just before serving.</p><p></p></div></td></tr></table><div class=recipe-notes itemprop=recipeNotes></div><h3>Nutrition</h3><div>Amount per serving</div><div>Serving size: 1 slice<meta content="1 slice" itemprop=recipeNutServingSize></div><div>Calories: 571<meta content=571 itemprop=recipeNutCalories></div><div>Total Fat: 41.5g<meta content=41.5 itemprop=recipeNutTotalFat></div><div>Saturated Fat: 24.7g<meta content=24.7 itemprop=recipeNutSaturatedFat></div><div>Cholesterol: 173mg<meta content=173 itemprop=recipeNutCholesterol></div><div>Sodium: 451mg<meta content=451 itemprop=recipeNutSodium></div><div>Total Carbohydrate: 42g<meta content=42 itemprop=recipeNutTotalCarbohydrate></div><div>Dietary Fiber: 1.5g<meta content=1.5 itemprop=recipeNutDietaryFiber></div><div>Sugars: 27.1g<meta content=27.1 itemprop=recipeNutSugars></div><div>Protein: 10.1g<meta content=10.1 itemprop=recipeNutProtein></div><h3>Photos</h3><div class=recipe-photos-div><img src=images/a2489b23-050f-5c8a-8e5d-12fdb9fdeea3_0.jpg class=recipe-photos itemprop=photo0/></div><hr/></div><div class=recipe-details><meta content=b0bcddc4-23e8-50cc-a879-22b1b2e63919 itemprop=recipeId><meta content="" itemprop=recipeShareId><meta content=False itemprop=recipeIsFavourite><meta content=0 itemprop=recipeRating><table><tr><td><img src=images/b0bcddc4-23e8-50cc-a879-22b1b2e63919_0.jpg class="recipe-photo"/></td><td style=vertical-align:top><h2 itemprop=name>Chocolate Chip Cookies</h2><div>Courses: <span itemprop=recipeCourse>Snack</span></div><div>Categories: <span>Cookie</span><meta content=Cookie itemprop=recipeCategory></div><div>Collections: <span></span></div><div>Source: <span itemprop=recipeSource>Recipe Keeper</span></div><div>Serving size: <span itemprop=recipeYield>12</span></div><div>Preparation time: <span>5 mins</span><meta content=PT5M itemprop=prepTime></div><div>Cooking time: <span>12 mins</span><meta content=PT12M itemprop=cookTime></div></td></tr><tr><td style=vertical-align:top;width:250px><h3>Ingredients</h3><div class=recipe-ingredients itemprop=recipeIngredients><p>2 1/4 cups all-purpose flour</p><p>1 teaspoon baking soda</p><p>1 teaspoon salt</p><p>1 cup butter</p><p>1 cup caster sugar</p><p>1 cup soft brown sugar</p><p>1 teaspoon vanilla extract</p><p>2 eggs</p><p>2 cups dark chocolate, broken into small pieces</p><p></p></div></td><td style=vertical-align:top><h3>Directions</h3><div itemprop=recipeDirections><p>1. In a large bowl combine the flour, baking soda and salt.</p><p></p><p>2. In a separate bowl, mix the butter, caster sugar, brown sugar and vanilla extract until smooth.</p><p></p><p>3. Add the eggs and the flour to the mixture and beat to combine.</p><p></p><p>4. Add the chocolate pieces and stir.</p><p></p><p>5. Drop well rounded spoonfuls of dough onto a greased cookie sheet.</p><p></p><p>6. Bake at 375F for 8-10 minutes.</p><p></p><p>7. Remove from the oven and place cookies on a wire rack to cool.</p><p></p></div></td></tr></table><div class=recipe-notes itemprop=recipeNotes></div><h3>Nutrition</h3><div>Amount per serving</div><div>Serving size: 1 cookie<meta content="1 cookie" itemprop=recipeNutServingSize></div><div>Calories: 491<meta content=491 itemprop=recipeNutCalories></div><div>Total Fat: 24.6g<meta content=24.6 itemprop=recipeNutTotalFat></div><div>Saturated Fat: 15.8g<meta content=15.8 itemprop=recipeNutSaturatedFat></div><div>Cholesterol: 74mg<meta content=74 itemprop=recipeNutCholesterol></div><div>Sodium: 443mg<meta content=443 itemprop=recipeNutSodium></div><div>Total Carbohydrate: 63.2g<meta content=63.2 itemprop=recipeNutTotalCarbohydrate></div><div>Dietary Fiber: 1.6g<meta content=1.6 itemprop=recipeNutDietaryFiber></div><div>Sugars: 43g<meta content=43 itemprop=recipeNutSugars></div><div>Protein: 5.7g<meta content=5.7 itemprop=recipeNutProtein></div><h3>Photos</h3><div class=recipe-photos-div><img src=images/b0bcddc4-23e8-50cc-a879-22b1b2e63919_0.jpg class=recipe-photos itemprop=photo0/></div><hr/></div></body></html>"#
        }
    }

    mod results {
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        };

        use super::*;

        #[allow(clippy::too_many_lines)]
        pub fn html() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: vec![DurationOrText::Text("PT1H".into())],
                    prep_time: vec![DurationOrText::Text("PT10M".into())],
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Cake".into())],
                    image: vec![RecipeImageFieldEnum::URL("a2489b23-050f-5c8a-8e5d-12fdb9fdeea3_0.jpg".into())],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("571 kcal")],
                        carbohydrate_content: vec![Mass::new("42 g")],
                        cholesterol_content: vec![Mass::new("173 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("41.5 g")],
                        protein_content: vec![Mass::new("10.1 g")],
                        sodium_content: vec![Mass::new("451 mg")],
                        saturated_fat_content: vec![Mass::new("24.7 g")],
                        sugar_content: vec![Mass::new("27.1 g")],
                        fiber_content: vec![Mass::new("1.5 g")],
                        serving_size: vec!["1 slice".into()],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section(
                            "For the tart shell",
                            &[
                                "1 1/2 cups all-purpose flour",
                                "1/2 cup icing sugar",
                                "2/3 cup softened butter",
                                "pinch of salt",
                                "1 egg yolk",
                            ],
                        ),
                        RecipeRecipeIngredientFieldEnum::new_section(
                            "For the lemon curd",
                            &[
                                "6 lemons",
                                "6 large eggs",
                                "1 1/2 cups caster sugar",
                                "1 1/2 cups cream",
                            ],
                        ),
                        RecipeRecipeIngredientFieldEnum::new_section(
                            "To serve",
                            &["Icing sugar", "Cream"],
                        ),
                    ],
                    recipe_category: vec!["Dessert".into()],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Melt the butter and pour onto the cracker crumbs and mix well. Press into the bottom of a 9-inch spring form pan. Bake at 325°F until the crust is set, about 10-12 minutes. Allow to cool.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "In a large bowl beat the cream cheese with the flour, caster sugar, eggs, soured cream and vanilla extract with an electric mixer until light and fluffy.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Pour the mixture into the pan and bake for 35-40 minutes until set. Remove from the oven and leave to cool.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Heat half the blueberries in a pan with 2 tablespoons icing sugar and stir gently until juicy. Squash the blueberries with a fork then continue to cook for a few minutes. Add the remaining blueberries, remove from the heat and allow to cool.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Pour the blueberries over the cheesecake just before serving.".into(),
                        ),
                    ],
                    recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("Serves 6-8".into())],
                    name: vec!["Blueberry Cheesecake".into()],
                    is_based_on: to_is_based_on("Recipe Keeper"),
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    cook_time: vec![DurationOrText::Text("PT12M".into())],
                    prep_time: vec![DurationOrText::Text("PT5M".into())],
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Cookie".into())],
                    image: vec![RecipeImageFieldEnum::URL("b0bcddc4-23e8-50cc-a879-22b1b2e63919_0.jpg".into())],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("491 kcal")],
                        carbohydrate_content: vec![Mass::new("63.2 g")],
                        cholesterol_content: vec![Mass::new("74 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("24.6 g")],
                        protein_content: vec![Mass::new("5.7 g")],
                        sodium_content: vec![Mass::new("443 mg")],
                        saturated_fat_content: vec![Mass::new("15.8 g")],
                        sugar_content: vec![Mass::new("43 g")],
                        fiber_content: vec![Mass::new("1.6 g")],
                        serving_size: vec!["1 cookie".into()],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 1/4 cups all-purpose flour".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 teaspoon baking soda".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 teaspoon salt".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup butter".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup caster sugar".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 cup soft brown sugar".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 teaspoon vanilla extract".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 eggs".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 cups dark chocolate, broken into small pieces".into(),
                        ),
                    ],
                    recipe_category: vec!["Snack".into()],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                           "In a large bowl combine the flour, baking soda and salt.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "In a separate bowl, mix the butter, caster sugar, brown sugar and vanilla extract until smooth.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the eggs and the flour to the mixture and beat to combine.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the chocolate pieces and stir.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Drop well rounded spoonfuls of dough onto a greased cookie sheet.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Bake at 375F for 8-10 minutes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Remove from the oven and place cookies on a wire rack to cool.".into(),
                        ),
                    ],
                    recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("12".into())],
                    name: vec!["Chocolate Chip Cookies".into()],
                    is_based_on: to_is_based_on("Recipe Keeper"),
                    ..Default::default()
                },
            ]
        }
    }
}
