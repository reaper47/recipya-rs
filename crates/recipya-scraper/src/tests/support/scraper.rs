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
use wreq::Client;
use wreq_util::Emulation;

use support::fs::MockFs;

use crate::{ENABLE_JS, client::HttpClient};
use crate::{FORBIDDEN, websites::Website};
use crate::{Result, Scraper};

/// A mock HTTP client for use in tests to avoid sending real HTTP requests.
pub struct MockHttpClient;

#[async_trait::async_trait]
impl HttpClient for MockHttpClient {
    async fn get_async<'a>(&'a self, host: Website, url: &str) -> Result<String> {
        let path = get_html_file_path(
            host,
            url.rsplit_once("<number>")
                .unwrap_or(("0", "0"))
                .1
                .parse()
                .unwrap_or(0),
        );
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

    fn get(&self, _host: Website, _url: &str) -> Result<String> {
        unimplemented!()
    }

    async fn get_bytes(&self, _url: &str) -> Result<Bytes> {
        Ok(Bytes::new())
    }
}

/// A mock scraper initialised with the mock HTTP client for use in the scraper tests.
fn mock_scraper() -> &'static Scraper {
    static INSTANCE: OnceLock<Scraper> = OnceLock::new();

    INSTANCE.get_or_init(|| Scraper::with_client(Arc::new(MockHttpClient), Arc::new(MockFs)))
}

/// Fetches the recipe from a website and stores the content in an HTML file.
///
/// # Panics
///
/// Panics if:
/// - The `number` index is out of bounds for the website's test URLs
/// - The HTML file cannot be created at the specified path
/// - The response bytes cannot be written to the file
pub async fn scrape(website: Website, number: usize) -> Result<Recipe> {
    let urls = website.test_urls();
    let url = urls.get(number).expect("url to test not in vector of urls");

    let path = get_html_file_path(website, number);

    if !path.exists() {
        fs::File::create(path)
            .unwrap()
            .write(&fetch_html(url).await?)
            .inspect_err(|err| error!("Could not write {website}: {err}"))
            .unwrap();
    }

    let url = format!("{url}<number>{number}");
    mock_scraper().scrape(&url).await
}

async fn fetch_html(url: &str) -> Result<Bytes> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let is_client_error = res.status().is_client_error();

    let bytes = res.bytes().await?;
    let bytes_vec = bytes.to_vec();
    let text = String::from_utf8_lossy(&bytes_vec);

    if is_client_error || text.contains(ENABLE_JS) || text.contains(FORBIDDEN) {
        let client = Client::builder().emulation(Emulation::Chrome145).build()?;
        let resp = client.get(url).send().await?;
        Ok(resp.bytes().await?)
    } else {
        Ok(bytes)
    }
}

fn get_html_file_path(website: Website, number: usize) -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    let fname = &format!(
        "{}_{number}.html",
        website.to_string().trim_end_matches('/')
    );

    for part in &[
        "crates",
        "recipya-scraper",
        "src",
        "tests",
        "data",
        "html",
        fname,
    ] {
        path = path.join(part);
    }

    let path1_from: PathBuf = ["crates", "recipya-scraper", "crates", "recipya-scraper"]
        .iter()
        .collect();
    let path1_to: PathBuf = ["crates", "recipya-scraper"].iter().collect();
    let path2_from: PathBuf = ["crates", "router", "crates", "recipya-scraper"]
        .iter()
        .collect();

    path.to_string_lossy()
        .replace(
            path1_from.to_string_lossy().as_ref(),
            path1_to.to_string_lossy().as_ref(),
        )
        .replace(
            path2_from.to_string_lossy().as_ref(),
            path1_to.to_string_lossy().as_ref(),
        )
        .into()
}

/// Scrapes some test websites for use in tests outside the scraper.
///
/// # Panics
///
/// Panics if:
/// - The selected website has no test URLs configured
/// - The HTML file cannot be created
/// - The response bytes cannot be written to the file
#[allow(unused)]
pub async fn scrape_test_websites(number: usize) -> Result<()> {
    let website = match number {
        1 => Website::Zweigles,
        2 => Website::ZumaValley,
        3 => Website::ZsuzsaIsInTheKitchen,
        _ => Website::Zenbelly,
    };

    let url = website
        .test_urls()
        .first()
        .copied()
        .expect("url to test not in vector of urls");

    let path = get_html_file_path(website, 0);

    if !path.exists() {
        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(res) => {
                dbg!(&path);
                fs::File::create(path)
                    .unwrap()
                    .write(&res.bytes().await?)
                    .inspect_err(|err| error!("Could not write {website}: {err}"))
                    .unwrap();
            }
            Err(err) => {
                error!("Could not fetch {website}: {err}");
            }
        }
    }

    Ok(())
}
