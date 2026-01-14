use std::time::Duration;

use auth::token::generate_verification_token;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Result;

/// Represents a password reset token in the database.
#[derive(Clone, Debug, PartialEq, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = schema::password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

/// A struct for creating a new password reset token from client-provided data.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::password_reset_tokens)]
pub struct PasswordResetTokenForCreate {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
}

impl PasswordResetTokenForCreate {
    /// Creates a new email verification token for a user.
    pub fn new(user_id: Uuid, exp_hours: u64) -> Self {
        Self {
            user_id,
            token: generate_verification_token(),
            expires_at: OffsetDateTime::now_utc() + Duration::from_hours(exp_hours),
        }
    }
}

impl PasswordResetToken {
    /// Creates a new token.
    pub async fn new(mm: &ModelManager, token_c: PasswordResetTokenForCreate) -> Result<Self> {
        let mut conn = mm.pool.get().await?;

        let token = diesel::insert_into(schema::password_reset_tokens::table)
            .values(&token_c)
            .get_result::<Self>(&mut conn)
            .await?;

        Ok(token)
    }

    /// Finds an a password token by its token.
    pub async fn find_by_token(mm: &ModelManager, token: &str) -> Result<Option<Self>> {
        let mut conn = mm.pool.get().await?;

        let token = schema::password_reset_tokens::table
            .filter(schema::password_reset_tokens::token.eq(token))
            .first::<Self>(&mut conn)
            .await
            .optional()?;

        Ok(token)
    }

    /// Deletes a token.
    pub async fn delete(mm: &ModelManager, token: &str) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::delete(schema::password_reset_tokens::table)
            .filter(schema::password_reset_tokens::token.eq(token))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Deletes all tokens for a user.
    pub async fn delete_all_for_user(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::delete(schema::password_reset_tokens::table)
            .filter(schema::password_reset_tokens::user_id.eq(user_id))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Checks whether the token has expired.
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() > self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_password_reset_token {
        use super::*;

        #[test]
        fn test_token_is_expired() {
            let token = PasswordResetToken {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                token: "test_token".to_string(),
                expires_at: OffsetDateTime::now_utc() - time::Duration::days(1),
                created_at: OffsetDateTime::now_utc(),
            };

            assert!(token.is_expired());
        }
    }
}
