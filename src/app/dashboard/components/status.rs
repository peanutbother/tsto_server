use super::{
    events::EventSection, health::HealthSection, lobby::LobbySection, players::PlayersSection,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn StatusSection() -> Element {
    rsx! {
        div { class: "p-6 pr-3",
            h2 { class: "text-lg font-semibold", {t!("status_header")} }
            HealthSection {}
            PlayersSection {}
            LobbySection {}
            EventSection {}
        }
    }
}
