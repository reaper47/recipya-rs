use crate::services::repository::RepositoryService;
use crate::Error::UserExists;
use crate::{models, Error};
use axum::async_trait;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MockRepository {
    users: Arc<Mutex<Vec<models::User>>>,
}

#[async_trait]
impl RepositoryService for MockRepository {
    async fn register(&self, email: &str, plain_password: &str) -> Result<i64, Error> {
        let mut users = self.users.lock().unwrap();
        let user = models::User::new(users.len() as u64, email, plain_password);

        if users.iter().any(|u| u.email == email) {
            return Err(UserExists);
        }

        users.push(user);
        Ok(users.len() as i64)
    }

    async fn users(&self) -> Vec<models::User> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .map(|u| u.clone())
            .collect()
    }
}
