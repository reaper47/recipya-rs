use maud::{Markup, PreEscaped, html};

use models::recipe::timeline::RecipeTimeline;

use crate::recipes::common::{RatingSize, render_rating};
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
        Self {
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

pub fn render_dialog(recipe_id: i64, events: &[Event]) -> Markup {
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
                form .py-4 hx-post=(format!("/recipes/{recipe_id}/timeline")) hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none"
                     _="on submit call document.querySelector('#timeline-new-event-dialog').close()
                        on htmx:afterRequest[detail.successful] reset() me" {
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
                                (render_rating("rating-event", Some(3), None, false, None))
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

pub fn render_events(recipe_id: i64, events: &[Event]) -> Markup {
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
                                (render_rating(&format!("rating-timeline-{index}"), Some(rating), Some(RatingSize::Small), true, None))
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
                p .text-center.pb-2 {
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
    let event_id = event.id;
    let local_time = event.created_at.format("%Y-%m-%dT%H:%M:%S").to_string();
    let timeline_id = format!("timeline-event-{event_id}");
    let form_id = format!("{timeline_id}-edit");
    let date_id = format!("event-date-{event_id}");
    let cally_timeline_id = format!("cally-timeline-{event_id}");

    html! {
        li id=(timeline_id) {
            hr;
            form id=(form_id) hx-put=(format!("/recipes/{recipe_id}/timelines/{event_id}")) hx-encoding="multipart/form-data" hx-target=(format!("#{timeline_id}")) hx-indicator="#fullscreen-loader" hx-swap="outerHTML" {}

            input type="hidden" name="index" value=(index) form=(form_id);
            input type="hidden" name="max-index" value=(num_events) form=(form_id);

            div class="timeline-start timeline-box p-0" {
                div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md" {
                    div class="card-body max-w-60" {
                        h2 class="card-title" {
                            input type="text" name="title" class="input w-full" value=(event.title) form=(form_id);
                        }
                        textarea #event-comment name="comment" placeholder="How did your dish go today?" rows="5" class="textarea w-full h-full resize-none rounded-none focus:outline-none" form=(form_id) {
                            (event.comment.unwrap_or_default())
                        }
                        div class="flex items-baseline justify-between" {
                            (render_rating("rating-event", event.rating, Some(RatingSize::Small), false, Some(&form_id)))
                            div class="join" {
                                button type="button" class="btn btn-ghost btn-square btn-sm join-item"
                                        hx-get=(format!("/recipes/{recipe_id}/timelines/{event_id}?index={index}&max-index={num_events}"))
                                        hx-target=(format!("#{timeline_id}"))
                                        hx-swap="outerHTML" {
                                    (icon_x_mark())
                                }
                                button form=(form_id) class="btn btn-ghost btn-square btn-sm join-item" {
                                    (icon_check())
                                }
                            }
                        }
                    }
                }
            }
            div class="timeline-middle" {
                button id=(cally_timeline_id) type="button" popovertarget="cally-popover-timeline" class="cally-timeline input input-border w-full" style="anchor-name:--cally-timeline" {
                    ("")
                }
                div popover #cally-popover-timeline class="dropdown bg-base-100 rounded-box shadow-lg" style="position-anchor:--cally-timeline" {
                    calendar-date class="cally" _=(format!(r"
                        on change
                            put my value into value of #{date_id}
                            put my value into innerText of #{cally_timeline_id}
                            call #cally-popover-timeline.hidePopover()")) {
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
            div class="timeline-end lg:w-60" {
                input type="hidden" id=(date_id) class="event-date" name="date" value=("") form=(form_id);

                figure class="image-preview" {
                    @if let Some(image) = event.image {
                        @let image = format!("/data/images/Timelines/{image}.webp");
                        img src=(image) alt="Event image" class="w-full h-60 object-cover";
                    } @else {
                        div class="w-full h-60 bg-base-200 flex items-center justify-center" {
                            span class="text-base-content/50" { "No image" }
                        }
                    }
                }
                @let file_id = format!("image-upload-{event_id}");
                input type="file" id=(file_id) name="image" accept="image/*" form=(form_id)
                        class="file-input file-input-sm file-input-bordered w-full max-w-sm"
                        _=(format!(r"
                            on change
                                set img to the first <img/> in the previous <figure/>
                                make an FileReader called reader
                                if event.dataTransfer
                                    get event.dataTransfer.files[0]
                                else
                                    get event.target.files[0]
                                end
                                set {{src: window.URL.createObjectURL(it)}} on img
                            on load
                                set img to the first <img/> in the previous <figure/>
                                if img exists
                                    call loadURLToInputField(img.src, '{file_id}')
                                end",
                        ));

                (PreEscaped(format!(r"<script>
                    document.querySelector('#{date_id}').value = (new Date('{local_time}')).toISOString().split('T')[0];
                    document.querySelector('#{cally_timeline_id}').innerText = (new Date('{local_time}')).toISOString().split('T')[0];
                </script>")))
            }
            @if index != num_events - 1 {
                hr;
            }
        }
    }
}
