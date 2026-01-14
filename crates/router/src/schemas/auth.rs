use serde::{Deserialize, Serialize};
use validator::Validate;

use models::user::UserForCreate;

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct ChangePasswordForm {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordResetForm {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

/// A form structure used for logging in a user.
#[derive(Default, Validate, Deserialize, Serialize)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub remember_me: Option<bool>,
}

impl LoginForm {
    /// Returns whether the user has opted to be remembered across sessions.
    pub fn is_remember_me(&self) -> bool {
        self.remember_me.unwrap_or(false)
    }
}

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct RegisterForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[serde(rename = "password-confirm")]
    #[validate(must_match(other = "password"))]
    pub password_confirm: String,
}

impl RegisterForm {
    /// Converts the form to a UserForCreate.
    pub fn to_user(&self) -> UserForCreate {
        UserForCreate {
            email: self.email.clone(),
            password_clear: self.password.clone(),
        }
    }
}
