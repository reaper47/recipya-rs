use core::fmt;
use std::borrow::Cow;
use std::io::{BufRead, Read, Seek};
use std::path::Path;

use humantime::parse_duration;
use scraper::{Html, Selector};
use serde::Deserialize;
use tracing::error;
use url::Url;
use zip::ZipArchive;

use schema_org::field::{
    AggregateRatingRatingValueFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
    RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    RecipeVideoFieldEnum,
};
use schema_org::{
    AggregateRating, AtType, Comment, Energy, Mass, NutritionInformation, Recipe, VideoObject,
    at_context,
};
use support::strings::extract_number;

use super::helpers::{extract_archive_contents, update_recipe_image_paths};
use crate::apps::helpers::Parsers;
use crate::helpers::{seconds_to_duration, to_is_based_on, to_yield};
use crate::{Error, Result};

#[derive(Deserialize)]
struct CookbookXML<'a> {
    #[serde(rename = "recipe")]
    recipes: Vec<CookmateRecipe<'a>>,
}

#[allow(dead_code)]
#[derive(Default, Deserialize)]
struct CookmateRecipe<'a> {
    title: Cow<'a, str>,
    preptime: Cow<'a, str>,
    cooktime: Cow<'a, str>,
    totaltime: Cow<'a, str>,
    description: Cow<'a, str>,
    ingredient: List<'a>,
    recipetext: List<'a>,
    url: Cow<'a, str>,
    imagepath: Cow<'a, str>,
    imageurl: Cow<'a, str>,
    quantity: Cow<'a, str>,
    #[serde(default)]
    comments: Option<List<'a>>,
    nutrition: List<'a>,
    lang: Cow<'a, str>,
    rating: i64,
    source: Cow<'a, str>,
    video: Cow<'a, str>,
    #[serde(rename = "category", default)]
    categories: Vec<Cow<'a, str>>,
    #[serde(rename = "tag", default)]
    tags: Vec<Cow<'a, str>>,
}

#[derive(Default, Deserialize)]
struct List<'a> {
    #[serde(default, rename = "li")]
    items: Vec<Cow<'a, str>>,
}

impl List<'_> {
    const fn new() -> Self {
        Self { items: Vec::new() }
    }
    const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl From<List<'_>> for NutritionInformation {
    fn from(v: List) -> Self {
        let extract_to_energy = |prefix: &str| {
            v.items
                .iter()
                .find(|i| i.starts_with(prefix))
                .map(|i| i.strip_prefix(prefix))
                .unwrap_or_default()
                .filter(|s| s != &"0 g")
                .map(|i| vec![Energy::new(i)])
                .unwrap_or_default()
        };

        let extract_to_mass = |prefix: &str| {
            v.items
                .iter()
                .find(|i| i.starts_with(prefix))
                .map(|i| i.strip_prefix(prefix))
                .unwrap_or_default()
                .filter(|s| s != &"0 g")
                .map(|i| vec![Mass::new(i)])
                .unwrap_or_default()
        };

        Self {
            calories: extract_to_energy("calories : "),
            carbohydrate_content: extract_to_mass("carbohydrateContent : "),
            cholesterol_content: extract_to_mass("cholesterolContent : "),
            context: at_context(),
            fat_content: extract_to_mass("fatContent : "),
            fiber_content: extract_to_mass("fiberContent : "),
            protein_content: extract_to_mass("proteinContent : "),
            saturated_fat_content: extract_to_mass("saturatedFatContent : "),
            serving_size: vec![],
            sodium_content: extract_to_mass("sodiumContent : "),
            sugar_content: extract_to_mass("sugarContent : "),
            r#type: AtType::NutritionInformation.to_opt(),
            trans_fat_content: extract_to_mass("saturatedFatContent : "),
            unsaturated_fat_content: extract_to_mass("unsaturatedFatContent : "),
        }
    }
}

impl fmt::Display for List<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.items.join(", "))
    }
}

impl From<CookmateRecipe<'_>> for Recipe {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_precision_loss)]
    fn from(r: CookmateRecipe) -> Self {
        let categories = r.categories.split_first();
        let url = Url::parse(&r.url).ok();

        let comments = r
            .comments
            .unwrap_or_default()
            .items
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let num_comments = comments.len();

        let mut keywords = categories
            .map(|(_, b)| {
                b.iter()
                    .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_lowercase()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        keywords.extend_from_slice(
            &r.tags
                .into_iter()
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_lowercase()))
                .collect::<Vec<_>>(),
        );

        Self {
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: if r.rating > 0 {
                vec![AggregateRating {
                    rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(
                        r.rating as f32,
                    )],
                    ..Default::default()
                }]
            } else {
                vec![]
            },
            comment: comments
                .into_iter()
                .filter_map(|c| {
                    (!c.is_empty()).then_some(Comment {
                        text: vec![c.to_string()],
                        ..Default::default()
                    })
                })
                .collect(),
            comment_count: vec![
                i32::try_from(num_comments)
                    .inspect_err(|err| error!("Failed to convert comment_count to i32: {err}"))
                    .unwrap_or_default(),
            ]
            .into_iter()
            .filter(|c| *c > 0)
            .collect(),
            cook_time: match parse_duration(&r.cooktime) {
                Ok(d) => seconds_to_duration(i32::try_from(d.as_secs()).unwrap_or_default()),
                Err(err) => {
                    error!(
                        "Failed to parse cook time '{}' of an AccuChef recipe: {err}",
                        r.cooktime
                    );
                    vec![]
                }
            },
            description: if r.description.is_empty() {
                vec![]
            } else {
                vec![RecipeDescriptionFieldEnum::Text(r.description.to_string())]
            },
            image: vec![r.imageurl, r.imagepath]
                .into_iter()
                .filter(|image| !image.trim().is_empty())
                .map(|s| RecipeImageFieldEnum::URL(s.to_string()))
                .collect::<Vec<_>>(),
            is_based_on: if r.source.is_empty() {
                to_is_based_on(&r.url)
            } else {
                to_is_based_on(&r.source)
            },
            keywords,
            name: vec![r.title.to_string()],
            nutrition: if r.nutrition.is_empty() {
                vec![]
            } else {
                vec![NutritionInformation::from(r.nutrition)]
            },
            prep_time: match parse_duration(&r.preptime) {
                Ok(d) => seconds_to_duration(i32::try_from(d.as_secs()).unwrap_or_default()),
                Err(err) => {
                    error!(
                        "Failed to parse prep time '{}' of a CookMate XML recipe: {err}",
                        r.preptime
                    );
                    vec![]
                }
            },
            recipe_category: categories
                .map(|(a, _b)| vec![a.to_string()])
                .unwrap_or_default(),
            recipe_ingredient: r
                .ingredient
                .items
                .into_iter()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.to_string()))
                .collect(),
            recipe_instructions: r
                .recipetext
                .items
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.to_string()))
                .collect(),
            recipe_yield: to_yield(extract_number(&r.quantity).unwrap_or_default()),
            url: url.map(|u| vec![u.to_string()]).unwrap_or_default(),
            video: Some(
                vec![r.video]
                    .into_iter()
                    .filter_map(|s| Url::parse(&s).ok())
                    .map(|url| {
                        let s = url.to_string();

                        RecipeVideoFieldEnum::VideoObject(Box::new(VideoObject {
                            r#type: AtType::VideoObject.to_opt(),
                            content_url: vec![s.clone()],
                            embed_url: vec![s],
                            ..Default::default()
                        }))
                    })
                    .collect::<Vec<_>>(),
            )
            .filter(|v| !v.is_empty())
            .unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Parses a `COOKmate` archive.
