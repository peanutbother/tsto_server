use crate::app::dashboard::components::{config::ConfigSection, status::StatusSection};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "body-container md:grid-cols-2",

            StatusSection {}
            ConfigSection {}
        }
    }
}
