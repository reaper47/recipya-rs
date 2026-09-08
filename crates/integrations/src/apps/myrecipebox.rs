use std::{
    borrow::Cow,
    collections::HashMap,
    env::temp_dir,
    fmt::Write,
    fs::File,
    io::{self, Read, Seek},
    path::PathBuf,
};

use itertools::Itertools;
use scraper::{Html, Node, Selector};
use serde::Deserialize;
use tracing::error;
use zip::ZipArchive;

use schema_org::{
    AggregateRating, AtType, Comment, DurationOrText, Energy, Mass, NutritionInformation, Recipe,
    VideoObject, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeVideoFieldEnum, RecipeYieldFieldEnum,
    },
};

use crate::{
    Result,
    apps::helpers::{
        Ingredient, Instruction, Parsers, ToSections, parse_archive_helper, read_file,
    },
};

#[derive(Debug, Deserialize)]
pub struct CsvRecord<'a> {
    pub title: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    #[serde(rename = "preparationTime")]
    pub preparation_time: Option<Cow<'a, str>>,
    #[serde(rename = "cookingTime")]
    pub cooking_time: Option<Cow<'a, str>>,
    pub quantity: Option<Cow<'a, str>>,
    pub ingredients: Cow<'a, str>,
    pub instructions: Cow<'a, str>,
    pub notes: Option<Cow<'a, str>>,
    pub nutrition: Option<Cow<'a, str>>,
    pub rating: Option<Cow<'a, str>>,
    pub categories: Option<Cow<'a, str>>,
    pub tags: Option<Cow<'a, str>>,
    pub video: Option<Cow<'a, str>>,
    pub source: Option<Cow<'a, str>>,
    #[serde(rename = "originalPicture")]
    pub original_picture: Option<Cow<'a, str>>,
}

