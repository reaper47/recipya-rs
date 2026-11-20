mod data;
mod error;
mod states;

pub use data::DataDir;
pub use error::{Error, Result};
pub use states::{AppEnvState, AutologinState, DemoState, SignupsState, States};

use std::{borrow::Cow, env};

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
            database_url: "postgres://postgres:postgres@localhost:5432".into(),
            states: States::default(),
        }
    }
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            base_url: get_env_on_load("RECIPYA_BASE_URL")?.to_string(),
            database_url: format!("{}/recipya", get_env_on_load("DATABASE_URL")?),
            states: States {
                autologin: AutologinState::from(get_env_on_load("RECIPYA_AUTOLOGIN")? == "true"),
                demo: DemoState::from(get_env_on_load("RECIPYA_IS_DEMO")? == "true"),
                signups: SignupsState::from(get_env_on_load("RECIPYA_ALLOW_SIGNUPS")? == "true"),
                app_env: AppEnvState::from(get_env_on_load("RECIPYA_APP_ENV")?.as_ref()),
            },
        })
    }

    /// Returns whether the application's environment is set to production.
    pub fn is_env_prod(&self) -> bool {
        self.states.app_env == AppEnvState::Prod
    }
}

fn get_env_on_load<'a>(name: &'static str) -> Result<Cow<'a, str>> {
    env::var(name).map_or_else(
        |_| {
            println!("Missing environment variable: {name} (default set)");
            match name {
                "RECIPYA_ALLOW_SIGNUPS" | "RECIPYA_AUTOLOGIN" | "RECIPYA_IS_DEMO" => {
                    Ok(Cow::Borrowed("false"))
                }
                "RUST_LOG" => Ok(Cow::Borrowed(
                    "debug,tokio_cron_scheduler=off,reqwest=warn,hyper=warn",
                )),
                _ => Err(Error::MissingEnv(name)),
            }
        },
        |v| {
            let trimmed = v.trim_matches('"').to_string();
            if trimmed.is_empty() {
                Err(Error::MissingEnv(name))
            } else {
                Ok(Cow::Owned(trimmed))
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_env_prod_when_dev() {
        let mut config = Config::default();
        config.states.app_env = AppEnvState::Dev;

        assert!(!config.is_env_prod())
    }

    #[test]
    fn test_is_env_prod_when_test() {
        let mut config = Config::default();
        config.states.app_env = AppEnvState::Test;

        assert!(!config.is_env_prod())
    }

    #[test]
    fn test_is_env_prod_when_prod() {
        let mut config = Config::default();
        config.states.app_env = AppEnvState::Prod;

        assert!(config.is_env_prod())
    }
}
