use crate::{
    app::{
        dashboard::{
            components::{
                address::AddressSection, dlc::DlcSection, donuts::DefaultDonuts,
                events::EventDropdownSection,
            },
            server::*,
        },
        models::dashboard::ServerConfigResponse,
    },
    load_gate,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Debug)]
pub enum OnChangeKey {
    ServerAddress,
    DlcFolder,
    Port,
    DefaultDonuts,
    CurrentEvent,
}

pub type OnChangeCallback = Callback<(OnChangeKey, String)>;

fn validate_config(config: &ServerConfigResponse) -> bool {
    !config.dlc_folder.is_empty() && !config.server_address.is_empty()
}

#[component]
pub fn ConfigSection() -> Element {
    let config = use_resource(get_config);
    let events = use_resource(get_events);

    let mut submit_disabled = use_signal(|| false);
    let mut current_config = use_signal(|| None);
    let mut current_event: Signal<Option<u64>> = use_signal(|| None);

    use_effect(move || {
        if let Some(Ok(config)) = config() {
            current_config.set(Some(config));
        }
        if let Some(Ok(config)) = events() {
            current_event.set(Some(config.active));
        }
    });

    let on_change: OnChangeCallback = use_callback(move |(key, value): (OnChangeKey, String)| {
        // safety: on_change can only run once mounted which is only after current_config was updated by use_effect
        let mut new_config: ServerConfigResponse = current_config().unwrap();

        #[cfg(debug_assertions)]
        tracing::debug!("on_change of type {key:?} with {value}");

        match key {
            OnChangeKey::ServerAddress => new_config.server_address = value,
            OnChangeKey::DlcFolder => new_config.dlc_folder = value,
            // safety: values come pre-checked from sub-components
            OnChangeKey::Port => {
                new_config.port = value
                    .parse()
                    .expect("provided callback value should already be validated")
            }
            OnChangeKey::DefaultDonuts => {
                new_config.default_donuts = value
                    .parse()
                    .expect("provided callback value should already be validated")
            }
            OnChangeKey::CurrentEvent => current_event.set(Some(
                value
                    .parse()
                    .expect("provided callback value should already be validated"),
            )),
        };

        submit_disabled.set(!validate_config(&new_config));

        current_config.set(Some(new_config));
    });

    rsx! {
        div { class: "p-6 overflow-scroll",
            h2 { class: "text-lg font-semibold", {t!("config_header")} }
            div {
                {
                    load_gate! {
                        config(), config => { rsx! { AddressSection { address : config.server_address
                        .clone(), port : config.port, on_change } DlcSection { path : config
                        .dlc_folder.clone(), on_change } DefaultDonuts { donuts : config
                        .default_donuts, on_change } if let Some(Ok(events)) = events() {
                        EventDropdownSection { data : events, on_change } } button { class :
                        "mt-3 btn btn-primary", disabled : submit_disabled, onclick : move | _ | {
                        let config = config.clone(); async move { if let Some(new_config) =
                        current_config() { if new_config == config { return; } submit_disabled
                        .set(true); set_config(new_config). await .ok(); submit_disabled.set(false);
                        } } }, { t!("config_save") } } } }
                    }
                }
            }
        }
    }
}
