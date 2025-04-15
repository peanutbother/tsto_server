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
            auth::{AuthController, Session},
            dashboard::DashboardController,
            direction::DirectionController,
            mayhem::MayhemController,
            proxy::ProxyController,
            user::UserController,
        },
        config::OPTIONS,
        database::Database,
    };
    use axum::{
        extract::Request,
        middleware::{self, Next},
        response::{IntoResponse, Redirect, Response},
        Extension, Router,
    };
    use axum_login::{
        tower_sessions::{Expiry, SessionManagerLayer},
        AuthManagerLayerBuilder,
    };
    use std::net::{Ipv4Addr, SocketAddr};
    use time::Duration;
    use tokio::net::TcpListener;
    use tracing::{debug, instrument};

    const PROTECTED_ROUTES: &[&str] = &["/", "/logs"];

    pub async fn create_router() -> anyhow::Result<Router> {
        use crate::app::dashboard::App;
        use dioxus::{fullstack::server::DioxusRouterExt, prelude::*};

        let auth_controller = AuthController::default();
        let session_manager = SessionManagerLayer::new(Database::session_store()?)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        Ok(super::routes::create_router()
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
            .layer(Database::extension().await?)
            .layer(Extension(UserController::default()))
            .layer(Extension(DashboardController::default()))
            .layer(Extension(DirectionController))
            .layer(Extension(MayhemController::default()))
            .layer(Extension(ProxyController::default()))
            .layer(Extension(UserController::default()))
            .layer(Extension(auth_controller.clone()))
            .layer(middleware::from_fn(auth_middleware))
            .layer(AuthManagerLayerBuilder::new(auth_controller, session_manager).build()))
    }

    pub async fn create_listener() -> anyhow::Result<TcpListener> {
        let ip =
            dioxus::cli_config::server_ip().unwrap_or(std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        let port = dioxus::cli_config::server_port().unwrap_or(OPTIONS.take().port);
        let addr = SocketAddr::new(ip, port);

        Ok(tokio::net::TcpListener::bind(addr).await?)
    }

    #[instrument(skip(session))]
    async fn auth_middleware(session: Session, request: Request, next: Next) -> Response {
        if (PROTECTED_ROUTES).contains(&request.uri().path()) {
            return require_auth(session, request, next).await;
        }

        next.run(request).await
    }

    #[instrument(skip(session))]
    pub async fn require_auth(session: Session, request: Request, next: Next) -> Response {
        if session.user.is_none() {
            debug!("unauthenticated request");
            return Redirect::to("/login").into_response();
        }

        next.run(request).await
    }
}
