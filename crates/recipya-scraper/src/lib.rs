mod client;
mod custom;
mod error;
mod parsers;
mod websites;

pub mod tests;

pub use websites::*;

pub use client::{AppHttpClient, HttpClient};
pub use error::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Semaphore;

use schema_org::Recipe;
use support::fs::FsSupport;

use crate::parsers::Parser;

pub(crate) const ENABLE_JS: &str =
    r#"<span id="challenge-error-text">Enable JavaScript and cookies to continue</span>"#;

pub(crate) const FORBIDDEN: &str = "403 Forbidden";

const MAX_CONCURRENT_SCRAPED_WEBSITES: usize = 64;

/// Represents the object responsible for scraping recipes from websites.
#[derive(Clone)]
pub struct Scraper {
    client: Arc<dyn HttpClient + Send + Sync>,
    fs_support: Arc<dyn FsSupport + Send + Sync>,
    pub semaphore: Arc<Semaphore>,
}

impl Scraper {
    /// Create a new Scraper with the given HTTP client. Mainly used in tests.
    pub fn with_client(
        client: Arc<dyn HttpClient + Send + Sync>,
        fs_support: Arc<dyn FsSupport + Send + Sync>,
    ) -> Self {
        Self {
            client,
            fs_support,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_SCRAPED_WEBSITES)),
        }
    }

    /// Scrapes the given URL and returns a `RecipeSchema`.
    pub async fn scrape(&self, url: &str) -> Result<Recipe> {
        let website = Website::from(url)?;
        let content = self.client.get(website, url).await?;
        let parser = Parser::new(url, &content, website);
        parser.parse()
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
