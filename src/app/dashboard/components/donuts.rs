use super::config::{OnChangeCallback, OnChangeKey};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn DefaultDonuts(donuts: u32, on_change: OnChangeCallback) -> Element {
    let mut value = use_signal(|| donuts);
    let mut donuts_error = use_signal(|| false);

    let oninput = move |e: Event<FormData>| {
        if let Ok(parsed_port) = e.parsed::<u32>() {
            value.set(parsed_port);
            donuts_error.set(false);
            on_change.call((OnChangeKey::DefaultDonuts, e.value()));
        } else {
            donuts_error.set(true);
        }
    };

    rsx! {
        div { class: "mt-3",
            label { class: "block", {t!("default_donuts_header")} }
            input {
                r#type: "number",
                name: "port",
                class: "input input-primary w-full",
                value,
                oninput,
            }

            if donuts_error() {
                label { class: "text-error", {t!("default_donuts_empty")} }
            }
        }
    }
}
