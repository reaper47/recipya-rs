pub mod assert;
pub mod server;

#[cfg(test)]
pub mod test_utils {
    use std::{collections::HashMap, sync::Arc};

    use lib_auth::token::{generate_web_token, Token};
    use lib_core::model::store::test_db::TestDb;
    use lib_core::{
        ctx::Ctx,
        model::{
            user::{UserBmc, UserForCreate},
            ModelManager,
        },
    };
    use lib_web::AppState;
    use tokio::sync::Mutex;

    pub fn state(db: TestDb) -> AppState {
        AppState {
            mm: ModelManager {
                db: db.pool,
                email: None,
            },
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_token(mm: ModelManager) -> Token {
        let ctx = Ctx::root_ctx();
        let email = "confirm@test.com".to_string();

        UserBmc::create(
            &ctx,
            &mm,
            UserForCreate {
                email: email.clone(),
                password_clear: "12345678".to_string(),
            },
        )
        .await
        .unwrap();

        let user = UserBmc::first_by_email(&ctx, &mm, &email)
            .await
            .unwrap()
            .unwrap();

        generate_web_token(&user.email, user.token_salt).unwrap()
    }
}
