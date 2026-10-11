use l10n::Messages;
use maud::{Markup, html};

/// Renders the markup to cancel a form.
pub fn cancel_submit_form_actions(
    submit_content: &Markup,
    button_id: &str,
    is_submit_disabled: bool,
    messages: &Messages,
) -> Markup {
    html! {
        div class="card-actions justify-end" {
            button type="button" class="btn btn-sm" onclick="this.closest('dialog').close()" {
                (messages.action_cancel())
            }
            div .cursor-not-allowed {
                button id=(button_id) type="submit" class="btn btn-sm" disabled=[is_submit_disabled.then_some("")] {
                    img id={ (button_id) "-spinner" } .htmx-indicator src="/public/img/bars.svg" alt=(messages.action_loading());
                    (submit_content)
                }
            }
        }
    }
}
