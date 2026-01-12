use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
pub struct UserRowParams {
    #[serde(rename = "row-index")]
    pub row_index: usize,
}

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct UpdatePasswordForm {
    #[serde(rename = "new-password")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
    #[serde(rename = "new-password-confirm")]
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: String,
    #[serde(rename = "row-index")]
    pub row_index: usize,
}
