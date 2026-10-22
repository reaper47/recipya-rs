use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Result;

/// Represents a user's states in the database.
#[derive(Clone)]
pub struct UserStates<'a> {
    mm: &'a ModelManager,
    user_id: Uuid,
}

impl<'a> UserStates<'a> {
    pub const fn new(mm: &'a ModelManager, user_id: Uuid) -> Self {
        Self { mm, user_id }
    }

    /// Verifies whether the user is currently importing recipes from an app or an API.
    pub async fn is_importing_app_or_api(&self) -> Result<bool> {
        use schema::user_states::dsl::{is_importing_api, is_importing_app, user_id, user_states};

        Ok(diesel::select(exists(
            user_states
                .filter(user_id.eq(self.user_id))
                .filter(is_importing_app.or(is_importing_api)),
        ))
        .get_result(&mut self.mm.pool.get().await?)
        .await?)
    }

    /// Sets the user's `is_importing_app` field to `true`.
    pub async fn set_is_importing_app(&self) -> Result<()> {
        self.set_is_importing_app_helper(true).await
    }

    async fn set_is_importing_app_helper(&self, state: bool) -> Result<()> {
        use schema::user_states::dsl::{is_importing_app, user_id, user_states};

        diesel::update(user_states.filter(user_id.eq(self.user_id)))
            .set(is_importing_app.eq(state))
            .execute(&mut self.mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Sets the user's `is_importing_api` field to `true`.
    pub async fn set_is_importing_api(&self) -> Result<()> {
        self.set_is_importing_api_helper(true).await
    }

    async fn set_is_importing_api_helper(&self, state: bool) -> Result<()> {
        use schema::user_states::dsl::{is_importing_api, user_id, user_states};

        diesel::update(user_states.filter(user_id.eq(self.user_id)))
            .set(is_importing_api.eq(state))
            .execute(&mut self.mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Clears the import flags.
    async fn unset_import_flags(mm: &ModelManager, id: Uuid) -> Result<()> {
        use schema::user_states::dsl::{is_importing_api, is_importing_app, user_id, user_states};

        diesel::update(user_states.filter(user_id.eq(id)))
            .set(is_importing_app.eq(false))
            .execute(&mut mm.pool.get().await?)
            .await?;

        diesel::update(user_states.filter(user_id.eq(id)))
            .set(is_importing_api.eq(false))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Clears all flags for all users.
    pub async fn reset(mm: &ModelManager) -> Result<()> {
        use schema::user_states::dsl::{is_importing_api, is_importing_app, user_states};

        diesel::update(user_states)
            .set(is_importing_api.eq(false))
            .execute(&mut mm.pool.get().await?)
            .await?;

        diesel::update(user_states)
            .set(is_importing_app.eq(false))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }
}

impl Drop for UserStates<'_> {
    fn drop(&mut self) {
        let mm = self.mm.clone();
        let user_id = self.user_id;

        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            error!(
                ?user_id,
                "Failed to reset import state because no tokio runtime found"
            );
            return;
        };

        handle.spawn(async move {
            if let Err(err) = UserStates::unset_import_flags(&mm, user_id).await {
                error!(?err, ?user_id, "Failed to reset import state on drop");
            }
        });
    }
}
