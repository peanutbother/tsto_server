use crate::{
    app::{controllers::direction::DirectionController, models::direction::Direction},
    util::Xml,
    xml_response,
};
use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use axum_response_cache::CacheLayer;
use std::collections::HashMap;
use tracing::{instrument, trace};

// /director/api
pub fn create_router() -> Router {
    let router = Router::new()
        .route("/:platform/getDirectionByPackage", get(by_package))
        .route("/:platform/getDirectionByBundle", get(by_bundle))
        .layer(CacheLayer::with_lifespan(3600));

    Router::new().nest("/api", router)
}

#[instrument]
async fn by_package(
    Path(platform): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Direction>, Xml> {
    trace!("got director/by_package request");
    let package_id = query
        .get("packageId")
        .ok_or(xml_response!("No packageId"))?;

    Ok(Json(
        DirectionController::by_package(&platform, package_id).await,
    ))
}

#[instrument]
async fn by_bundle(
    Path(platform): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Direction>, Xml> {
    trace!("got director/by_bundle request");
    let bundle_id = query.get("bundleId").ok_or(xml_response!["No bundleId"])?;

    Ok(Json(
        DirectionController::by_bundle(&platform, bundle_id).await,
    ))
}
