use std::io::Read;

use async_trait::async_trait;
use axum::body::Bytes;
use flate2::read::GzDecoder;
use tracing::error;
use wreq::header::LOCATION;
use wreq_util::Emulation;

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
    client: reqwest::Client,
    client_wreq: wreq::Client,
}

impl Default for AppHttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::default(),
            client_wreq: wreq::Client::builder()
                .emulation(Emulation::Chrome145)
                .build()
                .expect("wreq client to be initialized"),
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

        let res = self.client.get(url).send().await?;
        let bytes_vec = if res.status().is_success() {
            let bytes = res.bytes().await?;
            bytes.to_vec()
        } else {
            let wres = self.client_wreq.get(url).send().await?;
            if !wres.status().is_success() {
                let status = wres.status();
                let bytes = wres.bytes().await?;
                error!(?url, ?status, body = ?bytes, "Failed to scrape");
                return Err(Error::Fetch(format!("HTTP error: {status}")));
            }

            let bytes = wres.bytes().await?;
            bytes.to_vec()
        };

        let text = {
            let initial = String::from_utf8_lossy(&bytes_vec);

            if initial.contains(ENABLE_JS) || initial.contains(FORBIDDEN) {
                let res = self.client_wreq.get(url).send().await?;
                if res.status().is_redirection() {
                    let location = res
                        .headers()
                        .get(LOCATION)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");

                    self.client_wreq.get(location).send().await?.text().await?
                } else {
                    res.text().await?
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
        let res = self.client.get(url).send().await?;
        let body = res.bytes().await?;
        Ok(body)
    }
}
