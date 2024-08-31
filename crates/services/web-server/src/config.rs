use std::sync::OnceLock;

use lib_utils::envs::get_env;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("Fatal - Could not load configuration: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub DOCS_FOLDER: String,
    pub WEB_FOLDER: String,
}

impl WebConfig {
    fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
        Ok(WebConfig {
            DOCS_FOLDER: get_env("SERVICE_DOCS_FOLDER")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}
