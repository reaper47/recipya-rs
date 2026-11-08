pub mod admin;
pub mod auth;
pub mod general;
pub mod recipes;
pub mod reports;
pub mod settings;
pub mod shared;
pub mod shopping;
pub mod static_files;

use l10n::Messages;
use support::states::AuthenticatedState;
use tracing::error;
use uuid::Uuid;

use app::message::ToastCtx;
use app::state::AppState;
use models::settings::UserSettingDetails;

use crate::{Error, Result};

/// Handler to get a user's settings.
pub async fn get_settings(
    state: &AppState,
    user_id: Uuid,
    messages: &Messages,
) -> Result<UserSettingDetails> {
    match UserSettingDetails::get(&state.mm, user_id).await {
        Ok(settings) => Ok(settings),
        Err(err) => {
            let toast = ToastCtx::<AuthenticatedState>::new(state, user_id, messages);
            error!(?user_id, ?err, "Error fetching user settings");
            toast
                .error(&messages.toast_users_fetch_settings_failed())
                .await;
            Err(Error::Database)
        }
    }
}
