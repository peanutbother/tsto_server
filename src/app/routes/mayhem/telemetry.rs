use crate::{protos::com::ea::simpsons::client::telemetry::ClientTelemetryMessage, util::Protobuf};
use axum::{extract::Query, http::HeaderMap, routing::post, Router};
use std::collections::HashMap;
use tracing::{debug, instrument};

// /mh/clienttelemetry
pub fn create_router() -> Router {
    Router::new().route("/", post(client_telemetry))
}

#[instrument(skip(telemetry))]
// Ignore the request, but make the client know we received it
async fn client_telemetry(
    _headers: HeaderMap,
    Query(_query): Query<HashMap<String, String>>,
    Protobuf(telemetry): Protobuf<ClientTelemetryMessage>,
) -> Result<(), ()> {
    debug!(target: concat!(env!("CARGO_PKG_NAME"),"::on_telemetry"), "{telemetry:?}");

    Ok(())
}
