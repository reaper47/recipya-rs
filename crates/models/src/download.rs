use std::path::PathBuf;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{Result, user::User};
use repository::{ModelManager, schema};

/// Represents a download item.
#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable)]
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
    pub created_at: NaiveDateTime,
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
    pub const fn new(user_id: Uuid, token: Uuid, file_path: PathBuf) -> Self {
        Self {
            user_id,
            token,
            file_path,
        }
    }
}

impl Download {
    /// Creates a new download item in the database.
    pub async fn create(mm: &ModelManager, dl_c: DownloadForCreate) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::insert_into(schema::downloads::table)
            .values(&DownloadForInsert::from(dl_c))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Finds a download item by its token.
    pub async fn find_by_token(mm: &ModelManager, token: Uuid) -> Result<Option<Self>> {
        let mut conn = mm.pool.get().await?;

        Ok(schema::downloads::table
            .filter(schema::downloads::token.eq(token))
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .optional()?)
    }

    /// Deletes a download item by its token and deletes the file associated with it.
    pub async fn delete_by_token(mm: &ModelManager, token: Uuid, file_path: String) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::delete(schema::downloads::table)
            .filter(schema::downloads::token.eq(token))
            .execute(&mut conn)
            .await?;

        tokio::fs::remove_file(file_path).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use testing::utils::{TestDb, build_server_anonymous, create_app_state};

    use super::*;
    use crate::user::User;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const TEST_FILE_PATH: &str = "/tmp/recipes.zip";

    fn test_file_path() -> PathBuf {
        PathBuf::from(TEST_FILE_PATH)
    }

    fn a_download_for_create(user_id: Uuid) -> DownloadForCreate {
        DownloadForCreate::new(user_id, Uuid::new_v4(), test_file_path())
    }

    #[tokio::test]
    async fn test_create_download_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let _ = build_server_anonymous(config.clone()).await?;
        let user_id = User::all(&state.mm).await?[0].id;

        let dl_c = a_download_for_create(user_id);
        let result = Download::create(&state.mm, dl_c).await;

        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_token_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let _ = build_server_anonymous(config.clone()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let token = Uuid::new_v4();
        let dl_c = DownloadForCreate::new(user_id, token, test_file_path());
        Download::create(&state.mm, dl_c).await?;

        let dl = Download::find_by_token(&state.mm, token).await?;

        assert!(dl.is_some());
        let dl = dl.unwrap();
        pretty_assertions::assert_eq!(dl.token, token);
        pretty_assertions::assert_eq!(dl.user_id, user_id);
        pretty_assertions::assert_eq!(dl.file_path, test_file_path());
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_token_not_found() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let _ = build_server_anonymous(config.clone()).await?;

        let dl = Download::find_by_token(&state.mm, Uuid::new_v4()).await?;

        assert!(dl.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_token_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let _ = build_server_anonymous(config.clone()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let file_path = test_file_path();
        tokio::fs::write(&file_path, b"dummy content").await?;
        let token = Uuid::new_v4();
        let dl_c = DownloadForCreate::new(user_id, token, file_path.clone());
        Download::create(&state.mm, dl_c).await?;

        Download::delete_by_token(&state.mm, token, file_path.to_string_lossy().into_owned())
            .await?;

        assert!(Download::find_by_token(&state.mm, token).await?.is_none());
        assert!(!tokio::fs::try_exists(&file_path).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_token_missing_file_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let _ = build_server_anonymous(config.clone()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let token = Uuid::new_v4();
        let dl_c = DownloadForCreate::new(user_id, token, PathBuf::from("/tmp/nonexistent.zip"));
        Download::create(&state.mm, dl_c).await?;

        let res =
            Download::delete_by_token(&state.mm, token, "/tmp/nonexistent.zip".to_string()).await;

        assert!(res.is_err());
        Ok(())
    }
}
