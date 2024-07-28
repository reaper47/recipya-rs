use std::sync::{Arc, Mutex};

use axum::async_trait;

use crate::ctx::Ctx;
use crate::model::user::UserForCreate;
use crate::model::Error;
use crate::{
    model,
    model::{store::RepositoryService, user::User},
};

#[derive(Default)]
pub struct MockRepository {
    users: Arc<Mutex<Vec<User>>>,
}

#[async_trait]
impl RepositoryService for MockRepository {
    async fn user_create(&self, ctx: &Ctx, user_c: UserForCreate) -> model::Result<i64> {
        let mut users = self.users.lock().unwrap();
        let id = users.len() + 1;
        users.push(User {
            id: id as i64,
            email: user_c.email,
            created_at: Default::default(),
            updated_at: Default::default(),
        });
        Ok(id as i64)
    }

    async fn user_get(&self, ctx: &Ctx, id: i64) -> model::Result<User> {
        let users = self.users.lock().unwrap();
        match users.iter().find(|u| u.id == id).cloned() {
            None => return Err(Error::EntityNotFound { entity: "user", id }),
            Some(user) => Ok(user),
        }
    }

    async fn user_delete(&self, _ctx: &Ctx, id: i64) -> model::store::Result<()> {
        todo!()
    }
    /*async fn register(&self, email: &str, plain_password: &str) -> Result<i64, Error> {
        let mut users = self.users.lock().unwrap();
        let user = models::User::new(users.len() as u64, email, plain_password);

        if users.iter().any(|u| u.email == email) {
            return Err(Error::UserExists);
        }

        users.push(user);
        Ok(users.len() as i64)
    }

    async fn users(&self) -> Vec<models::User> {
        self.users.lock().unwrap().iter().cloned().collect()
    }*/
}
