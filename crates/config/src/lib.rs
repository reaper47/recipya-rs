mod data;
mod error;

pub use data::DataDir;
pub use error::{Error, Result};
use tracing::warn;

use std::env;

/// Configuration struct for the application.
#[derive(Eq, PartialEq, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct Config {
    pub base_url: String,
    pub database_url: String,
    pub is_autologin: bool,
    pub is_demo: bool,
    pub is_no_signups: bool,
    pub is_production: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8078".into(),
            database_url: "postgres://postgres:postgres@localhost:5432/recipya".into(),
            is_autologin: false,
            is_demo: false,
            is_no_signups: false,
            is_production: false,
        }
    }
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    pub fn load_from_env() -> Result<Self> {
        let database_url = {
            let base = get_env_on_load("DATABASE_URL")?
                .trim_end_matches('/')
                .to_string();

            if base.ends_with("/recipya") {
                base
            } else {
                format!("{base}/recipya")
            }
        };

        Ok(Self {
            base_url: get_env_on_load("RECIPYA_BASE_URL")?,
            database_url,
            is_autologin: get_env_on_load("RECIPYA_IS_AUTOLOGIN")? == "true",
            is_demo: get_env_on_load("RECIPYA_IS_DEMO")? == "true",
            is_no_signups: get_env_on_load("RECIPYA_IS_ALLOW_SIGNUPS")? == "false",
            is_production: get_env_on_load("RECIPYA_IS_PRODUCTION")? == "true",
        })
    }
}

fn get_env_on_load(name: &'static str) -> Result<String> {
    env::var(name).map_or_else(
        |_| {
            warn!("Missing environment variable: {name} (default set)");
            match name {
                "RECIPYA_IS_ALLOW_SIGNUPS"
                | "RECIPYA_IS_AUTOLOGIN"
                | "RECIPYA_IS_DEMO"
                | "RECIPYA_IS_PRODUCTION" => Ok("false".into()),
                "RUST_LOG" => Ok("debug,tokio_cron_scheduler=off,reqwest=warn,hyper=warn".into()),
                _ => Err(Error::MissingEnv(name)),
            }
        },
        |v| {
            let trimmed = v.trim_matches('"');
            if trimmed.is_empty() {
                Err(Error::MissingEnv(name))
            } else {
                Ok(trimmed.to_string())
            }
        },
    )
}
