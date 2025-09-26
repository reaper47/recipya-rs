use maud::{Markup, PreEscaped, html};
use models::recipe::timeline::RecipeTimeline;

use crate::recipes::common::render_rating;
use crate::templates::icons::{icon_check, icon_pencil, icon_x_mark};

#[derive(Default)]
pub struct Event {
    pub id: i64,
    pub date: String,
    pub title: String,
    pub image: Option<String>,
    pub comment: Option<String>,
    pub rating: Option<i16>,
}

impl From<RecipeTimeline> for Event {
    fn from(value: RecipeTimeline) -> Self {
        Event {
            id: value.id,
            date: value.created_at.date().format("%x").to_string(),
            title: value.title,
            image: value
                .image
                .map(|u| format!("/data/images/Timelines/{u}.webp")),
            comment: value.comment,
            rating: value.rating,
        }
    }
}

pub fn render_dialog(recipe_id: i64, events: Vec<Event>) -> Markup {
    html! {
        dialog #timeline-dialog .modal {
            div class="modal-box relative w-fit max-w-[80vw] overflow-auto" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost fixed top-4 right-4 z-30" aria-label="Close" { "✕" }
                }
                h3 class="font-bold text-lg" { "Recipe Timeline" }
                ul #timeline-dialog-result class="timeline timeline-vertical lg:timeline-horizontal overflow-x-auto" {
                    (render_events(recipe_id, events))
                }
            }
        }

        dialog #timeline-new-event-dialog .modal {
            div .modal-box {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 class="font-bold text-lg" { "Add event to timeline" }
                form .py-4 hx-post=(format!("/recipes/{recipe_id}/timeline")) hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#timeline-new-event-dialog').close()" {
                    div #timeline-event-card class="card card-sm bg-base-100 shadow-sm max-w-md" {
                        div class="card-body p-0" {
                            fieldset class="fieldset border-base-300 rounded-box border p-4" {
                                label class="label" { "Title" }
                                input type="hidden" name="title" value="Recipe made";
                                input type="text" name="title" class="input w-full" value="Recipe made" disabled;

                                label class="label" { "Date" }
                                input type="hidden" #event-date name="date" _="on load set my value to window.todayISO() then put my value into #cally-timeline's innerText";
                                button #cally-timeline type="button" popovertarget="cally-popover-timeline" class="input input-border w-full" style="anchor-name:--cally-timeline" {
                                    "Pick a date"
                                }
                                div popover #cally-popover-timeline class="dropdown bg-base-100 rounded-box shadow-lg" style="position-anchor:--cally-timeline" {
                                    calendar-date class="cally" _="
                                        on change
                                            put my value into #event-date's value
                                            put my value into #cally-timeline's innerText
                                            call #cally-popover-timeline.hidePopover()" {
                                        svg aria-label="Previous" class="fill-current size-6" slot="previous" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                                            path d="M15.75 19.5 8.25 12l7.5-7.5" {}
                                        }
                                        svg aria-label="Next" class="fill-current size-6" slot="next" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                                            path d="m8.25 4.5 7.5 7.5-7.5 7.5" {}
                                        }
                                        calendar-month {}
                                    }
                                }

                                label class="label" for="event-image" { "Image" }
                                input #event-image type="file" accept="image/*,video/*" name="image" class="file-input file-input-bordered w-full";

                                label class="label" for="event-comment" { "Comment" }
                                textarea #event-comment name="comment" placeholder="How did your dish go today?" rows="5" class="textarea w-full h-full resize-none rounded-none focus:outline-none" {}

                                label class="label" { "Rating" }
                                (render_rating("rating-event", Some(3), "", false))
                            }
                        }
                    }
                    button class="btn btn-block btn-primary btn-sm" {
                        "Submit"
                    }
                }
            }
        }
    }
}

pub fn render_events(recipe_id: i64, events: Vec<Event>) -> Markup {
    let num_events = events.len();

    html! {
        @for (idx, event) in events.iter().enumerate() {
            (render_event(event, idx, num_events, recipe_id))
        }
    }
}

