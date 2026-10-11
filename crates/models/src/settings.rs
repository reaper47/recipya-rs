use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable, SelectableHelper};
use diesel_async::RunQueryDsl;
use time_tz::{TimeZone, Tz, timezones};
use uuid::Uuid;

use math::cooking::units::system::MeasurementSystem;
use nutrition::{NutritionDataSource, tables::NutritionSource};
use repository::{ModelManager, schema};

use crate::paper::{PaperSize, PaperSizes};
use crate::theme::Theme;
use crate::{Error, Result};

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
    language_id: i64,
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
    pub selected_locale: String,
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
            selected_locale: "en-CA".into(),
            paper_size_id: 0,
            paper_sizes: PaperSizes::default(),
            timezone: timezones::db::UTC,
        }
    }
}

impl UserSettingDetails {
    /// Retrieves the user settings for the given user ID from the database.
    pub async fn get(mm: &ModelManager, user_id: Uuid) -> Result<Self> {
        let mut conn = mm.pool.get().await?;

        let settings: UserSetting = schema::user_settings::table
            .filter(schema::user_settings::user_id.eq(user_id))
            .select(UserSetting::as_select())
            .first(&mut conn)
            .await?;

        let default_theme_name: Theme = schema::themes::table
            .find(settings.default_theme)
            .select(schema::themes::name)
            .first::<String>(&mut conn)
            .await?
            .parse()
            .map_err(|_| Error::ThemeNotFound)?;

        let selected_theme_name: Theme = schema::themes::table
            .find(settings.selected_theme)
            .select(schema::themes::name)
            .first::<String>(&mut conn)
            .await?
            .parse()
            .map_err(|_| Error::ThemeNotFound)?;

        let selected_i18n_code: String = schema::languages::table
            .filter(schema::languages::id.eq(settings.language_id))
            .select(schema::languages::locale)
            .first(&mut conn)
            .await?;

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
            selected_locale: selected_i18n_code,
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
