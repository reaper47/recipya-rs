use std::{
    fs,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use lib_scraper::{schema::recipe::RecipeSchema, websites::Website, HttpClient, Result, Scraper};

pub fn mock_scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper {
        client: Arc::new(MockHttpClient),
    })
}

pub struct MockHttpClient;

#[async_trait::async_trait]
impl HttpClient for MockHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, _url: &str) -> Result<String> {
        Ok("".to_string())
    }
    fn get(&self, host: Website, _url: &str) -> Result<String> {
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

pub fn scrape(website: Website) -> RecipeSchema {
    let url = lib_scraper::websites().get(&website).unwrap();
    mock_scraper().scrape(url).unwrap()
}
