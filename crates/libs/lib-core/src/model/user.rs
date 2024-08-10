use diesel::{
    ExpressionMethods,
    {Insertable, OptionalExtension, QueryDsl, Queryable, Selectable, SelectableHelper},
};
use diesel_async::RunQueryDsl;
use lib_auth::{pwd, pwd::ContentToHash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    model::{schema, Error, Result},
    {ctx::Ctx, model::ModelManager},
};

#[derive(Clone, Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub email: String,

    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

// For app API (sent from client), e.g. UserBmc::create argument
#[derive(Deserialize)]
pub struct UserForCreate {
    pub email: String,
    pub password_clear: String,
}

// For user module implementation, e.g. inside UserBmc::create fn
#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub(in crate::model) struct UserForInsert {
    pub(in crate::model) email: String,
    pub(in crate::model) password: String,
    pub(in crate::model) password_salt: Uuid,
}

pub struct UserForLogin {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

pub struct UserForAuth {
    pub id: i64,
    pub email: String,
    pub token_salt: Uuid,
}

/// User backend model controller.
pub struct UserBmc;

impl UserBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i64> {
        let password_salt = Uuid::new_v4();
        let password = pwd::hash(&ContentToHash {
            content: user_c.password_clear,
            salt: password_salt,
        })?;

        let res = diesel::insert_into(schema::users::table)
            .values(&UserForInsert {
                email: user_c.email.to_string(),
                password,
                password_salt,
            })
            .returning(schema::users::id)
            .get_result(&mut *mm.connection().await?)
            .await?;

        Ok(res)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<User> {
        schema::users::dsl::users
            .filter(schema::users::id.eq(id))
            .select(User::as_select())
            .first(&mut *mm.connection().await?)
            .await
            .optional()?
            .ok_or(Error::EntityNotFound { entity: "user", id })
    }

    pub async fn first_by_email(
        _ctx: &Ctx,
        mm: &ModelManager,
        email: &str,
    ) -> Result<Option<User>> {
        Ok(schema::users::dsl::users
            .filter(schema::users::email.eq(email))
            .select(User::as_select())
            .first(&mut *mm.connection().await?)
            .await
            .optional()?)
    }

    pub async fn first_by_email_auth(
        _ctx: &Ctx,
        mm: &ModelManager,
        email: &str,
    ) -> Result<UserForAuth> {
        let user = Self::first_by_email(_ctx, mm, email)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user",
                id: -1,
            })?;

        Ok(UserForAuth {
            id: user.id,
            email: user.email,
            token_salt: user.token_salt,
        })
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let u = diesel::delete(schema::users::dsl::users.filter(schema::users::id.eq(id)))
            .execute(&mut *mm.connection().await?)
            .await;

        match u {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::EntityNotFound { entity: "user", id }),
        }
    }

    pub async fn update_password(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        password_clear: &str,
    ) -> Result<()> {
        let user = Self::get(ctx, mm, id).await?;
        let user = UserForLogin {
            id,
            email: user.email,
            password: user.password,
            password_salt: user.password_salt,
            token_salt: user.token_salt,
        };

        let password = pwd::hash(&ContentToHash {
            content: password_clear.to_string(),
            salt: user.password_salt,
        })?;

        diesel::update(schema::users::dsl::users.find(id))
            .set(schema::users::dsl::password.eq(password))
            .execute(&mut *mm.connection().await?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use futures::FutureExt;

    use crate::model::store::test_db::TestDb;
    use crate::model::store::Pool;

    use super::*;

    #[tokio::test]
    async fn test_create_ok() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);

                let id = UserBmc::create(
                    &ctx,
                    &mm,
                    UserForCreate {
                        email: "example@test.com".to_string(),
                        password_clear: "12345678".to_string(),
                    },
                )
                .await
                .unwrap();

                assert_eq!(id, 1);
            }
            .boxed()
        })
        .await
    }

    #[tokio::test]
    async fn test_create_err_user_exists() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);
                let _res = UserBmc::create(
                    &ctx,
                    &mm,
                    UserForCreate {
                        email: "example@test.com".to_string(),
                        password_clear: "12345678".to_string(),
                    },
                )
                .await
                .expect("should not have failed");

                let res = UserBmc::create(
                    &ctx,
                    &mm,
                    UserForCreate {
                        email: "example@test.com".to_string(),
                        password_clear: "12345678".to_string(),
                    },
                )
                .await;

                assert!(matches!(res, Err(Error::Diesel(..))));
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn test_first_by_email_ok() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);
                let fx_email = "test@example.com";
                let _ = add_user(&ctx, &mm, fx_email).await;

                let user = UserBmc::first_by_email(&ctx, &mm, fx_email)
                    .await
                    .context("Should have user 'demo1'")
                    .unwrap()
                    .unwrap();

                assert_eq!(user.email, fx_email);
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn test_get_ok() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);
                let fx_email = "example@test.com";
                let fx_id = add_user(&ctx, &mm, fx_email).await;

                let user = UserBmc::get(&ctx, &mm, fx_id).await;

                assert_eq!(user.unwrap().email, fx_email);
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn test_get_err_not_found() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);

                let res = UserBmc::get(&ctx, &mm, 100).await;

                assert!(matches!(
                    res,
                    Err(Error::EntityNotFound {
                        entity: "user",
                        id: 100
                    })
                ));
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn test_delete_ok() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);
                let fx_id = add_user(&ctx, &mm, "hello@test.com").await;

                let res = UserBmc::delete(&ctx, &mm, fx_id).await;

                assert!(res.is_ok());
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn test_delete_err_not_found() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);
                let fx_id = add_user(&ctx, &mm, "hello@test.com").await;

                let _ = UserBmc::delete(&ctx, &mm, fx_id)
                    .await
                    .expect("should have succeeded");

                let res = UserBmc::get(&ctx, &mm, fx_id).await;
                assert!(matches!(
                    res,
                    Err(Error::EntityNotFound {
                        entity: "user",
                        id: _fx_id,
                    })
                ))
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn test_update_ok() {
        let db = TestDb::new().await;
        db.run_test(|| {
            let db = db.pool.clone();

            async move {
                let (mm, ctx) = setup(db);
                let fx_id = add_user(&ctx, &mm, "hello@test.com").await;
                let old_password = UserBmc::get(&ctx, &mm, fx_id).await.unwrap().password;

                let _ = UserBmc::update_password(&ctx, &mm, fx_id, "Ukraine is Love").await;

                let _ = UserBmc::get(&ctx, &mm, fx_id).await.unwrap();
                assert_ne!(
                    old_password,
                    UserBmc::get(&ctx, &mm, fx_id).await.unwrap().password
                );
            }
            .boxed()
        })
        .await;
    }

    fn setup(db: Pool) -> (ModelManager, Ctx) {
        (ModelManager { db, email: None }, Ctx::root_ctx())
    }

    async fn add_user(ctx: &Ctx, mm: &ModelManager, email: impl Into<String>) -> i64 {
        UserBmc::create(
            &ctx,
            &mm,
            UserForCreate {
                email: email.into(),
                password_clear: "12345678".to_string(),
            },
        )
        .await
        .unwrap()
    }
}