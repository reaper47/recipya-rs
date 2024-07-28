use uuid::Uuid;

use crate::model::{Error, Result};
use crate::{ctx::Ctx, model::ModelManager};

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct UserForCreate {
    pub email: String,
    pub pwd_clear: String,
}

/*impl TryFrom<Row> for User {
    type Error = Error;

    fn try_from(row: Row) -> Result<Self> {
        Ok(User {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
        })
    }
}*/

pub struct UserForInsert {
    pub email: String,
}

pub struct UserForLogin {
    pub id: i64,
    pub email: String,
    pub password_encrypted: Option<String>,
}

pub struct UserForAuth {
    pub id: i64,
    pub email: String,
    pub token_salt: Uuid,
}

pub struct UserBmc;

impl UserBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i64> {
        mm.db().user_create(ctx, user_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<User> {
        mm.db().user_get(ctx, id).await
    }

    pub async fn first_by_email(
        ctx: &Ctx,
        mm: &ModelManager,
        email: String,
    ) -> Result<Option<User>> {
        todo!()
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new_test();
        let ctx = Ctx::root_ctx();

        let user = UserBmc::get(&ctx, &mm, i64::MAX).await;

        assert!(user.is_err());
        assert!(matches!(
            user,
            Err(Error::EntityNotFound {
                entity: "user",
                id: i64::MAX
            })
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = ModelManager::new_test();
        let ctx = Ctx::root_ctx();
        let fx_email = "test@example.com";

        let user_c = UserForCreate {
            email: fx_email.to_string(),
            pwd_clear: "12345678".to_string(),
        };
        let id = UserBmc::create(&ctx, &mm, user_c).await?;

        let user = UserBmc::get(&ctx, &mm, id).await?;
        assert_eq!(user.email, fx_email);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_ok() -> Result<()> {
        todo!()
    }
}
