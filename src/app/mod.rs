#[cfg(feature = "server")]
pub mod controllers;
pub mod dashboard;
pub mod models;
#[cfg(feature = "server")]
mod routes;

#[cfg(feature = "server")]
pub use r#mod::*;
#[cfg(feature = "server")]
mod r#mod {
    use crate::{
        app::controllers::{
            dashboard::DashboardController, direction::DirectionController,
            mayhem::MayhemController, proxy::ProxyController, user::UserController,
        },
        config::OPTIONS,
        database::Database,
    };
    use axum::{Extension, Router};
    use std::net::{Ipv4Addr, SocketAddr};
    use tokio::net::TcpListener;

    pub async fn create_router() -> anyhow::Result<Router> {
        use crate::app::dashboard::App;
        use dioxus::{fullstack::server::DioxusRouterExt, prelude::*};

        Ok(super::routes::create_router()
            .layer(Database::extension().await?)
            .serve_dioxus_application(
                ServeConfigBuilder::new()
                    .incremental(
                        IncrementalRendererConfig::new()
                            .static_dir(
                                std::env::current_exe()?
                                    .parent()
                                    .expect("path is valid utf-8")
                                    .join("public"),
                            )
                            .clear_cache(false),
                    )
                    .enable_out_of_order_streaming(),
                App,
            )
            .layer(Extension(UserController::default()))
            .layer(Extension(DashboardController::default()))
            .layer(Extension(DirectionController))
            .layer(Extension(MayhemController::default()))
            .layer(Extension(ProxyController::default()))
            .layer(Extension(UserController::default())))
    }

    pub async fn create_listener() -> anyhow::Result<TcpListener> {
        let ip =
            dioxus::cli_config::server_ip().unwrap_or(std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        let port = dioxus::cli_config::server_port().unwrap_or(OPTIONS.take().port);
        let addr = SocketAddr::new(ip, port);

        Ok(tokio::net::TcpListener::bind(addr).await?)
    }
}
