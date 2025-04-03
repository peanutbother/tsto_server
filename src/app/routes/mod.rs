mod connect;
mod dashboard;
mod director;
mod mayhem;
mod proxy;
mod tracking;
mod user;

use crate::config::OPTIONS;
use crate::logger::with_tracing;
use crate::util::Xml;
use axum::extract::Request;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tracing::warn;

pub fn create_router() -> Router {
    let mh_router = mayhem::create_router();
    let director_router = director::create_router();
    let user_router = user::create_router();
    let tracking_router = tracking::create_router();
    let proxy_router = proxy::create_router();
    let connect_router = connect::create_router();
    let dashboard_router = dashboard::create_router();
    let dlc_service = game_assets();

    let mut service = Router::new()
    .route("/probe", get(probe))
    .nest("/mh", mh_router)
    .nest("/director", director_router)
    .nest("/user", user_router.clone())
    .nest("//user", user_router)
    .nest("/tracking", tracking_router.clone())
    .nest("//tracking", tracking_router)
    .nest("/proxy", proxy_router.clone())
    .nest("//proxy", proxy_router) /* //proxy/identity/geoagerequirements?client_id=simpsons4-android-client  */
    .nest("/connect", connect_router)
    .nest("/dashboard", dashboard_router)
    .layer(with_tracing!());

    let options = OPTIONS.take();
    for path in options.dlc_routes.iter() {
        service = service.nest_service(path, dlc_service.clone());
    }

    if options.log_assets {
        service = service.layer(with_tracing!(
            concat!(env!("CARGO_PKG_NAME"), "::on_request_asset"),
            concat!(env!("CARGO_PKG_NAME"), "::on_response_asset")
        ))
    }

    service
}

fn game_assets() -> ServeDir<axum::routing::MethodRouter> {
    ServeDir::new(OPTIONS.take().dlc_folder()).fallback(get(fallback_no_file))
}

#[tracing::instrument]
async fn fallback_no_file(req: Request) -> Xml {
    warn!(target: concat!(env!("CARGO_PKG_NAME"),"::on_request_asset"), "requested unknown uri: {}", req.uri());

    (
        404_u16,
        r#"<?xml version="1.0" encoding="UTF-8"?>
            <error code="404" type="NOT_FOUND" field="file not found"/>"#,
    )
        .into()
}

#[tracing::instrument]
// Send empty response, as that's what the client expects
async fn probe() -> Result<(), String> {
    Ok(())
}
