use tracing::{Instrument, Span, error};
use uuid::Uuid;

use app::{message::ToastCtx, state::AppState};
use l10n::Messages;
use models::{export::ExportOptions, paper::PaperSize, settings::UserSettingDetails};
use support::states::AuthenticatedState;

use crate::{Error, Result};

/// Fetches the user's export options.
#[allow(clippy::cast_possible_truncation)]
pub async fn export_options(
    state: &AppState,
    user_id: Uuid,
    messages: &Messages,
) -> Result<ExportOptions> {
    let settings = UserSettingDetails::get(&state.mm, user_id)
        .await
        .inspect_err(|err| error!(?err, "Failed to get user settings"))
        .map_err(|_| {
            let state = state.clone();
            let messages = messages.clone();

            tokio::spawn(
                async move {
                    let toast = ToastCtx::<AuthenticatedState>::new(&state, user_id, &messages);
                    toast
                        .error(&messages.toast_users_fetch_settings_failed())
                        .await;
                }
                .instrument(Span::current()),
            );
            Error::Database
        })?;

    let paper_size = PaperSize::get(&state.mm, settings.paper_size_id)
        .await
        .inspect_err(|err| error!(user = ?user_id, ?err, "Failed to get paper size"))
        .map_err(|_| {
            let state = state.clone();
            let messages = messages.clone();

            tokio::spawn(
                async move {
                    let toast = ToastCtx::<AuthenticatedState>::new(&state, user_id, &messages);
                    toast.error(&messages.toast_paper_sizes_failed()).await;
                }
                .instrument(Span::current()),
            );
            Error::Database
        })?;

    Ok(ExportOptions {
        paper_size: (
            paper_size.width_in as f32 * 72.0,
            paper_size.height_in as f32 * 72.0,
        ),
    })
}
