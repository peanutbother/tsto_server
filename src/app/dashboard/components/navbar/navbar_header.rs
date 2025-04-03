use crate::app::dashboard::{components::navbar::navbar_locale::NavbarLocale, router::Route};
use dioxus::prelude::*;

#[component]
pub fn NavbarHeader(children: Element) -> Element {
    rsx! {
        nav { class: "fixed navbar bg-base-100 shadow-xs",
            div { class: "flex-1",
                Link { to: Route::Home {}, "TSTO Server" }
            }
            div { class: "flex-none",
                div { class: "dropdown dropdown-end dropdown-hover",
                    div { class: "btn m-1", role: "button", tabindex: 0, "⋯" }
                    ul {
                        class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-xs",
                        tabindex: 0,
                        {children}
                    }
                }
            }
            NavbarLocale {}
        }
    }
}
