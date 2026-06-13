use std::{path::Path, sync::Arc};

use diesel::{data_types::PgInterval, prelude::*};
use time::{Duration, PrimitiveDateTime};
use uuid::Uuid;

use repository::schema;
use support::fs::FsSupport;

use super::recipe::Recipe;

/// Represents the data required to insert an additional image associated with a recipe into
/// the `additional_images_recipe` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::additional_images_recipe)]
#[diesel(belongs_to(Recipe))]
pub(crate) struct AdditionalImageForInsert {
    pub recipe_id: i64,
    pub image: Uuid,
}

/// Represents a video associated with a recipe.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Video {
    pub video: Uuid,
    pub duration: Option<Duration>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
    pub created_at: PrimitiveDateTime,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            video: Uuid::default(),
            duration: None,
            content_url: None,
            embed_url: None,
            created_at: PrimitiveDateTime::MIN,
        }
    }
}

/// Represents the association between a video and a recipe in the `videos_recipes` table.
#[derive(Debug, Eq, PartialEq, Identifiable, Selectable, Queryable, Associations)]
#[diesel(table_name = schema::videos_recipes)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VideoRecipe {
    pub id: i64,
    pub video: Uuid,
    pub recipe_id: i64,
    pub duration: Option<PgInterval>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
    pub created_at: PrimitiveDateTime,
}

/// Represents the data required to create a new video associated with a recipe.
#[derive(Clone, Eq, PartialEq)]
pub struct VideoForCreate {
    pub video: Uuid,
    pub duration: Option<Duration>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
}

impl VideoForCreate {
    /// Creates the object from a Path.
    pub async fn from_path(fs_support: Arc<dyn FsSupport + Sync + Send>, path: &Path) -> Self {
        Self {
            video: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
                .parse::<Uuid>()
                .unwrap_or_default(),
            duration: fs_support
                .calc_video_duration(path.to_str().unwrap_or_default())
                .await
                .ok(),
            content_url: None,
            embed_url: None,
        }
    }
}

impl From<Video> for VideoForCreate {
    fn from(v: Video) -> Self {
        Self {
            video: v.video,
            duration: v.duration,
            content_url: v.content_url,
            embed_url: v.embed_url,
        }
    }
}

/// Represents the data required to insert a new video associated with a recipe into
/// the `videos_recipes` table.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::videos_recipes)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct VideoForInsert {
    pub video: Uuid,
    pub recipe_id: i64,
    pub duration: Option<PgInterval>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
}
