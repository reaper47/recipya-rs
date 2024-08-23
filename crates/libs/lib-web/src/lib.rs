mod templates;

pub mod error;
pub mod handlers;
pub mod log;
pub mod middleware;
pub mod routes;
pub mod utils;

use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use lib_core::model::ModelManager;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub mm: ModelManager,
    pub subscribers: Arc<Mutex<HashMap<i64, Vec<WebSocket>>>>,
}

impl AppState {
    pub async fn new() -> error::Result<Self> {
        Ok(Self {
            mm: ModelManager::new().await?,
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn broadcast(&self, user_id: i64, message: Message) {
        if let Some(vec) = self.subscribers.lock().await.get_mut(&user_id) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (idx, ws) in vec.iter_mut().enumerate() {
                if ws.send(Message::Ping(Vec::new())).await.is_err() {
                    to_remove.push(idx);
                }

                let _ = ws.send(message.clone()).await;
            }

            for &idx in to_remove.iter().rev() {
                vec.remove(idx);
            }
        }
    }
}
