use sendgrid::v3::{Content, Message, Personalization, Sender};
use tracing::error;

use crate::core::email::config::email_config;
use crate::core::email::error::{Error, Result};
use crate::core::email::{Email, EmailSender};

/// A struct representing a SendGrid email sender responsible for sending
/// emails using the SendGrid API.
#[derive(Clone)]
pub struct SendGridEmailSender;

impl SendGridEmailSender {
    /// Creates a new SMTP email sender based on the SMTP environment variables.
    pub fn new() -> Result<Self> {
        if email_config().is_sendgrid() {
            Ok(Self {})
        } else {
            Err(Error::SendGridNotInitialized)
        }
    }
}

impl EmailSender for SendGridEmailSender {
    fn send_email(&self, email: &Email) -> Result<()> {
        let m = Message::new(sendgrid::v3::Email::new(&email_config().email_admin))
            .set_subject(&email.subject)
            .set_reply_to(sendgrid::v3::Email::new(&email_config().email_admin))
            .add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(&email.body),
            )
            .add_personalization(Personalization::new(sendgrid::v3::Email::new(&email.to)));

        let subject = String::from(&email.subject);
        let api_key = String::from(&email_config().sendgrid_api_key);

        tokio::spawn(async move {
            let sender = Sender::new(api_key, None);

            if let Err(err) = sender.send(&m).await {
                error!("Could not send email '{subject}': {:?}", err);
            }
        });

        Ok(())
    }
}
