use std::sync::OnceLock;

use support::envs::get_env;

/// Configuration struct for the email client.
#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
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
            smtp_host: get_env("SMTP_HOST").unwrap_or_default(),
            smtp_port: get_env("SMTP_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .unwrap_or_default(),
            smtp_username: get_env("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: get_env("SMTP_PASSWORD").unwrap_or_default(),
            smtp_from_email: get_env("SMTP_FROM_EMAIL").unwrap_or_default(),
        }
    }

    /// Returns whether the email is configured for use with SMTP.
    pub const fn is_smtp(&self) -> bool {
        !self.smtp_from_email.is_empty()
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
                ("SMTP_HOST", Some("smtp_host")),
                ("SMTP_PORT", Some("587")),
                ("SMTP_USERNAME", Some("smtp_username")),
                ("SMTP_PASSWORD", Some("smtp_password")),
                ("SMTP_FROM_EMAIL", Some("smtp_admin")),
            ],
            || {
                let got = Config::load_from_env();

                pretty_assertions::assert_eq!(
                    got,
                    Config {
                        smtp_host: "smtp_host".to_string(),
                        smtp_port: 587,
                        smtp_username: "smtp_username".to_string(),
                        smtp_password: "smtp_password".to_string(),
                        smtp_from_email: "smtp_admin".to_string(),
                    }
                );
            },
        );
    }

    #[test]
    fn test_is_smtp_valid() {
        let config = Config {
            smtp_host: "smtp_host".to_string(),
            smtp_port: 587,
            smtp_username: "smtp_username".to_string(),
            smtp_password: "smtp_password".to_string(),
            smtp_from_email: "smtp_admin".to_string(),
        };

        pretty_assertions::assert_eq!(config.is_smtp(), true);
    }

    #[test]
    fn test_is_smtp_invalid() {
        let config = Config {
            smtp_host: "smtp_host".to_string(),
            smtp_port: 587,
            smtp_username: "smtp_username".to_string(),
            smtp_password: "smtp_password".to_string(),
            smtp_from_email: "".to_string(),
        };

        pretty_assertions::assert_eq!(config.is_smtp(), false);
    }
}
