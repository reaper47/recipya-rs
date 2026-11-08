mod common;

use std::sync::LazyLock;

use divan::{Bencher, black_box};
use time::OffsetDateTime;
use uuid::Uuid;

use models::tokens::{
    EmailVerificationToken, EmailVerificationTokenForCreate, PasswordResetToken,
    PasswordResetTokenForCreate, RefreshToken, RefreshTokenForCreate,
};
use test_db::default_config;
use test_fixtures::insert_other_user;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::common::{RUNTIME, Seeded};

const AN_EMAIL_TOKEN: &str = "email token";
const A_PASSWORD_TOKEN: &str = "pwd token";
const A_REFRESH_TOKEN: &str = "refresh token";
const A_MISSING_TOKEN: &str = "missing token";

static SEEDED: LazyLock<Seeded> = LazyLock::new(|| {
    RUNTIME.block_on(async {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await.unwrap();

        EmailVerificationToken::new(
            &state.mm,
            EmailVerificationTokenForCreate {
                user_id: user.id,
                token: AN_EMAIL_TOKEN.into(),
                expires_at: in_one_day(),
            },
        )
        .await
        .unwrap();

        PasswordResetToken::new(
            &state.mm,
            PasswordResetTokenForCreate {
                user_id: user.id,
                token: A_PASSWORD_TOKEN.into(),
                expires_at: in_one_day(),
            },
        )
        .await
        .unwrap();

        RefreshToken::new(
            &state.mm,
            RefreshTokenForCreate {
                user_id: user.id,
                is_remember_me: false,
                token: A_REFRESH_TOKEN.into(),
            },
        )
        .await
        .unwrap();

        Seeded { state, user }
    })
});

fn in_one_day() -> OffsetDateTime {
    OffsetDateTime::now_utc() + time::Duration::days(1)
}

fn unique_token() -> String {
    format!("token-{}", Uuid::new_v4())
}

fn main() {
    let s = &*SEEDED;

    RUNTIME.block_on(async {
        let mm = &s.state.mm;
        assert!(
            EmailVerificationToken::find_by_token(mm, AN_EMAIL_TOKEN)
                .await
                .unwrap()
                .is_some()
        );
        assert!(
            PasswordResetToken::find_by_token(mm, A_PASSWORD_TOKEN)
                .await
                .unwrap()
                .is_some()
        );
        assert!(
            RefreshToken::find_by_token(mm, A_REFRESH_TOKEN)
                .await
                .unwrap()
                .is_some()
        );
    });

    divan::main();
}

#[divan::bench_group]
mod expiry {
    use super::{
        Bencher, EmailVerificationToken, OffsetDateTime, PasswordResetToken, RefreshToken, Uuid,
        black_box, in_one_day,
    };

    #[divan::bench]
    fn email_token_is_expired(bencher: Bencher) {
        let token = EmailVerificationToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token: "token".into(),
            expires_at: OffsetDateTime::now_utc() - time::Duration::days(1),
            created_at: OffsetDateTime::now_utc(),
        };

        bencher.bench_local(|| black_box(&token).is_expired());
    }

    #[divan::bench]
    fn password_token_is_expired(bencher: Bencher) {
        let token = PasswordResetToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token: "token".into(),
            expires_at: OffsetDateTime::now_utc() - time::Duration::days(1),
            created_at: OffsetDateTime::now_utc(),
        };

        bencher.bench_local(|| black_box(&token).is_expired());
    }

    #[divan::bench(args = [true, false])]
    fn refresh_token_is_expired(bencher: Bencher, expired: bool) {
        let token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token: "token".into(),
            expires_at: if expired {
                OffsetDateTime::now_utc() - time::Duration::days(1)
            } else {
                in_one_day()
            },
            created_at: OffsetDateTime::now_utc(),
            is_remember_me: false,
            is_used: false,
            used_at: None,
        };

        bencher.bench_local(|| black_box(&token).is_expired());
    }

    #[divan::bench(args = [true, false])]
    fn refresh_token_is_valid(bencher: Bencher, expired: bool) {
        let token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token: "token".into(),
            expires_at: if expired {
                OffsetDateTime::now_utc() - time::Duration::days(1)
            } else {
                in_one_day()
            },
            created_at: OffsetDateTime::now_utc(),
            is_remember_me: false,
            is_used: false,
            used_at: None,
        };

        bencher.bench_local(|| black_box(&token).is_valid());
    }
}

