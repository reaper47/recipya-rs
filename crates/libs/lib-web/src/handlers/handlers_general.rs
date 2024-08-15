use axum::response::Redirect;
use lib_core::config;

pub async fn index() -> Redirect {
    let mut redirect_url = "/guide";
    if config().IS_BYPASS_GUIDE {
        redirect_url = "/auth/login";
    }

    Redirect::to(redirect_url)
}

pub async fn redirect_to_login() -> Redirect {
    Redirect::permanent("/auth/login")
}
