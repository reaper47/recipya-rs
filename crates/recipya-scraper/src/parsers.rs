use itertools::Itertools;
use scraper::{Html, Selector};

use schema_org::{
    AtType, GraphObject, Recipe, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};
use serde_json::Value;
use tracing::error;
use url::Url;

use crate::{Error, Result, Website};

pub struct Parser<'a> {
    doc: Html,
    url: &'a str,
    website: Website,
}

impl<'a> Parser<'a> {
    pub fn new(url: &'a str, content: &str, website: Website) -> Self {
        Self {
            doc: Html::parse_document(content),
            url,
            website,
        }
    }
}

impl Parser<'_> {
    /// Tries to parse the page using the available parsers and returns a `Recipe` if one is found.
    ///
    /// It tries the following parsers in order:
    /// 1. LD+JSON
    /// 2. hRecipe
    /// 3. custom
    pub fn parse(&self) -> Result<Recipe> {
        match self.parse_ld_json() {
            Ok(recipe) => Ok(recipe),
            Err(Error::NoRecipeFound) => match self.parse_hrecipe() {
                Ok(recipe) => Ok(recipe),
                Err(Error::NoRecipeFound) => self.parse_manually(),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    /// Parses the LD+JSON script tag on the page and returns a `Recipe` if one is found.
    fn parse_ld_json(&self) -> Result<Recipe> {
        self.doc
            .select(&Selector::parse("script[type='application/ld+json']")?)
            .filter_map(|el| {
                let json = el.inner_html().split_whitespace().join(" ");

                let value: serde_json::Value = serde_json::from_str(&json).ok()?;
                let object = value
                    .as_array()
                    .and_then(|arr| arr.first())
                    .cloned()
                    .unwrap_or(value);
                let type_recipe = AtType::Recipe.to_opt();

                let is_recipe_type = |obj: &Value| match obj.get("@type") {
                    Some(t) if t.as_str() == type_recipe.as_deref() => true,
                    Some(t) => t.as_array().is_some_and(|arr| {
                        arr.iter().any(|v| v.as_str() == type_recipe.as_deref())
                    }),
                    None => false,
                };

                if is_recipe_type(&object) {
                    return serde_json::from_value::<Recipe>(object)
                        .inspect_err(|err| {
                            error!(?err, url = self.url, ?json, "Error parsing schema");
                        })
                        .ok();
                }

                object
                    .get("@graph")
                    .and_then(|g| g.as_array())
                    .and_then(|graph| {
                        graph
                            .iter()
                            .find(|item| is_recipe_type(item))
                            .and_then(|item| {
                                serde_json::from_value::<Recipe>(item.clone())
                                    .inspect_err(|err| {
                                        error!(url = self.url, ?err, "Parsing failed");
                                    })
                                    .ok()
                            })
                    })
            })
            .find_map(|recipe| {
                let mut recipe = match recipe.graph {
                    Some(graph) => graph.into_iter().find_map(|item| match item {
                        GraphObject::Recipe(mut r) => {
                            r.r#type = AtType::Recipe.to_opt();
                            Some(*r)
                        }
                        GraphObject::Unknown(_) => None,
                    }),
                    None => Some(self.website.augment_ld_json(&self.doc, recipe)),
                };

                if let Some(r) = recipe.as_mut() {
                    r.context = at_context();
                    if let Some(author) = r.author.first()
                        && author.is_default()
                    {
                        r.author = vec![];
                    }
                    r.is_part_of = vec![]; // Note: It would be nice if the serde deserialization skips deserialization if default.
                    r.url = vec![
                        self.url
                            .rsplit_once("<number>")
                            .unwrap_or((self.url, ""))
                            .0
                            .into(),
                    ];
                }

                recipe
            })
            .filter(|r| r.r#type == AtType::Recipe.to_opt())
            .ok_or(Error::NoRecipeFound)
    }

    /// Parses an hRecipe on the page and returns a `Recipe` if one is found.
    fn parse_hrecipe(&self) -> Result<Recipe> {
        self.doc
            .select(&Selector::parse(".h-recipe")?)
            .next()
            .map_or(Err(Error::NoRecipeFound), |el| {
                Ok(Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    date_published: el
                        .select(&Selector::parse(".dt-published")?)
                        .next()
                        .map(|el| el.attr("datetime").unwrap_or_default())
                        .filter(|s| !s.is_empty())
                        .map_or(Vec::new(), |s| vec![s.into()]),
                    name: el
                        .select(&Selector::parse(".p-name")?)
                        .next()
                        .map_or(Vec::new(), |el| vec![el.text().collect::<String>()]),
                    description: el
                        .select(&Selector::parse(".p-summary")?)
                        .map(|el| RecipeDescriptionFieldEnum::Text(el.text().collect::<String>()))
                        .collect_vec(),
                    recipe_ingredient: el
                        .select(&Selector::parse(".p-ingredient").unwrap())
                        .map(|el| {
                            RecipeRecipeIngredientFieldEnum::Text(el.text().collect::<String>())
                        })
                        .collect_vec(),
                    recipe_instructions: el
                        .select(&Selector::parse(".e-instructions li, .p-instructions").unwrap())
                        .flat_map(|el| {
                            el.html()
                                .split("<br>")
                                .map(str::trim)
                                .filter(|s| !s.is_empty())
                                .map(Html::parse_fragment)
                                .map(|el| el.root_element().text().collect::<String>())
                                .map(RecipeRecipeInstructionsFieldEnum::Text)
                                .collect_vec()
                        })
                        .collect_vec(),
                    image: el
                        .select(&Selector::parse("img.u-photo").unwrap())
                        .filter_map(|el| el.attr("src").map(String::from))
                        .map(|url| match Url::parse(&url) {
                            Ok(url) => RecipeImageFieldEnum::URL(url.into()),
                            Err(url::ParseError::RelativeUrlWithoutBase)
                                if let Ok(original) = Url::parse(self.url)
                                    && let Some(host) = original.host_str() =>
                            {
                                RecipeImageFieldEnum::URL(format!(
                                    "{}://{host}{url}",
                                    original.scheme(),
                                ))
                            }
                            Err(_) => RecipeImageFieldEnum::URL(url),
                        })
                        .collect_vec(),
                    url: vec![self.url.to_string()],
                    ..Default::default()
                })
            })
    }

    /// Parses the page using a custom parser and returns a `Recipe` if one is found.
    fn parse_manually(&self) -> Result<Recipe> {
        self.website.parse_manually(&self.doc, self.url)
    }
}
