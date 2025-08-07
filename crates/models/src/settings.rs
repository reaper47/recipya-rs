use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable, SelectableHelper};
use diesel_async::RunQueryDsl;

use math::cooking::units::MeasurementSystem;
use repository::{ModelManager, schema};

use crate::Result;

/// Represents a user's settings in the database.
#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::user_settings)]
#[diesel(belongs_to(MeasurementSystem))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSetting {
    pub id: i64,
    pub user_id: i64,
    pub measurement_system_id: i16,
    pub calculate_nutrition: bool,
    pub convert_automatically: bool,
    pub cookbooks_view: i32,
}

#[derive(Debug, PartialEq)]
pub struct UserSettingDetails {
    pub measurement_system: MeasurementSystem,
    pub is_calculate_nutrition: bool,
    pub is_convert_automatically: bool,
    pub cookbooks_view: i32,
}

impl UserSettingDetails {
    pub async fn get_settings(mm: &ModelManager, user_id: i64) -> Result<Self> {
        use schema::user_settings;

        let mut conn = mm.pool.get().await?;

        let settings: UserSetting = user_settings::table
            .filter(user_settings::user_id.eq(user_id))
            .select(UserSetting::as_select())
            .first(&mut conn)
            .await?;

        Ok(Self {
            measurement_system: MeasurementSystem::from_id(settings.measurement_system_id)?,
            is_calculate_nutrition: settings.calculate_nutrition,
            is_convert_automatically: settings.convert_automatically,
            cookbooks_view: settings.cookbooks_view,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testing::utils::{TestDb, create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_get_settings() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;

        let got = UserSettingDetails::get_settings(&state.mm, user.id).await?;

        pretty_assertions::assert_eq!(
            got,
            UserSettingDetails {
                measurement_system: MeasurementSystem::ImperialUK,
                is_calculate_nutrition: false,
                is_convert_automatically: false,
                cookbooks_view: 0,
            }
        );
        Ok(())
    }
}
