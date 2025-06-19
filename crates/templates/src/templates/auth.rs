use maud::{Markup, html};

use crate::templates::layouts;

/// Renders the forgot password request form.
pub fn forgot_password() -> Markup {
    layouts::auth(
        "Forgot Password",
        html! {
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

pub fn forgot_password_reset(user_id: i64) -> Markup {
    layouts::auth(
        "Reset Password",
        html! {
            div #container {
                form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-boost="true" hx-target="#container" hx-swap="none" hx-post="/auth/forgot-password/reset" {
                    div class="card-body" {
                        h2 class="card-title underline self-center" {
                            "Change Password"
                        }
                        input name="user-id" type="hidden" value=(user_id);
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
pub fn login(is_demo: bool, is_no_signups: bool) -> Markup {
    layouts::auth(
        "Login",
        html! {
            form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/login" action="/auth/login" method="post" {
                div class="card-body" {
                    h2 class="card-title underline self-center" {
                        "Log In"
                    }
                    fieldset class="fieldset" {
                        label class="label" for="email" { "Email" }
                        input #email type="email" required placeholder="Enter your email address" class="input" name="email" value=@if is_demo { "demo@demo.com" };
                    }
                    fieldset class="fieldset" {
                        label class="label block" for="password" {
                            "Password"
                            a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password" {
                                "Forgot your password?"
                            }
                        }
                        input #password type="password" required placeholder="Enter your password" class="input" name="password" value=@if is_demo { "demo" };
                    }
                    fieldset class="fieldset p-4 bg-base-100 border border-base-300 rounded-box w-64 grid self-center mb-2" {
                      legend class="fieldset-legend" { "Login options" }
                      label class="fieldset-label" {
                        input name="remember-me" type="checkbox" checked="checked" class="checkbox" checked="checked";
                        "Remember me"
                      }
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-primary btn-block btn-sm" {
                            "Log In"
                        }
                    }
                    div class="grid place-content-center text-center gap-2" {
                        @if !is_no_signups {
                            div {
                                p class="text-center" {
                                    "Don't have an account?"
                                }
                                a class="btn btn-sm btn-block btn-outline" href="/auth/register" {
                                    "Sign Up"
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
        html! {
             form class="card w-80 sm:w-96 bg-base-100 shadow-xl" action="/auth/register" method="post" {
                div class="card-body" {
                    h2 class="card-title underline self-center" {
                        "Create your Account"
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
                    div class="grid place-content-center text-center gap-2" {
                        div {
                            p class="text-center" {
                                "Already have an account?"
                            }
                            a class="btn btn-sm btn-block btn-outline" href="/auth/login" {
                                "Log In"
                            }
                        }
                    }
                }
            }
        },
    )
}
