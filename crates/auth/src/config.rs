use std::fs;
use std::sync::OnceLock;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use serde::{Deserialize, Serialize};

use support::fs::get_base_dir;
use tracing::{error, info, warn};

use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    #[serde(skip)]
    pub decoded_password_key: Vec<u8>,
    pub password_key: String,

    #[serde(skip)]
    pub decoded_token_key: Vec<u8>,
    pub token_key: String,

    pub token_duration_sec: f64,
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
    fn load_from_file() -> Result<AuthConfig> {
        let data_dir = get_base_dir()?;
        let config_path = data_dir.join("auth_config.json");

        match fs::read_to_string(&config_path) {
            Ok(config_str) => {
                info!("Loading auth config from file");
                let mut auth: AuthConfig = serde_json::from_str(&config_str)?;
                auth.decoded_password_key = URL_SAFE_NO_PAD
                    .decode(&auth.password_key)
                    .expect("Invalid auth config password key");
                auth.decoded_token_key = URL_SAFE_NO_PAD
                    .decode(&auth.token_key)
                    .expect("Invalid auth config token key");
                Ok(auth)
            }
            Err(err) => {
                warn!("Failed to read auth config: {err}");
                info!("Creating auth config");

                if let Err(err) = fs::create_dir_all(data_dir) {
                    error!("Failed to create parent directories for auth config: {err:?}");
                    return Err(Error::ConfigFileWriteFailed);
                }

                let password_key = URL_SAFE_NO_PAD.encode(generate_key());
                let token_key = URL_SAFE_NO_PAD.encode(generate_key());

                let config = AuthConfig {
                    password_key: password_key.clone(),
                    token_key: token_key.clone(),
                    token_duration_sec: 1800.0,
                    decoded_password_key: URL_SAFE_NO_PAD
                        .decode(password_key)
                        .expect("Valid auth config password key"),
                    decoded_token_key: URL_SAFE_NO_PAD
                        .decode(token_key)
                        .expect("Valid auth config token key"),
                };

                match fs::write(config_path, serde_json::to_string_pretty(&config)?) {
                    Ok(_) => Ok(config),
                    Err(err) => {
                        error!("Failed to write auth config to file: {err:?}");
                        Err(Error::ConfigFileWriteFailed)
                    }
                }
            }
        }
    }
}

fn generate_key() -> [u8; 64] {
    let mut rng = rand::rng();
    let mut key = [0u8; 64];
    rng.fill(&mut key);
    key
}