#[divan::bench_group]
mod find_by_token {
    use super::{
        A_MISSING_TOKEN, A_PASSWORD_TOKEN, A_REFRESH_TOKEN, AN_EMAIL_TOKEN, Bencher,
        EmailVerificationToken, PasswordResetToken, RUNTIME, RefreshToken, SEEDED,
    };

    #[divan::bench(args = [AN_EMAIL_TOKEN, A_MISSING_TOKEN])]
    fn email(bencher: Bencher, token: &str) {
        let mm = &SEEDED.state.mm;

        bencher.bench_local(|| RUNTIME.block_on(EmailVerificationToken::find_by_token(mm, token)));
    }

    #[divan::bench(args = [A_PASSWORD_TOKEN, A_MISSING_TOKEN])]
    fn password_reset(bencher: Bencher, token: &str) {
        let mm = &SEEDED.state.mm;

        bencher.bench_local(|| RUNTIME.block_on(PasswordResetToken::find_by_token(mm, token)));
    }

    #[divan::bench(args = [A_REFRESH_TOKEN, A_MISSING_TOKEN])]
    fn refresh(bencher: Bencher, token: &str) {
        let mm = &SEEDED.state.mm;

        bencher.bench_local(|| RUNTIME.block_on(RefreshToken::find_by_token(mm, token)));
    }
}

#[divan::bench_group]
mod new {
    use super::{
        Bencher, EmailVerificationToken, EmailVerificationTokenForCreate, PasswordResetToken,
        PasswordResetTokenForCreate, RUNTIME, RefreshToken, RefreshTokenForCreate, SEEDED,
        in_one_day, unique_token,
    };

    #[divan::bench]
    fn email(bencher: Bencher) {
        let s = &SEEDED;

        bencher
            .with_inputs(|| EmailVerificationTokenForCreate {
                user_id: s.user.id,
                token: unique_token(),
                expires_at: in_one_day(),
            })
            .bench_values(|token| {
                RUNTIME.block_on(EmailVerificationToken::new(&s.state.mm, token))
            });
    }

    #[divan::bench]
    fn password_reset(bencher: Bencher) {
        let s = &SEEDED;

        bencher
            .with_inputs(|| PasswordResetTokenForCreate {
                user_id: s.user.id,
                token: unique_token(),
                expires_at: in_one_day(),
            })
            .bench_values(|token| RUNTIME.block_on(PasswordResetToken::new(&s.state.mm, token)));
    }

    #[divan::bench]
    fn refresh(bencher: Bencher) {
        let s = &SEEDED;

        bencher
            .with_inputs(|| RefreshTokenForCreate {
                user_id: s.user.id,
                token: unique_token(),
                is_remember_me: false,
            })
            .bench_values(|token| RUNTIME.block_on(RefreshToken::new(&s.state.mm, token)));
    }
}

#[divan::bench_group]
mod delete {
    use super::{
        A_MISSING_TOKEN, Bencher, EmailVerificationToken, EmailVerificationTokenForCreate,
        PasswordResetToken, PasswordResetTokenForCreate, RUNTIME, RefreshToken,
        RefreshTokenForCreate, SEEDED, in_one_day, unique_token,
    };

    #[divan::bench]
    fn email(bencher: Bencher) {
        let s = &SEEDED;

        bencher
            .with_inputs(|| {
                RUNTIME
                    .block_on(EmailVerificationToken::new(
                        &s.state.mm,
                        EmailVerificationTokenForCreate {
                            user_id: s.user.id,
                            token: unique_token(),
                            expires_at: in_one_day(),
                        },
                    ))
                    .unwrap()
                    .token
            })
            .bench_values(|token| {
                RUNTIME.block_on(EmailVerificationToken::delete(&s.state.mm, &token))
            });
    }

