mod helpers;
mod message;

pub(crate) mod admin;
pub(crate) mod auth;
pub(crate) mod context;
pub(crate) mod general;
pub(crate) mod recipes;
pub(crate) mod settings;
pub(crate) mod shared;

pub mod static_files;

use tracing::error;

use app::state::AppState;
use models::settings::UserSettingDetails;

use crate::Error;
use crate::handlers::message::broadcast_error;

pub(crate) async fn get_settings(
    state: &AppState,
    user_id: i64,
) -> crate::Result<UserSettingDetails> {
    match UserSettingDetails::get(&state.mm, user_id).await {
        Ok(settings) => Ok(settings),
        Err(err) => {
            error!("Error fetching user settings for user {user_id}: {err}");
            broadcast_error(state, user_id, "Error fetching user settings.").await;
            Err(Error::Database)
        }
    }
}
