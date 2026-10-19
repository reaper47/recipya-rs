use auth::types::Password;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
pub struct UserRowParams {
    #[serde(rename = "row-index")]
    pub row_index: usize,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePasswordForm {
    #[serde(rename = "new-password")]
    pub new_password: Password,
    #[serde(rename = "new-password-confirm")]
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: Password,
    #[serde(rename = "row-index")]
    pub row_index: usize,
}
