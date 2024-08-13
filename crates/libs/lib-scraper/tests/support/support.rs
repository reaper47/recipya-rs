use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use lib_scraper::{schema::recipe::RecipeSchema, websites::Website, HttpClient, Result, Scraper};

use super::websites::websites_for_tests;

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

pub fn scrape(website: Website, number: usize) -> RecipeSchema {
    let url = match websites_for_tests().get(&website) {
        Some(urls) => urls.get(number).expect("url to test not in vector of urls"),
        None => panic!("website '{}' not found in map", website),
    };

    {
        let file_name = format!("./crates/libs/lib-scraper/tests/data/{}.html", website);
        let path = Path::new(file_name.as_str());

        let file_name = format!("./tests/data/{}.html", website);
        let path2 = Path::new(file_name.as_str());

        if !path.exists() && !path2.exists() {
            let client = reqwest::blocking::Client::new();
            match client.get(url).send() {
                Ok(res) => {
                    fs::File::create(path)
                        .unwrap_or_else(|_| {
                            fs::File::create(format!("./tests/data/{}.html", website)).unwrap()
                        })
                        .write(&res.bytes().unwrap())
                        .inspect_err(|err| println!("Could not write {}: {:?}", website, err))
                        .unwrap();
                }
                Err(err) => println!("Could not fetch {}: {:?}", website, err),
            };
        }
    }

    mock_scraper().scrape(url).unwrap()
}
