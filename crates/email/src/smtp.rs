use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::error;

use super::config::email_config;
use super::{Email, EmailSender, Error, Result};

/// A struct representing an SMTP email sender responsible for sending emails
/// using the SMTP protocol.
#[derive(Clone)]
pub struct SmtpEmailSender;

impl SmtpEmailSender {
    /// Creates a new SMTP email sender based on the SMTP environment variables.
    pub fn new() -> Result<Self> {
        if email_config().is_smtp() {
            Ok(Self {})
        } else {
            Err(Error::SmtpNotInitialized)
        }
    }
}

impl EmailSender for SmtpEmailSender {
    fn send_email(&self, email: &Email) -> Result<()> {
        let from: Mailbox = email_config()
            .email_admin
            .parse()
            .map_err(|_| Error::MissingConfig)?;
        let to: Mailbox = email.to.parse().map_err(|_| Error::MissingConfig)?;

        let email = Message::builder()
            .from(from.clone())
            .reply_to(from)
            .to(to)
            .subject(&email.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(email.body.clone())?;

        let username = &email_config().smtp_username;
        let password = &email_config().smtp_password;

        let mailer = match SmtpTransport::relay(&email_config().smtp_host) {
            Ok(transport) => {
                let creds = Credentials::new(username.into(), password.into());

                transport.credentials(creds).build()
            }
            Err(err) => {
                error!(
                    "Failed to set up relay {}: {:?}",
                    email_config().smtp_host,
                    err
                );
                return Err(Error::General(err.to_string()));
            }
        };

        if let Err(err) = mailer.send(&email) {
            error!("Send email failed: {:?}", err);
            return Err(Error::General(err.to_string()));
        }

        Ok(())
    }
}
