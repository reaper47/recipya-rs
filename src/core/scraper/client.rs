use crate::core::scraper::Result;
use crate::core::scraper::websites::Website;

/// A trait defining HTTP client functionality for synchronous and asynchronous requests.
#[async_trait::async_trait]
pub(in crate::core::scraper) trait HttpClient {
    /// Performs an asynchronous HTTP GET request.
    async fn get_async<'a>(&'a self, host: Website, url: &str) -> Result<String>;

    /// Performs a synchronous HTTP GET request.
    fn get(&self, host: Website, url: &str) -> Result<String>;
}

/// A wrapper around `reqwest::Client` for making HTTP requests.
pub(in crate::core::scraper) struct AppHttpClient {
    client: reqwest::Client,
}

impl AppHttpClient {
    /// Creates a new instance of `AppHttpClient`.
    pub(in crate::core::scraper) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl HttpClient for AppHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, url: &str) -> Result<String> {
        let res = self.client.get(url).send().await?;
        let body = res.text().await?;
        Ok(body)
    }

    fn get(&self, _host: Website, url: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(url).send()?;
        let body = res.text()?;
        Ok(body)
    }
}
