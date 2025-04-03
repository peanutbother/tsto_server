pub mod app;
pub mod assets;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "server")]
pub mod database;
pub mod logger;
#[cfg(feature = "server")]
pub mod protos;
pub mod util;

#[macro_export]
macro_rules! load_gate {
    ($data:expr, $data_ident:ident => $body:block) => {
        match $data {
            None => rsx! {
                p {
                    class: "mt-3",
                    {dioxus_i18n::t!("msg_loading")}
                }
            },
            Some(Err(_e)) => {
                #[cfg(debug_assertions)]
                {
                    tracing::error!("failed to load dashboard: {_e}");
                }

                rsx! {
                    p {
                        class: "mt-3",
                        {dioxus_i18n::t!("error_server_response")}
                    }
                }
            }
            Some(Ok($data_ident)) => $body,
        }
    };
    ($data:expr, $data_ident:ident => $body:block, $error:ident => $error_body:block) => {
        match $data {
            None => rsx! {
                p {
                    class: "mt-3",
                    {dioxus_i18n::t!("msg_loading")}
                }
            },
            Some(Err($error)) => $error_body,
            Some(Ok($data_ident)) => $body,
        }
    };
    ($data:expr, $data_ident:ident => $body:block, fallback: $fallback:block) => {
        if let Some(Ok($data_ident)) = $data {
            $body
        } else {
            $fallback,
        }
    };
}