impl From<CsvRecord<'_>> for Recipe {
    #[allow(clippy::too_many_lines)]
    fn from(r: CsvRecord) -> Self {
        let cat = r.categories.unwrap_or_default();
        let parts = cat.split(';').collect_vec();
        let (category, keywords) = parts.as_slice().split_first().unwrap_or((&"", &[""]));

        let tag = r.tags.unwrap_or_default();
        let mut tags = tag.split(';').collect_vec();
        tags.extend(keywords);
        tags.sort_unstable();

        Self {
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: r.rating.map_or(Vec::new(), |rating| {
                vec![AggregateRating::new(rating.parse().unwrap_or(0.), 1)]
            }),
            context: at_context(),
            comment: r.notes.map_or(Vec::new(), |c| {
                vec![Comment {
                    text: vec![c.trim().to_string()],
                    ..Default::default()
                }]
            }),
            description: r.description.map_or(Vec::new(), |d| {
                vec![RecipeDescriptionFieldEnum::Text(d.to_string())]
            }),
            name: vec![r.title.trim().trim_matches('"').to_string()],
            prep_time: r.preparation_time.map_or(Vec::new(), |d| {
                vec![DurationOrText::Text(if d.ends_with("min") {
                    d.to_string()
                } else {
                    format!("{d} min")
                })]
            }),
            cook_time: r.cooking_time.map_or(Vec::new(), |d| {
                vec![DurationOrText::Text(if d.ends_with("min") {
                    d.to_string()
                } else {
                    format!("{d} min")
                })]
            }),
            keywords: tags
                .iter()
                .unique()
                .filter(|s| !s.trim().is_empty())
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_string()))
                .collect_vec(),
            nutrition: r.nutrition.map_or(Vec::new(), |s| {
                let nut = s.lines().map(str::trim).collect_vec();

                let extract_mass = |prefix: &str| {
                    nut.iter()
                        .find(|s| s.starts_with(prefix))
                        .map_or(Vec::new(), |&s| {
                            vec![Mass::new(s.trim_start_matches(prefix))]
                        })
                };

                let nut = NutritionInformation {
                    calories: nut
                        .iter()
                        .find(|s| s.starts_with("calories : "))
                        .map_or(Vec::new(), |&s| {
                            vec![Energy::new(s.trim_start_matches("calories : "))]
                        }),
                    carbohydrate_content: extract_mass("carbohydrate : "),
                    cholesterol_content: extract_mass("cholesterol : "),
                    context: at_context(),
                    fat_content: extract_mass("fat : "),
                    fiber_content: extract_mass("fiber : "),
                    protein_content: extract_mass("protein : "),
                    serving_size: nut
                        .iter()
                        .find(|s| s.starts_with("servingSize : "))
                        .map_or(Vec::new(), |&s| {
                            vec![s.trim_start_matches("servingSize : ").to_string()]
                        }),
                    sodium_content: extract_mass("sodium : "),
                    sugar_content: extract_mass("sugar : "),
                    trans_fat_content: extract_mass("transFat : "),
                    saturated_fat_content: extract_mass("saturatedFat : "),
                    unsaturated_fat_content: extract_mass("unsaturatedFat : "),
                    r#type: AtType::NutritionInformation.to_opt(),
                };

                if nut.is_empty() { vec![] } else { vec![nut] }
            }),
            recipe_category: if category.is_empty() {
                vec![]
            } else {
                vec![category.to_string()]
            },
            recipe_yield: r.quantity.map_or(Vec::new(), |q| {
                vec![RecipeYieldFieldEnum::Number(q.parse().unwrap_or_default())]
            }),
            recipe_ingredient: r
                .ingredients
                .lines()
                .map(|s| Ingredient::Line(Cow::Borrowed(s)))
                .collect_vec()
                .to_sections(),
            recipe_instructions: r
                .instructions
                .split("\n\n")
                .map(|s| Instruction::Line(Cow::Borrowed(s)))
                .collect_vec()
                .to_sections(),
            image: r.original_picture.map_or(Vec::new(), |s| {
                vec![RecipeImageFieldEnum::URL(s.to_string())]
            }),
            video: r.video.map_or(Vec::new(), |s| {
                vec![RecipeVideoFieldEnum::VideoObject(
                    VideoObject {
                        r#type: AtType::VideoObject.to_opt(),
                        context: at_context(),
                        url: vec![s.into()],
                        ..Default::default()
                    }
                    .into(),
                )]
            }),
            url: r.source.map_or(Vec::new(), |s| vec![s.to_string()]),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeRtk {
    pub title: String,
    pub description: String,
    pub preparation_time: String,
    pub cooking_time: String,
    pub inactive_time: String,
    pub total_time: String,
    pub quantity: String,
    pub ingredients: String,
    pub instructions: String,
    pub pictures: Vec<String>,
    pub url: String,
    pub video: String,
    pub notes: String,
    pub cookware: String,
    pub nutrition: String,
    pub rating: f64,
    pub last_modified_date: String,
    pub uuid: String,
    pub original_picture: String,
    pub categories: Vec<Tag>,
    pub tags: Vec<Tag>,
    pub keywords: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Tag {
    pub title: String,
}

impl From<RecipeRtk> for CsvRecord<'_> {
    fn from(r: RecipeRtk) -> Self {
        Self {
            title: Cow::Owned(r.title),
            description: (!r.description.is_empty()).then_some(Cow::Owned(r.description)),
            preparation_time: (!r.preparation_time.is_empty())
                .then_some(Cow::Owned(r.preparation_time)),
            cooking_time: (!r.cooking_time.is_empty()).then_some(Cow::Owned(r.cooking_time)),
            quantity: (!r.quantity.is_empty()).then_some(Cow::Owned(r.quantity)),
            ingredients: Cow::Owned(r.ingredients),
            instructions: Cow::Owned(r.instructions),
            notes: (!r.notes.is_empty()).then_some(Cow::Owned(r.notes)),
            nutrition: (!r.nutrition.is_empty()).then_some(Cow::Owned(r.nutrition)),
            rating: (!r.rating.eq(&0.0)).then_some(Cow::Owned(r.rating.to_string())),
            categories: (!r.categories.is_empty()).then(|| {
                Cow::Owned(
                    r.categories
                        .into_iter()
                        .map(|t| t.title)
                        .collect_vec()
                        .join(";"),
                )
            }),
            tags: (!r.tags.is_empty())
                .then(|| Cow::Owned(r.tags.into_iter().map(|t| t.title).collect_vec().join(";"))),
            video: (!r.video.is_empty()).then_some(Cow::Owned(r.video)),
            source: (!r.url.is_empty()).then_some(Cow::Owned(r.url)),
            original_picture: (!r.pictures.is_empty())
                .then(|| Cow::Owned(r.pictures.first().cloned().unwrap_or_default())),
        }
    }
}

/// Parses a `My Recipe Box` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let recipes = parse_archive_helper(
        r,
        &Parsers {
            csv: Some(parse_csv),
            html: Some(parse_html),
            ..Default::default()
        },
    )?;

    Ok(recipes
        .into_iter()
        .filter(|r| r.name.first() != Some(&String::new()))
        .collect_vec())
}

