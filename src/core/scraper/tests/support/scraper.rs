use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use axum::body::Bytes;
use tracing::error;

use super::websites::websites_for_tests;
use crate::core::scraper::Result;
use crate::core::scraper::Scraper;
use crate::core::scraper::client::HttpClient;
use crate::core::scraper::schema::RecipeSchema;
use crate::core::scraper::websites::Website;
use crate::core::support::fs::MockFs;

/// A mock HTTP client for use in tests to avoid sending real HTTP requests.
pub struct MockHttpClient;

#[async_trait::async_trait]
impl HttpClient for MockHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, _url: &str) -> Result<String> {
        Ok("".to_string())
    }

    fn get(&self, host: Website, _url: &str) -> Result<String> {
        let content = fs::read_to_string(PathBuf::from(format!(
            "./src/core/scraper/tests/data/{host}.html"
        )))
        .unwrap();

        Ok(content)
    }

    fn get_bytes(&self, _url: &str) -> Result<Bytes> {
        Ok(Bytes::new())
    }
}

/// A mock scraper initialised with the mock HTTP client for use in the scraper tests.
fn mock_scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper::with_client(Arc::new(MockHttpClient), Arc::new(MockFs)))
}

/// Fetches the recipe from a website and stores the content in an HTML file.
pub fn scrape(website: Website, number: usize) -> Result<RecipeSchema> {
    let url = match websites_for_tests().get(&website) {
        Some(urls) => urls.get(number).expect("url to test not in vector of urls"),
        None => panic!("website '{website}' not found in map"),
    };

    let path = PathBuf::from(format!("./src/core/scraper/tests/data/{website}.html"));

    if !path.exists() {
        let client = reqwest::blocking::Client::new();
        match client.get(url).send() {
            Ok(res) => {
                fs::File::create(path)
                    .unwrap()
                    .write(&res.bytes()?)
                    .inspect_err(|err| error!("Could not write {}: {:?}", website, err))
                    .unwrap();
            }
            Err(err) => error!("Could not fetch {}: {:?}", website, err),
        };
    }

    mock_scraper().scrape(url)
}

/// Scrapes some test websites for use in tests outside the scraper.
#[allow(unused)]
pub async fn scrape_test_websites(number: usize) -> Result<()> {
    let website = match number {
        1 => Website::AllRecipesDotCom,
        2 => Website::ACoupleCooksDotCom,
        3 => Website::AfghanKitchenRecipesDotCom,
        _ => Website::AfghanKitchenRecipesDotCom,
    };

    let url = match websites_for_tests().get(&website) {
        Some(urls) => urls.get(0).expect("url to test not in vector of urls"),
        None => panic!("website '{website}' not found in map"),
    };

    let path = PathBuf::from(format!("./src/core/scraper/tests/data/{website}.html"));

    if !path.exists() {
        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(res) => {
                fs::File::create(path)
                    .unwrap()
                    .write(&res.bytes().await?)
                    .inspect_err(|err| error!("Could not write {}: {:?}", website, err))
                    .unwrap();
            }
            Err(err) => error!("Could not fetch {}: {:?}", website, err),
        };
    }

    Ok(())
}