pub fn parse_backup<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let archive = ZipArchive::new(r)?;
    let (mut recipes, images) = extract_archive_contents(
        archive,
        &Parsers {
            xml: Some(parse_xml),
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

/// Parses a `COOKmate` recipe file in the XML format.
///
/// # Panics
///
/// Panics if the file cannot be read or parsed.
pub fn parse_xml<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + BufRead,
{
    let root: CookbookXML =
        quick_xml::de::from_reader(r).map_err(|err| Error::Parse(err.to_string()))?;

    Ok(root.recipes.into_iter().map(Recipe::from).collect())
}

/// Parses a `COOKmate` recipe file in the HTML format.
///
/// # Panics
///
/// - When the file cannot be read or parsed
/// - When the CSS selectors are invalid.
pub fn parse_html<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;

    let doc = Html::parse_document(&buf);

    let txt = |sel: &str| {
        doc.select(&Selector::parse(sel).unwrap())
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
    };

    let list = |sel: &str| {
        doc.select(&Selector::parse(sel).unwrap())
            .fold(List::new(), |mut acc, el| {
                acc.items.push(Cow::Owned(el.text().collect::<String>()));
                acc
            })
    };

    let notes = doc
        .select(&Selector::parse("span[itemprop='note']").unwrap())
        .fold(List::new(), |mut acc, el| {
            acc.items.extend(
                el.text()
                    .map(str::trim)
                    .map(Cow::Borrowed)
                    .collect::<Vec<_>>(),
            );
            acc
        });

    let recipe = CookmateRecipe {
        title: Cow::Borrowed(&txt("h1[itemprop='name']")),
        preptime: Cow::Borrowed(&txt("span[itemprop='prepTime']")),
        cooktime: Cow::Borrowed(&txt("span[itemprop='cookTime']")),
        totaltime: Cow::Borrowed(&txt("span[itemprop='totalTime']")),
        description: Cow::Borrowed(&txt("span[itemprop='description']")),
        ingredient: list("li[itemprop='recipeIngredient'] p"),
        recipetext: list("li[itemprop='recipeInstructions'] p"),
        url: Cow::Borrowed(&txt("a[itemprop='note']")),
        imagepath: doc
            .select(&Selector::parse("img[itemprop='image']").unwrap())
            .next()
            .map(|el| {
                Cow::Borrowed(
                    el.attr("src")
                        .unwrap_or_default()
                        .trim_start_matches("img/"),
                )
            })
            .unwrap_or_default(),
        quantity: Cow::Borrowed(&txt("span[itemprop='recipeYield']")),
        nutrition: doc
            .select(&Selector::parse("span[itemprop='nutrition']").unwrap())
            .fold(List::new(), |mut acc, el| {
                acc.items.extend(
                    el.text()
                        .map(str::trim)
                        .map(Cow::Borrowed)
                        .collect::<Vec<_>>(),
                );
                acc
            }),
        rating: doc
            .select(&Selector::parse("img.rating").unwrap())
            .next()
            .map(|el| el.attr("src").unwrap_or_default())
            .map(|s| {
                s.trim_start_matches("./rating_")
                    .trim_end_matches(".svg")
                    .parse::<i64>()
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        comments: if notes.is_empty() { None } else { Some(notes) },
        source: Cow::Borrowed(&txt("span[itemprop='author']")),
        video: Cow::Borrowed(&txt("a[itemprop='video']")),
        categories: doc
            .select(&Selector::parse("span[itemprop='recipeCategory']").unwrap())
            .map(|el| el.text().collect::<String>())
            .map(Cow::Owned)
            .collect::<Vec<_>>(),
        tags: doc
            .select(&Selector::parse("span[itemprop='recipeTag']").unwrap())
            .map(|el| el.text().collect::<String>())
            .map(Cow::Owned)
            .collect::<Vec<_>>(),
        ..Default::default()
    };

    Ok(if recipe.title.is_empty() {
        vec![]
    } else {
        vec![recipe.into()]
    })
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
        fn test_cm_xml_ok() -> Result<()> {
            let file = files::xml_file();
            let buf = Cursor::new(file);

            let got = parse_xml(buf)?;

            pretty_assertions::assert_eq!(got, results::xml_recipes());
            Ok(())
        }

        #[test]
        fn test_cm_xml2_ok() -> Result<()> {
            let file = files::xml_file2();
            let buf = Cursor::new(file);

            let got = parse_xml(buf)?;

            pretty_assertions::assert_eq!(got, results::xml2_recipes());
            Ok(())
        }

        #[test]
        fn test_cm_backup_ok() -> Result<()> {
            let buf = files::backup();

            let mut got = parse_backup(buf)?;

            let want = results::xml_recipes();
            assert!(match got[4].image[0].clone() {
                schema_org::field::FieldEnum22::ImageObject(_) => false,
                schema_org::field::FieldEnum22::URL(u) =>
                    u.starts_with("/tmp") || u.contains(r"\Temp\"),
            });
            got[4].image = want[4].image.clone();
            pretty_assertions::assert_eq!(got[..5], want);
            Ok(())
        }

        #[test]
        fn test_cm_zip_ok() -> Result<()> {
            let buf = open_test_file("integrations/cookmate.zip");

            let got = parse_backup(buf)?;

            pretty_assertions::assert_eq!(got.len(), 3);
            let want = results::xml2_recipes();
            let want = want
                .into_iter()
                .zip(got.clone())
                .map(|(mut a, b)| {
                    a.image = b.image;
                    a
                })
                .collect::<Vec<_>>();
            pretty_assertions::assert_eq!(got, want);
            Ok(())
        }
    }

    mod files {
        use std::io::Cursor;
        use test_fixtures::open_test_file;

        #[allow(clippy::too_many_lines)]
        pub fn xml_file<'a>() -> &'a str {
            r#"<?xml version="1.0" encoding="utf-8"?><cookbook version="71"><recipe><title>Asparagus Soup (Zuppa Di Asparagi)</title><preptime></preptime><cooktime></cooktime><totaltime></totaltime><description></description><ingredient><li>2 tb Extra-virgin olive oil 1 qt Chicken broth</li><li>2 Cloves garlic, minced 4 Eggs</li><li>2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or</li><li>-and cut (1 inch pieces) -pecorino cheese</li><li>Salt and pepper 6 sl Italian bread, toasted</li></ingredient><recipetext><li>Heat the oil and garlic in a soup pot until the garlic is golden. Add the</li><li>asparagus and cook until they begin to color. Season with salt and pepper.</li><li>Add the broth and bring to a boil; reduce the heat and simmer for 15</li><li>minutes, or until the asparagus is tender.</li><li></li><li>Beat the eggs and cheese together. When the asparagus is tender, reduce</li><li>the heat so the soup is no longer simmering. Very slowly ladle some of the</li><li>hot soup into the beaten eggs, stirring continuously. After adding about 2</li><li>cups of the hot soup to the eggs, reverse the process and gradually stir</li><li>the eggs mixture into the soup pot. The soup must not boil or the eggs</li><li>will scramble. Heat until thickened.</li><li></li><li>Put one slice of toasted bread into each soup dish. Ladle the hot soup on</li><li>top and pass additional grated cheese.</li><li></li><li>Serves 6.</li><li></li><li>NOTE: To trim asparagus, hold the tip in one hand and the base of the</li><li>stalk in the other. Bend gently. The asparagus will snap, leaving the</li><li>tender part with the tip.</li><li></li><li>[ &quot;We Called It Macaroni&quot;; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]</li><li></li><li>Posted by Fred Peters.</li></recipetext><url>MMF</url><imagepath></imagepath><imageurl></imageurl><quantity>6 servings</quantity><comments></comments><nutrition></nutrition><lang></lang><rating>0</rating><source></source><video></video><category>Italian</category><category>Soups/stews</category><category>Vegetables</category></recipe><recipe><title>Aubergine and Sesame Pate</title><preptime></preptime><cooktime></cooktime><totaltime></totaltime><description></description><ingredient><li>1/2 md Aubergine 1/4 Juice of 1 lemon</li><li>1 Crushed garlic cloves 1 tb Olive oil</li><li>1 1/2 tb Tahini Seasoning</li><li>Toasted Sesame seeds Flatleaf Parsley</li><li>Cayenne Pepper</li><li>25-30 minutes until tender. Cool slightly , then peel and</li></ingredient><recipetext><li>1&gt; Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for</li><li>puree the flesh in a blender or processor.</li><li></li><li>Add the garlic, tahini and lemon juice and process until mixed.</li><li>With the motor running, drizzle in the oil to make a smooth paste.</li><li>Season to taste.</li><li></li><li>Transfer to a serving dish, garnish and serve cold with pitta bread.</li></recipetext><url>MMF</url><imagepath></imagepath><imageurl></imageurl><quantity>2 servings</quantity><comments></comments><nutrition></nutrition><lang></lang><rating>0</rating><source></source><video></video><category>Vegetarian</category><category>Appetizers</category><category>Greek</category></recipe><recipe><title>Aubergines a la Toulousaine (Eggplant A La Toulouse)</title><preptime></preptime><cooktime></cooktime><totaltime></totaltime><description></description><ingredient><li>1 md Eggplant 2 tb Snipped parsley</li><li>1/4 c Salad oil 1 cl Galic, minced</li><li>3 lg Tomatoes, peeled 1 tb Salad oil</li><li>2 c Fresh bread cubes 1/4 c Grated Parmesan cheese</li></ingredient><recipetext><li>Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper</li><li>towels; sprinkle each generously with salt. let stand for 30 minutes; then</li><li>blot dry with paper towels. Start heating oven to 400 deg. F. Saute</li><li>eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut</li><li>tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2</li><li>inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in</li><li>all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.</li><li>Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and</li><li>cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread</li><li>cubes are golden and eggplant is tender.</li><li></li><li>SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book</li><li>Publishers Chicago 1, Illinois 1958</li></recipetext><url>MMF</url><imagepath></imagepath><imageurl></imageurl><quantity>4 servings</quantity><comments></comments><nutrition></nutrition><lang></lang><rating>0</rating><source></source><video></video><category>Vegetables</category><category>Casseroles</category><category>French</category></recipe><recipe><title>August Goerg's Grilled Steak (Spiessbraten August Goerg)</title><preptime></preptime><cooktime></cooktime><totaltime></totaltime><description></description><ingredient><li>1 Shallot or small onion cut 1 pn Mace</li><li>-into small pieces 1 lg Steak (just over 1 lb), at</li><li>Freshly ground black pepper -least 1 1/4 inches</li></ingredient><recipetext><li>((Note: Per Horst Scharfenberg, this recipe originated in the town of</li><li>Idar-Oberstein in the 19 th century, when gemstone prospectors returning</li><li>from South America created their own version of gaucho-grilled steaks. The</li><li>dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))</li><li></li><li>Per person: thick, trimmed</li><li></li><li>Mix together the shallot or onion with the pepper and mace. Insert a few</li><li>shallot pieces into the steak using the point of a small knife. Coat the</li><li>steak with the shallot mixture, pressing it in so it will adhere.</li><li></li><li>Remove the loose shallot pieces and grill the steak (over a fire of oak</li><li>logs, says August Goerg, from which the bark has been removed).* Take the</li><li>steaks off the grill while they are still pink inside. Sprinkle them with</li><li>salt.</li><li></li><li>*Note: A special grill is used, suspended with 3 chains from an iron</li><li>tripod and constantly swinging through the flames.</li><li></li><li>From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &amp;</li><li>Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking</li><li>Echo, 8/92</li></recipetext><url>MMF</url><imagepath></imagepath><imageurl></imageurl><quantity>6 servings</quantity><comments></comments><nutrition></nutrition><lang></lang><rating>0</rating><source></source><video></video><category>Beef</category><category>German</category></recipe><recipe><title>Aunt Julia's Paella</title><preptime></preptime><cooktime></cooktime><totaltime></totaltime><description></description><ingredient><li>1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento</li><li>-and legs) 2 ts Capers, with juice</li><li>Salt and pepper to thaste 4 oz Jar pimento-stiffed green</li><li>1 lb Lean pork, cut into 1-inch -olives</li><li>-cubes 1/2 lb Calamari (squid), cleaned</li><li>1 md Onion, minced -and sliced</li><li>2 Toes garlic, minced 5 c Water</li><li>Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes</li><li>-strips: 1 ts Saffron threads</li><li>1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,</li><li>1 lg Carrot -uncooked</li><li>1 Stalk celery 3 Hard boiled eggs, sliced</li><li>1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)</li><li>1 1/2 lb Peeled shrimp Oil for frying</li></ingredient><recipetext><li>{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }</li><li></li><li>In a large electric skillet or paella pan, brown the chicken pieces (that</li><li>have been seasoned with salt and pepper) in a little oil. Remove from the</li><li>pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.</li><li>Remove from the pan. To the pan drippings (add a little more oil if</li><li>necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry</li><li>for 2 minutes.</li><li></li><li>Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.</li><li>Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the</li><li>bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.</li><li></li><li>Gently stir the rice into the skillet mixture. Slowly pour in enough of</li><li>the bouillon mixture to cover the rice and chicken pieces. Cover and cook</li><li>over low heat for about 20 minutes. Uncover and decoaratively arrange the</li><li>egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary</li><li>to keep the rice moist.</li><li></li><li>Cover and steam for another 10 minutes until the shrimp are cooked and the</li><li>rice is tender. (Paella should be moist but not wet!) Place the pan on a</li><li>hot pad on the serving table and let everyone help themselves.</li><li></li><li>Serve with a mixed green salad, red ripe tomatoes and some French bread.</li><li>Also mix up a pitcher of Sangria and enjoy!</li><li></li><li>Serves: 12.</li><li></li><li>[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]</li><li></li><li>Posted by Fred Peters</li></recipetext><url>MMF</url><imagepath>/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/Aunt_Julias_Paella.jpg</imagepath><imageurl></imageurl><quantity>6 servings</quantity><comments></comments><nutrition></nutrition><lang></lang><rating>0</rating><source></source><video></video><category>Pork/ham</category><category>Poultry</category><category>Fish/sea</category><category>Spanish</category></recipe></cookbook>"#
        }

        #[allow(clippy::too_many_lines)]
        pub fn xml_file2<'a>() -> &'a str {
            r#"<?xml version="1.0" encoding="utf-8"?><cookbook version="71"><recipe><title>Simple White Cake</title><preptime>10m</preptime><cooktime>30m</cooktime><totaltime>40m</totaltime><description>This easy cake recipe requires just 7 ingredients and tastes like you spent hours making it, even though it's out of the oven in under an hour.</description><ingredient><li>1 cup white sugar</li><li>0.5 cup unsalted butter</li><li>2 large eggs</li><li>2 teaspoons vanilla extract</li><li>1.5 cups all-purpose flour</li><li>1.75 teaspoons baking powder</li><li>1/4 teaspoon table salt</li><li>0.5 cup milk</li></ingredient><recipetext><li>Gather all ingredients. Preheat the oven to 350 degrees F (175 degrees C). Grease and flour a 9-inch square cake pan.</li><li></li><li>Beat sugar and butter together in a mixing bowl with an electric mixer until lighter in color and fluffy, 3 to 4 minutes. Add eggs, one at a time, beating briefly after each addition, 30 seconds total. Mix in vanilla, about 15 seconds.</li><li></li><li>Whisk flour, baking powder, and salt in a separate bowl. With mixer on low speed, add flour mixture to butter mixture in 3 batches, alternating with milk, beginning and ending with flour. Mix just until combined stopping to scrape down sides if needed, about 2 minutes.</li><li></li><li>Spread cake batter into the prepared pan.</li><li></li><li>Bake cake in the preheated oven until a toothpick inserted into the center comes out clean, about 30 minutes.</li><li></li><li>Remove cake from the oven and let cool in pan on a wire rack for 10 minutes. Invert cake onto wire rack; remove pan and let cake cool completely before frosting. Enjoy!</li></recipetext><url>https://www.allrecipes.com/recipe/17481/simple-white-cake/</url><imagepath>/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/Simple_White_Cake.jpg</imagepath><imageurl>https://www.allrecipes.com/thmb/rceSb4HUcHI64nQj_8o6jnvAN-0=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/17481-simple-white-cake-DDMFS-4x3-89bf9ff32d0c40179a3d752d4d25f22a.jpg</imageurl><quantity>12</quantity><comments></comments><nutrition><li>calories : 209 kcal</li><li>carbohydrateContent : 29 g</li><li>cholesterolContent : 52 mg</li><li>fatContent : 9 g</li><li>fiberContent : 0 g</li><li>proteinContent : 3 g</li><li>saturatedFatContent : 5 g</li><li>sodiumContent : 142 mg</li><li>sugarContent : 17 g</li><li>unsaturatedFatContent : 0 g</li></nutrition><lang></lang><rating>0</rating><source>SCOTTOSMAN</source><video>https://content.jwplatform.com/videos/58S3UXco-K3AjnAEN.mp4</video></recipe><recipe><title>The Best Chicken Fried Steak</title><preptime>20m</preptime><cooktime>20m</cooktime><totaltime>40m</totaltime><description>This chicken fried steak recipe is a Southern favorite. It features crispy, breaded cube steaks drenched in a creamy gravy. It'll be a hit with everyone.</description><ingredient><li>4 (1/2 pound) beef cube steaks</li><li>2.25 cups all-purpose flour, divided</li><li>2 teaspoons baking powder</li><li>1 teaspoon baking soda</li><li>1 teaspoon black pepper</li><li>0.75 teaspoon salt</li><li>1.5 cups buttermilk</li><li>1 large egg</li><li>1 tablespoon hot pepper sauce (e.g. Tabasco™)</li><li>2 cloves garlic, minced</li><li>3 cups vegetable shortening for frying</li><li>4 cups milk</li><li>kosher salt and ground black pepper to taste</li></ingredient><recipetext><li>Place steaks between two sheets of heavy plastic on a solid, level surface; firmly pound with a meat mallet to a ¼-inch thickness.</li><li>Place 2 cups flour in a shallow bowl.</li><li>Combine baking powder, baking soda, 1 teaspoon pepper, and ¾ teaspoon salt in a separate shallow bowl; stir in buttermilk, egg, Tabasco, and garlic to combine.</li><li>Heat shortening in a deep cast-iron skillet to 325 degrees F (165 degrees C). Place a wire rack over a sheet of parchment paper.</li><li>Meanwhile, dredge 1 steak in flour to coat; shake off excess. Dip into buttermilk batter; lift up so excess batter drips back into the bowl. Dredge in flour again to coat both sides completely. Place breaded steak on the prepared wire rack. Repeat with remaining steaks.</li><li>Fry steaks, in batches if necessary, until evenly golden brown, 3 to 5 minutes per side. Transfer steaks to a paper towel-lined plate to drain. Cover with foil to keep warm.</li><li>Drain fat from the skillet, reserving ¼ cup and as much solid remnants as possible.</li><li>Place skillet over medium-low heat. Add reserved ¼ cup oil; whisk in remaining ¼ cup flour. Scrape the brown bits of food off the bottom of the skillet with a spatula.</li><li>Stir in milk; increase heat to medium and bring gravy to a simmer. Cook, stirring often, until thick, 6 to 7 minutes. Season gravy with salt and black pepper.</li><li>Transfer steaks to a platter; pour gravy over top.</li></recipetext><url>https://www.allrecipes.com/recipe/150306/the-best-chicken-fried-steak/</url><imagepath>/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/The_Best_Chicken_Fried_Steak.jpg</imagepath><imageurl>https://www.allrecipes.com/thmb/3hN7uCQpPhYlm-K1QlxPB-Z5ato=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/150306-the-best-chicken-fried-steak-DDMFS-4x3-22a44d8658c24fd6880232cc2cbf9c65.jpg</imageurl><quantity>4</quantity><comments><li>This is my favorite ourite recipe</li><li></li></comments><nutrition><li>calories : 832 kcal</li><li>carbohydrateContent : 71 g</li><li>cholesterolContent : 206 mg</li><li>fatContent : 29 g</li><li>fiberContent : 2 g</li><li>proteinContent : 68 g</li><li>saturatedFatContent : 12 g</li><li>sodiumContent : 1273 mg</li><li>unsaturatedFatContent : 0 g</li></nutrition><lang></lang><rating>0</rating><source>norah</source><video>https://cdn.jwplayer.com/videos/S8RFFmAS-K3AjnAEN.mp4</video><category>Starter</category><category>Main course</category><tag>cheeses</tag><tag>baloney</tag></recipe><recipe><title>To Die For Fettuccine Alfredo</title><preptime>15m</preptime><cooktime>15m</cooktime><totaltime>30m</totaltime><description>Fettuccine Alfredo with Romano and Parmesan cheeses, cream, and butter. This delicious sauce for fettuccine pasta is rich in taste yet simple to make.</description><ingredient><li>24 ounces dry fettuccine pasta</li><li>1 cup butter</li><li>0.75 pint heavy cream</li><li>1 dash garlic salt</li><li>salt and pepper to taste</li><li>0.75 cup grated Romano cheese</li><li>0.5 cup grated Parmesan cheese</li></ingredient><recipetext><li>Gather all ingredients.</li><li>Fill a large pot with lightly salted water and bring to a rolling boil. Cook fettuccine at a boil until tender yet firm to the bite, about 8 minutes. Drain.</li><li>Heat butter and cream in a large saucepan over low heat until butter melted; add garlic salt, salt, and black pepper.</li><li>Increase the heat to medium; stir in Romano and Parmesan cheeses until melted and sauce has thickened.</li><li>Add cooked pasta to sauce; toss until thoroughly coated. Serve immediately.</li></recipetext><url>https://www.allrecipes.com/recipe/23431/to-die-for-fettuccine-alfredo/</url><imagepath>/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/To_Die_For_Fettuccine_Alfredo.jpg</imagepath><imageurl>https://www.allrecipes.com/thmb/ey5aal-JHk7PoNP92is_VCambmA=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/AR-23431-to-die-for-fettuccine-alfredo-DDMFS-beauty-3x4-b64d36c7ff314cb39774e261c5b18352.jpg</imageurl><quantity>6</quantity><comments></comments><nutrition><li>calories : 964 kcal</li><li>carbohydrateContent : 84 g</li><li>cholesterolContent : 184 mg</li><li>fatContent : 61 g</li><li>fiberContent : 4 g</li><li>proteinContent : 24 g</li><li>saturatedFatContent : 37 g</li><li>sodiumContent : 582 mg</li><li>sugarContent : 4 g</li><li>unsaturatedFatContent : 0 g</li></nutrition><lang></lang><rating>5</rating><source>ERINMARIE</source><video>https://content.jwplatform.com/videos/snjcnaUM-K3AjnAEN.mp4</video><category>Dessert</category><tag>baloney</tag></recipe></cookbook>"#
        }

        pub fn backup() -> Cursor<Vec<u8>> {
            open_test_file("integrations/cookmate1.mcb")
        }
    }

    mod results {
        use schema_org::{
            AtType, DurationOrText, Energy, Mass, NutritionInformation,
            field::RecipeIsBasedOnFieldEnum,
        };

        use super::*;

        use crate::helpers::to_yield;

        #[allow(clippy::too_many_lines)]
        pub fn xml_recipes() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("MMF")],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("soups/stews".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("vegetables".into()),
                    ],
                    name: vec!["Asparagus Soup (Zuppa Di Asparagi)".into()],
                    recipe_category: vec!["Italian".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 tb Extra-virgin olive oil 1 qt Chicken broth".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 Cloves garlic, minced 4 Eggs".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or"
                                .into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "-and cut (1 inch pieces) -pecorino cheese".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Salt and pepper 6 sl Italian bread, toasted".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Heat the oil and garlic in a soup pot until the garlic is golden. Add the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "asparagus and cook until they begin to color. Season with salt and pepper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the broth and bring to a boil; reduce the heat and simmer for 15".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("minutes, or until the asparagus is tender.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Beat the eggs and cheese together. When the asparagus is tender, reduce".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "the heat so the soup is no longer simmering. Very slowly ladle some of the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "hot soup into the beaten eggs, stirring continuously. After adding about 2".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "cups of the hot soup to the eggs, reverse the process and gradually stir".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "the eggs mixture into the soup pot. The soup must not boil or the eggs".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("will scramble. Heat until thickened.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Put one slice of toasted bread into each soup dish. Ladle the hot soup on".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("top and pass additional grated cheese.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Serves 6.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "NOTE: To trim asparagus, hold the tip in one hand and the base of the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "stalk in the other. Bend gently. The asparagus will snap, leaving the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("tender part with the tip.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Posted by Fred Peters.".into()),
                    ],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("MMF")],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("appetizers".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("greek".into()),
                    ],
                    name: vec!["Aubergine and Sesame Pate".into()],
                    recipe_category: vec!["Vegetarian".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1/2 md Aubergine 1/4 Juice of 1 lemon".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 Crushed garlic cloves 1 tb Olive oil".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 1/2 tb Tahini Seasoning".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Toasted Sesame seeds Flatleaf Parsley".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text("Cayenne Pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "25-30 minutes until tender. Cool slightly , then peel and".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("puree the flesh in a blender or processor.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the garlic, tahini and lemon juice and process until mixed.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "With the motor running, drizzle in the oil to make a smooth paste.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Season to taste.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Transfer to a serving dish, garnish and serve cold with pitta bread.".into(),
                        ),
                    ],
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("MMF")],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("casseroles".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("french".into()),
                    ],
                    name: vec!["Aubergines a la Toulousaine (Eggplant A La Toulouse)".into()],
                    recipe_category: vec!["Vegetables".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 md Eggplant 2 tb Snipped parsley".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1/4 c Salad oil 1 cl Galic, minced".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "3 lg Tomatoes, peeled 1 tb Salad oil".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 c Fresh bread cubes 1/4 c Grated Parmesan cheese".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "towels; sprinkle each generously with salt. let stand for 30 minutes; then".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "blot dry with paper towels. Start heating oven to 400 deg. F. Saute".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("cubes are golden and eggplant is tender.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Publishers Chicago 1, Illinois 1958".into()),
                    ],
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("MMF")],
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("german".into())],
                    name: vec!["August Goerg's Grilled Steak (Spiessbraten August Goerg)".into()],
                    recipe_category: vec!["Beef".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 Shallot or small onion cut 1 pn Mace".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "-into small pieces 1 lg Steak (just over 1 lb), at".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Freshly ground black pepper -least 1 1/4 inches".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "((Note: Per Horst Scharfenberg, this recipe originated in the town of".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Idar-Oberstein in the 19 th century, when gemstone prospectors returning".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "from South America created their own version of gaucho-grilled steaks. The".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Per person: thick, trimmed".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mix together the shallot or onion with the pepper and mace. Insert a few".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "shallot pieces into the steak using the point of a small knife. Coat the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "steak with the shallot mixture, pressing it in so it will adhere.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Remove the loose shallot pieces and grill the steak (over a fire of oak".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "logs, says August Goerg, from which the bark has been removed).* Take the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "steaks off the grill while they are still pink inside. Sprinkle them with".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("salt.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "*Note: A special grill is used, suspended with 3 chains from an iron".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("tripod and constantly swinging through the flames.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Echo, 8/92".into()),
                    ],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("MMF")],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("poultry".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("fish/sea".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("spanish".into()),
                    ],
                    image: vec![
                        RecipeImageFieldEnum::URL(
                            "/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/Aunt_Julias_Paella.jpg".into(),
                        ),
                    ],
                    name: vec!["Aunt Julia's Paella".into()],
                    recipe_category: vec!["Pork/ham".into()],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "-and legs) 2 ts Capers, with juice".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Salt and pepper to thaste 4 oz Jar pimento-stiffed green".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 lb Lean pork, cut into 1-inch -olives".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "-cubes 1/2 lb Calamari (squid), cleaned".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 md Onion, minced -and sliced".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "2 Toes garlic, minced 5 c Water".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "-strips: 1 ts Saffron threads".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text("1 lg Carrot -uncooked".into()),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 Stalk celery 3 Hard boiled eggs, sliced".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 1/2 lb Peeled shrimp Oil for frying".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "In a large electric skillet or paella pan, brown the chicken pieces (that".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "have been seasoned with salt and pepper) in a little oil. Remove from the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Remove from the pan. To the pan drippings (add a little more oil if".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("for 2 minutes.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Gently stir the rice into the skillet mixture. Slowly pour in enough of".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "the bouillon mixture to cover the rice and chicken pieces. Cover and cook".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "over low heat for about 20 minutes. Uncover and decoaratively arrange the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("to keep the rice moist.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cover and steam for another 10 minutes until the shrimp are cooked and the".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "rice is tender. (Paella should be moist but not wet!) Place the pan on a".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "hot pad on the serving table and let everyone help themselves.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Serve with a mixed green salad, red ripe tomatoes and some French bread.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Also mix up a pitcher of Sangria and enjoy!".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Serves: 12.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text("Posted by Fred Peters".into()),
                    ],
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
            ]
        }

        #[allow(clippy::too_many_lines)]
        pub fn xml2_recipes() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    recipe_yield: to_yield(12),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("1 cup white sugar".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.5 cup unsalted butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 large eggs".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 teaspoons vanilla extract".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1.5 cups all-purpose flour".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1.75 teaspoons baking powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1/4 teaspoon table salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.5 cup milk".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Gather all ingredients. Preheat the oven to 350 degrees F (175 degrees C). Grease and flour a 9-inch square cake pan.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Beat sugar and butter together in a mixing bowl with an electric mixer until lighter in color and fluffy, 3 to 4 minutes. Add eggs, one at a time, beating briefly after each addition, 30 seconds total. Mix in vanilla, about 15 seconds.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Whisk flour, baking powder, and salt in a separate bowl. With mixer on low speed, add flour mixture to butter mixture in 3 batches, alternating with milk, beginning and ending with flour. Mix just until combined stopping to scrape down sides if needed, about 2 minutes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Spread cake batter into the prepared pan.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Bake cake in the preheated oven until a toothpick inserted into the center comes out clean, about 30 minutes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Remove cake from the oven and let cool in pan on a wire rack for 10 minutes. Invert cake onto wire rack; remove pan and let cake cool completely before frosting. Enjoy!".into(),
                        ),
                    ],
                    prep_time: vec![DurationOrText::Text("PT600S".into())],
                    cook_time: vec![DurationOrText::Text("PT1800S".into())],
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
                        trans_fat_content: vec![Mass::new("5 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("SCOTTOSMAN")],
                    video: vec![
                        RecipeVideoFieldEnum::VideoObject(
                            VideoObject {
                                r#type: AtType::VideoObject.to_opt(),
                                content_url: vec![
                                    "https://content.jwplatform.com/videos/58S3UXco-K3AjnAEN.mp4".into(),
                                ],
                                embed_url: vec![
                                    "https://content.jwplatform.com/videos/58S3UXco-K3AjnAEN.mp4".into(),
                                ],
                                ..Default::default()
                            }.into(),
                        ),
                    ],
                    image: vec![
                        RecipeImageFieldEnum::URL(
                            "https://www.allrecipes.com/thmb/rceSb4HUcHI64nQj_8o6jnvAN-0=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/17481-simple-white-cake-DDMFS-4x3-89bf9ff32d0c40179a3d752d4d25f22a.jpg".into(),
                        ),
                        RecipeImageFieldEnum::URL(
                            "/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/Simple_White_Cake.jpg".into(),
                        ),
                    ],
                    description: vec![
                        RecipeDescriptionFieldEnum::Text(
                            "This easy cake recipe requires just 7 ingredients and tastes like you spent hours making it, even though it's out of the oven in under an hour.".into(),
                        ),
                    ],
                    url: vec![
                        "https://www.allrecipes.com/recipe/17481/simple-white-cake/".into(),
                    ],
                    name: vec![
                        "Simple White Cake".into(),
                    ],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    recipe_yield: to_yield(4),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("4 (1/2 pound) beef cube steaks".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2.25 cups all-purpose flour, divided".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 teaspoons baking powder".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 teaspoon baking soda".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 teaspoon black pepper".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.75 teaspoon salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1.5 cups buttermilk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 large egg".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 tablespoon hot pepper sauce (e.g. Tabasco™)".into()),
                        RecipeRecipeIngredientFieldEnum::Text("2 cloves garlic, minced".into()),
                        RecipeRecipeIngredientFieldEnum::Text("3 cups vegetable shortening for frying".into()),
                        RecipeRecipeIngredientFieldEnum::Text("4 cups milk".into()),
                        RecipeRecipeIngredientFieldEnum::Text("kosher salt and ground black pepper to taste".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place steaks between two sheets of heavy plastic on a solid, level surface; firmly pound with a meat mallet to a ¼-inch thickness.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place 2 cups flour in a shallow bowl.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Combine baking powder, baking soda, 1 teaspoon pepper, and ¾ teaspoon salt in a separate shallow bowl; stir in buttermilk, egg, Tabasco, and garlic to combine.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Heat shortening in a deep cast-iron skillet to 325 degrees F (165 degrees C). Place a wire rack over a sheet of parchment paper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Meanwhile, dredge 1 steak in flour to coat; shake off excess. Dip into buttermilk batter; lift up so excess batter drips back into the bowl. Dredge in flour again to coat both sides completely. Place breaded steak on the prepared wire rack. Repeat with remaining steaks.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Fry steaks, in batches if necessary, until evenly golden brown, 3 to 5 minutes per side. Transfer steaks to a paper towel-lined plate to drain. Cover with foil to keep warm.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Drain fat from the skillet, reserving ¼ cup and as much solid remnants as possible.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place skillet over medium-low heat. Add reserved ¼ cup oil; whisk in remaining ¼ cup flour. Scrape the brown bits of food off the bottom of the skillet with a spatula.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Stir in milk; increase heat to medium and bring gravy to a simmer. Cook, stirring often, until thick, 6 to 7 minutes. Season gravy with salt and black pepper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Transfer steaks to a platter; pour gravy over top.".into(),
                        ),
                    ],
                    recipe_category: vec!["Starter".into()],
                    prep_time: vec![DurationOrText::Text("PT1200S".into())],
                    cook_time: vec![DurationOrText::Text("PT1200S".into())],
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
                        trans_fat_content: vec![Mass::new("12 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    comment: vec![
                        Comment {
                            text: vec!["This is my favorite ourite recipe".into()],
                            ..Default::default()
                        },
                    ],
                    comment_count: vec![1],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("main course".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("cheeses".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("baloney".into()),
                    ],
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("norah")],
                    video: vec![
                        RecipeVideoFieldEnum::VideoObject(
                            VideoObject {
                                r#type: AtType::VideoObject.to_opt(),
                                content_url: vec![
                                    "https://cdn.jwplayer.com/videos/S8RFFmAS-K3AjnAEN.mp4".into(),
                                ],
                                embed_url: vec![
                                    "https://cdn.jwplayer.com/videos/S8RFFmAS-K3AjnAEN.mp4".into(),
                                ],
                                ..Default::default()
                            }.into(),
                        ),
                    ],
                    image: vec![
                        RecipeImageFieldEnum::URL(
                            "https://www.allrecipes.com/thmb/3hN7uCQpPhYlm-K1QlxPB-Z5ato=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/150306-the-best-chicken-fried-steak-DDMFS-4x3-22a44d8658c24fd6880232cc2cbf9c65.jpg".into(),
                        ),
                        RecipeImageFieldEnum::URL(
                            "/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/The_Best_Chicken_Fried_Steak.jpg".into(),
                        ),
                    ],
                    description: vec![
                        RecipeDescriptionFieldEnum::Text(
                            "This chicken fried steak recipe is a Southern favorite. It features crispy, breaded cube steaks drenched in a creamy gravy. It'll be a hit with everyone.".into(),
                        ),
                    ],
                    url: vec![
                        "https://www.allrecipes.com/recipe/150306/the-best-chicken-fried-steak/".into(),
                    ],
                    name: vec!["The Best Chicken Fried Steak".into()],
                    ..Default::default()
                },
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    prep_time: vec![DurationOrText::Text("PT900S".into())],
                    cook_time: vec![DurationOrText::Text("PT900S".into())],
                    recipe_yield: to_yield(6),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("24 ounces dry fettuccine pasta".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 cup butter".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.75 pint heavy cream".into()),
                        RecipeRecipeIngredientFieldEnum::Text("1 dash garlic salt".into()),
                        RecipeRecipeIngredientFieldEnum::Text("salt and pepper to taste".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.75 cup grated Romano cheese".into()),
                        RecipeRecipeIngredientFieldEnum::Text("0.5 cup grated Parmesan cheese".into()),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Gather all ingredients.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Fill a large pot with lightly salted water and bring to a rolling boil. Cook fettuccine at a boil until tender yet firm to the bite, about 8 minutes. Drain.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Heat butter and cream in a large saucepan over low heat until butter melted; add garlic salt, salt, and black pepper.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Increase the heat to medium; stir in Romano and Parmesan cheeses until melted and sauce has thickened.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Add cooked pasta to sauce; toss until thoroughly coated. Serve immediately.".into(),
                        ),
                    ],
                    recipe_category: vec![
                        "Dessert".into(),
                    ],
                    keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("baloney".into())],
                    is_based_on: vec![RecipeIsBasedOnFieldEnum::new_creative_work_text("ERINMARIE")],
                    aggregate_rating: vec![
                        AggregateRating {
                            rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(5.0)],
                            ..Default::default()
                        },
                    ],
                    video: vec![
                        RecipeVideoFieldEnum::VideoObject(
                            VideoObject {
                                r#type: AtType::VideoObject.to_opt(),
                                content_url: vec![
                                    "https://content.jwplatform.com/videos/snjcnaUM-K3AjnAEN.mp4".into(),
                                ],
                                embed_url: vec![
                                    "https://content.jwplatform.com/videos/snjcnaUM-K3AjnAEN.mp4".into(),
                                ],
                                ..Default::default()
                            }.into(),
                        ),
                    ],
                    image: vec![
                        RecipeImageFieldEnum::URL(
                            "https://www.allrecipes.com/thmb/ey5aal-JHk7PoNP92is_VCambmA=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/AR-23431-to-die-for-fettuccine-alfredo-DDMFS-beauty-3x4-b64d36c7ff314cb39774e261c5b18352.jpg".into(),
                        ),
                        RecipeImageFieldEnum::URL(
                            "/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/To_Die_For_Fettuccine_Alfredo.jpg".into(),
                        ),
                    ],
                    description: vec![
                        RecipeDescriptionFieldEnum::Text(
                            "Fettuccine Alfredo with Romano and Parmesan cheeses, cream, and butter. This delicious sauce for fettuccine pasta is rich in taste yet simple to make.".into(),
                        ),
                    ],
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
                        trans_fat_content: vec![Mass::new("37 g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        ..Default::default()
                    }],
                    url: vec!["https://www.allrecipes.com/recipe/23431/to-die-for-fettuccine-alfredo/".into()],
                    name: vec!["To Die For Fettuccine Alfredo".into()],
                    ..Default::default()
                },
            ]
        }
    }
}
