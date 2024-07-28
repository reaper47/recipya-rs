use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::Error;

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

#[derive(Default, Serialize)]
pub struct ToastBuilder {
    #[serde(rename = "showToast")]
    pub show_toast: ShowToast,
}

#[derive(Default, Serialize)]
struct ShowToast {
    pub action: String,
    pub background: String,
    pub message: String,
    pub title: String,
}

impl ToastBuilder {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            show_toast: ShowToast {
                title: title.into(),
                message: message.into(),
                ..Default::default()
            },
        }
    }

    pub fn action(&mut self, action: impl Into<String>) -> &mut Self {
        self.show_toast.action = action.into();
        self
    }

    pub fn background(&mut self, background: impl Into<String>) -> &mut Self {
        self.show_toast.background = background.into();
        self
    }

    pub fn build(self) -> Result<Self, Error> {
        Ok(Self {
            show_toast: ShowToast {
                action: "".to_string(),
                background: "".to_string(),
                message: "".to_string(),
                title: "".to_string(),
            },
        })
    }
}
