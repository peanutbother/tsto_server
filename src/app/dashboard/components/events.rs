use super::{
    super::server::*,
    config::{OnChangeCallback, OnChangeKey},
};
use crate::{app::models::dashboard::EventsResponse, load_gate};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn EventSection() -> Element {
    let data = use_resource(get_event);

    rsx! {
        div { class: "mt-3",
            {
                load_gate!(
                    data(), data => { rsx! { p { strong { { t!("events_current_status") } }
                    "{data.1}" } } }
                )
            }
        }
    }
}

#[component]
pub fn EventDropdownSection(data: EventsResponse, on_change: OnChangeCallback) -> Element {
    rsx! {
        div { class: "mt-3",
            label { class: "block", {t!("events_header")} }
            select {
                class: "select select-primary w-full",
                oninput: move |e| async move {
                    on_change.call((OnChangeKey::CurrentEvent, e.value()));
                },
                for (ts , name) in data.events.iter() {
                    option { value: ts.to_string(), selected: *ts == data.active, "{name}" }
                }
            }
        }
    }
}
