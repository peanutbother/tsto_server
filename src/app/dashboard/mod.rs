pub mod server;
use dioxus::prelude::*;
use dioxus_i18n::{
    prelude::{use_init_i18n, I18nConfig},
    unic_langid::{langid, LanguageIdentifier},
};
use dioxus_sdk::storage::use_persistent;
use providers::use_user_provider;
use router::Route;

use crate::assets::LOCALES;

mod components;
mod providers;
mod router;
mod views;

static FAVICON: Asset = asset!("/assets/favicon.ico");
static STYLESHEET: Asset = asset!("/assets/tailwind.css");

fn match_i18n_str(locale: String) -> LanguageIdentifier {
    match locale.as_ref() {
        "en-US" => langid!("en-US"),
        "de" => langid!("de"),
        _ => unimplemented!("{locale} has no translation"),
    }
}

#[component]
pub fn App() -> Element {
    let locale: Signal<String> = use_persistent("locale", || "en-US".to_owned());
    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(match_i18n_str(locale()))
            // en-US
            .with_locale((langid!("en-US"), LOCALES.en_us))
            // de
            .with_locale((langid!("de"), LOCALES.de))
            // fallback
            .with_fallback(langid!("en-US"))
    });

    use_effect(move || {
        i18n.set_language(match_i18n_str(locale()));
    });

    use_user_provider();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: STYLESHEET }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/gh/lipis/flag-icons@7.2.3/css/flag-icons.min.css" }
        document::Title { "TSTO Server" }


        Router::<Route> {}
    }
}
