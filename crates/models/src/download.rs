use std::path::{Path, PathBuf};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use time::PrimitiveDateTime;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::{Result, user::User};

/// Represents a download item.
#[derive(Debug, Eq, PartialEq, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::downloads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Download {
    /// The unique identifier of the download item.
    pub id: i64,
    /// The foreign key linking the shared recipe to its creator.
    pub user_id: Uuid,
    /// The token associated with the download item.
    pub token: Uuid,
    /// The path of the file associated with the download item.
    pub file_path: String,
    /// The timestamp when the download item was created.
    pub created_at: PrimitiveDateTime,
}

/// Represents a download item for creation.
pub struct DownloadForCreate {
    pub user_id: Uuid,
    pub token: Uuid,
    pub file_path: PathBuf,
}

#[derive(Associations, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::downloads)]
struct DownloadForInsert {
    user_id: Uuid,
    token: Uuid,
    file_path: String,
}

impl From<DownloadForCreate> for DownloadForInsert {
    fn from(dl_c: DownloadForCreate) -> Self {
        Self {
            user_id: dl_c.user_id,
            token: dl_c.token,
            file_path: dl_c.file_path.to_string_lossy().into_owned(),
        }
    }
}

impl DownloadForCreate {
    /// Creates a new download item for creation.
    pub fn new(user_id: Uuid, token: Uuid, file_path: &Path) -> Self {
        Self {
            user_id,
            token,
            file_path: file_path.to_path_buf(),
        }
    }
}

impl Download {
    /// Creates a new download item in the database.
    pub async fn create(mm: &ModelManager, dl_c: DownloadForCreate) -> Result<()> {
        diesel::insert_into(schema::downloads::table)
            .values(&DownloadForInsert::from(dl_c))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Finds a download item by its token.
    pub async fn find_by_token(mm: &ModelManager, token: Uuid) -> Result<Option<Self>> {
        Ok(schema::downloads::table
            .filter(schema::downloads::token.eq(token))
            .select(Self::as_select())
            .first(&mut mm.pool.get().await?)
            .await
            .optional()?)
    }

    /// Deletes a download item by its token and deletes the file associated with it.
    pub async fn delete_by_token<T: AsRef<str>>(
        mm: &ModelManager,
        token: Uuid,
        file_path: T,
    ) -> Result<()> {
        diesel::delete(schema::downloads::table)
            .filter(schema::downloads::token.eq(token))
            .execute(&mut mm.pool.get().await?)
            .await?;

        tokio::fs::remove_file(file_path.as_ref()).await?;

        Ok(())
    }
}
