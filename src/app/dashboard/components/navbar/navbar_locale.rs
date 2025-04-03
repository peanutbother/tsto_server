use crate::app::dashboard::match_i18n_str;
use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_sdk::storage::use_persistent;

#[component]
pub fn NavbarLocale() -> Element {
    let mut i18n = i18n();
    let mut locale = use_persistent("locale", || "en-US".to_owned());
    let country = use_memo(move || {
        match locale().as_ref() {
            "en-US" => "us",
            c => c,
        }
        .to_owned()
    });

    rsx! {
        div { class: "flex-none",
            div { class: "dropdown dropdown-end dropdown-hover",
                span {
                    class: "btn m-1 fi fi-{country()} fis",
                    role: "button",
                    tabindex: 0,
                }
                ul {
                    class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-xs",
                    tabindex: 0,
                    li {
                        class: "cursor-pointer",
                        onclick: move |_| {
                            i18n.set_language(match_i18n_str("en-US".to_owned()));
                            locale.set("en-US".to_owned());
                        },
                        {"English"}
                    }
                    li {
                        class: "cursor-pointer",
                        onclick: move |_| {
                            i18n.set_language(match_i18n_str("de".to_owned()));
                            locale.set("de".to_owned());
                        },
                        {"German"}
                    }
                }
            }
        }
    }
}
