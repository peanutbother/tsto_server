use super::config::{OnChangeCallback, OnChangeKey};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AddressSection(address: String, port: u16, on_change: OnChangeCallback) -> Element {
    rsx! {
        ServerAddress { address, on_change }
        ServerPort { port, on_change }
    }
}

#[component]
fn ServerAddress(address: String, on_change: OnChangeCallback) -> Element {
    // let _i18n = i18n();
    // let mut address = use_signal(|| address);
    let mut value = use_signal(|| address);

    let oninput = move |e: Event<FormData>| {
        let address = e.value();
        value.set(address.clone());

        if !address.is_empty() {
            on_change.call((OnChangeKey::ServerAddress, address));
        }
    };

    rsx! {
        div { class: "mt-3",
            label { class: "block", {t!("address_header")} }
            input {
                r#type: "text",
                name: "address",
                class: "input input-primary w-full",
                value,
                oninput,
            }

            if value().is_empty() {
                label { class: "text-error", {t!("address_empty")} }
            }
        }
    }
}

#[component]
fn ServerPort(port: u16, on_change: OnChangeCallback) -> Element {
    let mut value = use_signal(|| port);
    let mut port_error = use_signal(|| false);

    let oninput = move |e: Event<FormData>| {
        if let Ok(parsed_port) = e.parsed::<u16>() {
            value.set(parsed_port);
            port_error.set(false);
            on_change.call((OnChangeKey::Port, e.value()));
        } else {
            port_error.set(true);
        }
    };

    rsx! {
        div { class: "mt-3",
            label { class: "block", {t!("address_header")} }
            input {
                r#type: "number",
                name: "port",
                class: "input input-primary w-full",
                value,
                oninput,
            }

            if port_error() {
                label { class: "text-error", {t!("address_empty")} }
            }
        }
    }
}
