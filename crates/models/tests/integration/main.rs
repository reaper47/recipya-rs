mod data;
mod download;
mod export;
mod language;
mod paper;
mod recipe;
mod reports;
mod settings;
mod share;
mod shopping;
mod theme;
mod token;
mod user;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
