use std::sync::OnceLock;

use support::envs;
use support::envs::{get_env_b64u_as_u8s, get_env_parse};

/// Returns a reference to the global `AuthConfig` instance.
/// The configuration is initialized once on the first call by loading it from the environment.
///
/// # Panics
///
/// It panics when one or more environment variables related to authentication are unset.
pub fn auth_config() -> &'static AuthConfig {
    static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| AuthConfig::load_from_env().expect("Failed to load auth config"))
}

/// Configuration for authentication mechanisms.
#[allow(non_snake_case)]
pub struct AuthConfig {
    /// The cryptographic key used for password encryption.
    pub PASSWORD_KEY: Vec<u8>,

    /// The cryptographic key used for token encryption.
    pub TOKEN_KEY: Vec<u8>,
    /// The duration (in seconds) for which an authentication token is valid.
    pub TOKEN_DURATION_SEC: f64,
}

impl AuthConfig {
    fn load_from_env() -> envs::Result<AuthConfig> {
        Ok(AuthConfig {
            PASSWORD_KEY: get_env_b64u_as_u8s("RECIPYA_AUTH_PASSWORD_KEY")?,

            TOKEN_KEY: get_env_b64u_as_u8s("RECIPYA_AUTH_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("RECIPYA_AUTH_TOKEN_DURATION_SEC")?,
        })
    }
}
