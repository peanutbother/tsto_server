use axum::{extract::Query, http::HeaderMap, routing::post, Router};
use std::collections::HashMap;
use tracing::{instrument, trace};

// /mh/userstats
pub fn create_router() -> Router {
    Router::new().route("/", post(user_stats))
}

#[instrument]
async fn user_stats(
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(), String> {
    trace!(target: concat!(env!("CARGO_PKG_NAME"),"::on_userstats"), "got user stats");

    Ok(())
}
