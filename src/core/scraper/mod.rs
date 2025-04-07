pub mod schema;
pub mod tests;
pub(super) mod websites;

mod client;
mod error;

pub use client::{AppHttpClient, HttpClient};
pub use error::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use scraper::{Html, Selector};
use tracing::error;

use crate::core::scraper::schema::{AtType, GraphObject, RecipeSchema};
use crate::core::scraper::websites::Website;
use crate::core::support::fs::FsSupport;

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
    pub fn scrape(&self, url: &str) -> Result<RecipeSchema> {
        let content = self.client.get(Website::from(url)?, url)?;
        let doc = Html::parse_document(&content);

        let sel = Selector::parse(r#"script[type='application/ld+json']"#)?;
        doc.select(&sel)
            .filter_map(|el| {
                let json = el.inner_html();
                serde_json::from_str::<RecipeSchema>(&json)
                    .map_err(|err| {
                        error!("Error parsing schema: {err}\nURL: {url}\nJSON: {json}\n-----");
                        err
                    })
                    .ok()
            })
            .find_map(|recipe| {
                recipe.at_graph.and_then(|graph| {
                    graph.into_iter().find_map(|item| match item {
                        GraphObject::Recipe(mut r) => {
                            r.at_type = Some(AtType::Recipe);
                            Some(*r)
                        }
                        _ => None,
                    })
                })
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // mod tests_scraper_0_to_9;
//     // mod tests_scraper_a;
//     // mod tests_scraper_b;
//     // mod tests_scraper_d;
//     // mod tests_scraper_e;
//     // mod tests_scraper_f;
//     // mod tests_scraper_g;
//     // mod tests_scraper_h;
//     // mod tests_scraper_i;
//     // mod tests_scraper_j;
//     // mod tests_scraper_k;
//     // mod tests_scraper_l;
//     // mod tests_scraper_m;
//     // mod tests_scraper_o;
//     // mod tests_scraper_n;
//     // mod tests_scraper_p;
//     // mod tests_scraper_q;
//     // mod tests_scraper_r;
//     // mod tests_scraper_s;
//     // mod tests_scraper_t;
//     // mod tests_scraper_u;
//     // mod tests_scraper_v;
//     // mod tests_scraper_w;
//     // mod tests_scraper_y;
//     // mod tests_scraper_z;
// }
