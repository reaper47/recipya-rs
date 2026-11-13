use serde::{Deserialize, Deserializer, Serialize};
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
    #[serde(default, rename = "remember-me", deserialize_with = "checkbox")]
    pub remember_me: bool,
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

fn checkbox<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(matches!(value.as_deref(), Some("on" | "true" | "1")))
}
