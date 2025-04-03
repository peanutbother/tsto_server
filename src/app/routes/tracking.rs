use axum::{http::HeaderMap, routing::post, Json, Router};
use serde_json::Value;
use tracing::{instrument, trace};

use crate::app::models::tracking::TrackingData;

// /tracking/
pub fn create_router() -> Router {
    let router = Router::new().route("/core/logEvent", post(log_event));

    Router::new().nest("/api", router)
}

#[instrument(skip(body))]
async fn log_event(headers: HeaderMap, body: Json<Vec<TrackingData>>) -> Json<Value> {
    trace!(target: concat!(env!("CARGO_PKG_NAME"),"::on_event"), "{body:?}");
    Json(serde_json::json!({"status": "ok"}))
}
