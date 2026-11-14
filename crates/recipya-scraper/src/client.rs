use std::io::Read;

use async_trait::async_trait;
use axum::body::Bytes;
use flate2::read::GzDecoder;
use support::net::SsrfSafeClient;
use tracing::error;
use url::Url;

use crate::error::fetch_err;
use crate::websites::Website;
use crate::{ENABLE_JS, Error, FORBIDDEN, Result};

/// A trait defining HTTP client functionality for synchronous and asynchronous requests.
#[async_trait]
pub trait HttpClient {
    /// Performs an asynchronous HTTP GET request.
    async fn get<'a>(&'a self, host: Website, url: &str) -> Result<String>;

    /// Fetches the content from a URL and uploads it to the temporary directory.
    async fn get_bytes(&self, url: &str) -> Result<Bytes>;
}

/// A wrapper around `reqwest::Client` for making HTTP requests.
pub struct AppHttpClient {
    client: SsrfSafeClient,
}

impl Default for AppHttpClient {
    fn default() -> Self {
        Self {
            client: SsrfSafeClient::new().expect("wreq client to be initialized"),
        }
    }
}

#[async_trait::async_trait]
impl HttpClient for AppHttpClient {
    async fn get<'a>(&'a self, host: Website, url: &str) -> Result<String> {
        let url = match host {
            Website::AllRecipes => {
                if url.ends_with('/') {
                    url
                } else {
                    &format!("{url}/")
                }
            }
            _ => url,
        };

        let parsed_url = Url::parse(url)?;
        let res = self
            .client
            .get(parsed_url.clone())
            .await
            .map_err(fetch_err)?;

        if !res.status().is_success() {
            let status = res.status();
            let bytes = res.bytes().await.map_err(fetch_err)?;
            error!(?url, ?status, body = ?bytes, "Failed to scrape");
            return Err(Error::Fetch(format!("HTTP error: {status}")));
        }

        let bytes_vec = res.bytes().await.map_err(fetch_err)?.to_vec();

        let text = {
            let initial = String::from_utf8_lossy(&bytes_vec);

            if initial.contains(ENABLE_JS) || initial.contains(FORBIDDEN) {
                let res = self
                    .client
                    .get(parsed_url.clone())
                    .await
                    .map_err(fetch_err)?;

                if res.status().is_redirection() {
                    let Some(location) = res
                        .headers()
                        .get("location")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| parsed_url.join(s).ok())
                    else {
                        return Err(Error::InvalidUrl);
                    };

                    self.client
                        .get(location)
                        .await
                        .map_err(fetch_err)?
                        .text()
                        .await
                        .map_err(fetch_err)?
                } else {
                    res.text().await.map_err(fetch_err)?
                }
            } else {
                initial.into_owned()
            }
        };

        let bytes = text.as_bytes();
        if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
            let mut d = GzDecoder::new(bytes);
            let mut s = String::new();
            d.read_to_string(&mut s).unwrap();
            Ok(s)
        } else {
            Ok(text)
        }
    }

    async fn get_bytes(&self, url: &str) -> Result<Bytes> {
        let parsed_url = Url::parse(url)?;
        let res = self.client.get(parsed_url).await.map_err(fetch_err)?;
        let body = res.bytes().await.map_err(fetch_err)?;
        Ok(body)
    }
}
