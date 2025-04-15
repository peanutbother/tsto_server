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

#[macro_export]
macro_rules! require_auth {
    ($session:ident => $body:block) => {
        $crate::extract!($session: Session);

        if $session.user.is_some() {
            return $body
        } else {
            return Err(AuthControllerError::Unauthorized.into());
        }
    };
    ($perm:expr, $session:ident => $body:block) => {
        $crate::extract!($session: Session);

        if $session.user.is_some() {
            $crate::require_perm!($session, $perm);
            return $body
        } else {
            return Err(AuthControllerError::Unauthorized.into());
        }
    };
}

#[macro_export]
macro_rules! require_perm {
    ($session:ident, $perm:expr) => {
        if !axum_login::AuthzBackend::has_perm(
            &$session.backend,
            $session.user.as_ref().unwrap(),
            $perm,
        )
        .await?
        {
            return Err(AuthControllerError::MissingPermissions.into());
        }
    };
}
