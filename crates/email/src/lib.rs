mod config;
mod error;
mod smtp;

pub use config::{Config, email_config};
pub use error::{Error, Result};

use mrml::prelude::render::RenderOptions;

use crate::smtp::SmtpEmailSender;

/// The payload of an email.
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,

    pub template: Option<Template>,
    pub data: Option<Data>,
}

/// Enumeration of the different email templates.
pub enum Template {
    ForgotPassword,
    Intro,
    SecurityAlert,
}

/// The data to include in the email template.
#[derive(Default)]
pub struct Data {
    pub token: String,
    pub username: String,
    pub url: String,
}

/// A client for sending emails using different email services.
#[derive(Clone)]
pub struct EmailClient {
    pub is_connected: bool,
    service: EmailService,
}

impl EmailClient {
    /// Creates a new `EmailClient` based on the configured email provider.
    pub fn new() -> Result<Self> {
        if let Ok(service) = SmtpEmailSender::new() {
            let service = EmailService::Smtp(service);

            return Ok(Self {
                is_connected: service.test_connection(),
                service,
            });
        }

        Err(Error::EmailNotSetup)
    }

    /// Sends an email using the configured email provider.
    pub fn send(&self, email: &Email) -> Result<()> {
        self.service.send_email(email)
    }

    /// Tests the SMTP connection.
    pub fn test_connection(&self) -> bool {
        self.service.test_connection()
    }
}

#[derive(Clone)]
enum EmailService {
    Smtp(SmtpEmailSender),
}

trait EmailSender {
    fn send_email(&self, email: &Email) -> Result<()>;
    fn test_connection(&self) -> bool;
}

impl EmailSender for EmailService {
    fn send_email(&self, email: &Email) -> Result<()> {
        let mut email_to_send = Email {
            to: email.to.clone(),
            subject: email.subject.clone(),
            body: email.body.clone(),
            template: None,
            data: None,
        };

        if let Some(template) = &email.template {
            let template = match template {
                Template::ForgotPassword => include_str!("templates/forgot-password.mjml"),
                Template::Intro => include_str!("templates/intro.mjml"),
                Template::SecurityAlert => include_str!("templates/security-alert.mjml"),
            };

            if let Some(data) = &email.data {
                email_to_send.body = match mrml::parse(template) {
                    Ok(file) => match file.element.render(&RenderOptions::default()) {
                        Ok(content) => content
                            .replace("[[.Token]]", &data.token)
                            .replace("[[.URL]]", &data.url)
                            .replace("[[.UserName]]", &data.username),
                        Err(_) => return Err(Error::RenderFail),
                    },
                    Err(_) => return Err(Error::RenderFail),
                };
            }
        }

        match self {
            Self::Smtp(sender) => sender.send_email(&email_to_send),
        }
    }

    fn test_connection(&self) -> bool {
        match self {
            Self::Smtp(sender) => sender.test_connection(),
        }
    }
}
