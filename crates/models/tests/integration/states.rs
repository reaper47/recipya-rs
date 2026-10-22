use models::states::UserStates;
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::Result;

mod tests_is_importing_app_or_api {
    use super::*;

    #[tokio::test]
    async fn test_not_importing_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_states = UserStates::new(&state.mm, user.id);

        let got = user_states.is_importing_app_or_api().await?;

        assert!(!got);
        Ok(())
    }

    #[tokio::test]
    async fn test_importing_app_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_states = UserStates::new(&state.mm, user.id);
        user_states.set_is_importing_app().await?;

        let got = user_states.is_importing_app_or_api().await?;

        assert!(got);
        Ok(())
    }

    #[tokio::test]
    async fn test_import_api_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_states = UserStates::new(&state.mm, user.id);
        user_states.set_is_importing_api().await?;

        let got = user_states.is_importing_app_or_api().await?;

        assert!(got);
        Ok(())
    }
}
