mod helpers;
mod message;

pub mod admin;
pub mod auth;
pub mod general;
pub mod recipes;
pub mod reports;
pub mod settings;
pub mod shared;
pub mod shopping;
pub mod static_files;

use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::settings::UserSettingDetails;

use crate::handlers::message::broadcast_error;
use crate::{Error, Result};

pub async fn get_settings(state: &AppState, user_id: Uuid) -> Result<UserSettingDetails> {
    match UserSettingDetails::get(&state.mm, user_id).await {
        Ok(settings) => Ok(settings),
        Err(err) => {
            error!(?user_id, ?err, "Error fetching user settings");
            broadcast_error(state, user_id, "Error fetching user settings.").await;
            Err(Error::Database)
        }
    }
}
