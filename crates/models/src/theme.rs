use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Result;

/// Enumeration of all UI themes.
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
    /// Fetches a theme by id.
    pub async fn get_id(&self, mm: &ModelManager) -> Result<i32> {
        Ok(schema::themes::table
            .filter(schema::themes::name.eq(self.to_string()))
            .select(schema::themes::id)
            .first::<i32>(&mut mm.pool.get().await?)
            .await?)
    }

    /// Saves the default theme, which is set by the admin for all users.
    pub async fn save_default(&self, mm: &ModelManager, user_id: Uuid) -> Result<()> {
        self.update_theme(mm, user_id, true).await
    }

    /// Saves the theme selected by the user.
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

/// Represents the database's theme entity.
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
        Ok(schema::themes::table
            .select(Self::as_select())
            .load(&mut mm.pool.get().await?)
            .await?)
    }
}
