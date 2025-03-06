use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::auth::pwd::{ContentToHash, hash_pwd};
use crate::core::model::error::{Error, Result};
use crate::core::repository::ModelManager;
use crate::core::repository::schema::users;

/// Represents a user in the system.
#[derive(Clone, Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub email: String,
    pub is_remember_me: bool,
    pub is_confirmed: bool,

    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

/// A struct representing a user for login purposes.
pub struct UserForLogin<'a> {
    pub id: i64,
    pub email: &'a str,
    pub password: &'a str,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

/// A struct for creating a new user from client-provided data.
#[derive(Deserialize)]
pub struct UserForCreate {
    pub email: String,
    pub password_clear: String,
}

/// A struct for inserting a new user into the database.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub(crate) struct UserForInsert {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) password_salt: Uuid,
}

/// A struct representing a user with authentication-related data.
pub struct UserForAuth {
    pub id: i64,
    pub email: String,
    pub token_salt: Uuid,
    pub is_remember_me: bool,
}

impl User {
    /// Deletes a user from the database.
    pub async fn delete(mm: &ModelManager, user_id: i64) -> Result<()> {
        use crate::core::repository::schema::users::dsl::*;

        let mut conn = mm.pool.get().await?;

        let num_deleted = diesel::delete(users.filter(id.eq(user_id)))
            .execute(&mut conn)
            .await?;

        match num_deleted {
            0 => Err(Error::EntityNotFound {
                entity: "user",
                id: user_id,
            }),
            _ => Ok(()),
        }
    }

    /// Finds a user by their email address.
    pub async fn get_user_by_email(
        mm: &ModelManager,
        user_email: impl Into<String>,
    ) -> Result<Option<User>> {
        use crate::core::repository::schema::users::dsl::*;

        let mut conn = mm.pool.get().await?;

        let user = users
            .filter(email.eq(user_email.into()))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .await
            .optional()?;

        Ok(user)
    }

    /// Finds a user by their user ID.
    pub async fn get_user_by_id(mm: &ModelManager, user_id: i64) -> Result<Option<User>> {
        use crate::core::repository::schema::users::dsl::*;

        let mut conn = mm.pool.get().await?;

        let user = users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .await
            .optional()?;

        Ok(user)
    }

    /// Finds user authentication data by email.
    pub async fn get_user_auth_by_email(
        mm: &ModelManager,
        user_email: impl Into<String>,
    ) -> Result<Option<UserForAuth>> {
        match Self::get_user_by_email(mm, user_email.into()).await? {
            Some(user) => Ok(Some(UserForAuth {
                id: user.id,
                email: user.email,
                token_salt: user.token_salt,
                is_remember_me: user.is_remember_me,
            })),
            None => Err(Error::EntityNotFound {
                id: -1,
                entity: "user for auth",
            }),
        }
    }

    /// Creates a new user from the provided user creation data.
    pub async fn new(mm: &ModelManager, user_c: UserForCreate) -> Result<User> {
        use crate::core::repository::schema::users::dsl::*;

        let new_password_salt = Uuid::new_v4();
        let new_password = hash_pwd(ContentToHash {
            content: user_c.password_clear,
            salt: new_password_salt,
        })
        .await?;

        let mut conn = mm.pool.get().await?;

        let user = diesel::insert_into(users)
            .values(&UserForInsert {
                email: user_c.email.to_string(),
                password: new_password,
                password_salt: new_password_salt,
            })
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(user)
    }

    /// Marks the user as confirmed in the database.
    pub async fn set_is_confirmed(&self, mm: &ModelManager) -> Result<()> {
        use crate::core::repository::schema::users::dsl::*;

        let mut conn = mm.pool.get().await?;

        diesel::update(users.find(self.id))
            .set(is_confirmed.eq(true))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Updates the user's password.
    pub async fn update_password(
        &self,
        mm: &ModelManager,
        password_clear: impl Into<String>,
    ) -> Result<()> {
        use crate::core::repository::schema::users::dsl::*;

        let user = UserForLogin {
            id: self.id,
            email: &self.email,
            password: &self.password,
            password_salt: self.password_salt,
            token_salt: self.token_salt,
        };

        let hashed_password = hash_pwd(ContentToHash {
            content: password_clear.into(),
            salt: user.password_salt,
        })
        .await?;

        let mut conn = mm.pool.get().await?;

        diesel::update(users.find(id))
            .set(password.eq(hashed_password))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Updates the user password for the specified user ID.
    pub async fn update_password_by_user_id(
        mm: &ModelManager,
        id: i64,
        new_password: &str,
    ) -> Result<()> {
        match User::get_user_by_id(mm, id).await? {
            Some(user) => user.update_password(mm, new_password).await,
            None => Err(Error::EntityNotFound {
                id: -1,
                entity: "user for auth",
            }),
        }
    }

    /// Updates the "remember me" flag for the user.
    pub async fn update_remember_me(
        mm: &ModelManager,
        user_id: i64,
        new_value: bool,
    ) -> Result<()> {
        use crate::core::repository::schema::users::dsl::*;

        // TODO: Remove .unwrap()
        let mut conn = mm.pool.get().await?;

        diesel::update(users.find(user_id))
            .set(is_remember_me.eq(new_value))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::server::AppState;
    use crate::server::test_utils::{TEST_USER_EMAIL, TestDb, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_user_new_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;

        insert_user(config.clone()).await?;

        let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("Test user should have been present");
        pretty_assertions::assert_eq!(user.email, TEST_USER_EMAIL);
        Ok(())
    }

    #[tokio::test]
    async fn test_user_new_already_exists_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        insert_user(config.clone()).await?;

        let res = insert_user(config.clone()).await;

        assert!(res.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user_not_exist_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;

        match User::delete(&state.mm, 999).await {
            Ok(_) => Err("An error was supposed to be thrown".into()),
            Err(_) => Ok(()),
        }
    }

    #[tokio::test]
    async fn test_delete_user_exists_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;

        User::delete(&state.mm, user.id).await?;

        match User::get_user_by_email(&state.mm, TEST_USER_EMAIL).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    #[tokio::test]
    async fn test_get_user_by_id_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;

        let got_user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");

        pretty_assertions::assert_eq!(user.email, got_user.email);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_auth_by_email_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;

        let got_user = User::get_user_auth_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("User should have been present");

        pretty_assertions::assert_eq!(user.email, got_user.email);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_is_confirmed_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;

        user.set_is_confirmed(&state.mm).await?;

        let user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");
        pretty_assertions::assert_eq!(user.is_confirmed, true);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;
        let password_before = String::from(&user.password);

        user.update_password(&state.mm, "new password").await?;

        let password_after = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present")
            .password;
        pretty_assertions::assert_ne!(password_before, password_after);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_by_user_id_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;
        let password_before = user.password;

        User::update_password_by_user_id(&state.mm, user.id, "new password").await?;

        let user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");
        let password_after = user.password;
        pretty_assertions::assert_ne!(password_before, password_after);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_remember_me_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = AppState::new(config.clone()).await?;
        let user = insert_user(config.clone()).await?;

        User::update_remember_me(&state.mm, user.id, true).await?;

        let user = User::get_user_by_id(&state.mm, user.id)
            .await?
            .expect("User should have been present");
        pretty_assertions::assert_eq!(user.is_remember_me, true);
        Ok(())
    }
}
