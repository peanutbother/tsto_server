use crate::{
    app::{
        controllers::user::UserController,
        models::user::{DeviceIdResponse, UidResponse},
    },
    json_error,
    util::error::ErrorMessage,
};
use axum::{extract::Query, routing::get, Json, Router};
use std::collections::HashMap;
use tracing::{instrument, trace};

// /user/api
pub fn create_router() -> Router {
    let router = Router::new()
        .route("/:platform/getDeviceID", get(get_device_id))
        .route("/:plaform/validateDeviceID", get(validate_device_id))
        .route("/:platform/getAnonUid", get(get_anon_uid));

    Router::new().nest("/api", router)
}

#[instrument]
async fn get_device_id() -> Json<DeviceIdResponse> {
    Json(UserController::get_device_id())
}

#[instrument]
async fn validate_device_id(
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<DeviceIdResponse>, ErrorMessage> {
    trace!("got user/validate_device_id request");
    let device_id = query
        .get("eadeviceid")
        .ok_or_else(|| json_error!("missing eadeviceid"))?
        .to_owned();

    Ok(Json(UserController::validate_device_id(device_id)?))
}

#[instrument]
async fn get_anon_uid() -> Json<UidResponse> {
    Json(UserController::get_anon_uid())
}
