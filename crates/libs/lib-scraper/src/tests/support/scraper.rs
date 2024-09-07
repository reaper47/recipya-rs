use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use crate::{schema::recipe::RecipeSchema, websites::Website, HttpClient, Scraper};

use super::{websites::websites_for_tests, Result};

pub struct MockHttpClient;

#[async_trait::async_trait]
impl HttpClient for MockHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, _url: &str) -> crate::Result<String> {
        Ok("".to_string())
    }

    fn get(&self, host: Website, _url: &str) -> crate::Result<String> {
        let content = fs::read_to_string(PathBuf::from(format!(
            "./crates/libs/lib-scraper/src/tests/data/{}.html",
            host
        )))
        .unwrap_or_else(|_| {
            fs::read_to_string(PathBuf::from(format!("./src/tests/data/{}.html", host))).unwrap()
        });

        Ok(content)
    }
}

fn mock_scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper {
        client: Arc::new(MockHttpClient),
    })
}

pub fn scrape(website: Website, number: usize) -> Result<RecipeSchema> {
    let url = match websites_for_tests().get(&website) {
        Some(urls) => urls.get(number).expect("url to test not in vector of urls"),
        None => panic!("website '{}' not found in map", website),
    };

    {
        let path1 = PathBuf::from(format!(
            "./crates/libs/lib-scraper/tests/data/{}.html",
            website
        ));
        let path2 = PathBuf::from(format!("./src/tests/data/{}.html", website));

        if !path1.exists() && !path2.exists() {
            let client = reqwest::blocking::Client::new();
            match client.get(url).send() {
                Ok(res) => {
                    fs::File::create(path1)
                        .unwrap_or_else(|_| fs::File::create(path2).unwrap())
                        .write(&res.bytes().unwrap())
                        .inspect_err(|err| println!("Could not write {}: {:?}", website, err))
                        .unwrap();
                }
                Err(err) => println!("Could not fetch {}: {:?}", website, err),
            };
        }
    }

    Ok(mock_scraper().scrape(url)?)
}
