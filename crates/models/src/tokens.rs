use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use jiff::{SignedDuration, Timestamp, ToSpan};
use jiff_diesel::ToDiesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use auth::token::generate_verification_token;
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
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub created_at: Timestamp,
}

/// A struct for creating a new email verification token from client-provided data.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::email_verification_tokens)]
pub struct EmailVerificationTokenForCreate {
    pub user_id: Uuid,
    pub token: String,
    #[diesel(serialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
}

impl EmailVerificationTokenForCreate {
    /// Creates a new email verification token for a user.
    pub fn new(user_id: Uuid, exp_hours: i64) -> Self {
        Self {
            user_id,
            token: generate_verification_token(),
            expires_at: Timestamp::now() + exp_hours.hours(),
        }
    }
}

impl EmailVerificationToken {
    /// Creates an email verification token.
    pub async fn new(mm: &ModelManager, token_c: EmailVerificationTokenForCreate) -> Result<Self> {
        let token = diesel::insert_into(schema::email_verification_tokens::table)
            .values(token_c)
            .returning(Self::as_returning())
            .get_result(&mut mm.pool.get().await?)
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
        Timestamp::now() > self.expires_at
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
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub created_at: Timestamp,
}

/// A struct for creating a new password reset token from client-provided data.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::password_reset_tokens)]
pub struct PasswordResetTokenForCreate {
    pub user_id: Uuid,
    pub token: String,
    #[diesel(serialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
}

impl PasswordResetTokenForCreate {
    /// Creates a new password reset token for a user.
    pub fn new(user_id: Uuid, exp_hours: i64) -> Self {
        Self {
            user_id,
            token: generate_verification_token(),
            expires_at: Timestamp::now() + exp_hours.hours(),
        }
    }
}

impl PasswordResetToken {
    /// Creates a new token.
    pub async fn new(mm: &ModelManager, token_c: PasswordResetTokenForCreate) -> Result<Self> {
        let token = diesel::insert_into(schema::password_reset_tokens::table)
            .values(token_c)
            .returning(Self::as_returning())
            .get_result(&mut mm.pool.get().await?)
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
        Timestamp::now() > self.expires_at
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
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
    pub is_used: bool,
    #[diesel(serialize_as = jiff_diesel::NullableTimestamp, deserialize_as = jiff_diesel::NullableTimestamp)]
    pub used_at: Option<Timestamp>,
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub created_at: Timestamp,
}

/// A struct for creating a new password reset token from client-provided data.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::refresh_tokens)]
pub struct RefreshTokenForCreate {
    pub user_id: Uuid,
    pub is_remember_me: bool,
    pub token: String,
    #[diesel(serialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
}

impl RefreshTokenForCreate {
    /// Creates a refresh token for a user.
    pub fn new(user_id: Uuid, is_remember_me: bool) -> Self {
        let ttl = if is_remember_me {
            SignedDuration::from_hours(30)
        } else {
            SignedDuration::ZERO
        };

        Self {
            user_id,
            is_remember_me,
            token: generate_verification_token(),
            expires_at: Timestamp::now() + ttl,
        }
    }

    /// Creates a refresh token for a user with an expiry date.
    pub fn with_expiry(user_id: Uuid, is_remember_me: bool, expires_at: Timestamp) -> Self {
        Self {
            user_id,
            is_remember_me,
            token: generate_verification_token(),
            expires_at,
        }
    }
}

impl RefreshToken {
    /// Creates a new token.
    pub async fn new(mm: &ModelManager, token_c: RefreshTokenForCreate) -> Result<Self> {
        let token = diesel::insert_into(schema::refresh_tokens::table)
            .values(token_c)
            .returning(Self::as_returning())
            .get_result(&mut mm.pool.get().await?)
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
                schema::refresh_tokens::used_at.eq(Timestamp::now().to_diesel()),
            ))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Checks whether the token has expired.
    pub fn is_expired(&self) -> bool {
        Timestamp::now() > self.expires_at
    }

    // Check if token is valid (not expired AND not used)
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used
    }
}
