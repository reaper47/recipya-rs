use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use scraper::{Html, Selector};

use crate::{schema::recipe::RecipeSchema, websites::Website};

pub use self::error::{Error, Result};

mod custom;
mod error;
pub mod schema;
pub mod websites;

pub fn websites() -> &'static HashMap<Website, String> {
    static INSTANCE: OnceLock<HashMap<Website, String>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let data = fs::read_to_string(PathBuf::from(
            "./crates/libs/lib-scraper/tests/websites.json",
        ))
        .unwrap_or_else(|_| fs::read_to_string(PathBuf::from("./tests/websites.json")).unwrap());

        let pre_map: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        let mut map: HashMap<Website, String> = HashMap::new();

        for (key, value) in pre_map.iter() {
            let website = match Website::from(&value[..]) {
                Ok(site) => site,
                Err(_) => {
                    println!("Could not find Website {key}");
                    continue;
                }
            };
            map.insert(website, value.clone());
        }

        map
    })
}

#[async_trait::async_trait]
pub trait HttpClient {
    async fn get_async<'a>(&'a self, host: Website, url: &str) -> Result<String>;
    fn get(&self, host: Website, url: &str) -> Result<String>;
}

struct AppHttpClient {
    client: reqwest::Client,
}

impl AppHttpClient {
    pub fn new() -> Self {
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

pub struct Scraper {
    pub client: Arc<dyn HttpClient + Sync + Send>,
}

impl Scraper {
    pub fn scrape(&self, url: &str) -> Result<RecipeSchema> {
        let host = match Website::from(url) {
            Ok(host) => host,
            Err(err) => return Err(err),
        };
        let content = self.client.get(host, url)?;
        let doc = Html::parse_document(&content);

        let sel = Selector::parse(r#"script[type='application/ld+json']"#)?;
        for el in doc.select(&sel).into_iter() {
            let json = &el.inner_html();
            let recipe: RecipeSchema = match serde_json::from_str(json) {
                Ok(value) => value,
                Err(error) => {
                    println!("Error while parsing schema of {url}: {error}");
                    continue;
                }
            };
            return Ok(recipe);
        }

        Err(Error::DomainNotImplemented)
    }
}
