mod scraper;
mod websites;

use std::{fs, path::PathBuf};

use lib_scraper::{websites::Website, HttpClient};

pub use scraper::*;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub struct MockHttpClient;

#[async_trait::async_trait]
impl HttpClient for MockHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, _url: &str) -> lib_scraper::Result<String> {
        Ok("".to_string())
    }

    fn get(&self, host: Website, _url: &str) -> lib_scraper::Result<String> {
        let content = fs::read_to_string(PathBuf::from(format!(
            "./crates/libs/lib-scraper/tests/data/{}.html",
            host
        )))
        .unwrap_or_else(|_| {
            fs::read_to_string(PathBuf::from(format!("./tests/data/{}.html", host))).unwrap()
        });

        Ok(content)
    }
}
