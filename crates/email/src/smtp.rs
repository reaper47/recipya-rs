use std::time::Duration;

use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::{error, info, warn};

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
            Ok(Self)
        } else {
            Err(Error::SmtpNotInitialized)
        }
    }
}

impl EmailSender for SmtpEmailSender {
    fn send_email(&self, email: Email) -> Result<()> {
        let from: Mailbox = format!("Recipya <{}>", email_config().smtp_from_email)
            .parse()
            .map_err(|_| Error::MissingConfig)?;
        let to: Mailbox = email.to.parse().map_err(|_| Error::MissingConfig)?;

        let email = Message::builder()
            .from(from.clone())
            .reply_to(from)
            .to(to)
            .subject(&email.subject)
            .header(ContentType::TEXT_HTML)
            .body(email.body)?;

        let username = &email_config().smtp_username;
        let password = &email_config().smtp_password;

        let mailer = match SmtpTransport::starttls_relay(&email_config().smtp_host) {
            Ok(transport) => {
                let creds = Credentials::new(username.clone(), password.clone());

                transport
                    .port(email_config().smtp_port)
                    .credentials(creds)
                    .build()
            }
            Err(err) => {
                error!(
                    config = email_config().smtp_host,
                    ?err,
                    "Failed to set up relay"
                );
                return Err(Error::General(err.to_string()));
            }
        };

        if let Err(err) = mailer.send(&email) {
            error!(?err, "Send email failed");
            return Err(Error::General(err.to_string()));
        }

        Ok(())
    }

    fn test_connection(&self) -> bool {
        let config = &email_config();

        match SmtpTransport::relay(&config.smtp_host) {
            Ok(relay) => match relay
                .credentials(Credentials::new(
                    config.smtp_username.clone(),
                    config.smtp_password.clone(),
                ))
                .timeout(Some(Duration::from_secs(1)))
                .build()
                .test_connection()
            {
                Ok(_) => {
                    info!("SMTP connection established");
                    true
                }
                Err(err) => {
                    warn!("Failed to test SMTP connection: {err}");
                    false
                }
            },
            Err(err) => {
                warn!("Failed to create email module: {err}");
                false
            }
        }
    }
}
