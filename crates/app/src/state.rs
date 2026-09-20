use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;

use tokio::sync::{Mutex, RwLock, broadcast, watch};
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
use crate::message::{Broadcaster, Snack};

/// Shared application state for the Axum web server.
#[derive(Clone)]
pub struct AppState {
    pub channels: Channels,
    pub config: Arc<RwLock<Config>>,
    pub data_dir: DataDir,
    pub email_service: Option<EmailClient>,
    pub fs_support: Arc<dyn FsSupport + Send + Sync>,
    pub mm: ModelManager,
    pub scraper: Scraper,

    recipe_cache: Arc<Mutex<RecipeCache>>,
}

#[derive(Clone)]
pub struct Channels {
    shutdown_channel: (watch::Sender<bool>, watch::Receiver<bool>),
    connected_clients: Arc<Mutex<HashMap<Uuid, ClientConnection>>>,
}

#[derive(Clone)]
struct ClientConnection {
    sender: broadcast::Sender<String>,
    open_connections: i64,
}

impl ClientConnection {
    fn new() -> Self {
        Self {
            sender: broadcast::channel(16).0,
            open_connections: 0,
        }
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self {
            shutdown_channel: watch::channel(false),
            connected_clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Channels {
    /// Subscribes a client connection to the broadcast channel for a given user.
    /// A new broadcast channel is created when the user is not already connected.
    pub async fn subscribe_client(&self, user_id: Uuid) -> broadcast::Receiver<String> {
        let mut clients = self.connected_clients.lock().await;
        let conn = clients.entry(user_id).or_insert_with(ClientConnection::new);
        conn.open_connections += 1;
        let rx = conn.sender.subscribe();
        drop(clients);
        rx
    }

    /// Unsubscribes a client connection from the broadcast channel for a given user.
    pub async fn unsubscribe_client(&self, user_id: Uuid) {
        let mut clients = self.connected_clients.lock().await;
        if let Some(conn) = clients.get_mut(&user_id) {
            conn.open_connections -= 1;
            if conn.open_connections == 0 {
                clients.remove(&user_id);
            }
        }
    }

    /// Sends a message to a specific user.
    pub async fn broadcast_to_client(&self, message: &str, user_id: Uuid) {
        let clients = self.connected_clients.lock().await;
        if let Some(conn) = clients.get(&user_id) {
            let _ = conn.sender.send(message.to_string());
        }
    }

    /// Gets a clone of the shutdown sender.
    pub fn shutdown_tx(&self) -> watch::Sender<bool> {
        self.shutdown_channel.clone().0
    }

    /// Gets a clone of the shutdown receiver.
    pub fn shutdown_rx(&self) -> watch::Receiver<bool> {
        self.shutdown_channel.clone().1
    }
}

impl AppState {
    /// Creates a new instance of `AppState` by initializing the `ModelManager`
    /// with the provided database URL.
    ///
    /// # Panics
    ///
    /// This function contains an infallible expect that will never panic in practice.
    pub async fn new(
        config: Config,
        http_client: Arc<dyn HttpClient + Send + Sync>,
        fs_support: Arc<dyn FsSupport + Send + Sync>,
    ) -> Result<Self> {
        let data_dir = DataDir::new()?;
        data_dir.log();

        Ok(Self {
            channels: Channels::default(),
            config: Arc::new(RwLock::new(config.clone())),
            data_dir,
            email_service: EmailClient::new().ok(),
            fs_support,
            mm: ModelManager::new(config.database_url).await?,
            recipe_cache: Arc::new(Mutex::new(RecipeCache::new(
                NonZeroUsize::new(1000).expect("lru to be initialized"),
            ))),
            scraper: Scraper::with_client(http_client, Arc::new(AppFs)),
        })
    }

    /// Creates a new instance of `AppState` by initializing the `ModelManager`
    /// with the provided database URL.
    ///
    /// Only meant for use in tests because the ModelManager uses a single test
    /// database throughout the tests.
    ///
    /// # Panics
    ///
    /// This function contains an infallible expect that will never panic in practice.
    #[doc(hidden)]
    pub fn new_for_test(
        config: Config,
        mm: ModelManager,
        http_client: Arc<dyn HttpClient + Send + Sync>,
        fs_support: Arc<dyn FsSupport + Send + Sync>,
    ) -> Result<Self> {
        Ok(Self {
            channels: Channels::default(),
            config: Arc::new(RwLock::new(config)),
            data_dir: DataDir::new()?,
            email_service: EmailClient::new().ok(),
            fs_support,
            mm,
            recipe_cache: Arc::new(Mutex::new(RecipeCache::new(
                NonZeroUsize::new(1000).expect("lru to be initialized"),
            ))),
            scraper: Scraper::with_client(http_client, Arc::new(AppFs)),
        })
    }

    /// Hides the SSE's frontend notification.
    pub async fn hide_broadcast(&self, user_id: Uuid) {
        self.broadcast_progress("", -1, -1, false, user_id).await;
    }

    /// Sends an HX-Trigger event over SSE.
    pub async fn broadcast_trigger(&self, event: &str, user_id: Uuid) {
        let message = format!(r#"{{"headers": {{"HX-Trigger": "{event}"}}}}"#);
        self.channels.broadcast_to_client(&message, user_id).await;
    }

    /// Broadcasts a progress notification.
    #[allow(clippy::cast_precision_loss)]
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
            r#"<div id="sse-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default {}">
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

        Snack::broadcast_success(self, user_id, &content).await;
    }

    /// Gets a recipe from the cache if present.
    pub async fn get_cached_recipe(&self, key: RecipeCacheKey) -> Option<Arc<ViewRecipe>> {
        let mut cache = self.recipe_cache.lock().await;
        cache.get(&key).cloned()
    }

    /// Inserts the recipe into the cache.
    pub async fn cache_recipe(&self, key: RecipeCacheKey, recipe: &ViewRecipe) {
        let mut cache = self.recipe_cache.lock().await;
        cache.put(key, Arc::new(recipe.clone()));
    }

    /// Removes an entry from the cache.
    pub async fn remove_cached_recipe(&self, key: RecipeCacheKey) {
        let mut cache = self.recipe_cache.lock().await;
        cache.pop(&key);
    }

    /// Scrapes a recipe from the specified website.
    pub async fn scrape(&self, url: Url) -> Result<Recipe> {
        let url = url.as_str();
        Ok(self.scraper.scrape(url).await?)
    }
}
