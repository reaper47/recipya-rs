mod data;
mod error;
mod states;

pub use data::DataDir;
pub use error::{Error, Result};
pub use states::{AutologinState, DemoState, ProductionState, SignupsState, States};

use std::env;

use tracing::warn;

/// Configuration struct for the application.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub database_url: String,
    pub states: States,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8078".into(),
            database_url: "postgres://postgres:postgres@localhost:5432/recipya".into(),
            states: States::default(),
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
            states: States {
                autologin: AutologinState::from(get_env_on_load("RECIPYA_IS_AUTOLOGIN")? == "true"),
                demo: DemoState::from(get_env_on_load("RECIPYA_IS_DEMO")? == "true"),
                signups: SignupsState::from(get_env_on_load("RECIPYA_IS_ALLOW_SIGNUPS")? == "true"),
                production: ProductionState::from(
                    get_env_on_load("RECIPYA_IS_PRODUCTION")? == "true",
                ),
            },
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
