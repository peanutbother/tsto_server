use dioxus::prelude::*;

use crate::app::models::dashboard::Credit;
// use ui::{Echo, Hero};

#[component]
pub fn Credits() -> Element {
    let credits = use_server_future(get_credits)?;

    rsx! {
        div { class: "body-container",
            div { class: "p-6 pr-3",
                div { class: "mt-3",
                    h2 { class: "text-1xl font-bold text-center", "Credits" }
                    {
                        match credits() {
                            Some(Ok(credits)) => rsx! {
                                CreditsComponent { credits }
                            },
                            _ => rsx! { "Failed to load credits" },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CreditsComponent(credits: Vec<Credit>) -> Element {
    rsx! {
        ul { class: "p-4 list-disc",
            for credit in credits {
                li {
                    strong {
                        if let Some(source) = &credit.source {
                            a {
                                href: "{source}",
                                target: "_blank",
                                class: "text-blue-800 dark:text-green-400 hover:underline",
                                "{credit.name}"
                            }
                        } else {
                            "{credit.name}"
                        }
                    }
                    " - {credit.comment}"
                }
            }
        }
        p { class: "mt-3",
            "Big thanks go to these people!"
            br {}
            "Without them this project wouldn't be possible."
        }
    }
}

#[server]
async fn get_credits() -> Result<Vec<Credit>, ServerFnError> {
    use crate::util::CREDITS;

    Ok(CREDITS.clone())
}
