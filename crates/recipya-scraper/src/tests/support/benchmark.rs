use std::{collections::HashMap, fs, io::Read, path::Path};

use async_trait::async_trait;
use axum::body::Bytes;
use flate2::read::GzDecoder;

use crate::{HttpClient, Result, Website};

pub struct InMemoryClient {
    pub pages: HashMap<String, String>,
}

impl InMemoryClient {
    pub fn new(url: &str, html: &str) -> Self {
        Self {
            pages: HashMap::from([(url.to_string(), html.to_string())]),
        }
    }
}

#[async_trait]
impl HttpClient for InMemoryClient {
    async fn get<'a>(&'a self, _host: Website, url: &str) -> Result<String> {
        Ok(self.pages[url].clone())
    }

    async fn get_bytes(&self, _url: &str) -> Result<Bytes> {
        Ok(Bytes::new())
    }
}

pub fn load_fixture(website: Website, number: usize) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tests")
        .join("data")
        .join("html")
        .join(format!(
            "{}_{number}.html",
            website.to_string().trim_end_matches('/')
        ));

    let bytes = fs::read(&path).unwrap();

    if bytes.starts_with(&[0x1f, 0x8b]) {
        let mut html = String::new();
        GzDecoder::new(&bytes[..])
            .read_to_string(&mut html)
            .unwrap();
        html
    } else {
        String::from_utf8_lossy(&bytes).into_owned()
    }
}
