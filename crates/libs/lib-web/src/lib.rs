mod templates;

pub mod error;
pub mod handlers;
pub mod log;
pub mod middleware;
pub mod routes;
pub mod utils;

use lib_core::model::ModelManager;

#[derive(Clone)]
pub struct AppState {
    pub mm: ModelManager,
}

impl AppState {
    pub async fn new() -> error::Result<Self> {
        Ok(Self {
            mm: ModelManager::new().await?,
        })
    }
}
