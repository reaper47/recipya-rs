mod sendgrid;

pub(crate) use sendgrid::Sendgrid;

pub(crate) enum Template {
    ForgotPassword,
    Intro,
}

#[derive(Default)]
pub(crate) struct Data {
    pub token    : String,
    pub username : String,
    pub url  : String,
}
