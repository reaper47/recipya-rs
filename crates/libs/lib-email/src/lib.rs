pub use sendgrid::Sendgrid;

mod config;
mod error;
mod sendgrid;

pub enum Template {
    ForgotPassword,
    Intro,
}

#[derive(Default)]
pub struct Data {
    pub token: String,
    pub username: String,
    pub url: String,
}
