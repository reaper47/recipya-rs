mod client;
mod custom;
mod error;
mod websites;

pub mod tests;

use serde_json::Value;
pub use websites::*;

pub use client::{AppHttpClient, HttpClient};
pub use error::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use schema_org::{AtType, GraphObject, Recipe, at_context};
use tracing::error;

use scraper::{Html, Selector};
use support::fs::FsSupport;

pub(crate) const ENABLE_JS: &str =
    r#"<span id="challenge-error-text">Enable JavaScript and cookies to continue</span>"#;

pub(crate) const FORBIDDEN: &str = "403 Forbidden";

/// Represents the object responsible for scraping recipes from websites.
#[derive(Clone)]
pub struct Scraper {
    client: Arc<dyn HttpClient + Send + Sync>,
    fs_support: Arc<dyn FsSupport + Send + Sync>,
}

impl Scraper {
    /// Create a new Scraper with the given HTTP client. Mainly used in tests.
    pub fn with_client(
        client: Arc<dyn HttpClient + Send + Sync>,
        fs_support: Arc<dyn FsSupport + Send + Sync>,
    ) -> Self {
        Self { client, fs_support }
    }

    /// Scrapes the given URL and returns a `RecipeSchema`.
    pub async fn scrape(&self, url: &str) -> Result<Recipe> {
        let website = Website::from(url)?;
        let content = self.client.get(website, url).await?;
        let doc = Html::parse_document(&content);

        match Self::parse_ld_json(url, &doc, website) {
            Ok(recipe) => {
                if recipe.r#type == AtType::Recipe.to_opt() {
                    Ok(recipe)
                } else {
                    website.parse_manually(&doc, url)
                }
            }
            Err(Error::DomainNotImplemented) => website.parse_manually(&doc, url),
            Err(err) => Err(err),
        }
    }

    fn parse_ld_json(url: &str, doc: &Html, website: Website) -> Result<Recipe> {
        let sel = Selector::parse("script[type='application/ld+json']")?;

        doc.select(&sel)
            .filter_map(|el| {
                let json = el
                    .inner_html()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");

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
                            error!("Error parsing schema: {err}\nURL: {url}\nJSON: {json}\n-----");
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
                                    .inspect_err(|err| error!("Failed for '{url}': {err}"))
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
                    None => Some(website.augment_ld_json(doc, recipe)),
                };

                if let Some(r) = recipe.as_mut() {
                    r.context = at_context();
                    if let Some(author) = r.author.first()
                        && author.is_default()
                    {
                        r.author = vec![];
                    }
                    r.is_part_of = vec![]; // Note: It would be nice if the serde deserialization skips deserialization if default.
                    r.url = vec![url.rsplit_once("<number>").unwrap_or((url, "")).0.into()];
                }

                recipe
            })
            .ok_or(Error::DomainNotImplemented)
    }

    /// Fetches the content of a URL and uploads it the temporary directory.
    pub async fn fetch_and_upload_to_temp(&self, url: &str) -> Result<PathBuf> {
        let content = self.client.get_bytes(url).await?;
        let path = self
            .fs_support
            .upload_to_temp(content)
            .await
            .map_err(|_| Error::Filesystem)?;
        Ok(path)
    }
}
