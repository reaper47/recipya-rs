use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub remember_me: Option<bool>,
}


#[derive(Default, Validate, Deserialize, Serialize)]
pub struct RegisterForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub password_confirm: String,
}

pub fn collect_errors<T: Validate>(form: &T) -> Vec<String> {
    match form.validate() {
        Ok(_) => Vec::new(),
        Err(errors) => errors
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |err| {
                    format!(
                        "Field '{}': {}",
                        field,
                        err.message.as_deref().unwrap_or("Unknown error")
                    )
                })
            })
            .collect(),
    }
}
