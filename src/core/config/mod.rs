mod data;
mod error;

pub use data::{DataDir, get_base_dir};
pub use error::{Error, Result};

use std::env;

/// Configuration struct for the application.
#[derive(PartialEq, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub database_url: String,
    pub is_autologin: bool,
    pub is_demo: bool,
    pub is_no_signups: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8078".into(),
            database_url: "postgres://postgres:postgres@localhost:5432/recipya".into(),
            is_autologin: false,
            is_demo: false,
            is_no_signups: false,
        }
    }
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    pub fn load_from_env() -> crate::core::config::Result<Self> {
        Ok(Self {
            base_url: get_env_on_load("RECIPYA_BASE_URL")?,
            database_url: get_env_on_load("RECIPYA_DATABASE_URL")?,
            is_autologin: get_env_on_load("RECIPYA_IS_AUTOLOGIN")? == "true",
            is_demo: get_env_on_load("RECIPYA_IS_DEMO")? == "true",
            is_no_signups: get_env_on_load("RECIPYA_IS_NO_SIGNUPS")? == "true",
        })
    }
}

fn get_env_on_load(name: &'static str) -> crate::core::config::Result<String> {
    match env::var(name) {
        Ok(v) => {
            let trimmed = v.trim_matches('"');
            if trimmed.is_empty() {
                Err(Error::MissingEnv(name))
            } else {
                Ok(trimmed.to_string())
            }
        }
        Err(_) => Err(Error::MissingEnv(name)),
    }
}
