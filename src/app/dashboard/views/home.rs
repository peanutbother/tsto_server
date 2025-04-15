use crate::app::{
    dashboard::{
        components::{config::ConfigSection, status::StatusSection},
        providers::{use_loggedin, use_permissions},
        router::Route,
    },
    models::auth::Role,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let logged_in = use_loggedin();

    // workaround if auth state changes because browser router does not trigger reloads
    if !logged_in {
        let navigator = navigator();
        navigator.push(Route::Login {});
        return rsx! {};
    }

    rsx! {
        div { class: "body-container md:grid-cols-2",

            StatusSection {}

            if use_permissions(Role::Moderator) {
                ConfigSection {}
            }
        }
    }
}
