use std::thread;

use mrml::prelude::render::RenderOptions;
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};

use crate::services::email::{Data, Template};

pub struct Sendgrid {
    from: String,
    sender: Sender,
}

impl Sendgrid {
    pub fn new(api_key: String, from: String) -> Option<Self> {
        if api_key.is_empty() {
            return None;
        }

        Some(Self {
            from,
            sender: Sender::new(api_key),
        })
    }

    pub fn send(&self, to: String, subject: String, template: Template, data: Data) {
        let from = String::from(&self.from);
        let func = self.sender.clone();

        thread::spawn(move || {
            let template = match template {
                Template::ForgotPassword => include_str!("templates/forgot-password.mjml"),
                Template::Intro => include_str!("templates/intro.mjml"),
            };

            let content = match mrml::parse(template) {
                Ok(file) => {
                    match file.render(&RenderOptions::default()) {
                        Ok(content) => content
                            .replace("[[.Token]]", &data.token)
                            .replace("[[.URL]]", &data.url)
                            .replace("[[.UserName]]", &data.username),
                        Err(error) => {
                            // TODO: Log the error
                            println!("{error}");
                            return;
                        }
                    }
                }
                Err(error) => {
                    // TODO: Log the error
                    println!("{error}");
                    return;
                }
            };

            match func.blocking_send(
                &Message::new(Email::new(&from))
                    .set_subject(&subject)
                    .set_reply_to(Email::new(&from))
                    .add_content(
                        Content::new()
                            .set_content_type("text/html")
                            .set_value(content),
                    )
                    .add_personalization(Personalization::new(Email::new(to))),
            ) {
                Ok(_) => {},
                Err(error) => {
                    // TODO: Log error
                    println!("{error}");
                    return;
                }
            }
        });
    }
}

