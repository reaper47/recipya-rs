use std::io::Read;

use serde::Deserialize;
use tracing::error;

use crate::core::integrations::error::{Error, Result};

/// Represents the parsed recipes from an XML COOKmate file.
#[derive(Debug, Deserialize, PartialEq)]
pub struct CookbookXML {
    #[serde(rename = "recipe")]
    recipes: Vec<Recipe>,
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, PartialEq)]
struct List {
    #[serde(rename = "li")]
    items: Vec<String>,
}

impl CookbookXML {
    /// Parses a COOKmate XML recipe file.
    pub fn parse_xml<R>(r: R) -> Result<CookbookXML>
    where
        R: Read,
    {
        serde_xml_rs::from_reader(r).map_err(|err| {
            error!("Failed to read COOKmate XML cookbook: {err}");
            Error::Parse(err.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;

        use std::io::Cursor;

        use files::*;
        use results::*;

        #[test]
        fn test_xml_ok() -> Result<()> {
            let file = xml_file();
            let buf = Cursor::new(file);

            let got = CookbookXML::parse_xml(buf)?;

            pretty_assertions::assert_eq!(got, xml_recipes());
            Ok(())
        }
    }

    mod files {
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
</cookbook>"##
        }
    }

    mod results {
        use super::*;

        pub fn xml_recipes() -> CookbookXML {
            CookbookXML {
                    recipes: vec![
                            Recipe{
                                title: "Asparagus Soup (Zuppa Di Asparagi)".into(),
                                preptime: "".into(),
                                cooktime: "".into(),
                                totaltime: "".into(),
                                description: "".into(),
                                ingredient: List {
                                        items: Vec::from([
                                                "2 tb Extra-virgin olive oil 1 qt Chicken broth".into(),
                                            "2 Cloves garlic, minced 4 Eggs".into(),
                                            "2 lb Asparagus, trimmed, peeled 1/2 c Freshly grated Parmesan or".into(),
                                            "-and cut (1 inch pieces) -pecorino cheese".into(),
                                            "Salt and pepper 6 sl Italian bread, toasted".into(),
                                        ]),
                                    },
                                recipetext: List {
                                        items: vec![
                                                "Heat the oil and garlic in a soup pot until the garlic is golden. Add the".into(),
                                            "asparagus and cook until they begin to color. Season with salt and pepper.".into(),
                                            "Add the broth and bring to a boil; reduce the heat and simmer for 15".into(),
                                            "minutes, or until the asparagus is tender.".into(),
                                            "".into(),
                                            "Beat the eggs and cheese together. When the asparagus is tender, reduce".into(),
                                            "the heat so the soup is no longer simmering. Very slowly ladle some of the".into(),
                                            "hot soup into the beaten eggs, stirring continuously. After adding about 2".into(),
                                            "cups of the hot soup to the eggs, reverse the process and gradually stir".into(),
                                            "the eggs mixture into the soup pot. The soup must not boil or the eggs".into(),
                                            "will scramble. Heat until thickened.".into(),
                                            "".into(),
                                            "Put one slice of toasted bread into each soup dish. Ladle the hot soup on".into(),
                                            "top and pass additional grated cheese.".into(),
                                            "".into(),
                                            "Serves 6.".into(),
                                            "".into(),
                                            "NOTE: To trim asparagus, hold the tip in one hand and the base of the".into(),
                                            "stalk in the other. Bend gently. The asparagus will snap, leaving the".into(),
                                            "tender part with the tip.".into(),
                                            "".into(),
                                            "[ \"We Called It Macaroni\"; Nancy Verde Barr; Knopf; ISBN 0-394-55798-0 ]".into(),
                                            "".into(),
                                            "Posted by Fred Peters.".into(),
                                        ],
                                    },
                                url: "MMF".into(),
                                imagepath: "".into(),
                                imageurl: "".into(),
                                quantity: "6 servings".into(),
                                comments: "".into(),
                                nutrition: "".into(),
                                lang: "".into(),
                                rating: 0,
                                source: "".into(),
                                video: "".into(),
                                categories: vec![
                                        "Italian".into(),
                                    "Soups/stews".into(),
                                    "Vegetables".into(),
                                ],
                            },
                        Recipe {
                            title: "Aubergine and Sesame Pate".into(),
                            preptime: "".into(),
                            cooktime: "".into(),
                            totaltime: "".into(),
                            description: "".into(),
                            ingredient: List {
                                items: vec![
                                    "1/2 md Aubergine 1/4 Juice of 1 lemon".into(),
                                    "1 Crushed garlic cloves 1 tb Olive oil".into(),
                                    "1 1/2 tb Tahini Seasoning".into(),
                                    "Toasted Sesame seeds Flatleaf Parsley".into(),
                                    "Cayenne Pepper".into(),
                                    "25-30 minutes until tender. Cool slightly , then peel and".into(),
                                ],
                            },
                            recipetext: List {
                                items: vec![
                                    "1> Preheat the oven to 200c/400f/Gas 6. Bake the aubergine for".into(),
                                    "puree the flesh in a blender or processor.".into(),
                                    "".into(),
                                    "Add the garlic, tahini and lemon juice and process until mixed.".into(),
                                    "With the motor running, drizzle in the oil to make a smooth paste.".into(),
                                    "Season to taste.".into(),
                                    "".into(),
                                    "Transfer to a serving dish, garnish and serve cold with pitta bread.".into(),
                                ],
                            },
                            url: "MMF".into(),
                            imagepath: "".into(),
                            imageurl: "".into(),
                            quantity: "2 servings".into(),
                            comments: "".into(),
                            nutrition: "".into(),
                            lang: "".into(),
                            rating: 0,
                            source: "".into(),
                            video: "".into(),
                            categories: vec![
                                "Vegetarian".into(),
                                "Appetizers".into(),
                                "Greek".into(),
                            ],
                        },
                        Recipe {
                            title: "Aubergines a la Toulousaine (Eggplant A La Toulouse)".into(),
                            preptime: "".into(),
                            cooktime: "".into(),
                            totaltime: "".into(),
                            description: "".into(),
                            ingredient: List {
                                items: vec![
                                    "1 md Eggplant 2 tb Snipped parsley".into(),
                                    "1/4 c Salad oil 1 cl Galic, minced".into(),
                                    "3 lg Tomatoes, peeled 1 tb Salad oil".into(),
                                    "2 c Fresh bread cubes 1/4 c Grated Parmesan cheese".into(),
                                ],
                            },
                            recipetext: List {
                                items: vec![
                                    "Cut eggplant into 1/2-inch thick slices: pared. Place slices on paper".into(),
                                    "towels; sprinkle each generously with salt. let stand for 30 minutes; then".into(),
                                    "blot dry with paper towels. Start heating oven to 400 deg. F. Saute".into(),
                                    "eggplant in 1/4 cup salad oil until golden. Add more oils as needed. Cut".into(),
                                    "tomatoes into 1/2-inch thick slices; saute in same skillet. In a 10x6x2".into(),
                                    "inch baking dish, arrange eggplant and tomatoes in alternate layers, (4 in".into(),
                                    "all), sprinkling each layer with 1/4 teaspoon salt and 1/8 teaspoon pepper.".into(),
                                    "Combine bread cubes with parsley, garlic, 1 tablespoon salad oil and".into(),
                                    "cheese. Toss well. Sprinkle over top layer. Bake 20 minutes or until bread".into(),
                                    "cubes are golden and eggplant is tender.".into(),
                                    "".into(),
                                    "SOURCE: Good Houskeeping's Around The World Cookbook. Consolidated Book".into(),
                                    "Publishers Chicago 1, Illinois 1958".into(),
                                ],
                            },
                            url: "MMF".into(),
                            imagepath: "".into(),
                            imageurl: "".into(),
                            quantity: "4 servings".into(),
                            comments: "".into(),
                            nutrition: "".into(),
                            lang: "".into(),
                            rating: 0,
                            source: "".into(),
                            video: "".into(),
                            categories: vec![
                                "Vegetables".into(),
                                "Casseroles".into(),
                                "French".into(),
                            ],
                        },
                    ],
            }
        }
    }
}
