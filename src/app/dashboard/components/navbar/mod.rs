use super::super::router::Route;
use crate::app::{
    dashboard::providers::{use_loggedin, use_permissions},
    models::auth::Role,
};
use dioxus::prelude::*;
use dioxus_i18n::t;
use navbar_header::NavbarHeader;
use navbar_outlet::NavbarOutlet;

mod navbar_header;
mod navbar_locale;
mod navbar_outlet;

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn Navbar() -> Element {
    let logged_in = use_loggedin();
    let is_operator = use_permissions(Role::Operator);

    rsx! {
        NavbarHeader {
            Link { to: Route::Home {}, {t!("route_home")} }
            if logged_in {
                a { href: "/dashboard/logout", {t!("route_logout")} }
            } else {
                Link { to: Route::Login {}, {t!("route_login")} }
            }
            if is_operator {
                Link { to: Route::Logs {}, {t!("route_logs")} }
            }
            Link { to: Route::Credits {}, {t!("route_credits")} }
        }

        NavbarOutlet {
            Outlet::<Route> {}

            p { class: "dark:text-white bg-",
                em {
                    // TODO handle RTL layout
                    dangerous_inner_html: "Copyright 2025 MIT License &middot;&nbsp;",
                }
                Link { to: Route::Credits {}, {t!("route_credits")} }
            }
        }
    }
}
