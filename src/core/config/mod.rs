mod error;

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
            base_url: String::from("http://localhost:8078"),
            database_url: String::from("postgres://postgres:postgres@localhost:5432/recipya"),
            is_autologin: false,
            is_demo: false,
            is_no_signups: false,
        }
    }
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            base_url: get_env_on_load("RECIPYA_BASE_URL")?,
            database_url: get_env_on_load("RECIPYA_DATABASE_URL")?,
            is_autologin: get_env_on_load("RECIPYA_IS_AUTOLOGIN")? == "true",
            is_demo: get_env_on_load("RECIPYA_IS_DEMO")? == "true",
            is_no_signups: get_env_on_load("RECIPYA_IS_NO_SIGNUPS")? == "true",
        })
    }
}

fn get_env_on_load(name: &'static str) -> Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn set_vars() {
        env::set_var("RECIPYA_IS_AUTOLOGIN", "true");
        env::set_var("RECIPYA_IS_DEMO", "true");
        env::set_var("RECIPYA_IS_NO_SIGNUPS", "true");
    }

    fn remove_vars() {
        env::remove_var("RECIPYA_IS_AUTOLOGIN");
        env::remove_var("RECIPYA_IS_DEMO");
        env::remove_var("RECIPYA_IS_NO_SIGNUPS");
    }

    fn config() -> Result<Config> {
        Ok(Config::load_from_env()?)
    }

    #[test]
    fn test_config_ok() -> Result<()> {
        set_vars();

        let config = config()?;
        remove_vars();

        pretty_assertions::assert_eq!(config.is_autologin, true);
        pretty_assertions::assert_eq!(config.is_demo, true);
        pretty_assertions::assert_eq!(config.is_no_signups, true);
        Ok(())
    }
}
