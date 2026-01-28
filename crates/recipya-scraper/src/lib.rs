mod client;
mod custom;
mod error;

pub mod tests;

pub(crate) mod websites;

pub use client::{AppHttpClient, HttpClient};
pub use error::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use schema_org::{AtType, GraphObject, Recipe};
use tracing::error;

use scraper::{Html, Selector};
use support::fs::FsSupport;

use crate::websites::Website;

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
            Ok(recipe) => Ok(recipe),
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

                serde_json::from_str::<Recipe>(&json)
                    .inspect_err(|err| {
                        error!("Error parsing schema: {err}\nURL: {url}\nJSON: {json}\n-----");
                    })
                    .ok()
            })
            .find_map(|recipe| {
                if recipe.r#type != AtType::Recipe.to_opt() {
                    return None;
                }

                match recipe.graph {
                    Some(graph) => graph.into_iter().find_map(|item| match item {
                        GraphObject::Recipe(mut r) => {
                            r.r#type = AtType::Recipe.to_opt();
                            Some(*r)
                        }
                        _ => None,
                    }),
                    None => match website {
                        Website::ZabihaHalal => Some(custom::zabihahalal::add_info(doc, recipe)),
                        _ => Some(recipe),
                    },
                }
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
