use mrml::prelude::render::RenderOptions;
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};

pub use crate::error::{Error, Result};
use crate::{config::config, Data, Template};

#[derive(Clone)]
pub struct Sendgrid {
    from: String,
    sender: Sender,
}

impl Sendgrid {
    pub fn new() -> Self {
        Self {
            from: config().EMAIL_FROM.clone(),
            sender: Sender::new(config().EMAIL_SENDGRID_API_KEY.clone()),
        }
    }

    pub async fn send(
        &self,
        to: String,
        subject: String,
        template: Template,
        data: Data,
    ) -> Result<reqwest::Response> {
        let from = String::from(&self.from);
        let func = self.sender.clone();

        let template = match template {
            Template::ForgotPassword => include_str!("templates/forgot-password.mjml"),
            Template::Intro => include_str!("templates/intro.mjml"),
        };

        let content = match mrml::parse(template) {
            Ok(file) => match file.render(&RenderOptions::default()) {
                Ok(content) => content
                    .replace("[[.Token]]", &data.token)
                    .replace("[[.URL]]", &data.url)
                    .replace("[[.UserName]]", &data.username),
                Err(_) => return Err(Error::RenderFail),
            },
            Err(_) => return Err(Error::RenderFail),
        };

        func.send(
            &Message::new(Email::new(&from))
                .set_subject(&subject)
                .set_reply_to(Email::new(&from))
                .add_content(
                    Content::new()
                        .set_content_type("text/html")
                        .set_value(content),
                )
                .add_personalization(Personalization::new(Email::new(to))),
        )
        .await
        .map_err(|e| Error::SendFail(e.to_string()))
    }
}

impl Default for Sendgrid {
    fn default() -> Self {
        Self::new()
    }
}
