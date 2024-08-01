use std::sync::OnceLock;

use lib_utils::envs::get_env;

pub fn config() -> &'static EmailConfig {
    static INSTANCE: OnceLock<EmailConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        EmailConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("Fatal - Could not load configuration: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct EmailConfig {
    pub EMAIL_FROM: String,
    pub EMAIL_SENDGRID_API_KEY: String,
}

impl EmailConfig {
    fn load_from_env() -> lib_utils::envs::Result<Self> {
        Ok(Self {
            EMAIL_FROM: get_env("SERVICE_EMAIL_FROM")?,
            EMAIL_SENDGRID_API_KEY: get_env("SERVICE_EMAIL_SENDGRID_API_KEY")?,
        })
    }
}
