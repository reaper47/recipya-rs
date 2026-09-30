use strum::IntoEnumIterator;

use models::{
    settings::UserSettingDetails,
    theme::{Theme, ThemeModel},
};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::Result;

#[tokio::test]
async fn test_all_themes_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;

    let got = ThemeModel::all(&state.mm).await?;

    pretty_assertions::assert_eq!(got.len(), Theme::iter().len());
    Ok(())
}

#[tokio::test]
async fn test_theme_id_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let theme = Theme::Halloween;

    let got = theme.get_id(&state.mm).await?;

    pretty_assertions::assert_eq!(got, 24);
    Ok(())
}

#[tokio::test]
async fn test_update_theme_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let theme1 = Theme::Halloween;
    let theme2 = Theme::Autumn;

    theme1.save_default(&state.mm, user.id).await?;
    theme2.save_selected(&state.mm, user.id).await?;

    let settings = UserSettingDetails::get(&state.mm, user.id).await?;
    pretty_assertions::assert_eq!(settings.default_theme, theme1);
    pretty_assertions::assert_eq!(settings.selected_theme, theme2);
    Ok(())
}
