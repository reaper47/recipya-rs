#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub email: String,
    hashed_password: String,
}

impl User {
    pub fn new(id: u64, email: &str, password: &str) -> Self {
        Self {
            id,
            email: email.to_string(),
            hashed_password: password.to_string(),
        }
    }
}
