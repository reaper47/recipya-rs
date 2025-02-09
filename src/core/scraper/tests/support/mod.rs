mod scraper;
mod websites;

pub use scraper::*;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
