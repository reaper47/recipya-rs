mod error;
mod sendgrid;

pub use sendgrid::Sendgrid;

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
