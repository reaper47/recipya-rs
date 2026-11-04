pub mod scraper;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
