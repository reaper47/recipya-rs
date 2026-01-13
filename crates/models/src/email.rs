use std::time::Duration;

use auth::token::generate_verification_token;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Result;

/// Represents an email verification token in the database.
#[derive(Clone, Debug, PartialEq, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = schema::email_verification_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

/// A struct for creating a new email verification token from client-provided data.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::email_verification_tokens)]
pub struct EmailVerificationTokenForCreate {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
}

impl EmailVerificationTokenForCreate {
    /// Creates a new email verification token for a user.
    pub fn new(user_id: Uuid, exp_hours: u64) -> Self {
        Self {
            user_id,
            token: generate_verification_token(),
            expires_at: OffsetDateTime::now_utc() + Duration::from_hours(exp_hours),
        }
    }
}

impl EmailVerificationToken {
    /// Creates an email verification token.
    pub async fn new(mm: &ModelManager, token_c: EmailVerificationTokenForCreate) -> Result<Self> {
        let mut conn = mm.pool.get().await?;

        let token = diesel::insert_into(schema::email_verification_tokens::table)
            .values(&token_c)
            .get_result::<Self>(&mut conn)
            .await?;

        Ok(token)
    }

    /// Finds an email verification token by its token.
    pub async fn find_by_token(mm: &ModelManager, token: &str) -> Result<Option<Self>> {
        let mut conn = mm.pool.get().await?;

        let token = schema::email_verification_tokens::table
            .filter(schema::email_verification_tokens::token.eq(token))
            .first::<Self>(&mut conn)
            .await
            .optional()?;

        Ok(token)
    }

    /// Deletes a token.
    pub async fn delete(mm: &ModelManager, token: &str) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::delete(schema::email_verification_tokens::table)
            .filter(schema::email_verification_tokens::token.eq(token))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Sets the user's email as valid.
    pub async fn verify_user_email(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::update(schema::users::table)
            .filter(schema::users::id.eq(user_id))
            .set(schema::users::is_email_verified.eq(true))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Checks if the email verification token has expired.
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() > self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use testing::utils::{TestDb, create_app_state, insert_user};

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_email_token {
        use crate::user::User;

        use super::*;

        #[test]
        fn test_email_verification_token_is_expired() {
            let token = EmailVerificationToken {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                token: "test_token".to_string(),
                expires_at: OffsetDateTime::now_utc() - time::Duration::days(1),
                created_at: OffsetDateTime::now_utc(),
            };

            assert!(token.is_expired());
        }

        #[tokio::test]
        async fn test_new_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = insert_user(config).await?;
            let token = EmailVerificationTokenForCreate {
                user_id: user.id,
                token: "test_token".to_string(),
                expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
            };

            let got = EmailVerificationToken::new(&state.mm, token).await?;

            pretty_assertions::assert_eq!(
                got,
                EmailVerificationToken {
                    id: got.id,
                    user_id: user.id,
                    token: "test_token".to_string(),
                    expires_at: got.expires_at,
                    created_at: got.created_at,
                }
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_find_by_token_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = insert_user(config).await?;
            let created = EmailVerificationToken::new(
                &state.mm,
                EmailVerificationTokenForCreate {
                    user_id: user.id,
                    token: "test_token".to_string(),
                    expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
                },
            )
            .await?;

            let got = EmailVerificationToken::find_by_token(&state.mm, &created.token).await?;

            pretty_assertions::assert_eq!(
                got,
                Some(EmailVerificationToken {
                    id: created.id,
                    user_id: user.id,
                    token: "test_token".to_string(),
                    expires_at: created.expires_at,
                    created_at: created.created_at,
                })
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_find_by_token_not_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;

            let got = EmailVerificationToken::find_by_token(&state.mm, "test_token").await?;

            assert!(got.is_none());
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = insert_user(config).await?;
            let created = EmailVerificationToken::new(
                &state.mm,
                EmailVerificationTokenForCreate {
                    user_id: user.id,
                    token: "test_token".to_string(),
                    expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
                },
            )
            .await?;

            EmailVerificationToken::delete(&state.mm, &created.token).await?;

            let got = EmailVerificationToken::find_by_token(&state.mm, &created.token).await?;
            assert!(got.is_none());
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_not_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;

            EmailVerificationToken::delete(&state.mm, "test_token").await?;

            let got = EmailVerificationToken::find_by_token(&state.mm, "test_token").await?;
            assert!(got.is_none());
            Ok(())
        }

        #[tokio::test]
        async fn test_verify_user_email_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = insert_user(config).await?;

            EmailVerificationToken::verify_user_email(&state.mm, user.id).await?;

            let got = User::get_user_by_id(&state.mm, user.id).await?;
            assert!(got.unwrap().is_email_verified);
            Ok(())
        }
    }
}
