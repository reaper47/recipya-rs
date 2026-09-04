use maud::{Markup, html};

use config::{DemoState, SignupsState};

use crate::templates::layouts;

/// Renders the forgot password request form.
pub fn forgot_password() -> Markup {
    layouts::auth(
        "Forgot Password",
        &html! {
            div #container {
                form
                    class="card w-80 sm:w-96 bg-base-100 shadow-xl"
                    hx-boost="true"
                    hx-target="#container"
                    hx-push-url="/auth/forgot-password/requested"
                    hx-post="/auth/forgot-password" {
                    div class="card-body" {
                        h2 class="card-title underline self-center" {
                            "Forgot Password"
                        }
                        fieldset class="fieldset" {
                            label class="label" for="email" { "Email" }
                            input #email type="email" required placeholder="Enter your email address" class="input" name="email";
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                "Reset password"
                            }
                        }
                    }
                }
            }
        },
    )
}

pub fn forgot_password_reset(token: &str) -> Markup {
    layouts::auth(
        "Reset Password",
        &html! {
            div #container {
                form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-boost="true" hx-target="#container" hx-swap="none" hx-post="/auth/forgot-password/reset" {
                    div class="card-body" {
                        input type="hidden" name="token" value=(token);
                        h2 class="card-title underline self-center" {
                            "Change Password"
                        }
                        fieldset class="fieldset" {
                            label class="label" for="password" { "New password" }
                            input #password type="password" required placeholder="Enter your new password" class="input" name="password";
                        }
                        fieldset class="fieldset" {
                            label class="label" for="confirm-password" { "Confirm password" }
                            input #confirm-password type="password" required placeholder="Retype your password" class="input" name="password-confirm";
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                "Change"
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Renders the login form template.
pub fn login(demo: &DemoState, signups: &SignupsState) -> Markup {
    layouts::auth(
        "Login",
        &html! {
            form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/login" {
                div class="card-body" {
                    div class="chat chat-end" {
                      div class="chat-image avatar" {
                        div class="w-10 rounded-full" {
                          img alt="Bananacat" src="/data/images/Icon/android-chrome-192x192.png";
                        }
                      }
                      div class="chat-bubble" { "The stove is hot. Shall we cook?" }
                    }
                    h2 class="card-title self-center underline" {
                        "Log in to Recipya"
                    }
                    fieldset class="fieldset" {
                        label class="label" for="email" { "Email" }
                        input #email type="email" required placeholder="Enter your email address" class="input" name="email"
                              value=@if demo == &DemoState::On { "demo@demo.com" };
                    }
                    fieldset class="fieldset" {
                        label class="label block" for="password" {
                            "Password"
                            a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password" {
                                "Forgot your password?"
                            }
                        }
                        input #password type="password" required placeholder="Enter your password" class="input" name="password"
                              value=@if demo == &DemoState::On { "demodemo" };
                    }
                    label class="fieldset-label py-2" {
                        input name="remember-me" type="checkbox" checked="checked" class="checkbox" checked="checked";
                        "Remember me"
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-primary btn-block btn-sm" {
                            "Login"
                        }
                    }
                    div class="grid text-center gap-2" {
                        @if signups == &SignupsState::On {
                            div {
                                div class="divider" { "OR" }
                                a class="btn btn-sm btn-block btn-outline" href="/auth/register" {
                                    "Create an account"
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Renders the user registration page.
pub fn register() -> Markup {
    layouts::auth(
        "Register",
        &html! {
             form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/register" {
                div class="card-body" {
                    div class="chat chat-end" {
                      div class="chat-image avatar" {
                        div class="w-10 rounded-full" {
                          img alt="Bananacat" src="/data/images/Icon/android-chrome-192x192.png";
                        }
                      }
                      div class="chat-bubble" { "Your culinary journey starts here!" }
                    }
                    h2 class="card-title underline self-center" {
                        "Create your account"
                    }
                    fieldset class="fieldset" {
                        label class="label" for="email" { "Email" }
                        input #email type="email" required placeholder="Enter your email address" class="input" name="email";
                    }
                    fieldset class="fieldset" {
                        label class="label" for="email" { "Password" }
                        input #password type="password" required placeholder="Enter your password" class="input" name="password";
                    }
                    fieldset class="fieldset" {
                        label class="label" for="password-confirm" { "Confirm password" }
                        input #password-confirm type="password" required  placeholder="Retype your password" class="input" name="password-confirm";
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-primary btn-block btn-sm" {
                            "Sign Up"
                        }
                    }
                    div class="grid place-content-center text-center gap-2 pt-1" {
                        div {
                            p class="text-center" {
                                "Already have an account?"
                            }
                            a class="btn btn-sm btn-block btn-outline" href="/auth/login" {
                                "Log in"
                            }
                        }
                    }
                }
            }
        },
    )
}
