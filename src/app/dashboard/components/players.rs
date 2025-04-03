use crate::load_gate;

use super::super::server::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn PlayersSection() -> Element {
    let data = use_resource(get_players);

    rsx! {
        div { class: "mt-3",
            {
                load_gate!(
                    data(), data => { rsx! { p { strong { { t!("players_status") } } "{data}" } }
                    }
                )
            }
        }
    }
}
