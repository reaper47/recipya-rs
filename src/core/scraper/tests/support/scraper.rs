use tracing::error;

use super::websites::websites_for_tests;
use crate::core::scraper::Result;
use crate::core::scraper::Scraper;
use crate::core::scraper::client::HttpClient;
use crate::core::scraper::schema::RecipeSchema;
use crate::core::scraper::websites::Website;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

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
}

fn mock_scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper::with_client(Arc::new(MockHttpClient)))
}

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

    Ok(mock_scraper().scrape(url)?)
}
