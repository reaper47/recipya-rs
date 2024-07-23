use crate::web;
use maud::{html, Markup};

pub fn simple(title: &str, content: &str) -> Markup {
    web::templates::layouts::auth(
        title,
        html!(
            div class="card w-80 sm:w-96 bg-base-100 shadow-xl" {
                div class="card-body" {
                    h2 class="card-title underline self-center" {
                        (title)
                    }
                    p {
                        (content)
                    }
                    div class="card-actions justify-end" {
                        a href="/" class="btn btn-primary btn-block btn-sm" {
                            "Back Home"
                        }
                    }
                }
            }
        ),
    )
}
