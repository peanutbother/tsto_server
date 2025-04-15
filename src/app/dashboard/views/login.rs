use crate::app::dashboard::{providers::use_loggedin, router::Route};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let logged_in = use_loggedin();

    if logged_in {
        let navigator = navigator();
        navigator.push(Route::Home {});
        return rsx! {};
    }

    rsx! {
        form {
            class: "body-container pt-4 fieldset w-xs bg-base-200 border border-base-300 p-4 rounded-box",
            action: "/dashboard/login",
            method: "POST",
            legend { class: "fieldset-legend", "Login" }
            label { class: "fieldset-label", "Username" }
            input {
                r#type: "text",
                placeholder: "Username",
                class: "input",
                autocomplete: "username",
                name: "username",
            }
            label { class: "fieldset-label", "Password" }
            input {
                placeholder: "Password",
                r#type: "password",
                class: "input",
                autocomplete: "current-password",
                name: "password",
            }
            button { class: "btn btn-primary mt-4", "Login" }
        }
    }
}
