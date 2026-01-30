mod client;
mod custom;
mod error;
mod websites;

pub mod tests;

pub use websites::*;

pub use client::{AppHttpClient, HttpClient};
pub use error::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use schema_org::{AtType, GraphObject, Recipe, at_context};
use tracing::error;

use scraper::{Html, Selector};
use support::fs::FsSupport;

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
    pub fn scrape(&self, url: &str) -> Result<Recipe> {
        let website = Website::from(url)?;
        let content = self.client.get(website, url)?;
        let doc = Html::parse_document(&content);

        match self.parse_ld_json(url, &doc, &website) {
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

    fn parse_ld_json(&self, url: &str, doc: &Html, website: &Website) -> Result<Recipe> {
        let sel = Selector::parse(r#"script[type='application/ld+json']"#)?;

        doc.select(&sel)
            .filter_map(|el| {
                let json = &el
                    .inner_html()
                    .trim()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");

                let value: serde_json::Value = serde_json::from_str(&json).ok()?;

                match value.get("@type").and_then(|v| v.as_str()) {
                    Some(_) => serde_json::from_str::<Recipe>(&json)
                        .inspect_err(|err| {
                            error!("Error parsing schema: {err}\nURL: {url}\nJSON: {json}\n-----");
                        })
                        .ok(),
                    None => {
                        if let Some(graph) = value.get("@graph").and_then(|g| g.as_array()) {
                            for item in graph {
                                if item.get("@type").and_then(|t| t.as_str()) == Some("Recipe") {
                                    return serde_json::from_value::<Recipe>(item.clone()).ok();
                                }
                            }
                        }
                        return None;
                    }
                }
            })
            .find_map(|recipe| {
                let mut recipe = match recipe.graph {
                    Some(graph) => graph.into_iter().find_map(|item| match item {
                        GraphObject::Recipe(mut r) => {
                            r.r#type = AtType::Recipe.to_opt();
                            Some(*r)
                        }
                        _ => None,
                    }),
                    None => Some(website.augment_ld_json(doc, recipe)),
                };

                recipe.as_mut().map(|r| {
                    r.context = at_context();
                    r.is_part_of = vec![]; // Note: It would be nice if the serde deserialization skips deserialization if default.
                    r.url = vec![url.into()]
                });
                recipe
            })
            .ok_or(Error::DomainNotImplemented)
    }

    /// Fetches the content of a URL and uploads it the temporary directory.
    pub async fn fetch_and_upload_to_temp(&self, url: &str) -> Result<PathBuf> {
        let content = self.client.get_bytes(url)?;
        let path = self
            .fs_support
            .upload_to_temp(content)
            .await
            .map_err(|_| Error::Filesystem)?;
        Ok(path)
    }
}
