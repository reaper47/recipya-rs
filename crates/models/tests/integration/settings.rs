use math::cooking::units::system::MeasurementSystem;
use models::{settings::UserSettingDetails, theme::Theme};
use nutrition::{NutritionDataSource, tables::NutritionSource};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::Result;

#[tokio::test]
async fn test_get_settings() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;

    let got = UserSettingDetails::get(&state.mm, user.id).await?;

    assert_eq!(got.user_id, user.id);
    assert_eq!(got.measurement_system, MeasurementSystem::ImperialUK);
    assert_eq!(
        got.nutrition_source,
        NutritionDataSource::USDAFoodDataCentral
    );
    assert_eq!(
        got.nutrition_sources,
        NutritionSource::all(&state.mm).await?
    );
    assert!(!got.is_convert_automatically);
    assert_eq!(got.cookbooks_view, 0);
    assert_eq!(got.default_theme, Theme::default());
    assert_eq!(got.selected_theme, Theme::Default);
    assert_eq!(got.paper_size_id, 8);
    assert_eq!(got.paper_sizes.len(), 8);
    Ok(())
}
