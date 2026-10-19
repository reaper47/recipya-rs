use serde::{Deserialize, Serialize};
use validator::Validate;

use auth::types::{Email, Password};
use models::user::UserForCreate;

#[derive(Validate, Deserialize, Serialize)]
pub struct ChangePasswordForm {
    pub password: Password,
    pub new_password: Password,
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: Password,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordForm {
    pub email: Email,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordResetForm {
    pub token: String,
    pub password: Password,
    #[validate(must_match(other = "password"))]
    pub confirm_password: Password,
}

/// A form structure used for logging in a user.
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginForm {
    pub email: Email,
    pub password: Password,
    pub remember_me: Option<bool>,
}

impl LoginForm {
    /// Returns whether the user has opted to be remembered across sessions.
    pub fn is_remember_me(&self) -> bool {
        self.remember_me.unwrap_or(false)
    }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct RegisterForm {
    pub email: Email,
    pub password: Password,
    #[serde(rename = "password-confirm")]
    #[validate(must_match(other = "password"))]
    pub password_confirm: Password,
}

impl RegisterForm {
    /// Converts the form to a `UserForCreate`.
    pub fn to_user(&self) -> UserForCreate {
        UserForCreate {
            email: self.email.clone(),
            password_clear: self.password.clone(),
        }
    }
}
