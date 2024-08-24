use lib_core::config::config;
use maud::{html, Markup};

use super::layouts;

pub fn forgot_password() -> Markup {
    layouts::auth(
        "Forgot Password",
        html!(
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
                        label class="form-control w-full" {
                            div class="label" {
                                span class="label-text font-semibold" {
                                    "Email"
                                }
                            }
                            input required type="email" placeholder="Enter your email address" class="input input-bordered w-full" name="email";
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                "Reset password"
                            }
                        }
                    }
                }
            }
        ),
    )
}

pub fn forgot_password_reset(user_id: i64) -> Markup {
    layouts::auth(
        "Reset Password",
        html!(
            div #container {
                form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-boost="true" hx-target="#container" hx-swap="none" hx-post="/auth/forgot-password/reset" {
                    div class="card-body" {
                        h2 class="card-title underline self-center" {
                            "Change Password"
                        }
                        input name="user-id" type="hidden" value=(user_id);
                        label class="form-control w-full" {
                            div class="label" {
                                span class="label-text font-semibold" {
                                    "New Password"
                                }
                            }
                            input required type="password" placeholder="Enter your new password" class="input input-bordered w-full" name="password";
                        }
                        label class="form-control w-full" {
                            div class="label pt-0" {
                                span class="label-text font-semibold" {
                                    "Confirm password"
                                }
                            }
                            input required type="password" placeholder="Retype your password" class="input input-bordered w-full" name="password-confirm";
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                "Change"
                            }
                        }
                    }
                }
            }
        ),
    )
}

pub fn login(is_err: bool) -> Markup {
    layouts::auth(
        "Login",
        html!(
            form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/login" action="/auth/login" method="post" {
                div class="card-body" {
                    h2 class="card-title underline self-center" {
                        "Log In"
                    }
                    label class="form-control w-full" {
                        div class="label" {
                            span class="label-text font-semibold" {
                                "Email"
                            }
                        }
                        input class="input input-bordered w-full" required type="email" placeholder="Enter your email address" name="email" value=@if config().IS_DEMO { "demo@demo.com" };
                    }
                    label class="form-control w-full" {
                        div class="label" {
                            span class="label-text font-semibold" {
                                "Password"
                            }
                        }
                        input class="input input-bordered w-full" required type="password" placeholder="Enter your password" name="password" value=@if config().IS_DEMO { "demo" };
                    }
                    div class="form-control grid place-content-center" {
                        label class="label cursor-pointer gap-2" {
                            span class="label-text" {
                                "Remember me"
                            }
                            input class="checkbox checkbox-primary" type="checkbox" name="remember_me" value="true";
                        }
                    }
                    @if is_err {
                        p class="text-red-600" {
                            "Credentials are invalid."
                        }
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-primary btn-block btn-sm" {
                            "Log In"
                        }
                    }
                    div class="grid place-content-center text-center gap-2" {
                        @if !config().IS_NO_SIGNUPS {
                            div {
                                p class="text-center" {
                                    "Don't have an account?"
                                }
                                a class="btn btn-sm btn-block btn-outline" href="/auth/register" {
                                    "Sign Up"
                                }
                            }
                        }
                        a class="btn btn-sm btn-ghost" href="/auth/forgot-password" {
                            "Forgot your password?"
                        }
                    }
                }
            }
        ),
    )
}

pub fn register(is_err: bool) -> Markup {
    layouts::auth(
        "Register",
        html!(
             form class="card w-80 sm:w-96 bg-base-100 shadow-xl" action="/auth/register" method="post" {
                div class="card-body" {
                    h2 class="card-title underline self-center" {
                        "Create your Account"
                    }
                    label class="form-control w-full" {
                        div class="label" {
                            span class="label-text font-semibold" {
                                "Email"
                            }
                        }
                        input required type="email" placeholder="Enter your email address" class="input input-bordered w-full" name="email";
                    }
                    label class="form-control w-full" {
                        div class="label pt-0" {
                            span class="label-text font-semibold" {
                                "Password"
                            }
                        }
                        input required type="password" placeholder="Enter your password" class="input input-bordered w-full" name="password";
                    }
                    label class="form-control w-full" {
                        div class="label pt-0" {
                            span class="label-text font-semibold" {
                                "Confirm password"
                            }
                        }
                        input required type="password" placeholder="Enter your password" class="input input-bordered w-full" name="password_confirm";
                    }
                    @if is_err {
                        p class="text-red-600" {
                            "This account is either already registered or your credentials don't match."
                        }
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
        ),
    )
}
