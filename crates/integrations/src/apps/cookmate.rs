use std::io::{Read, Seek};

use humantime::parse_duration;
use serde::Deserialize;
use support::strings::extract_number;
use tracing::{error, warn};
use url::Url;

use recipe_schema::{
    AggregateRating, AtType, ClipOrVideoObject, CommentType, DefinedTermOrTextOrUrl,
    ImageObjectOrUrl, NumberOrText, RecipeCategory, RecipeSchema, SectionItem, Sections,
    VideoObjectType,
};

use super::helpers::{extract_archive_contents, update_recipe_image_paths};
use crate::helpers::{
    seconds_to_duration, sections_to_itemlist, to_is_based_on, to_text, to_yield,
};
use crate::{Error, Result};

#[derive(Deserialize)]
struct CookbookXML {
    #[serde(rename = "recipe")]
    recipes: Vec<Recipe>,
}

#[derive(Deserialize)]
struct Recipe {
    title: String,
    preptime: String,
    cooktime: String,
    totaltime: String,
    description: String,
    ingredient: List,
    recipetext: List,
    url: String,
    imagepath: String,
    imageurl: String,
    quantity: String,
    comments: String,
    nutrition: String,
    lang: String,
    rating: i64,
    source: String,
    video: String,
    #[serde(rename = "category")]
    categories: Vec<String>,
}

#[derive(Deserialize)]
struct List {
    #[serde(rename = "li")]
    items: Vec<String>,
}

impl From<Recipe> for RecipeSchema {
    fn from(r: Recipe) -> Self {
        let categories = r.categories.split_first();
        let url = Url::parse(&r.url).ok();

        let comments = vec![r.comments]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let num_comments = comments.len();

        Self {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            aggregate_rating: if r.rating > 0 {
                Some(AggregateRating {
                    rating_value: Some(NumberOrText::Number(r.rating as f64)),
                    ..Default::default()
                })
            } else {
                None
            },
            comment: Some(
                comments
                    .into_iter()
                    .map(|c| CommentType {
                        text: c,
                        ..Default::default()
                    })
                    .collect(),
            )
            .filter(|v: &Vec<CommentType>| !v.is_empty()),
            comment_count: Some(num_comments as i64).filter(|c| *c > 0),
            cook_time: match parse_duration(&r.cooktime) {
                Ok(d) => seconds_to_duration(d.as_secs() as i32),
                Err(err) => {
                    error!(
                        "Failed to parse cook time '{}' of an AccuChef recipe: {err}",
                        r.cooktime
                    );
                    None
                }
            },
            description: to_text(r.description),
            image: {
                let urls = vec![r.imageurl, r.imagepath]
                    .into_iter()
                    .filter_map(|image| Url::parse(&image).ok())
                    .collect::<Vec<_>>();

                (!urls.is_empty()).then_some(ImageObjectOrUrl::Urls(urls))
            },
            is_based_on: if !r.source.is_empty() {
                to_is_based_on(r.source)
            } else {
                to_is_based_on(r.url)
            },
            keywords: Some(DefinedTermOrTextOrUrl::Text(
                categories
                    .map(|(_a, b)| b.iter().map(|s| s.to_string()).collect::<Vec<_>>())
                    .unwrap_or_default()
                    .join(","),
            )),
            name: Some(r.title),
            nutrition: if !r.nutrition.is_empty() {
                warn!("CookMate XML has nutrients: '{}'", r.nutrition);
                None
            } else {
                None
            },
            prep_time: match parse_duration(&r.preptime) {
                Ok(d) => seconds_to_duration(d.as_secs() as i32),
                Err(err) => {
                    error!(
                        "Failed to parse prep time '{}' of a CookMate XML recipe: {err}",
                        r.preptime
                    );
                    None
                }
            },
            recipe_category: RecipeCategory::Text(
                categories.map(|(a, _b)| a.to_string()).unwrap_or_default(),
            ),
            recipe_ingredient: Some(r.ingredient.items),
            recipe_instructions: sections_to_itemlist(Sections::from([(
                "".into(),
                r.recipetext.items.iter().map(SectionItem::new).collect(),
            )])),
            recipe_yield: to_yield(extract_number(r.quantity).unwrap_or_default()),
            url,
            video: Some(
                vec![r.video]
                    .into_iter()
                    .filter_map(|s| Url::parse(&s).ok())
                    .map(|url| {
                        ClipOrVideoObject::VideoObject(Box::new(VideoObjectType {
                            at_type: Default::default(),
                            content_url: url.clone(),
                            description: "".to_string(),
                            duration: None,
                            embed_url: url,
                            name: "".to_string(),
                            thumbnail_url: vec![],
                            upload_date: None,
                        }))
                    })
                    .collect::<Vec<_>>(),
            )
            .filter(|v| !v.is_empty()),
            ..Default::default()
        }
    }
}

