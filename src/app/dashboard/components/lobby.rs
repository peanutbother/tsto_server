use super::super::server::*;
use crate::load_gate;
use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn LobbySection() -> Element {
    let data = use_resource(get_lobby_time);

    rsx! {
        div { class: "mt-3",
            {
                load_gate!(
                    data(), data => { let time = Utc.timestamp_millis_opt(data as i64).single();
                    if let Some(time) = time { rsx! { p { strong { { t!("lobby_status") } }
                    "{time}" } } } else { rsx! { p { class : "text-error", {
                    t!("error_invalid_response") } } } } }
                )
            }
        }
    }
}