    #[divan::bench]
    fn password_reset(bencher: Bencher) {
        let s = &SEEDED;

        bencher
            .with_inputs(|| {
                RUNTIME
                    .block_on(PasswordResetToken::new(
                        &s.state.mm,
                        PasswordResetTokenForCreate {
                            user_id: s.user.id,
                            token: unique_token(),
                            expires_at: in_one_day(),
                        },
                    ))
                    .unwrap()
                    .token
            })
            .bench_values(|token| {
                RUNTIME.block_on(PasswordResetToken::delete(&s.state.mm, &token))
            });
    }

    #[divan::bench]
    fn refresh(bencher: Bencher) {
        let s = &SEEDED;

        bencher
            .with_inputs(|| {
                RUNTIME
                    .block_on(RefreshToken::new(
                        &s.state.mm,
                        RefreshTokenForCreate {
                            user_id: s.user.id,
                            token: unique_token(),
                            is_remember_me: false,
                        },
                    ))
                    .unwrap()
                    .token
            })
            .bench_values(|token| RUNTIME.block_on(RefreshToken::delete(&s.state.mm, &token)));
    }

    #[divan::bench]
    fn email_not_found(bencher: Bencher) {
        let mm = &SEEDED.state.mm;

        bencher
            .bench_local(|| RUNTIME.block_on(EmailVerificationToken::delete(mm, A_MISSING_TOKEN)));
    }
}

#[divan::bench_group]
mod delete_all_for_user {
    use super::{
        Bencher, PasswordResetToken, PasswordResetTokenForCreate, RUNTIME, RefreshToken,
        RefreshTokenForCreate, SEEDED, Uuid, in_one_day, insert_other_user, unique_token,
    };

    #[divan::bench]
    fn password_reset(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                RUNTIME.block_on(async {
                    let user =
                        insert_other_user(&s.state.mm, &format!("{}@test.bench", Uuid::new_v4()))
                            .await
                            .unwrap();

                    for _ in 0..2 {
                        PasswordResetToken::new(
                            &s.state.mm,
                            PasswordResetTokenForCreate {
                                user_id: user.id,
                                token: unique_token(),
                                expires_at: in_one_day(),
                            },
                        )
                        .await
                        .unwrap();
                    }
                    user.id
                })
            })
            .bench_values(|id| {
                RUNTIME.block_on(PasswordResetToken::delete_all_for_user(&s.state.mm, id))
            });
    }

    #[divan::bench]
    fn refresh(bencher: Bencher) {
        let s = &*SEEDED;

        bencher
            .with_inputs(|| {
                RUNTIME.block_on(async {
                    let user =
                        insert_other_user(&s.state.mm, &format!("{}@test.bench", Uuid::new_v4()))
                            .await
                            .unwrap();

                    for _ in 0..2 {
                        RefreshToken::new(
                            &s.state.mm,
                            RefreshTokenForCreate {
                                user_id: user.id,
                                token: unique_token(),
                                is_remember_me: false,
                            },
                        )
                        .await
                        .unwrap();
                    }
                    user.id
                })
            })
            .bench_values(|id| {
                RUNTIME.block_on(RefreshToken::delete_all_for_user(&s.state.mm, id))
            });
    }
}

#[divan::bench_group]
mod updates {
    use super::{A_REFRESH_TOKEN, Bencher, EmailVerificationToken, RUNTIME, RefreshToken, SEEDED};

    #[divan::bench]
    fn mark_used(bencher: Bencher) {
        let mm = &SEEDED.state.mm;

        bencher.bench_local(|| RUNTIME.block_on(RefreshToken::mark_as_used(mm, A_REFRESH_TOKEN)));
    }

    #[divan::bench]
    fn verify_user_email(bencher: Bencher) {
        let s = &*SEEDED;

        bencher.bench_local(|| {
            RUNTIME.block_on(EmailVerificationToken::verify_user_email(
                &s.state.mm,
                s.user.id,
            ))
        });
    }
}
