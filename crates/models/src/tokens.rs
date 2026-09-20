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
#[derive(Clone, Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Serialize)]
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
        let token = diesel::insert_into(schema::email_verification_tokens::table)
            .values(&token_c)
            .get_result::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(token)
    }

    /// Finds an email verification token by its token.
    pub async fn find_by_token(mm: &ModelManager, token: &str) -> Result<Option<Self>> {
        let token = schema::email_verification_tokens::table
            .filter(schema::email_verification_tokens::token.eq(token))
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .optional()?;

        Ok(token)
    }

    /// Deletes a token.
    pub async fn delete(mm: &ModelManager, token: &str) -> Result<()> {
        diesel::delete(schema::email_verification_tokens::table)
            .filter(schema::email_verification_tokens::token.eq(token))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Sets the user's email as valid.
    pub async fn verify_user_email(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        diesel::update(schema::users::table)
            .filter(schema::users::id.eq(user_id))
            .set(schema::users::is_email_verified.eq(true))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Checks whether the token has expired.
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() > self.expires_at
    }
}

/// Represents a password reset token in the database.
#[derive(Clone, Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Serialize)]
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
    /// Creates a new password reset token for a user.
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
        let token = diesel::insert_into(schema::password_reset_tokens::table)
            .values(&token_c)
            .get_result::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(token)
    }

    /// Finds a password token by its token.
    pub async fn find_by_token(mm: &ModelManager, token: &str) -> Result<Option<Self>> {
        let token = schema::password_reset_tokens::table
            .filter(schema::password_reset_tokens::token.eq(token))
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .optional()?;

        Ok(token)
    }

    /// Deletes a token.
    pub async fn delete(mm: &ModelManager, token: &str) -> Result<()> {
        diesel::delete(schema::password_reset_tokens::table)
            .filter(schema::password_reset_tokens::token.eq(token))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Deletes all tokens for a user.
    pub async fn delete_all_for_user(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        diesel::delete(schema::password_reset_tokens::table)
            .filter(schema::password_reset_tokens::user_id.eq(user_id))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Checks whether the token has expired.
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() > self.expires_at
    }
}

/// Represents a refresh token in the database.
#[derive(Clone, Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = schema::refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_remember_me: bool,
    pub token: String,
    pub expires_at: OffsetDateTime,
    pub is_used: bool,
    pub used_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

/// A struct for creating a new password reset token from client-provided data.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::refresh_tokens)]
pub struct RefreshTokenForCreate {
    pub user_id: Uuid,
    pub is_remember_me: bool,
    pub token: String,
}

impl RefreshTokenForCreate {
    /// Creates a refresh token for a user.
    pub fn new(user_id: Uuid, is_remember_me: bool) -> Self {
        Self {
            user_id,
            is_remember_me,
            token: generate_verification_token(),
        }
    }
}

impl RefreshToken {
    /// Creates a new token.
    pub async fn new(mm: &ModelManager, token_c: RefreshTokenForCreate) -> Result<Self> {
        let token = diesel::insert_into(schema::refresh_tokens::table)
            .values(&token_c)
            .get_result::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(token)
    }

    /// Finds a refresh token by its token.
    pub async fn find_by_token(mm: &ModelManager, token: &str) -> Result<Option<Self>> {
        let token = schema::refresh_tokens::table
            .filter(schema::refresh_tokens::token.eq(token))
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .optional()?;

        Ok(token)
    }

    /// Deletes the token.
    pub async fn delete(mm: &ModelManager, token: &str) -> Result<()> {
        diesel::delete(schema::refresh_tokens::table)
            .filter(schema::refresh_tokens::token.eq(token))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Deletes all the user's tokens.
    pub async fn delete_all_for_user(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        diesel::delete(schema::refresh_tokens::table)
            .filter(schema::refresh_tokens::user_id.eq(user_id))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Marks a token as used.
    pub async fn mark_as_used(mm: &ModelManager, token: &str) -> Result<()> {
        diesel::update(schema::refresh_tokens::table)
            .filter(schema::refresh_tokens::token.eq(token))
            .set((
                schema::refresh_tokens::is_used.eq(true),
                schema::refresh_tokens::used_at.eq(OffsetDateTime::now_utc()),
            ))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Checks whether the token has expired.
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() > self.expires_at
    }

    // Check if token is valid (not expired AND not used)
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used
    }
}
