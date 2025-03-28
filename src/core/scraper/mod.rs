pub mod schema;
pub mod websites;

mod client;
mod error;

pub use error::{Error, Result};

use std::sync::{Arc, OnceLock};

use scraper::{Html, Selector};
use tracing::error;

use crate::core::scraper::client::{AppHttpClient, HttpClient};
use crate::core::scraper::schema::{AtType, GraphObject, RecipeSchema};
use crate::core::scraper::websites::Website;

/// Returns a static reference to the global `Scraper` instance.
fn scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper {
        client: Arc::new(AppHttpClient::new()),
    })
}

/// Scrapes the given URL and returns a `RecipeSchema`.
pub fn scrape(url: impl Into<String>) -> Result<RecipeSchema> {
    scraper().scrape(&url.into())
}

struct Scraper {
    client: Arc<dyn HttpClient + Send + Sync>,
}

impl Scraper {
    fn scrape(&self, url: &str) -> Result<RecipeSchema> {
        let content = self.client.get(Website::from(url)?, url)?;
        let doc = Html::parse_document(&content);

        let sel = Selector::parse(r#"script[type='application/ld+json']"#)?;
        doc.select(&sel)
            .filter_map(|el| {
                let json = &el.inner_html();
                match serde_json::from_str::<RecipeSchema>(json) {
                    Ok(recipe) => Some(recipe),
                    Err(error) => {
                        error!(
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
            .ok_or(Error::DomainNotImplemented)
    }
}

/*
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
*/
