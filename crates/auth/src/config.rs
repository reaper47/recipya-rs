use std::fs;
use std::sync::OnceLock;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use support::fs::get_base_dir;
use tracing::{error, info, warn};

use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    #[serde(rename = "jwtSecret")]
    pub jwt_secret: String,
    #[serde(rename = "tokenDurationSec")]
    pub token_duration_sec: f64,

    #[serde(skip)]
    pub decoded_password_key: Vec<u8>,
    #[serde(rename = "passwordKey")]
    pub password_key: String,

    #[serde(skip)]
    pub decoded_token_key: Vec<u8>,
    #[serde(rename = "tokenKey")]
    pub token_key: String,
}

/// Returns a reference to the global `AuthConfig` instance.
/// The configuration is initialized once on the first call by loading it from the environment.
///
/// # Panics
///
/// It panics when one or more environment variables related to authentication are unset.
pub fn auth_config() -> &'static AuthConfig {
    static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| AuthConfig::load_from_file().expect("Failed to load auth config"))
}

impl AuthConfig {
    fn load_from_file() -> Result<Self> {
        let data_dir = get_base_dir()?;
        let config_path = data_dir.join("auth_config.json");

        if let Ok(s) = fs::read_to_string(&config_path) {
            info!("Loading auth config from file");
            return Self::from_str(&s);
        }

        warn!("Auth config not found, generating a new one");
        fs::create_dir_all(&data_dir).map_err(|err| {
            error!("Failed to create auth config directory: {err}");
            Error::ConfigFileWriteFailed
        })?;

        if let Ok(s) = fs::read_to_string(&config_path) {
            info!("Auth config was created by another process, loading it");
            return Self::from_str(&s);
        }

        let config = Self::generate();
        let json = serde_json::to_string_pretty(&config)?;
        let tmp_path = config_path.with_extension("tmp");

        fs::write(&tmp_path, &json).map_err(|err| {
            error!("Failed to write auth config: {err}");
            Error::ConfigFileWriteFailed
        })?;

        if let Err(err) = fs::rename(&tmp_path, &config_path) {
            error!("Failed to rename auth config (race condition): {err}");
            let s = fs::read_to_string(&config_path).map_err(|_| Error::ConfigFileWriteFailed)?;
            return Self::from_str(&s);
        }

        Ok(config)
    }

    fn from_str(s: &str) -> Result<Self> {
        let auth: Self = serde_json::from_str(s)?;
        Ok(auth.decode_keys())
    }

    fn decode_keys(mut self) -> Self {
        self.decoded_password_key = URL_SAFE_NO_PAD
            .decode(&self.password_key)
            .expect("Invalid auth config password key");
        self.decoded_token_key = URL_SAFE_NO_PAD
            .decode(&self.token_key)
            .expect("Invalid auth config token key");
        self
    }

    fn generate() -> Self {
        let password_key = URL_SAFE_NO_PAD.encode(generate_key());
        let token_key = URL_SAFE_NO_PAD.encode(generate_key());
        Self {
            jwt_secret: URL_SAFE_NO_PAD.encode(generate_key()),
            decoded_password_key: URL_SAFE_NO_PAD
                .decode(&password_key)
                .expect("Valid password key"),
            decoded_token_key: URL_SAFE_NO_PAD.decode(&token_key).expect("Valid token key"),
            password_key,
            token_key,
            token_duration_sec: 1800.0,
        }
    }
}

fn generate_key() -> [u8; 64] {
    let mut rng = rand::rng();
    let mut key = [0u8; 64];
    rng.fill(&mut key);
    key
}
