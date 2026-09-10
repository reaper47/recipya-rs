use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable, SelectableHelper};
use diesel_async::RunQueryDsl;
use strum::{Display, EnumIter, EnumString};
use time_tz::{TimeZone, Tz, timezones};
use uuid::Uuid;

use math::cooking::units::system::MeasurementSystem;
use nutrition::{NutritionDataSource, tables::NutritionSource};
use repository::{ModelManager, schema};

use crate::paper::{PaperSize, PaperSizes};
use crate::{Error, Result};

#[derive(Debug, Default, Eq, PartialEq, Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Theme {
    Default,
    #[default]
    System,
    Light,
    Dark,
    Abyss,
    Acid,
    Aqua,
    Autumn,
    Black,
    Bumblebee,
    Business,
    Caramellatte,
    Coffee,
    Corporate,
    Cmyk,
    Cupcake,
    Cyberpunk,
    Dim,
    Dracula,
    Emerald,
    Fantasy,
    Forest,
    Garden,
    Halloween,
    Lemonade,
    Lofi,
    Luxury,
    Night,
    Nord,
    Pastel,
    Retro,
    Silk,
    Sunset,
    Synthwave,
    Valentine,
    Wireframe,
    Winter,
}

impl Theme {
    pub async fn get_id(&self, mm: &ModelManager) -> Result<i32> {
        Ok(schema::themes::table
            .filter(schema::themes::name.eq(self.to_string()))
            .select(schema::themes::id)
            .first::<i32>(&mut mm.pool.get().await?)
            .await?)
    }

    pub async fn save_default(&self, mm: &ModelManager, user_id: Uuid) -> Result<()> {
        self.update_theme(mm, user_id, true).await
    }

    pub async fn save_selected(&self, mm: &ModelManager, user_id: Uuid) -> Result<()> {
        self.update_theme(mm, user_id, false).await
    }

    async fn update_theme(&self, mm: &ModelManager, user_id: Uuid, is_default: bool) -> Result<()> {
        use schema::user_settings;

        let theme_id = self.get_id(mm).await?;

        let mut conn = mm.pool.get().await?;

        if is_default {
            diesel::update(user_settings::table)
                .set(user_settings::default_theme.eq(theme_id))
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(user_settings::table.filter(user_settings::user_id.eq(user_id)))
                .set(user_settings::selected_theme.eq(theme_id))
                .execute(&mut conn)
                .await?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::themes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ThemeModel {
    pub id: i32,
    pub name: String,
}

impl ThemeModel {
    /// Fetches all the themes from the database.
    pub async fn all(mm: &ModelManager) -> Result<Vec<Self>> {
        use schema::themes;

        let mut conn = mm.pool.get().await?;

        Ok(themes::table
            .select(Self::as_select())
            .load(&mut conn)
            .await?)
    }
}

/// Represents a user's settings in the database.
#[allow(dead_code)]
#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::user_settings)]
#[diesel(belongs_to(MeasurementSystem))]
#[diesel(belongs_to(PaperSize))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserSetting {
    id: i64,
    user_id: Uuid,
    measurement_system_id: i16,
    nutrition_source_id: i16,
    bold_ingredients: bool,
    convert_automatically: bool,
    cookbooks_view: i32,
    default_theme: i32,
    selected_theme: i32,
    paper_size_id: i16,
    timezone: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UserSettingDetails {
    pub user_id: Uuid,
    pub measurement_system: MeasurementSystem,
    pub nutrition_source: NutritionDataSource,
    pub nutrition_sources: Vec<NutritionSource>,
    pub is_bold_ingredients: bool,
    pub is_convert_automatically: bool,
    pub cookbooks_view: i32,
    pub default_theme: Theme,
    pub selected_theme: Theme,
    pub paper_size_id: i16,
    pub paper_sizes: PaperSizes,
    pub timezone: &'static Tz,
}

impl Default for UserSettingDetails {
    fn default() -> Self {
        Self {
            user_id: Uuid::nil(),
            measurement_system: MeasurementSystem::default(),
            nutrition_source: NutritionDataSource::default(),
            nutrition_sources: Vec::new(),
            is_bold_ingredients: false,
            is_convert_automatically: false,
            cookbooks_view: 0,
            default_theme: Theme::default(),
            selected_theme: Theme::default(),
            paper_size_id: 0,
            paper_sizes: PaperSizes::default(),
            timezone: timezones::db::UTC,
        }
    }
}

impl UserSettingDetails {
    /// Retrieves the user settings for the given user ID from the database.
    pub async fn get(mm: &ModelManager, user_id: Uuid) -> Result<Self> {
        use schema::{themes, user_settings};

        let mut conn = mm.pool.get().await?;

        let settings: UserSetting = user_settings::table
            .filter(user_settings::user_id.eq(user_id))
            .select(UserSetting::as_select())
            .first(&mut conn)
            .await?;

        let default_theme_name: Theme = themes::table
            .find(settings.default_theme)
            .select(themes::name)
            .first::<String>(&mut conn)
            .await?
            .parse()
            .map_err(|_| Error::ThemeNotFound)?;

        let selected_theme_name: Theme = themes::table
            .find(settings.selected_theme)
            .select(themes::name)
            .first::<String>(&mut conn)
            .await?
            .parse()
            .map_err(|_| Error::ThemeNotFound)?;

        drop(conn);

        Ok(Self {
            user_id,
            measurement_system: MeasurementSystem::from_id(settings.measurement_system_id)?,
            nutrition_source: NutritionDataSource::from(settings.nutrition_source_id),
            nutrition_sources: NutritionSource::all(mm).await?,
            is_bold_ingredients: settings.bold_ingredients,
            is_convert_automatically: settings.convert_automatically,
            cookbooks_view: settings.cookbooks_view,
            default_theme: default_theme_name,
            selected_theme: selected_theme_name,
            paper_size_id: settings.paper_size_id,
            paper_sizes: PaperSize::get_all(mm).await?,
            timezone: timezones::get_by_name(&settings.timezone).unwrap_or(timezones::db::UTC),
        })
    }

    /// Returns the timezone name without the "Etc/" prefix.
    pub fn tz_name(&self) -> String {
        self.timezone.name().trim_start_matches("Etc/").to_string()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use test_db::default_config;
    use test_utils::{create_app_state, insert_user};

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_get_settings() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state).await?;

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
        let user = insert_user(&state).await?;
        let theme1 = Theme::Halloween;
        let theme2 = Theme::Autumn;

        theme1.save_default(&state.mm, user.id).await?;
        theme2.save_selected(&state.mm, user.id).await?;

        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        pretty_assertions::assert_eq!(settings.default_theme, theme1);
        pretty_assertions::assert_eq!(settings.selected_theme, theme2);
        Ok(())
    }
}
