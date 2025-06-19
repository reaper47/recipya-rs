use async_trait::async_trait;
use axum::body::Bytes;

use crate::Result;
use crate::websites::Website;

/// A trait defining HTTP client functionality for synchronous and asynchronous requests.
#[async_trait]
pub trait HttpClient {
    /// Performs an asynchronous HTTP GET request.
    async fn get_async<'a>(&'a self, host: Website, url: &str) -> Result<String>;

    /// Performs a synchronous HTTP GET request.
    fn get(&self, host: Website, url: &str) -> Result<String>;

    /// Fetches the content from a URL and uploads it to the temporary directory.
    fn get_bytes(&self, url: &str) -> Result<Bytes>;
}

/// A wrapper around `reqwest::Client` for making HTTP requests.
#[derive(Default)]
pub struct AppHttpClient {
    client: reqwest::Client,
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

    fn get_bytes(&self, url: &str) -> Result<Bytes> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(url).send()?;
        let body = res.bytes()?;
        Ok(body)
    }
}
