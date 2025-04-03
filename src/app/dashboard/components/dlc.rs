use super::config::{OnChangeCallback, OnChangeKey};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn DlcSection(path: String, on_change: OnChangeCallback) -> Element {
    let mut value = use_signal(|| path);

    rsx! {
        div { class: "mt-3",
            label { {t!("dlc_header")} }
            input {
                value,
                r#type: "text",
                name: "path",
                class: "input input-primary w-full",
                oninput: move |e| async move {
                    let folder = e.value();
                    value.set(folder.clone());
                    if !folder.is_empty() {
                        on_change.call((OnChangeKey::DlcFolder, folder));
                    }
                },
            }
            if value().is_empty() {
                label { r#for: "path", class: "text-error", {t!("dlc_empty")} }
            }
        }
    }
}