/// Renders a single timeline event item.
pub fn render_event(event: &Event, index: usize, num_events: usize, recipe_id: i64) -> Markup {
    let timeline_edit_url = format!(
        "/recipes/{recipe_id}/timelines/{}/edit?index={}&max-index={}",
        event.id, index, num_events
    );
    let timeline_event_id = format!("timeline-event-{}", event.id);

    html! {
        li id=(timeline_event_id) {
            @if index != 0 {
                hr;
            }
            div class="timeline-start timeline-box p-0" {
                div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md" {
                    div class="card-body max-w-60" {
                        h2 class="card-title" {
                            (event.title)
                        }
                        @if let Some(s) = event.comment.clone() {
                            p {
                                (s)
                            }
                        }
                        @if let Some(rating) = event.rating {
                            div class="flex justify-between items-baseline" {
                                (render_rating(&format!("rating-timeline-{index}"), Some(rating), "rating-sm", true))
                                (edit_button(&timeline_edit_url, &timeline_event_id))
                            }
                        } @else if index > 0 {
                            div class="card-actions justify-end" {
                                (edit_button(&timeline_edit_url, &timeline_event_id))
                            }
                        }
                    }
                }
            }
            div class="timeline-middle" {
                svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5" {
                    path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd";
                }
            }
            div class="timeline-end" {
                p .text-center {
                    (event.date)
                }
                @if let Some(image) = event.image.clone() {
                    figure {
                        img src=(image) alt="Timeline event" class="w-full lg:w-60 h-60 object-cover rounded-lg";
                    }
                }
            }
            @if index != num_events - 1 {
                hr;
            }
        }
    }
}

fn edit_button(timeline_edit_url: &str, hx_target: &str) -> Markup {
    html! {
        button class="btn btn-ghost btn-square btn-sm"
                hx-get=(timeline_edit_url)
                hx-target=(format!("#{hx_target}"))
                hx-swap="outerHTML" {
            (icon_pencil(false))
        }
    }
}

pub fn render_edit(
    event: RecipeTimeline,
    index: usize,
    num_events: usize,
    recipe_id: i64,
) -> Markup {
    let local_time = event.created_at.format("%Y-%m-%dT%H:%M:%S").to_string();
    let timeline_id = format!("timeline-event-{}", event.id);

    html! {
        li id=(timeline_id) {
            hr;
            form hx-put=(format!("/recipes/{recipe_id}/timelines/{}", event.id)) hx-encoding="multipart/form-data" hx-target=(format!("#{timeline_id}")) hx-indicator="#fullscreen-loader" hx-swap="outerHTML" {
                input type="hidden" name="index" value=(index);
                input type="hidden" name="max-index" value=(num_events);

                div class="timeline-start timeline-box p-0" {
                    div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md" {
                        div class="card-body max-w-60" {
                            h2 class="card-title" {
                                input type="text" name="title" class="input w-full" value=(event.title);
                            }
                            textarea #event-comment name="comment" placeholder="How did your dish go today?" rows="5" class="textarea w-full h-full resize-none rounded-none focus:outline-none" {
                                (event.comment.unwrap_or_default())
                            }
                            div class="flex items-baseline justify-between" {
                                (render_rating(&format!("rating-timeline-{}", event.id), event.rating, "rating-sm", false))
                                div class="join" {
                                    button type="button" class="btn btn-ghost btn-square btn-sm join-item"
                                            hx-get=(format!("/recipes/{recipe_id}/timelines/{}?index={index}&max-index={num_events}", event.id))
                                            hx-target=(format!("#{timeline_id}"))
                                            hx-swap="outerHTML" {
                                        (icon_x_mark())
                                    }
                                    button class="btn btn-ghost btn-square btn-sm join-item" {
                                        (icon_check())
                                    }
                                }
                            }
                        }
                    }
                }
                div class="timeline-middle" {
                     button id=(format!("cally-timeline-{}", event.id)) type="button" popovertarget="cally-popover-timeline" class="input input-border w-full" style="anchor-name:--cally-timeline" {
                        ("")
                    }
                    div popover #cally-popover-timeline class="dropdown bg-base-100 rounded-box shadow-lg" style="position-anchor:--cally-timeline" {
                        calendar-date class="cally" _="
                            on change
                                put my value into #event-date's value
                                put my value into #cally-timeline's innerText
                                call #cally-popover-timeline.hidePopover()" {
                            svg aria-label="Previous" class="fill-current size-6" slot="previous" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                                path d="M15.75 19.5 8.25 12l7.5-7.5" {}
                            }
                            svg aria-label="Next" class="fill-current size-6" slot="next" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                                path d="m8.25 4.5 7.5 7.5-7.5 7.5" {}
                            }
                            calendar-month {}
                        }
                    }
                }
                div class="timeline-end" {
                    input type="hidden" id=(format!("event-date-{}", event.id)) name="date" value=("");
                    @if let Some(image) = event.image {
                        figure {
                            img src=(image) alt="Event image" class="w-full lg:w-60 h-60 object-cover";
                        }
                    }
                    (PreEscaped(format!(r##"<script>
                        document.querySelector('#event-date-{}').value = (new Date('{local_time}')).toISOString().split('T')[0];
                        document.querySelector('#cally-timeline-{}').innerText = (new Date('{local_time}')).toISOString().split('T')[0];
                    </script>"##, event.id, event.id)))
                }
            }
            @if index != num_events - 1 {
                hr;
            }
        }
    }
}