/// Parses a COOKmate XML recipe file.
pub fn parse<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let root: CookbookXML =
        serde_xml_rs::from_reader(r).map_err(|err| Error::Parse(err.to_string()))?;
    Ok(root.recipes.into_iter().map(RecipeSchema::from).collect())
}

pub fn parse_backup<R>(r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read + Seek,
{
    let archive = zip::ZipArchive::new(r)?;
    let (mut recipes, images) = extract_archive_contents(archive)?;
    update_recipe_image_paths(&mut recipes, &images);
    Ok(recipes)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;

        use std::io::Cursor;

        #[test]
        fn test_xml_ok() -> Result<()> {
            let file = files::xml_file();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::xml_recipes());
            Ok(())
        }

        #[test]
        fn test_backup_ok() -> Result<()> {
            let buf = files::backup();

            let mut got = parse_backup(buf)?;

            let want = results::xml_recipes();
            got[4].image = want[4].image.clone();
            pretty_assertions::assert_eq!(got[..5], want);
            Ok(())
        }
    }

    mod files {
        use std::io::Cursor;
        use testing::utils::open_test_file;

        pub fn xml_file<'a>() -> &'a str {
            r##"<?xml version="1.0" encoding="utf-8"?>
<cookbook version="71">
<recipe>
<title>Asparagus Soup (Zuppa Di Asparagi)</title>
<preptime></preptime>
<cooktime></cooktime>
<totaltime></totaltime>
<description></description>
<ingredient><li>2 tb Extra-virgin olive oil 1 qt Chicken broth</li>
<li>2 Cloves garlic, minced 4 Eggs</li>
<li>2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or</li>
<li>-and cut (1 inch pieces) -pecorino cheese</li>
<li>Salt and pepper 6 sl Italian bread, toasted</li>
</ingredient>
<recipetext><li>Heat the oil and garlic in a soup pot until the garlic is golden. Add the</li>
<li>asparagus and cook until they begin to color. Season with salt and pepper.</li>
<li>Add the broth and bring to a boil; reduce the heat and simmer for 15</li>
<li>minutes, or until the asparagus is tender.</li>
<li></li>
<li>Beat the eggs and cheese together. When the asparagus is tender, reduce</li>
<li>the heat so the soup is no longer simmering. Very slowly ladle some of the</li>
<li>hot soup into the beaten eggs, stirring continuously. After adding about 2</li>
<li>cups of the hot soup to the eggs, reverse the process and gradually stir</li>
<li>the eggs mixture into the soup pot. The soup must not boil or the eggs</li>
<li>will scramble. Heat until thickened.</li>
<li></li>
<li>Put one slice of toasted bread into each soup dish. Ladle the hot soup on</li>
<li>top and pass additional grated cheese.</li>
<li></li>
<li>Serves 6.</li>
<li></li>
<li>NOTE: To trim asparagus, hold the tip in one hand and the base of the</li>
<li>stalk in the other. Bend gently. The asparagus will snap, leaving the</li>
<li>tender part with the tip.</li>
<li></li>
<li>[ &quot;We Called It Macaroni&quot;; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]</li>
<li></li>
<li>Posted by Fred Peters.</li>
</recipetext>
<url>MMF</url>
<imagepath></imagepath>
<imageurl></imageurl>
<quantity>6 servings</quantity>
<comments></comments>
<nutrition></nutrition>
<lang></lang>
<rating>0</rating>
<source></source>
<video></video>
<category>Italian</category>
<category>Soups/stews</category>
<category>Vegetables</category>
</recipe>

<recipe>
<title>Aubergine and Sesame Pate</title>
<preptime></preptime>
<cooktime></cooktime>
<totaltime></totaltime>
<description></description>
<ingredient><li>1/2 md Aubergine 1/4 Juice of 1 lemon</li>
<li>1 Crushed garlic cloves 1 tb Olive oil</li>
<li>1 1/2 tb Tahini Seasoning</li>
<li>Toasted Sesame seeds Flatleaf Parsley</li>
<li>Cayenne Pepper</li>
<li>25-30 minutes until tender. Cool slightly , then peel and</li>
</ingredient>
<recipetext><li>1&gt; Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for</li>
<li>puree the flesh in a blender or processor.</li>
<li></li>
<li>Add the garlic, tahini and lemon juice and process until mixed.</li>
<li>With the motor running, drizzle in the oil to make a smooth paste.</li>
<li>Season to taste.</li>
<li></li>
<li>Transfer to a serving dish, garnish and serve cold with pitta bread.</li>
</recipetext>
<url>MMF</url>
<imagepath></imagepath>
<imageurl></imageurl>
<quantity>2 servings</quantity>
<comments></comments>
<nutrition></nutrition>
<lang></lang>
<rating>0</rating>
<source></source>
<video></video>
<category>Vegetarian</category>
<category>Appetizers</category>
<category>Greek</category>
</recipe>

<recipe>
<title>Aubergines a la Toulousaine (Eggplant A La Toulouse)</title>
<preptime></preptime>
<cooktime></cooktime>
<totaltime></totaltime>
<description></description>
<ingredient><li>1 md Eggplant 2 tb Snipped parsley</li>
<li>1/4 c Salad oil 1 cl Galic, minced</li>
<li>3 lg Tomatoes, peeled 1 tb Salad oil</li>
<li>2 c Fresh bread cubes 1/4 c Grated Parmesan cheese</li>
</ingredient>
<recipetext><li>Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper</li>
<li>towels; sprinkle each generously with salt. let stand for 30 minutes; then</li>
<li>blot dry with paper towels. Start heating oven to 400 deg. F. Saute</li>
<li>eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut</li>
<li>tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2</li>
<li>inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in</li>
<li>all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.</li>
<li>Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and</li>
<li>cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread</li>
<li>cubes are golden and eggplant is tender.</li>
<li></li>
<li>SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book</li>
<li>Publishers Chicago 1, Illinois 1958</li>
</recipetext>
<url>MMF</url>
<imagepath></imagepath>
<imageurl></imageurl>
<quantity>4 servings</quantity>
<comments></comments>
<nutrition></nutrition>
<lang></lang>
<rating>0</rating>
<source></source>
<video></video>
<category>Vegetables</category>
<category>Casseroles</category>
<category>French</category>
</recipe>
<recipe>
<title>August Goerg's Grilled Steak (Spiessbraten August Goerg)</title>
<preptime></preptime>
<cooktime></cooktime>
<totaltime></totaltime>
<description></description>
<ingredient><li>1 Shallot or small onion cut 1 pn Mace</li>
<li>-into small pieces 1 lg Steak (just over 1 lb), at</li>
<li>Freshly ground black pepper -least 1 1/4 inches</li>
</ingredient>
<recipetext><li>((Note: Per Horst Scharfenberg, this recipe originated in the town of</li>
<li>Idar-Oberstein in the 19 th century, when gemstone prospectors returning</li>
<li>from South America created their own version of gaucho-grilled steaks. The</li>
<li>dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))</li>
<li></li>
<li>Per person: thick, trimmed</li>
<li></li>
<li>Mix together the shallot or onion with the pepper and mace. Insert a few</li>
<li>shallot pieces into the steak using the point of a small knife. Coat the</li>
<li>steak with the shallot mixture, pressing it in so it will adhere.</li>
<li></li>
<li>Remove the loose shallot pieces and grill the steak (over a fire of oak</li>
<li>logs, says August Goerg, from which the bark has been removed).* Take the</li>
<li>steaks off the grill while they are still pink inside. Sprinkle them with</li>
<li>salt.</li>
<li></li>
<li>*Note: A special grill is used, suspended with 3 chains from an iron</li>
<li>tripod and constantly swinging through the flames.</li>
<li></li>
<li>From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &amp;</li>
<li>Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking</li>
<li>Echo, 8/92</li>
</recipetext>
<url>MMF</url>
<imagepath></imagepath>
<imageurl></imageurl>
<quantity>6 servings</quantity>
<comments></comments>
<nutrition></nutrition>
<lang></lang>
<rating>0</rating>
<source></source>
<video></video>
<category>Beef</category>
<category>German</category>
</recipe>

<recipe>
<title>Aunt Julia's Paella</title>
<preptime></preptime>
<cooktime></cooktime>
<totaltime></totaltime>
<description></description>
<ingredient><li>1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento</li>
<li>-and legs) 2 ts Capers, with juice</li>
<li>Salt and pepper to thaste 4 oz Jar pimento-stiffed green</li>
<li>1 lb Lean pork, cut into 1-inch -olives</li>
<li>-cubes 1/2 lb Calamari (squid), cleaned</li>
<li>1 md Onion, minced -and sliced</li>
<li>2 Toes garlic, minced 5 c Water</li>
<li>Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes</li>
<li>-strips: 1 ts Saffron threads</li>
<li>1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,</li>
<li>1 lg Carrot -uncooked</li>
<li>1 Stalk celery 3 Hard boiled eggs, sliced</li>
<li>1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)</li>
<li>1 1/2 lb Peeled shrimp Oil for frying</li>
</ingredient>
<recipetext><li>{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }</li>
<li></li>
<li>In a large electric skillet or paella pan, brown the chicken pieces (that</li>
<li>have been seasoned with salt and pepper) in a little oil. Remove from the</li>
<li>pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.</li>
<li>Remove from the pan. To the pan drippings (add a little more oil if</li>
<li>necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry</li>
<li>for 2 minutes.</li>
<li></li>
<li>Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.</li>
<li>Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the</li>
<li>bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.</li>
<li></li>
<li>Gently stir the rice into the skillet mixture. Slowly pour in enough of</li>
<li>the bouillon mixture to cover the rice and chicken pieces. Cover and cook</li>
<li>over low heat for about 20 minutes. Uncover and decoaratively arrange the</li>
<li>egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary</li>
<li>to keep the rice moist.</li>
<li></li>
<li>Cover and steam for another 10 minutes until the shrimp are cooked and the</li>
<li>rice is tender. (Paella should be moist but not wet!) Place the pan on a</li>
<li>hot pad on the serving table and let everyone help themselves.</li>
<li></li>
<li>Serve with a mixed green salad, red ripe tomatoes and some French bread.</li>
<li>Also mix up a pitcher of Sangria and enjoy!</li>
<li></li>
<li>Serves: 12.</li>
<li></li>
<li>[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]</li>
<li></li>
<li>Posted by Fred Peters</li>
</recipetext>
<url>MMF</url>
<imagepath>/storage/emulated/0/Android/data/fr.cookbook/files/Pictures/Aunt_Julias_Paella.jpg</imagepath>
<imageurl></imageurl>
<quantity>6 servings</quantity>
<comments></comments>
<nutrition></nutrition>
<lang></lang>
<rating>0</rating>
<source></source>
<video></video>
<category>Pork/ham</category>
<category>Poultry</category>
<category>Fish/sea</category>
<category>Spanish</category>
</recipe>
</cookbook>"##
        }

        pub fn backup() -> Cursor<Vec<u8>> {
            open_test_file("integrations/cookmate1.mcb")
        }
    }

    mod results {
        use super::*;

        use crate::helpers::to_yield;
        use recipe_schema::CreativeWorkOrText;

        pub fn xml_recipes() -> Vec<RecipeSchema> {
            vec![
                RecipeSchema {
                    at_context: Default::default(),
                    at_type: Some(AtType::Recipe),
                    is_accessible_for_free: false,
                    is_based_on: Some(CreativeWorkOrText::Text("MMF".into())),
                    keywords: Some(DefinedTermOrTextOrUrl::Text(
                        ["Soups/stews", "Vegetables"].join(","),
                    )),
                    name: Some("Asparagus Soup (Zuppa Di Asparagi)".into()),
                    recipe_category: RecipeCategory::Text("Italian".into()),
                    recipe_ingredient: Some(vec![
                        "2 tb Extra-virgin olive oil 1 qt Chicken broth".into(),
                        "2 Cloves garlic, minced 4 Eggs".into(),
                        "2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or".into(),
                        "-and cut (1 inch pieces) -pecorino cheese".into(),
                        "Salt and pepper 6 sl Italian bread, toasted".into(),
                    ]),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            SectionItem::new(
                                "Heat the oil and garlic in a soup pot until the garlic is golden. Add the",
                            ),
                            SectionItem::new(
                                "asparagus and cook until they begin to color. Season with salt and pepper.",
                            ),
                            SectionItem::new(
                                "Add the broth and bring to a boil; reduce the heat and simmer for 15",
                            ),
                            SectionItem::new("minutes, or until the asparagus is tender."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Beat the eggs and cheese together. When the asparagus is tender, reduce",
                            ),
                            SectionItem::new(
                                "the heat so the soup is no longer simmering. Very slowly ladle some of the",
                            ),
                            SectionItem::new(
                                "hot soup into the beaten eggs, stirring continuously. After adding about 2",
                            ),
                            SectionItem::new(
                                "cups of the hot soup to the eggs, reverse the process and gradually stir",
                            ),
                            SectionItem::new(
                                "the eggs mixture into the soup pot. The soup must not boil or the eggs",
                            ),
                            SectionItem::new("will scramble. Heat until thickened."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Put one slice of toasted bread into each soup dish. Ladle the hot soup on",
                            ),
                            SectionItem::new("top and pass additional grated cheese."),
                            SectionItem::new(""),
                            SectionItem::new("Serves 6."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "NOTE: To trim asparagus, hold the tip in one hand and the base of the",
                            ),
                            SectionItem::new(
                                "stalk in the other. Bend gently. The asparagus will snap, leaving the",
                            ),
                            SectionItem::new("tender part with the tip."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]",
                            ),
                            SectionItem::new(""),
                            SectionItem::new("Posted by Fred Peters."),
                        ],
                    )])),
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                RecipeSchema {
                    at_context: Default::default(),
                    at_type: Some(AtType::Recipe),
                    is_accessible_for_free: false,
                    is_based_on: Some(CreativeWorkOrText::Text("MMF".into())),
                    keywords: Some(DefinedTermOrTextOrUrl::Text(
                        ["Appetizers", "Greek"].join(","),
                    )),
                    name: Some("Aubergine and Sesame Pate".into()),
                    recipe_category: RecipeCategory::Text("Vegetarian".into()),
                    recipe_ingredient: Some(vec![
                        "1/2 md Aubergine 1/4 Juice of 1 lemon".into(),
                        "1 Crushed garlic cloves 1 tb Olive oil".into(),
                        "1 1/2 tb Tahini Seasoning".into(),
                        "Toasted Sesame seeds Flatleaf Parsley".into(),
                        "Cayenne Pepper".into(),
                        "25-30 minutes until tender. Cool slightly , then peel and".into(),
                    ]),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            SectionItem::new(
                                "1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for",
                            ),
                            SectionItem::new("puree the flesh in a blender or processor."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Add the garlic, tahini and lemon juice and process until mixed.",
                            ),
                            SectionItem::new(
                                "With the motor running, drizzle in the oil to make a smooth paste.",
                            ),
                            SectionItem::new("Season to taste."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Transfer to a serving dish, garnish and serve cold with pitta bread.",
                            ),
                        ],
                    )])),
                    recipe_yield: to_yield(2),
                    ..Default::default()
                },
                RecipeSchema {
                    at_context: Default::default(),
                    at_type: Some(AtType::Recipe),
                    is_accessible_for_free: false,
                    is_based_on: Some(CreativeWorkOrText::Text("MMF".into())),
                    keywords: Some(DefinedTermOrTextOrUrl::Text(
                        ["Casseroles", "French"].join(","),
                    )),
                    name: Some("Aubergines a la Toulousaine (Eggplant A La Toulouse)".into()),
                    recipe_category: RecipeCategory::Text("Vegetables".into()),
                    recipe_ingredient: Some(vec![
                        "1 md Eggplant 2 tb Snipped parsley".into(),
                        "1/4 c Salad oil 1 cl Galic, minced".into(),
                        "3 lg Tomatoes, peeled 1 tb Salad oil".into(),
                        "2 c Fresh bread cubes 1/4 c Grated Parmesan cheese".into(),
                    ]),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            SectionItem::new(
                                "Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper",
                            ),
                            SectionItem::new(
                                "towels; sprinkle each generously with salt. let stand for 30 minutes; then",
                            ),
                            SectionItem::new(
                                "blot dry with paper towels. Start heating oven to 400 deg. F. Saute",
                            ),
                            SectionItem::new(
                                "eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut",
                            ),
                            SectionItem::new(
                                "tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2",
                            ),
                            SectionItem::new(
                                "inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in",
                            ),
                            SectionItem::new(
                                "all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.",
                            ),
                            SectionItem::new(
                                "Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and",
                            ),
                            SectionItem::new(
                                "cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread",
                            ),
                            SectionItem::new("cubes are golden and eggplant is tender."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book",
                            ),
                            SectionItem::new("Publishers Chicago 1, Illinois 1958"),
                        ],
                    )])),
                    recipe_yield: to_yield(4),
                    ..Default::default()
                },
                RecipeSchema {
                    at_context: Default::default(),
                    at_type: Some(AtType::Recipe),
                    is_accessible_for_free: false,
                    is_based_on: Some(CreativeWorkOrText::Text("MMF".into())),
                    keywords: Some(DefinedTermOrTextOrUrl::Text("German".into())),
                    name: Some("August Goerg's Grilled Steak (Spiessbraten August Goerg)".into()),
                    recipe_category: RecipeCategory::Text("Beef".into()),
                    recipe_ingredient: Some(vec![
                        "1 Shallot or small onion cut 1 pn Mace".into(),
                        "-into small pieces 1 lg Steak (just over 1 lb), at".into(),
                        "Freshly ground black pepper -least 1 1/4 inches".into(),
                    ]),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            SectionItem::new(
                                "((Note: Per Horst Scharfenberg, this recipe originated in the town of",
                            ),
                            SectionItem::new(
                                "Idar-Oberstein in the 19 th century, when gemstone prospectors returning",
                            ),
                            SectionItem::new(
                                "from South America created their own version of gaucho-grilled steaks. The",
                            ),
                            SectionItem::new(
                                "dish was then further refined by Scharfenberg's mentor August Goerg. K.B.))",
                            ),
                            SectionItem::new(""),
                            SectionItem::new("Per person: thick, trimmed"),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Mix together the shallot or onion with the pepper and mace. Insert a few",
                            ),
                            SectionItem::new(
                                "shallot pieces into the steak using the point of a small knife. Coat the",
                            ),
                            SectionItem::new(
                                "steak with the shallot mixture, pressing it in so it will adhere.",
                            ),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Remove the loose shallot pieces and grill the steak (over a fire of oak",
                            ),
                            SectionItem::new(
                                "logs, says August Goerg, from which the bark has been removed).* Take the",
                            ),
                            SectionItem::new(
                                "steaks off the grill while they are still pink inside. Sprinkle them with",
                            ),
                            SectionItem::new("salt."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "*Note: A special grill is used, suspended with 3 chains from an iron",
                            ),
                            SectionItem::new("tripod and constantly swinging through the flames."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "From: THE CUISINES OF GERMANY by Horst Scharfenberg, Simon &",
                            ),
                            SectionItem::new(
                                "Schuster/Poseidon Press, New York. 1989 Posted by: Karin Brewer, Cooking",
                            ),
                            SectionItem::new("Echo, 8/92"),
                        ],
                    )])),
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
                RecipeSchema {
                    at_context: Default::default(),
                    at_type: Some(AtType::Recipe),
                    is_accessible_for_free: false,
                    is_based_on: Some(CreativeWorkOrText::Text("MMF".into())),
                    keywords: Some(DefinedTermOrTextOrUrl::Text(
                        ["Poultry", "Fish/sea", "Spanish"].join(","),
                    )),
                    name: Some("Aunt Julia's Paella".into()),
                    recipe_category: RecipeCategory::Text("Pork/ham".into()),
                    recipe_ingredient: Some(vec![
                        "1 Chicken, cut up (Or 4 thighs 1 3/4 oz Jar sliced pimento".into(),
                        "-and legs) 2 ts Capers, with juice".into(),
                        "Salt and pepper to thaste 4 oz Jar pimento-stiffed green".into(),
                        "1 lb Lean pork, cut into 1-inch -olives".into(),
                        "-cubes 1/2 lb Calamari (squid), cleaned".into(),
                        "1 md Onion, minced -and sliced".into(),
                        "2 Toes garlic, minced 5 c Water".into(),
                        "Cut into 1 1/2 inch julliene 4 Chicken bouillon cubes".into(),
                        "-strips: 1 ts Saffron threads".into(),
                        "1/2 lg Bell pepper 2 1/2 c Uncle Ben's (c) rice,".into(),
                        "1 lg Carrot -uncooked".into(),
                        "1 Stalk celery 3 Hard boiled eggs, sliced".into(),
                        "1 c Frozen green peas 1/2 lb Unpeeled shrimp (heads on)".into(),
                        "1 1/2 lb Peeled shrimp Oil for frying".into(),
                    ]),
                    recipe_instructions: sections_to_itemlist(Sections::from([(
                        "".into(),
                        vec![
                            SectionItem::new(
                                "{ Submitted by Chiqui Collier, Cookery N'Orleans Restaurant }",
                            ),
                            SectionItem::new(""),
                            SectionItem::new(
                                "In a large electric skillet or paella pan, brown the chicken pieces (that",
                            ),
                            SectionItem::new(
                                "have been seasoned with salt and pepper) in a little oil. Remove from the",
                            ),
                            SectionItem::new(
                                "pan. Add the pork cubes to the drippinfs and brown for about 5 minutes.",
                            ),
                            SectionItem::new(
                                "Remove from the pan. To the pan drippings (add a little more oil if",
                            ),
                            SectionItem::new(
                                "necessary) add the onion, garlic, bell pepper, celery and carrot. Stir-fry",
                            ),
                            SectionItem::new("for 2 minutes."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Add the peas, peeled shrimp, pimentos, capers, chicken, calamari and pork.",
                            ),
                            SectionItem::new(
                                "Stir. In a separate pot, bring the 5 cups of water to a boil; stir in the",
                            ),
                            SectionItem::new(
                                "bouillon cubes and saffron. Let it stand for 5 minutes until dissolved.",
                            ),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Gently stir the rice into the skillet mixture. Slowly pour in enough of",
                            ),
                            SectionItem::new(
                                "the bouillon mixture to cover the rice and chicken pieces. Cover and cook",
                            ),
                            SectionItem::new(
                                "over low heat for about 20 minutes. Uncover and decoaratively arrange the",
                            ),
                            SectionItem::new(
                                "egg slices and raw unpeeled shrimp on the top. (Add more broth as necessary",
                            ),
                            SectionItem::new("to keep the rice moist."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Cover and steam for another 10 minutes until the shrimp are cooked and the",
                            ),
                            SectionItem::new(
                                "rice is tender. (Paella should be moist but not wet!) Place the pan on a",
                            ),
                            SectionItem::new(
                                "hot pad on the serving table and let everyone help themselves.",
                            ),
                            SectionItem::new(""),
                            SectionItem::new(
                                "Serve with a mixed green salad, red ripe tomatoes and some French bread.",
                            ),
                            SectionItem::new("Also mix up a pitcher of Sangria and enjoy!"),
                            SectionItem::new(""),
                            SectionItem::new("Serves: 12."),
                            SectionItem::new(""),
                            SectionItem::new(
                                "[ The Legends of Louisisna Cookbook; Sheila Ainbinder; ISBN 0-671-70817-1 ]",
                            ),
                            SectionItem::new(""),
                            SectionItem::new("Posted by Fred Peters"),
                        ],
                    )])),
                    recipe_yield: to_yield(6),
                    ..Default::default()
                },
            ]
        }
    }
}
