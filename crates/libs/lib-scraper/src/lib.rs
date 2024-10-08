mod custom;
mod error;
pub mod schema;
mod websites;

use scraper::{Html, Selector};
use std::sync::{Arc, OnceLock};

use crate::{
    schema::{
        recipe::{GraphObject, RecipeSchema},
        AtType,
    },
    websites::Website,
};

pub use self::error::{Error, Result};

#[async_trait::async_trait]
trait HttpClient {
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

fn scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper {
        client: Arc::new(AppHttpClient::new()),
    })
}

pub fn scrape(url: impl Into<String>) -> Result<RecipeSchema> {
    scraper().scrape(&url.into())
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

struct Scraper {
    client: Arc<dyn HttpClient + Sync + Send>,
}

impl Scraper {
    fn scrape(&self, url: &str) -> Result<RecipeSchema> {
        let doc = match Website::from(url) {
            Ok(host) => {
                let content = self.client.get(host, url)?;
                Html::parse_document(&content)
            }
            Err(err) => return Err(err),
        };

        let sel = Selector::parse(r#"script[type='application/ld+json']"#)?;
        doc.select(&sel)
            .filter_map(|el| {
                let json = &el.inner_html();
                match serde_json::from_str::<RecipeSchema>(json) {
                    Ok(recipe) => Some(recipe),
                    Err(error) => {
                        println!(
                            "Error while parsing schema: {error}\nURL: {url}\nJSON: {json}\n-----"
                        );
                        None
                    }
                }
            })
            .find_map(|recipe| match recipe.at_graph {
                None => Some(recipe),
                Some(graph) => graph.into_iter().find_map(|temp| {
                    if let GraphObject::Recipe(mut recipe) = temp {
                        recipe.at_type = Some(AtType::Recipe);
                        Some(*recipe)
                    } else {
                        None
                    }
                }),
            })
            .ok_or_else(|| Error::DomainNotImplemented)
    }
}

#[cfg(test)]
mod tests {
    mod support;
    mod tests_scraper_0_to_9;
    mod tests_scraper_a;
    mod tests_scraper_b;
    mod tests_scraper_c;
    mod tests_scraper_d;
    mod tests_scraper_e;
    mod tests_scraper_f;
    mod tests_scraper_g;
    mod tests_scraper_h;
    mod tests_scraper_i;
    mod tests_scraper_j;
    mod tests_scraper_k;
    mod tests_scraper_l;
    mod tests_scraper_m;
    mod tests_scraper_n;
    mod tests_scraper_o;
    mod tests_scraper_p;
    mod tests_scraper_q;
    mod tests_scraper_r;
    mod tests_scraper_s;
    mod tests_scraper_t;
    mod tests_scraper_u;
    mod tests_scraper_v;
    mod tests_scraper_w;
    mod tests_scraper_y;
    mod tests_scraper_z;
}
