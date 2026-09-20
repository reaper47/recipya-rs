use std::time::Duration;

use time::OffsetDateTime;
use uuid::Uuid;

use models::tokens::{
    EmailVerificationToken, EmailVerificationTokenForCreate, PasswordResetToken,
    PasswordResetTokenForCreate, RefreshToken, RefreshTokenForCreate,
};
use models::user::User;
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_email_token {
    use super::*;

    #[test]
    fn test_token_is_expired() {
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
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
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
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
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
        let state = create_app_state(default_config()).await;

        let got = EmailVerificationToken::find_by_token(&state.mm, "test_token").await?;

        assert!(got.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
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
        let state = create_app_state(default_config()).await;

        EmailVerificationToken::delete(&state.mm, "test_token").await?;

        let got = EmailVerificationToken::find_by_token(&state.mm, "test_token").await?;
        assert!(got.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_user_email_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;

        EmailVerificationToken::verify_user_email(&state.mm, user.id).await?;

        let got = User::get_user_by_id(&state.mm, user.id).await?;
        assert!(got.unwrap().is_email_verified);
        Ok(())
    }
}

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

    #[tokio::test]
    async fn test_delete_all_for_user_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created1 = PasswordResetToken::new(
            &state.mm,
            PasswordResetTokenForCreate {
                user_id: user.id,
                token: "test_token".to_string(),
                expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
            },
        )
        .await?;
        let created2 = PasswordResetToken::new(
            &state.mm,
            PasswordResetTokenForCreate {
                user_id: user.id,
                token: "test_token2".to_string(),
                expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
            },
        )
        .await?;

        PasswordResetToken::delete_all_for_user(&state.mm, user.id).await?;

        let got1 = PasswordResetToken::find_by_token(&state.mm, &created1.token).await?;
        let got2 = PasswordResetToken::find_by_token(&state.mm, &created2.token).await?;
        assert!(got1.is_none());
        assert!(got2.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created = PasswordResetToken::new(
            &state.mm,
            PasswordResetTokenForCreate {
                user_id: user.id,
                token: "test_token".to_string(),
                expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
            },
        )
        .await?;

        PasswordResetToken::delete(&state.mm, &created.token).await?;

        let got = PasswordResetToken::find_by_token(&state.mm, &created.token).await?;
        assert!(got.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_token_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created = PasswordResetToken::new(
            &state.mm,
            PasswordResetTokenForCreate {
                user_id: user.id,
                token: "test_token".to_string(),
                expires_at: OffsetDateTime::now_utc() + time::Duration::days(1),
            },
        )
        .await?;

        let got = PasswordResetToken::find_by_token(&state.mm, &created.token).await?;

        pretty_assertions::assert_eq!(
            got,
            Some(PasswordResetToken {
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
        let state = create_app_state(default_config()).await;

        let got = PasswordResetToken::find_by_token(&state.mm, "test_token").await?;

        assert!(got.is_none());
        Ok(())
    }
}

mod tests_refresh_token {
    use super::*;

    #[test]
    fn test_is_expired() {
        let token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            is_remember_me: false,
            token: "test_token".to_string(),
            expires_at: OffsetDateTime::now_utc() - Duration::from_hours(24),
            is_used: false,
            used_at: None,
            created_at: OffsetDateTime::now_utc(),
        };

        assert!(token.is_expired());
    }

    #[test]
    fn test_is_not_expired() {
        let token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            is_remember_me: false,
            token: "test_token".to_string(),
            expires_at: OffsetDateTime::now_utc() + Duration::from_hours(24),
            is_used: false,
            used_at: None,
            created_at: OffsetDateTime::now_utc(),
        };

        assert!(!token.is_expired());
    }

    #[test]
    fn test_is_valid() {
        let token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            is_remember_me: false,
            token: "test_token".to_string(),
            expires_at: OffsetDateTime::now_utc() + Duration::from_hours(24),
            is_used: false,
            used_at: None,
            created_at: OffsetDateTime::now_utc(),
        };

        assert!(token.is_valid());
    }

    #[tokio::test]
    async fn test_delete_all_for_user_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created1 = RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
            },
        )
        .await?;
        let created2 = RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: "test_token2".to_string(),
            },
        )
        .await?;

        RefreshToken::delete_all_for_user(&state.mm, user.id).await?;

        let got1 = RefreshToken::find_by_token(&state.mm, &created1.token).await?;
        let got2 = RefreshToken::find_by_token(&state.mm, &created2.token).await?;
        assert!(got1.is_none());
        assert!(got2.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created = RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
            },
        )
        .await?;

        RefreshToken::delete(&state.mm, &created.token).await?;

        let got = RefreshToken::find_by_token(&state.mm, &created.token).await?;
        assert!(got.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_token_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created = RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
            },
        )
        .await?;

        let got = RefreshToken::find_by_token(&state.mm, &created.token).await?;

        pretty_assertions::assert_eq!(
            got,
            Some(RefreshToken {
                id: created.id,
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
                created_at: created.created_at,
                expires_at: created.expires_at,
                is_used: false,
                used_at: None
            })
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_token_not_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created = RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
            },
        )
        .await?;

        let got = RefreshToken::find_by_token(&state.mm, &created.token).await?;

        pretty_assertions::assert_eq!(
            got,
            Some(RefreshToken {
                id: created.id,
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
                created_at: created.created_at,
                expires_at: created.expires_at,
                is_used: false,
                used_at: None
            })
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_mark_as_used_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let created = RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: "test_token".to_string(),
            },
        )
        .await?;

        RefreshToken::mark_as_used(&state.mm, &created.token).await?;

        let got = RefreshToken::find_by_token(&state.mm, &created.token).await?;
        assert!(got.unwrap().is_used);
        Ok(())
    }
}
