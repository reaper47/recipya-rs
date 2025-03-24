use maud::{Markup, PreEscaped, html};

/// Renders a new recipe category form.
pub fn new_recipe_category(category: &str) -> Markup {
    html! {
        div class="badge badge-outline p-3 pr-0" {
            form .inline-flex hx-delete="/recipes/categories" hx-target=(PreEscaped("closest <div/>")) hx-swap="delete" {
                input type="hidden" name="category" value=(category);
                span .select-none { (category) }
                button type="submit" .btn.btn-xs.btn-ghost { "X" }
            }
        }
        (empty_recipe_category())
    }
}

fn empty_recipe_category() -> Markup {
    html! {
        div class="badge badge-outline p-3 pr-0" {
            form .inline-flex hx-post="/recipes/categories" hx-target=(PreEscaped("closest <div/>")) hx-swap="outerHTML" {
                label class="input" {
                    input required type="text" placeholder="New category" class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off";
                }
                button .btn.btn-xs.btn-ghost { (PreEscaped("&#10003;")) }
            }
        }
    }
}
