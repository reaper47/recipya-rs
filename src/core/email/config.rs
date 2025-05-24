use std::sync::OnceLock;

use crate::core::support::envs::get_env;

/// Configuration struct for the email client.
#[derive(PartialEq, Debug)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub email_admin: String,
}

/// Gets the current email `Config` struct. It will be initialized if not already done.
pub fn email_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| Config::load_from_env())
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    pub fn load_from_env() -> Self {
        Self {
            smtp_host: get_env("RECIPYA_EMAIL_SMTP_HOST").unwrap_or_default(),
            smtp_username: get_env("RECIPYA_EMAIL_SMTP_USERNAME").unwrap_or_default(),
            smtp_password: get_env("RECIPYA_EMAIL_SMTP_PASSWORD").unwrap_or_default(),
            email_admin: get_env("RECIPYA_EMAIL_ADMIN").unwrap_or_default(),
        }
    }

    /// Returns whether the email is configured for use with SMTP.
    pub fn is_smtp(&self) -> bool {
        !self.email_admin.is_empty()
            && !self.smtp_host.is_empty()
            && !self.smtp_username.is_empty()
            && !self.smtp_password.is_empty()
    }
}
