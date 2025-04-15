use crate::app::{
    dashboard::providers::use_permissions,
    models::{auth::Role, dashboard::ServerLog},
};
use dioxus::prelude::*;
use dioxus_i18n::t;
use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use std::rc::Rc;
use tracing::{debug, error};
use web_sys::{wasm_bindgen::JsValue, Blob, Url};

type ScrollTarget = Option<Rc<MountedData>>;

#[component]
pub fn Logs() -> Element {
    let mut raw_output = use_signal(|| false);
    let mut messages_raw: Signal<Vec<String>> = use_signal(Vec::new);
    let messages: Memo<Vec<ServerLog>> = use_memo(move || {
        messages_raw()
            .iter()
            .map(|msg| serde_json::from_str::<ServerLog>(msg).unwrap_or_default())
            .collect()
    });
    let mut scroll_target: Signal<ScrollTarget> = use_signal(|| None);
    let callback = move |raw_output: bool| {
        let document = web_sys::window().unwrap().document().unwrap();
        let logs = if raw_output {
            messages_raw().join("\r\n")
        } else {
            messages()
                .iter()
                .map(ServerLog::to_string)
                .collect::<Vec<String>>()
                .join("\r\n")
        };
        let array = web_sys::js_sys::Array::new();
        array.push(&JsValue::from_str(&logs));
        let blob = Blob::new_with_str_sequence(&JsValue::from(&array)).unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let a = document.create_element("a").unwrap();
        a.set_attribute("href", &url).unwrap();
        a.set_attribute(
            "download",
            &format!("server_log.{}", if raw_output { "jsonl" } else { "log" }),
        )
        .unwrap();
        let event = document.create_event("MouseEvent").unwrap();
        event.init_event("click");
        a.dispatch_event(&event).unwrap();
        Url::revoke_object_url(&url).unwrap();
    };

    use_future(move || async move {
        let url = "/dashboard/logs";
        debug!("Connecting to websocket at {url}");
        let mut socket = WebSocket::open(url).unwrap();
        debug!("Connected to websocket.");

        loop {
            match socket.next().await {
                Some(Ok(Message::Text(msg))) => {
                    messages_raw.push(msg);

                    // scroll to newest log element
                    if let Some(scroll) = scroll_target() {
                        let _ = scroll.scroll_to(ScrollBehavior::Smooth).await;
                    }
                }
                Some(Ok(Message::Bytes(msg))) => {
                    error!("Received binary message: {:?}", msg);
                }
                Some(Err(err)) => {
                    error!("Error: {:?}", err);
                    break;
                }
                None => {
                    break;
                }
            }
        }

        debug!("Disconnected from websocket");
    });

    if !use_permissions(Role::Operator) {
        return rsx! {
            div { class: "body-container-h min-w-11/12 max-w-11/12 min-h-96 max-h-96 pt-4 mb-8",
                p { class: "text-center", "Missing permission for this page" }
            }
        };
    }

    rsx! {
        div { class: "body-container-h min-w-11/12 max-w-11/12 min-h-96 max-h-96 pt-4 mb-8",
            ul { class: "pl-4 pr-4 w-full min-h-80 max-h-80 overflow-scroll snap-y",
                if raw_output() {
                    for msg in messages_raw.iter() {
                        li {
                            class: "gap-2 snap-start scroll-mb-3 text-nowrap",
                            onmounted: move |cx| scroll_target.set(Some(cx.data())),
                            span { "{msg}" }
                        }
                    }
                } else {
                    for msg in messages.iter() {
                        li {
                            class: "gap-2 snap-start scroll-mb-3 text-nowrap",
                            onmounted: move |cx| scroll_target.set(Some(cx.data())),
                            em { class: "shrink-0 min-w-45", "{msg.timestamp}" }
                            b { class: "shrink-0, min-w-10", "{msg.level}" }
                            span { "{msg.fields.message}" }
                        }
                    }
                }
            }
            button {
                class: "p-4 ml-4 mt-0 btn bt-primary rounded-xl",
                onclick: move |_| {
                    raw_output.set(!raw_output());
                },
                svg {
                    class: "size-6",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: 1.5,
                    stroke: "currentColor",

                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M17.25 6.75 22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3-4.5 16.5",
                    }
                }
            }
            button { class: "ml-2",
                div { class: "dropdown dropdown-start dropdown-click",
                    span {
                        class: "btn btn-primary",
                        role: "button",
                        tabindex: 0,
                        {t!("logs_save")}
                    }
                    ul {
                        class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-xs",
                        tabindex: 0,
                        li {
                            class: "cursor-pointer",
                            onclick: move |_| {
                                callback(true);
                            },
                            {"JSON"}
                        }
                        li {
                            class: "cursor-pointer",
                            onclick: move |_| {
                                callback(false);
                            },
                            {"TXT"}
                        }
                    }
                }
            }
                // button { class: "p-4 ml-2 mt-0 btn bt-primary rounded-xl",
        // onclick: move |_| {
        //     let document = web_sys::window().unwrap().document().unwrap();
        //     let logs = if raw_output() {
        //         messages_raw().join("\r\n")
        //     } else {
        //         messages()
        //             .iter()
        //             .map(ServerLog::to_string)
        //             .collect::<Vec<String>>()
        //             .join("\r\n")
        //     };
        //     let array = web_sys::js_sys::Array::new();
        //     array.push(&JsValue::from_str(&logs));
        //     let blob = Blob::new_with_str_sequence(&JsValue::from(&array)).unwrap();
        //     let url = Url::create_object_url_with_blob(&blob).unwrap();
        //     let a = document.create_element("a").unwrap();
        //     a.set_attribute("href", &url).unwrap();
        //     a.set_attribute(
        //             "download",
        //             &format!("server_log.{}", if raw_output() { "jsonl" } else { "log" }),
        //         )
        //         .unwrap();
        //     let event = document.create_event("MouseEvent").unwrap();
        //     event.init_event("click");
        //     a.dispatch_event(&event).unwrap();
        //     Url::revoke_object_url(&url).unwrap();
        // },
        // {t!("logs_save")}
        // }
        }
    }
}
