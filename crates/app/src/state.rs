use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tracing::error;
use url::Url;

use config::{Config, DataDir};
use email::EmailClient;
use models::data::ViewRecipe;
use models::recipe::{RecipeCache, RecipeCacheKey};
use recipe_schema::RecipeSchema;
use recipya_scraper::{HttpClient, Scraper};
use repository::ModelManager;
use support::fs::{AppFs, FsSupport};

use crate::Result;

/// Shared application state for the Axum web server.
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub data_dir: DataDir,
    pub email_service: Option<EmailClient>,
    pub fs_support: Arc<dyn FsSupport + Send + Sync>,
    pub mm: ModelManager,
    pub scraper: Scraper,
    pub subscribers: Arc<Mutex<HashMap<i64, Vec<WebSocket>>>>,

    recipe_cache: Arc<Mutex<RecipeCache>>,
}

impl AppState {
    /// Creates a new instance of `AppState` by initializing the `ModelManager`
    /// with the provided database URL.
    pub async fn new(
        config: Config,
        http_client: Arc<dyn HttpClient + Send + Sync>,
        fs_support: Arc<dyn FsSupport + Send + Sync>,
    ) -> Result<Self> {
        let email_service = EmailClient::new().ok();

        let data_dir = DataDir::new()?;
        data_dir.log();

        Ok(Self {
            config: config.clone(),
            data_dir,
            email_service,
            fs_support,
            mm: ModelManager::new(config.database_url).await?,
            recipe_cache: Arc::new(Mutex::new(RecipeCache::new(
                NonZeroUsize::new(1000).expect("LRU to be initialized"),
            ))),
            scraper: Scraper::with_client(http_client, Arc::new(AppFs)),
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Hides the websocket's frontend notification.
    pub async fn hide_broadcast(&self, user_id: i64) {
        self.broadcast_progress("", -1, -1, false, user_id).await;
    }

    /// Broadcasts a progress notification.
    pub async fn broadcast_progress(
        &self,
        title: &str,
        current_value: i64,
        total: i64,
        is_notification_visible: bool,
        user_id: i64,
    ) {
        let percentage = if total.gt(&0) {
            (current_value as f64 / total as f64) * 100.0
        } else {
            Default::default()
        };

        let content = format!(
            r#"
            <div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default {}">
                <div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md">
                    <p class="font-medium text-center pb-1">{title}</p>
                    <div id="export-progress"><progress max="100" value="{percentage:.2}"></progress></div>
                </div>
            </div>"#,
            if is_notification_visible { "" } else { "hidden" }
        ).lines().map(str::trim).collect::<Vec<_>>().join("");

        self.broadcast(user_id, Message::Text(content.into())).await;
    }

    /// Broadcasts a message to all active WebSocket subscribers of a given user.
    pub async fn broadcast(&self, user_id: i64, message: Message) {
        let send_timeout = Duration::from_secs(10);

        if let Some(vec) = self.subscribers.lock().await.get_mut(&user_id) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (idx, ws) in vec.iter_mut().enumerate() {
                if ws.send(Message::Ping(Vec::new().into())).await.is_err() {
                    to_remove.push(idx);
                }

                match timeout(send_timeout, ws.send(message.clone())).await {
                    Ok(Ok(())) => {}
                    Ok(Err(err)) => {
                        error!("Failed to send broadcast message to user {user_id}: {err}");
                    }
                    Err(_) => {
                        error!("Send broadcast message to user {user_id} timed out");
                    }
                }
            }

            for &idx in to_remove.iter().rev() {
                vec.remove(idx);
            }
        }
    }

    /// Gets a recipe from the cache if present.
    pub async fn get_cached_recipe(&self, key: RecipeCacheKey) -> Option<ViewRecipe> {
        let mut cache = self.recipe_cache.lock().await;
        cache.get(&key).cloned()
    }

    /// Inserts the recipe into the cache.
    pub async fn cache_recipe(&self, key: RecipeCacheKey, recipe: &ViewRecipe) {
        let mut cache = self.recipe_cache.lock().await;
        cache.put(key, recipe.clone());
    }

    /// Removes an entry from the cache.
    pub async fn remove_cached_recipe(&self, key: RecipeCacheKey) {
        let mut cache = self.recipe_cache.lock().await;
        cache.pop(&key);
    }

    /// Scrapes a recipe from the specified website.
    pub fn scrape(&self, url: Url) -> Result<RecipeSchema> {
        let url = url.as_str();
        Ok(self.scraper.scrape(url)?)
    }
}
