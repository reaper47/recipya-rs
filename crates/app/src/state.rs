use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{Mutex, RwLock};
use url::Url;
use uuid::Uuid;

use config::{Config, DataDir};
use email::EmailClient;
use models::data::ViewRecipe;
use models::recipe::{RecipeCache, RecipeCacheKey};
use recipya_scraper::{HttpClient, Scraper};
use repository::ModelManager;
use schema_org::Recipe;
use support::fs::{AppFs, FsSupport};

use crate::Result;

/// Shared application state for the Axum web server.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
    pub data_dir: DataDir,
    pub email_service: Option<EmailClient>,
    pub fs_support: Arc<dyn FsSupport + Send + Sync>,
    pub mm: ModelManager,
    pub scraper: Scraper,
    pub subscribers: Arc<Mutex<HashMap<Uuid, Vec<WebSocket>>>>,

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
        let data_dir = DataDir::new()?;
        data_dir.log();

        Ok(Self {
            config: Arc::new(RwLock::new(config.clone())),
            data_dir,
            email_service: EmailClient::new().ok(),
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
    pub async fn hide_broadcast(&self, user_id: Uuid) {
        self.broadcast_progress("", -1, -1, false, user_id).await;
    }

    /// Broadcasts a progress notification.
    pub async fn broadcast_progress(
        &self,
        title: &str,
        current_value: i64,
        total: i64,
        is_notification_visible: bool,
        user_id: Uuid,
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

                    <div class="flex justify-between items-center text-sm mb-2">
                        <span class="font-semibold">{current_value} of {total}</span>
                        <span class="font-semibold">{percentage:.1}%</span>
                    </div>

                    <div id="export-progress">
                        <progress max="100" value="{percentage:.2}"></progress>
                    </div>
                </div>
            </div>"#,
            if is_notification_visible { "" } else { "hidden" }
        ).lines().map(str::trim).collect::<Vec<_>>().join("");

        self.broadcast(user_id, Message::Text(content.into())).await;
    }

    /// Broadcasts a message to all active WebSocket subscribers of a given user.
    pub async fn broadcast(&self, user_id: Uuid, message: Message) {
        use std::time::Duration;
        use tokio::time::timeout;

        let send_timeout = Duration::from_secs(10);

        let connections = {
            let mut subs = self.subscribers.lock().await;
            subs.remove(&user_id).unwrap_or_default()
        };

        let mut alive = Vec::with_capacity(connections.len());
        for mut ws in connections {
            if timeout(send_timeout, ws.send(Message::Ping(Vec::new().into())))
                .await
                .is_err()
            {
                continue;
            }

            match timeout(send_timeout, ws.send(message.clone())).await {
                Ok(Ok(())) => alive.push(ws),
                _ => {
                    // Broken pipe / timeout / other: drop silently
                }
            }
        }

        // Put back the ones that survived
        if !alive.is_empty() {
            let mut subs = self.subscribers.lock().await;
            subs.entry(user_id).or_default().extend(alive);
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
    pub fn scrape(&self, url: Url) -> Result<Recipe> {
        let url = url.as_str();
        Ok(self.scraper.scrape(url)?)
    }
}
