use std::sync::OnceLock;

use support::envs::get_env;

/// Configuration struct for the email client.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub email_admin: String,
}

/// Gets the current email `Config` struct. It will be initialized if not already done.
pub fn email_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(Config::load_from_env)
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    fn load_from_env() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_env() {
        temp_env::with_vars(
            [
                ("RECIPYA_EMAIL_SMTP_HOST", Some("smtp_host")),
                ("RECIPYA_EMAIL_SMTP_USERNAME", Some("smtp_username")),
                ("RECIPYA_EMAIL_SMTP_PASSWORD", Some("smtp_password")),
                ("RECIPYA_EMAIL_ADMIN", Some("smtp_admin")),
            ],
            || {
                let got = Config::load_from_env();

                pretty_assertions::assert_eq!(
                    got,
                    Config {
                        smtp_host: "smtp_host".to_string(),
                        smtp_username: "smtp_username".to_string(),
                        smtp_password: "smtp_password".to_string(),
                        email_admin: "smtp_admin".to_string(),
                    }
                );
            },
        );
    }

    #[test]
    fn test_is_smtp_valid() {
        let config = Config {
            smtp_host: "smtp_host".to_string(),
            smtp_username: "smtp_username".to_string(),
            smtp_password: "smtp_password".to_string(),
            email_admin: "smtp_admin".to_string(),
        };

        pretty_assertions::assert_eq!(config.is_smtp(), true);
    }

    #[test]
    fn test_is_smtp_invalid() {
        let config = Config {
            smtp_host: "smtp_host".to_string(),
            smtp_username: "smtp_username".to_string(),
            smtp_password: "smtp_password".to_string(),
            email_admin: "".to_string(),
        };

        pretty_assertions::assert_eq!(config.is_smtp(), false);
    }
}
