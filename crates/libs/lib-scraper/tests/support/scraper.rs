use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use lib_scraper::{schema::recipe::RecipeSchema, websites::Website, Scraper};

use crate::support::MockHttpClient;

use super::{Result, websites::websites_for_tests};

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
        let path2 = PathBuf::from(format!("./tests/data/{}.html", website));

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