/// Parses a `My Recipe Box` CSV file.
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

/// Parses a `My Recipe Box` RTK file.
pub fn parse_rtk<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let mut recipes = Vec::new();
    let mut images: HashMap<String, PathBuf> = HashMap::new();

    let mut archive = ZipArchive::new(r)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        if file_name.starts_with("recipes") {
            let recipes_de: Vec<RecipeRtk> = serde_json::from_reader(file)?;
            recipes.extend(recipes_de);
        } else if std::path::Path::new(&file_name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
        {
            let tmp_path = temp_dir().join(&file_name);
            let mut tmp_file = File::create(&tmp_path)?;
            io::copy(&mut file, &mut tmp_file)?;
            images.insert(file_name, tmp_path);
        }
    }

    Ok(recipes
        .into_iter()
        .map(|mut r| {
            for image in &mut r.pictures {
                *image = images
                    .get(image.trim_start_matches("/data/user/0/fr.recettetek/files/Pictures/"))
                    .map_or_else(|| image.clone(), |s| s.to_string_lossy().to_string());
            }
            r
        })
        .map(CsvRecord::from)
        .map(Into::into)
        .collect_vec())
}

fn parse_html<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let doc = Html::parse_document(&content);

    let extract_text_opt = |sel: &str| {
        doc.select(&Selector::parse(sel).unwrap())
            .next()
            .map(|el| el.text().collect::<String>())
            .map(Cow::Owned)
    };

    let extract_newlines = |sel: &str| {
        doc.select(&Selector::parse(sel).unwrap()).next().map(|el| {
            let mut out = String::new();
            for node in el.descendants() {
                match node.value() {
                    Node::Text(text) => write!(out, "{}", &**text).unwrap(),
                    Node::Element(element) if element.name() == "br" => writeln!(out).unwrap(),
                    _ => {}
                }
            }
            Cow::Owned(out)
        })
    };

    let extract_after_h4 = |heading: &str| {
        let target = doc.select(&Selector::parse("h4").unwrap()).find(|el| {
            el.text()
                .collect::<String>()
                .trim()
                .eq_ignore_ascii_case(heading)
        })?;

        let mut out = String::new();
        for sibling in target.next_siblings() {
            match sibling.value() {
                Node::Element(_) => break,
                Node::Text(text) => write!(out, "{}", &**text).unwrap(),
                _ => {}
            }
        }

        let trimmed = out.trim();
        (!trimmed.is_empty()).then(|| Cow::Owned(trimmed.to_string()))
    };

    let recipe = CsvRecord {
        title: extract_text_opt("h2[itemprop='name']").unwrap_or_default(),
        description: extract_text_opt("p[itemprop='description']"),
        preparation_time: extract_text_opt("span[itemprop='prepTime']"),
        cooking_time: extract_text_opt("span[itemprop='cookTime']"),
        quantity: extract_text_opt("span[itemprop='recipeYield']"),
        ingredients: extract_newlines("div[itemprop='recipeIngredient']").unwrap_or_default(),
        instructions: extract_newlines("div[itemprop='recipeInstructions']").unwrap_or_default(),
        notes: extract_after_h4("Notes"),
        nutrition: extract_newlines("div[itemprop='nutrition']"),
        rating: doc
            .select(&Selector::parse("p").unwrap())
            .find(|el| el.text().collect::<String>().trim_start().starts_with('★'))
            .and_then(|el| {
                let text = el.text().collect::<String>();
                text.split(':').nth(1).map(|s| s.trim().to_string())
            })
            .map(Cow::Owned),
        categories: None,
        tags: None,
        video: None,
        source: extract_text_opt("a"),
        original_picture: doc
            .select(&Selector::parse("img[itemprop='image']").unwrap())
            .next()
            .map(|el| el.attr("src").unwrap_or_default())
            .map(Cow::Borrowed),
    };

    Ok(vec![recipe.into()])
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use std::io::Cursor;

        use test_fixtures::open_test_file;

        use super::*;

        #[test]
        fn test_myrecipebox_csv_ok() -> Result<()> {
            let buf = Cursor::new(files::csv());

            let got = parse_csv(buf)?;

            assert_eq!(got.len(), 2);
            pretty_assertions::assert_eq!(got, vec![results::recipe1(), results::recipe2()]);
            Ok(())
        }

        #[test]
        fn test_myrecipebox_html1_ok() -> Result<()> {
            let buf = Cursor::new(files::html1());

            let got = parse_html(buf)?;

            assert_eq!(got.len(), 1);
            let mut expected = results::recipe1();
            expected.image = vec![RecipeImageFieldEnum::URL(
                "ee093b2f-ee1b-47ec-9b9e-23488f37be18.png".into(),
            )];
            expected.keywords.clear();
            expected.recipe_category.clear();
            pretty_assertions::assert_eq!(got, vec![expected]);
            Ok(())
        }

        #[test]
        fn test_myrecipebox_html2_ok() -> Result<()> {
            let buf = Cursor::new(files::html2());

            let got = parse_html(buf)?;

            assert_eq!(got.len(), 1);
            let mut expected = results::recipe2();
            expected.image = vec![RecipeImageFieldEnum::URL(
                "2eaffd07-8262-439d-90bd-9567968a87ef.png".into(),
            )];
            expected.keywords.clear();
            expected.recipe_category.clear();
            pretty_assertions::assert_eq!(got, vec![expected]);
            Ok(())
        }

        #[test]
        fn test_myrecipebox_rtk_ok() -> Result<()> {
            let buf = open_test_file("integrations/myrecipebox.rtk");

            let got = parse_rtk(buf)?;

            assert_eq!(got.len(), 2);
            let mut expected1 = results::recipe1();
            let mut expected2 = results::recipe2();
            expected1.image = got[0].image.clone();
            expected2.image = got[1].image.clone();
            pretty_assertions::assert_eq!(got, vec![expected1, expected2]);
            Ok(())
        }
    }

    mod files {
        pub fn csv<'a>() -> &'a str {
            r#"title,description,preparationTime,cookingTime,inactiveTime,totalTime,quantity,ingredients,instructions,notes,nutrition,favorite,rating,categories,tags,video,source,originalPicture
            "Chicken Spaghetti Recipe","Tender chicken and spaghetti baked in a creamy cheddar-Parmesan sauce with a little kick from tomatoes with chilies. Cozy, family-friendly, and perfectly bubbly.",20,40,,60,6,"8 ounces spaghetti
            2 cups cooked shredded chicken
            1 (14 ounce) can canned diced tomatoes with chilies (such as Rotel, or petite diced tomatoes, lightly drained)
            ¼ cup salted butter
            1 medium yellow onion (chopped)
            1 clove garlic (minced)
            ½ green bell pepper (chopped)
            ¼ cup all-purpose flour
            1 teaspoon Italian seasoning
            1 cup chicken broth
            1 cup half and half (or light cream)
            ½ cup shredded Parmesan cheese
            2 cups shredded sharp cheddar cheese (divided)
            ¼ teaspoon salt (more to taste)
            ¼ teaspoon black pepper (more to taste)","Preheat the oven to 375°F. Grease a 9x13-inch baking dish.

            Cook the spaghetti al dente in a large pot of salted water according to package directions. Drain well, do not rinse.

            Meanwhile, to make the sauce, in a large skillet, melt the butter over medium heat. Add onion, garlic, and bell pepper and cook until tender. Stir in the flour and Italian seasoning and cook for 1-2 minutes.

            Gradually add the broth and half and half, a bit at a time, whisking until smooth after each addition. Bring to a boil and let boil 1 minute or until thick and bubbly.

            Remove from heat and whisk in the Parmesan cheese, 1 cup cheddar cheese, and salt and pepper.

            Combine spaghetti, chicken, cream sauce, and canned tomatoes. Mix well. Spread in baking dish.

            Top with remaining 1 cup of cheddar cheese and bake for 25-30 minutes or until hot and bubbly.",,"calories : 518 kcal
            carbohydrate : 38 g
            protein : 22 g
            fat : 30 g
            saturatedFat : 18 g
            cholesterol : 95 mg
            sodium : 606 mg
            fiber : 2 g
            sugar : 3 g
            servingSize : 1 serving",false,3.0,Starter,Potluck;Delicious,,"https://www.spendwithpennies.com/homemade-chicken-spaghetti/","https://www.spendwithpennies.com/wp-content/uploads/2026/01/cropped-Chicken-Spaghetti-6-SpendWithPennies-480x270.jpg"
            "Finnish blueberry pie","This delicious dessert has a cookie-like base, juicy blueberries and creamy topping. Easy, delicious, and even better made ahead.",15,30,,45,8,"7 tbsp unsalted butter
            1 egg
            1/3 cup sugar
            1 cup all purpose flour
            3 tbsp oats (or oat flour)
            1 tsp baking powder
            1 pinch salt
            1/2 tsp cardamom
            1 1/2 cups blueberries (wild or regular, fresh or frozen (see notes))
            1 cup sour cream
            1/4 cup sugar
            1 egg
            1 tsp vanilla extract ((optional))","Preheat the oven to 375F/190C. Lightly butter a 10 - 11 inch (25 - 28cm) diameter round shallow baking dish.

            Place the ingredients for the base (butter, egg, sugar, flour, oats, baking powder, salt and cardamom) in a food processor and pulse together to mix them well. The mixture will come together as a fairly soft dough. If you don't have a food processor, you can beat the butter and sugar, then add in the egg followed by the dry ingredients.

            Press the base mixture into the baking dish, pressing all the way to the edges and up the sides slightly to give a slight rim (if your dish is shallow, around 1in/3cm deep) you can press right up to the top edge, but lower if the dish is deeper). Smooth the bottom of the dough relatively flat.

            Place the dish with the dough in the fridge to chill gently while you prepare the filling. Wash the blueberries and pick over ot remove any stems.

            Combine the sour cream, sugar, egg and vanilla extract, if using, and whisk together to be smooth and well combined.

            Place the blueberries over the base mixture in the baking dish and spread in an even layer. If you like, you can keep some back to place on top of the cream layer, or instead do as I did here and lift a few up after adding and cream and clean off the top edge so they show through a little.

            Pour the sour cream mixture over the blueberries and spread evenly. If you like, you can keep all the blueberries under the cream layer or, as mentioned above, you can lift a few up and clean off the top to let a little of the berry show through. Alternatively, if you reserved a few blueberries, dot them evenly so they are partly under cream, partly above.

            Place the dish on a baking sheet to catch any drips, then bake the pie for approximately 30 - 40 minutes until the edges are crisp and the top is dry and seems gently set (it will firm a little more as it cools). If the cream is gently brown at the edges, that's fine, but you want to avoid the base layer edges being too brown.

            Allow the pie to cool a good 30 minutes or more before serving.","The bestest notes! ","calories : 310 kcal
            carbohydrate : 36 g
            protein : 5 g
            fat : 17 g
            saturatedFat : 10 g
            transFat : 0.4 g
            cholesterol : 85 mg
            sodium : 32 mg
            fiber : 1 g
            sugar : 19 g
            unsaturatedFat : 5 g
            servingSize : 1 serving",false,4.5,"The main course;Dessert",Potluck;Delicious;Meat,,"https://www.carolinescooking.com/finnish-blueberry-pie/","https://www.carolinescooking.com/wp-content/uploads/2023/06/Finnish-blueberry-pie-Mustikkapiirakka-featured-pic-sq-480x270.jpg"
"#
        }

        pub fn html1<'a>() -> &'a str {
            r#"<html><body style="font-size: 16px;"><h2 itemprop='name'>Chicken Spaghetti Recipe</h2><p><img itemprop='image' height='150' src='ee093b2f-ee1b-47ec-9b9e-23488f37be18.png'></img></p><p>★ : 3.0</p><p itemprop='description'><i>Tender chicken and spaghetti baked in a creamy cheddar-Parmesan sauce with a little kick from tomatoes with chilies. Cozy, family-friendly, and perfectly bubbly.</i></p><b>Preparation : </b><span itemprop='prepTime'>20 min</span><br/><b>Cooking : </b><span itemprop='cookTime'>40 min</span><br/><b>Total : </b><span itemprop='totalTime'>1h</span><br/><b>Yield : </b><span itemprop='recipeYield'>6</span><br/><h4>Ingredients</h4  ><div itemprop='recipeIngredient'>8 ounces spaghetti<br/>2 cups cooked shredded chicken<br/>1 (14 ounce) can canned diced tomatoes with chilies (such as Rotel, or petite diced tomatoes, lightly drained)<br/>¼ cup salted butter<br/>1 medium yellow onion (chopped)<br/>1 clove garlic (minced)<br/>½ green bell pepper (chopped)<br/>¼ cup all-purpose flour<br/>1 teaspoon Italian seasoning<br/>1 cup chicken broth<br/>1 cup half and half (or light cream)<br/>½ cup shredded Parmesan cheese<br/>2 cups shredded sharp cheddar cheese (divided)<br/>¼ teaspoon salt (more to taste)<br/>¼ teaspoon black pepper (more to taste)</div><h4>Directions</h4  ><div itemprop='recipeInstructions'>Preheat the oven to 375°F. Grease a 9x13-inch baking dish.<br/><br/>Cook the spaghetti al dente in a large pot of salted water according to package directions. Drain well, do not rinse.<br/><br/>Meanwhile, to make the sauce, in a large skillet, melt the butter over medium heat. Add onion, garlic, and bell pepper and cook until tender. Stir in the flour and Italian seasoning and cook for 1-2 minutes.<br/><br/>Gradually add the broth and half and half, a bit at a time, whisking until smooth after each addition. Bring to a boil and let boil 1 minute or until thick and bubbly.<br/><br/>Remove from heat and whisk in the Parmesan cheese, 1 cup cheddar cheese, and salt and pepper.<br/><br/>Combine spaghetti, chicken, cream sauce, and canned tomatoes. Mix well. Spread in baking dish.<br/><br/>Top with remaining 1 cup of cheddar cheese and bake for 25-30 minutes or until hot and bubbly.</div><h4>Nutrition</h4  ><div itemprop='nutrition'>calories : 518 kcal<br/>carbohydrate : 38 g<br/>protein : 22 g<br/>fat : 30 g<br/>saturatedFat : 18 g<br/>cholesterol : 95 mg<br/>sodium : 606 mg<br/>fiber : 2 g<br/>sugar : 3 g<br/>servingSize : 1 serving</div><br/><a href='https://www.spendwithpennies.com/homemade-chicken-spaghetti/'>https://www.spendwithpennies.com/homemade-chicken-spaghetti/</a><div style='page-break-before:always'></div></body></html>"#
        }

        pub fn html2<'a>() -> &'a str {
            r#"<html><body style="font-size: 16px;"><h2 itemprop='name'>Finnish blueberry pie</h2><p><img itemprop='image' height='150' src='2eaffd07-8262-439d-90bd-9567968a87ef.png'></img></p><p>★ : 4.5</p><p itemprop='description'><i>This delicious dessert has a cookie-like base, juicy blueberries and creamy topping. Easy, delicious, and even better made ahead.</i></p><b>Preparation : </b><span itemprop='prepTime'>15 min</span><br/><b>Cooking : </b><span itemprop='cookTime'>30 min</span><br/><b>Total : </b><span itemprop='totalTime'>45 min</span><br/><b>Yield : </b><span itemprop='recipeYield'>8</span><br/><h4>Ingredients</h4  ><div itemprop='recipeIngredient'>7 tbsp unsalted butter<br/>1 egg<br/>1/3 cup sugar<br/>1 cup all purpose flour<br/>3 tbsp oats (or oat flour)<br/>1 tsp baking powder<br/>1 pinch salt<br/>1/2 tsp cardamom<br/>1 1/2 cups blueberries (wild or regular, fresh or frozen (see notes))<br/>1 cup sour cream<br/>1/4 cup sugar<br/>1 egg<br/>1 tsp vanilla extract ((optional))</div><h4>Directions</h4  ><div itemprop='recipeInstructions'>Preheat the oven to 375F/190C. Lightly butter a 10 - 11 inch (25 - 28cm) diameter round shallow baking dish.<br/><br/>Place the ingredients for the base (butter, egg, sugar, flour, oats, baking powder, salt and cardamom) in a food processor and pulse together to mix them well. The mixture will come together as a fairly soft dough. If you don't have a food processor, you can beat the butter and sugar, then add in the egg followed by the dry ingredients.<br/><br/>Press the base mixture into the baking dish, pressing all the way to the edges and up the sides slightly to give a slight rim (if your dish is shallow, around 1in/3cm deep) you can press right up to the top edge, but lower if the dish is deeper). Smooth the bottom of the dough relatively flat.<br/><br/>Place the dish with the dough in the fridge to chill gently while you prepare the filling. Wash the blueberries and pick over ot remove any stems.<br/><br/>Combine the sour cream, sugar, egg and vanilla extract, if using, and whisk together to be smooth and well combined.<br/><br/>Place the blueberries over the base mixture in the baking dish and spread in an even layer. If you like, you can keep some back to place on top of the cream layer, or instead do as I did here and lift a few up after adding and cream and clean off the top edge so they show through a little.<br/><br/>Pour the sour cream mixture over the blueberries and spread evenly. If you like, you can keep all the blueberries under the cream layer or, as mentioned above, you can lift a few up and clean off the top to let a little of the berry show through. Alternatively, if you reserved a few blueberries, dot them evenly so they are partly under cream, partly above.<br/><br/>Place the dish on a baking sheet to catch any drips, then bake the pie for approximately 30 - 40 minutes until the edges are crisp and the top is dry and seems gently set (it will firm a little more as it cools). If the cream is gently brown at the edges, that's fine, but you want to avoid the base layer edges being too brown.<br/><br/>Allow the pie to cool a good 30 minutes or more before serving.</div><h4>Notes</h4  >The bestest notes! <h4>Nutrition</h4  ><div itemprop='nutrition'>calories : 310 kcal<br/>carbohydrate : 36 g<br/>protein : 5 g<br/>fat : 17 g<br/>saturatedFat : 10 g<br/>transFat : 0.4 g<br/>cholesterol : 85 mg<br/>sodium : 32 mg<br/>fiber : 1 g<br/>sugar : 19 g<br/>unsaturatedFat : 5 g<br/>servingSize : 1 serving</div><h4>Cookware</h4  >1 wok<br/>1 frying pan<br/>Glass<br/><a href='https://www.carolinescooking.com/finnish-blueberry-pie/'>https://www.carolinescooking.com/finnish-blueberry-pie/</a><div style='page-break-before:always'></div></body></html>"#
        }
    }

    mod results {
        use schema_org::field::{
            RecipeDescriptionFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
            RecipeRecipeInstructionsFieldEnum,
        };

        use super::*;

        pub fn recipe1() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Chicken Spaghetti Recipe".into()],
                description: vec![RecipeDescriptionFieldEnum::Text("Tender chicken and spaghetti baked in a creamy cheddar-Parmesan sauce with a little kick from tomatoes with chilies. Cozy, family-friendly, and perfectly bubbly.".into())],
                prep_time: vec![DurationOrText::Text("20 min".into())],
                cook_time: vec![DurationOrText::Text("40 min".into())],
                recipe_yield: vec![RecipeYieldFieldEnum::Number(6.)],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("8 ounces spaghetti".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 cups cooked shredded chicken".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 (14 ounce) can canned diced tomatoes with chilies (such as Rotel, or petite diced tomatoes, lightly drained)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¼ cup salted butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 medium yellow onion (chopped)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 clove garlic (minced)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ green bell pepper (chopped)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¼ cup all-purpose flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 teaspoon Italian seasoning".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup chicken broth".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup half and half (or light cream)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ cup shredded Parmesan cheese".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 cups shredded sharp cheddar cheese (divided)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¼ teaspoon salt (more to taste)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¼ teaspoon black pepper (more to taste)".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 375°F. Grease a 9x13-inch baking dish.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Cook the spaghetti al dente in a large pot of salted water according to package directions. Drain well, do not rinse.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Meanwhile, to make the sauce, in a large skillet, melt the butter over medium heat. Add onion, garlic, and bell pepper and cook until tender. Stir in the flour and Italian seasoning and cook for 1-2 minutes.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Gradually add the broth and half and half, a bit at a time, whisking until smooth after each addition. Bring to a boil and let boil 1 minute or until thick and bubbly.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Remove from heat and whisk in the Parmesan cheese, 1 cup cheddar cheese, and salt and pepper.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Combine spaghetti, chicken, cream sauce, and canned tomatoes. Mix well. Spread in baking dish.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Top with remaining 1 cup of cheddar cheese and bake for 25-30 minutes or until hot and bubbly.".into()),
                ],
                nutrition: vec![
                    NutritionInformation {
                        calories: vec![Energy::new("518 kcal")],
                        carbohydrate_content: vec![Mass::new("38 g")],
                        cholesterol_content: vec![Mass::new("95 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("30 g")],
                        fiber_content: vec![Mass::new("2 g")],
                        protein_content: vec![Mass::new("22 g")],
                        saturated_fat_content: vec![Mass::new("18 g")],
                        serving_size: vec!["1 serving".into()],
                        sodium_content: vec![Mass::new("606 mg")],
                        sugar_content: vec![Mass::new("3 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()                    }
                ],
                aggregate_rating: vec![AggregateRating::new(3.0, 1)],
                recipe_category: vec!["Starter".into()],
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("Delicious".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Potluck".into()),
                ],
                url: vec!["https://www.spendwithpennies.com/homemade-chicken-spaghetti/".into()],
                image: vec![RecipeImageFieldEnum::URL("https://www.spendwithpennies.com/wp-content/uploads/2026/01/cropped-Chicken-Spaghetti-6-SpendWithPennies-480x270.jpg".into())],
                ..Default::default()
            }
        }

        pub fn recipe2() -> Recipe {
            Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                name: vec!["Finnish blueberry pie".into()],
                description: vec![RecipeDescriptionFieldEnum::Text("This delicious dessert has a cookie-like base, juicy blueberries and creamy topping. Easy, delicious, and even better made ahead.".into())],
                prep_time: vec![DurationOrText::Text("15 min".into())],
                cook_time: vec![DurationOrText::Text("30 min".into())],
                recipe_yield: vec![RecipeYieldFieldEnum::Number(8.)],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("7 tbsp unsalted butter".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 egg".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/3 cup sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup all purpose flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 tbsp oats (or oat flour)".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tsp baking powder".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 pinch salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/2 tsp cardamom".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 1/2 cups blueberries (wild or regular, fresh or frozen (see notes))".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup sour cream".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1/4 cup sugar".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 egg".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 tsp vanilla extract ((optional))".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 375F/190C. Lightly butter a 10 - 11 inch (25 - 28cm) diameter round shallow baking dish.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Place the ingredients for the base (butter, egg, sugar, flour, oats, baking powder, salt and cardamom) in a food processor and pulse together to mix them well. The mixture will come together as a fairly soft dough. If you don't have a food processor, you can beat the butter and sugar, then add in the egg followed by the dry ingredients.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Press the base mixture into the baking dish, pressing all the way to the edges and up the sides slightly to give a slight rim (if your dish is shallow, around 1in/3cm deep) you can press right up to the top edge, but lower if the dish is deeper). Smooth the bottom of the dough relatively flat.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Place the dish with the dough in the fridge to chill gently while you prepare the filling. Wash the blueberries and pick over ot remove any stems.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Combine the sour cream, sugar, egg and vanilla extract, if using, and whisk together to be smooth and well combined.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Place the blueberries over the base mixture in the baking dish and spread in an even layer. If you like, you can keep some back to place on top of the cream layer, or instead do as I did here and lift a few up after adding and cream and clean off the top edge so they show through a little.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Pour the sour cream mixture over the blueberries and spread evenly. If you like, you can keep all the blueberries under the cream layer or, as mentioned above, you can lift a few up and clean off the top to let a little of the berry show through. Alternatively, if you reserved a few blueberries, dot them evenly so they are partly under cream, partly above.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Place the dish on a baking sheet to catch any drips, then bake the pie for approximately 30 - 40 minutes until the edges are crisp and the top is dry and seems gently set (it will firm a little more as it cools). If the cream is gently brown at the edges, that's fine, but you want to avoid the base layer edges being too brown.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Allow the pie to cool a good 30 minutes or more before serving.".into()),
                ],
                comment: vec![Comment {
                    text: vec!["The bestest notes!".into()],
                    ..Default::default()
                }],
                nutrition: vec![
                    NutritionInformation {
                        calories: vec![Energy::new("310 kcal")],
                        carbohydrate_content: vec![Mass::new("36 g")],
                        cholesterol_content: vec![Mass::new("85 mg")],
                        context: at_context(),
                        fat_content: vec![Mass::new("17 g")],
                        fiber_content: vec![Mass::new("1 g")],
                        protein_content: vec![Mass::new("5 g")],
                        saturated_fat_content: vec![Mass::new("10 g")],
                        serving_size: vec!["1 serving".into()],
                        sodium_content: vec![Mass::new("32 mg")],
                        sugar_content: vec![Mass::new("19 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        trans_fat_content: vec![Mass::new("0.4 g")],
                        unsaturated_fat_content: vec![Mass::new("5 g")],
                    }
                ],
                aggregate_rating: vec![AggregateRating::new(4.5, 1)],
                recipe_category: vec!["The main course".into()],
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("Delicious".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Dessert".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Meat".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Potluck".into()),
                ],
                url: vec!["https://www.carolinescooking.com/finnish-blueberry-pie/".into()],
                image: vec![RecipeImageFieldEnum::URL("https://www.carolinescooking.com/wp-content/uploads/2023/06/Finnish-blueberry-pie-Mustikkapiirakka-featured-pic-sq-480x270.jpg".into())],
                ..Default::default()
            }
        }
    }
}
