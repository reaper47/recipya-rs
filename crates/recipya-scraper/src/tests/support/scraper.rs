use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use axum::body::Bytes;
use flate2::read::GzDecoder;
use schema_org::Recipe;
use tracing::error;

use support::fs::MockFs;

use crate::client::HttpClient;
use crate::websites::Website;
use crate::{Result, Scraper};

/// A mock HTTP client for use in tests to avoid sending real HTTP requests.
pub struct MockHttpClient;

const BASE_HTML_DIR: &str = "crates/recipya-scraper/src/tests/data/html";

#[async_trait::async_trait]
impl HttpClient for MockHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, _url: &str) -> Result<String> {
        Ok("".to_string())
    }

    fn get(&self, host: Website, _url: &str) -> Result<String> {
        let path = get_html_file_path(&host);
        let bytes = fs::read(&path).unwrap();

        if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
            let mut d = GzDecoder::new(&bytes[..]);
            let mut s = String::new();
            d.read_to_string(&mut s).unwrap();
            Ok(s)
        } else {
            let content = String::from_utf8_lossy(&bytes).to_string();
            Ok(content)
        }
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
pub fn scrape(website: Website, number: usize) -> Result<Recipe> {
    let urls = website.test_urls();
    let url = urls.get(number).expect("url to test not in vector of urls");

    let path = get_html_file_path(&website);

    if !path.exists() {
        let client = reqwest::blocking::Client::new();
        match client.get(*url).send() {
            Ok(res) => {
                fs::File::create(path)
                    .unwrap()
                    .write(&res.bytes()?)
                    .inspect_err(|err| error!("Could not write {website}: {err}"))
                    .unwrap();
            }
            Err(err) => error!("Could not fetch {website}: {err}"),
        };
    }

    mock_scraper().scrape(url)
}

fn get_html_file_path(website: &Website) -> PathBuf {
    let path = std::env::current_dir()
        .unwrap()
        .join(format!("{BASE_HTML_DIR}/{website}.html"));

    let path_str = path
        .to_string_lossy()
        .replace(
            "/crates/recipya-scraper/crates/recipya-scraper",
            "/crates/recipya-scraper",
        )
        .replace(
            "/crates/router/crates/recipya-scraper",
            "/crates/recipya-scraper",
        );

    PathBuf::from(path_str)
}

/// Scrapes some test websites for use in tests outside the scraper.
#[allow(unused)]
pub async fn scrape_test_websites(number: usize) -> Result<()> {
    let website = match number {
        1 => Website::AllRecipes,
        2 => Website::Zeezest,
        3 => Website::Zenbelly,
        _ => Website::Zenbelly,
    };

    let url = website
        .test_urls()
        .get(0)
        .cloned()
        .expect("url to test not in vector of urls");

    let path = get_html_file_path(&website);

    if !path.exists() {
        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(res) => {
                fs::File::create(path)
                    .unwrap()
                    .write(&res.bytes().await?)
                    .inspect_err(|err| error!("Could not write {website}: {err:?}"))
                    .unwrap();
            }
            Err(err) => {
                error!("Could not fetch {website}: {err:?}")
            }
        };
    }

    Ok(())
}
