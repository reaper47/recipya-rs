use maud::{Markup, html};

use config::{DemoState, SignupsState};
use l10n::Messages;

use crate::templates::layouts;

/// Renders the forgot password request form.
pub fn forgot_password(messages: &Messages) -> Markup {
    layouts::auth(
        &messages.forgot_password_form_title(),
        &html! {
            div #container {
                form
                    class="card w-80 sm:w-96 bg-base-100 shadow-xl"
                    hx-target="#container"
                    hx-push-url="/auth/forgot-password/requested"
                    hx-post="/auth/forgot-password" {
                    div .card-body {
                        h2 class="card-title underline self-center" {
                            (messages.forgot_password_form_title())
                        }
                        fieldset class="fieldset" {
                            label .label for="email" {
                                (messages.email_input_label())
                            }
                            input #email type="email" required .input name="email" placeholder=(messages.email_input_placeholder()) autocomplete="email";
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                (messages.reset_password_form_submit())
                            }
                        }
                    }
                }
            }
        },
        messages,
    )
}

/// Renders the forgot password reset form.
pub fn forgot_password_reset(token: &str, messages: &Messages) -> Markup {
    layouts::auth(
        &messages.reset_password_form_title(),
        &html! {
            div #container {
                form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-target="#container" hx-swap="none" hx-post="/auth/forgot-password/reset" {
                    div .card-body {
                        input type="hidden" name="token" value=(token);
                        h2 class="card-title underline self-center" {
                            (messages.reset_password_form_title())
                        }
                        fieldset .fieldset {
                            label .label for="password" {
                                (messages.new_password_input_label())
                            }
                            input #password type="password" required .input name="password" autocomplete="new-password" placeholder=(messages.new_password_input_placeholder());
                        }
                        fieldset .fieldset {
                            label .label for="confirm-password" {
                                (messages.confirm_password_input_label())
                            }
                            input #confirm-password type="password" required .input name="password-confirm" autocomplete="new-password" placeholder=(messages.confirm_password_input_placeholder());
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                (messages.reset_password_form_submit())
                            }
                        }
                    }
                }
            }
        },
        messages,
    )
}

/// Renders the login form template.
pub fn login(demo: &DemoState, signups: &SignupsState, messages: &Messages) -> Markup {
    layouts::auth(
        &messages.login_form_tab_title(),
        &html! {
            form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/login" {
                div .card-body {
                    div class="chat chat-end" {
                      div class="chat-image avatar" {
                        div class="w-10 rounded-full" {
                          img alt="Bananacat" src="/data/images/Icon/android-chrome-192x192.png";
                        }
                      }
                      div .chat-bubble {
                          (messages.login_form_greeting_1())
                      }
                    }
                    h2 class="card-title self-center underline" {
                        (messages.login_form_title())
                    }
                    fieldset .fieldset {
                        label .label for="email" {
                            (messages.email_input_label())
                        }
                        input #email type="email" required .input name="email" placeholder=(messages.email_input_placeholder()) autocomplete="username" value=@if demo == &DemoState::On { "demo@demo.com" };
                    }
                    fieldset .fieldset {
                        label class="label block" for="password" {
                            (messages.password_input_label())
                            a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password" {
                                (messages.login_form_forgot_password())
                            }
                        }
                        input #password type="password" required .input name="password" placeholder=(messages.password_input_placeholder()) autocomplete="current-password" value=@if demo == &DemoState::On { "demodemo" };
                    }
                    label class="fieldset-label py-2" {
                        input type="checkbox" .checkbox name="remember-me" checked;
                        (messages.login_form_remember_me())
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-primary btn-block btn-sm" {
                            (messages.login())
                        }
                    }
                    div class="grid text-center gap-2" {
                        @if signups == &SignupsState::On {
                            div {
                                div class="divider uppercase" {
                                    (messages.or())
                                }
                                a class="btn btn-sm btn-block btn-outline" href="/auth/register" {
                                    (messages.login_form_create_account())
                                }
                            }
                        }
                    }
                }
            }
        },
        messages,
    )
}

/// Renders the user registration page.
pub fn register(messages: &Messages) -> Markup {
    layouts::auth(
        &messages.register_form_tab_title(),
        &html! {
             form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/register" {
                div .card-body {
                    div class="chat chat-end" {
                      div class="chat-image avatar" {
                        div class="w-10 rounded-full" {
                          img alt="Bananacat" src="/data/images/Icon/android-chrome-192x192.png";
                        }
                      }
                      div .chat-bubble {
                          (messages.register_form_greeting_1())
                      }
                    }
                    h2 class="card-title underline self-center" {
                        (messages.register_form_title())
                    }
                    fieldset .fieldset {
                        label .label for="email" {
                            (messages.email_input_label())
                        }
                        input #email type="email" .input name="email" required placeholder=(messages.email_input_placeholder());
                    }
                    fieldset .fieldset {
                        label .label for="email" {
                            (messages.password_input_label())
                        }
                        input #password type="password" .input name="password" required placeholder=(messages.password_input_placeholder());
                    }
                    fieldset .fieldset {
                        label .label for="password-confirm" {
                            (messages.confirm_password_input_label())
                        }
                        input #password-confirm type="password" required .input name="password-confirm" placeholder=(messages.confirm_password_input_placeholder());
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-primary btn-block btn-sm" {
                            (messages.signup())
                        }
                    }
                    div class="grid place-content-center text-center gap-2 pt-1" {
                        div {
                            p .text-center {
                                (messages.register_form_account_exists())
                            }
                            a class="btn btn-sm btn-block btn-outline" href="/auth/login" {
                                (messages.login())
                            }
                        }
                    }
                }
            }
        },
        messages,
    )
}
