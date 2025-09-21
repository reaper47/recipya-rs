use maud::{Markup, html};
use crate::recipes::common::render_rating;
use crate::templates::icons::icon_pencil;

#[derive(Default)]
pub struct Event<'a> {
    pub date: String,
    pub title: &'a str,
    pub image: Option<String>,
    pub description: Option<String>,
    pub rating: Option<i16>,
}

pub fn render_timeline_dialog(events: Vec<Event>) -> Markup {
    html! {
        dialog #timeline-dialog .modal {
            div class="modal-box max-w-[80vw] max-h-[92vh] p-4 flex flex-col overflow-auto" {
                h3 class="font-bold text-lg" { "Recipe Timeline" }
                ul #timeline-dialog-result class="timeline timeline-vertical lg:timeline-horizontal" {
                    (render_timeline_events(events))
                }
                div class="modal-action block mt-4" {
                    form method="dialog" {
                        button class="btn btn-sm" {
                            "Close"
                        }
                    }
                }
            }
        }
    }
}

pub fn render_timeline_events(events: Vec<Event>) -> Markup {
    html! {
        @for (idx, event) in events.iter().enumerate() {
            li {
                @if idx != 0 {
                    hr;
                }
                @if idx % 2 != 0 {
                    div class="timeline-start timeline-box p-0" {
                        div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md" {
                            @if let Some(image) = event.image.clone() {
                                figure {
                                    img src=(image) alt=(event.title) class="w-full lg:w-40 h-40 object-cover";
                                }
                            }
                            div class="card-body" {
                                h2 class="card-title" {
                                    (event.title)
                                }
                                @if let Some(s) = event.description.clone() {
                                    p {
                                        (s)
                                    }
                                }
                                @if let Some(rating) = event.rating {
                                    div class="flex justify-between" {
                                        (render_rating(&format!("rating-timeline-{idx}"), Some(rating), "", true))
                                        button class="btn btn-ghost btn-square btn-sm" {
                                            (icon_pencil(false))
                                        }
                                    }
                                } @else if idx > 0 {
                                    div class="card-actions justify-end" {
                                        button class="btn btn-ghost btn-square btn-sm" {
                                            (icon_pencil(false))
                                        }
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
                        (event.date)
                    }
                } @else {
                    div class="timeline-start" {
                        (event.date)
                    }
                    div class="timeline-middle" {
                        svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5" {
                            path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd";
                        }
                    }
                    div class="timeline-end timeline-box p-0" {
                        div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md" {
                            @if let Some(image) = event.image.clone() {
                                figure { img src=(image) alt=(event.title) class="w-full lg:w-40 h-40 object-cover"; }
                            }
                            div class="card-body" {
                                h2 class="card-title" {
                                    (event.title)
                                }
                                @if let Some(s) = event.description.clone() {
                                    p {
                                        (s)
                                    }
                                }
                                @if let Some(rating) = event.rating {
                                    div class="flex justify-between" {
                                        (render_rating(&format!("rating-timeline-{idx}"), Some(rating), "", true))
                                        button class="btn btn-ghost btn-square btn-sm" {
                                            (icon_pencil(false))
                                        }
                                    }
                                } @else if idx > 0 {
                                    div class="card-actions justify-end" {
                                        button class="btn btn-ghost btn-square btn-sm" {
                                            (icon_pencil(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                @if idx != events.len() - 1 {
                    hr;
                }
            }
        }
    }
}
