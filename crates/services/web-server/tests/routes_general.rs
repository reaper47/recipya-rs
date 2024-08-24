mod support;

use axum::http::StatusCode;
use support::{assert::*, server::*, test_db::*};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_index {
    use super::*;

    const BASE_URI: &str = "/";

    #[tokio::test]
    async fn test_get_index_ok_anonymous() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(res.header("Location"), "/guide");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_index_ok_bypass_guide() -> Result<()> {
        std::env::set_var("SERVICE_BYPASS_GUIDE", "true");
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_BYPASS_GUIDE");
        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(res.header("Location"), "/auth/login");
        Ok(())
    }

    /* TODO: Move test to the /recipes test suite
    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_index_ok_hide_elements_when_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res.assert_status_ok();
        assert_not_in_html(
            res,
            vec![
                r##"<li class="border-2 rounded-b-lg hover:bg-blue-100 dark:border-gray-500 dark:hover:bg-blue-600"><a hx-post="/auth/logout" class="flex" href="#"><svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 ml-0 self-center" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/></svg><span class="pl-1 align-bottom">Log out</span></a></li>"##,
            ],
        );
        Ok(())
    }*/

    /* TODO: Move test to the /recipes test suite
    #[tokio::test]
    async fn test_get_index_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_ok();
        assert_html(
            res,
            vec![
                r#"<title hx-swap-oob="true">Recipes | Recipya</title>"#,
                r##"<button title="Open avatar menu" popovertarget="avatar_menu" popovertargetaction="toggle" class="" hx-get="/user-initials" hx-trigger="load" hx-target="#user-initials"><div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar placeholder"><div class="bg-neutral text-neutral-content w-10 rounded-full"><span id="user-initials">A</span></div></div></button>"##,
                r#"<div id="avatar_menu" popover style="inset: unset; top: 3.5rem; right: 0.5rem;" class="rounded-box z-10 shadow bg-base-200" _="on click if me.matches(':popover-open') then me.hidePopover()">"#,
                r#"<div class="bg-neutral text-neutral-content w-10 rounded-full"><span id="user-initials">A</span></div>"#,
                r#"<ul tabindex="0" class="menu">"#,
                r##"<li onclick="document.activeElement?.blur()"><a href="/admin" hx-get="/admin" hx-target="#content" hx-push-url="true"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21v-8.25M15.75 21v-8.25M8.25 21v-8.25M3 9l9-6 9 6m-1.5 12V10.332A48.36 48.36 0 0 0 12 9.75c-2.551 0-5.056.2-7.5.582V21M3 21h18M12 6.75h.008v.008H12V6.75Z"></path></svg>Admin</a></li>"##,
                r##"<li onclick="document.activeElement?.blur()"><a href="/reports" hx-get="/reports" hx-target="#content" hx-push-url="true"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3 3v1.5M3 21v-6m0 0 2.77-.693a9 9 0 0 1 6.208.682l.108.054a9 9 0 0 0 6.086.71l3.114-.732a48.524 48.524 0 0 1-.005-10.499l-3.11.732a9 9 0 0 1-6.085-.711l-.108-.054a9 9 0 0 0-6.208-.682L3 4.5M3 15V4.5"></path></svg>Reports</a></li><div class="divider m-0"></div>"##,
                r#"<li onclick="document.activeElement?.blur()"><a href="/guide/en/docs" target="_blank"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25"></path></svg>Guide</a></li>"#,
                r##"`<li class="cursor-pointer" onclick="settings_dialog.showModal()"><a hx-get="/settings" hx-target="#settings_dialog_content"><svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path> <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>Settings</a></li><div class="divider m-0"></div>"##,
                r#"<li><a hx-post="/auth/logout"><svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 ml-0 self-center" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path></svg>Log out</a></li></ul>"#,
                r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p></div></div>"#,
                "Add recipe",
                r#"<a class="tooltip tooltip-right active" data-tip="Recipes">"#,
                r#"<a class="tooltip tooltip-right" data-tip="Cookbooks"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">"#,
                r##"<button id="add_recipe" class="btn btn-primary btn-sm hover:btn-accent" hx-get="/recipes/add" hx-target="#content" hx-trigger="mousedown" hx-push-url="true">Add recipe</button>"##,
                r##"<button id="add_cookbook" class="btn btn-primary btn-sm hover:btn-accent" hx-post="/cookbooks" hx-prompt="Enter the name of your cookbook" hx-target="#cookbooks-display" hx-trigger="mousedown" hx-swap="beforeend">Add cookbook</button>"##,
            ],
        );
        Ok(())
    }*/

    /* TODO: Move test to the /recipes test suite
    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_index_ok_update_available() -> Result<()> {
        // app.Info.IsUpdateAvailable = true
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        // app.Info.IsUpdateAvailable = false
        res.assert_status_ok();
        assert_html(
            res,
            vec![
                r##"<button title="Open avatar menu" popovertarget="avatar_menu" popovertargetaction="toggle" class="indicator" hx-get="/user-initials" hx-trigger="load" hx-target="#user-initials"><div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar placeholder indicator"><span class="indicator-item indicator-start badge badge-sm badge-secondary z-30">New update</span><div class="bg-neutral text-neutral-content w-10 rounded-full"><span id="user-initials">A</span></div></div></button>"##,
            ],
        );
        Ok(())
    }*/

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_index_ok_redirect_to_recipes_when_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(res.header("Location"), "/recipes");
        Ok(())
    }
}
