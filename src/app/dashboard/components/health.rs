use super::super::server::*;
use crate::app::models::dashboard::{Status, StatusResponse};
use async_std::task::sleep;
use dioxus::prelude::*;
use dioxus_i18n::t;
use pretty_duration::pretty_duration;
use std::time::Duration;

#[component]
pub fn HealthSection() -> Element {
    let mut data = use_resource(get_health);
    let mut counter = use_signal(|| 0);
    let uptime = use_memo(move || {
        let res = data()
            .unwrap_or(Ok(StatusResponse::default()))
            .unwrap_or(StatusResponse::default());
        if res.status == Status::Offline {
            0
        } else {
            res.uptime + counter()
        }
    });

    // refetch status every 10 seconds
    use_future(move || async move {
        loop {
            data.restart();
            sleep(Duration::from_secs(10)).await;
        }
    });

    // update uptime every second (no refetch)
    use_future(move || async move {
        loop {
            counter.set(counter() + 1);
            sleep(Duration::from_secs(1)).await;
        }
    });

    rsx! {
        div { class: "mt-3",
            {
                let data = data()
                    .unwrap_or(Ok(StatusResponse::default()))
                    .unwrap_or(StatusResponse::default());
                let uptime = pretty_duration(&Duration::from_secs(uptime()), None);
                rsx! {
                    p {
                        strong { {t!("health_header")} }
                        {" "}
                        {
                            match data.status.clone() {
                                Status::Online => rsx! {
                                    span { class: "text-green-600 dark:text-green-400", {t!("status_online")} }
                                },
                                Status::Offline => rsx! {
                                    span { class: "text-red-400", {t!("status_offline")} }
                                },
                            }
                        }
                    }
                    p { class: "mt-3",
                        // TODO handle RTL layout
                        strong { {t!("health_uptime")} }
                        {uptime}
                    }
                }
            }
        }
    }
}
