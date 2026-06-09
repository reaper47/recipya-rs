use maud::{Markup, html};

pub fn cancel_submit_form_actions(
    submit_content: &Markup,
    button_id: &str,
    is_submit_disabled: bool,
) -> Markup {
    html! {
        div class="card-actions justify-end" {
            button type="button" class="btn btn-sm" onclick="this.closest('dialog').close()" { "Cancel" }
            div .cursor-not-allowed {
                button id=(button_id) type="submit" class="btn btn-sm" disabled=[is_submit_disabled.then_some("")] {
                    img id=(format!("{button_id}-spinner")) class="htmx-indicator" src="/public/img/bars.svg" alt="Loading...";
                    (submit_content)
                }
            }
        }
    }
}
