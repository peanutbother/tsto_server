use crate::protos::data::GameplayConfigResponse;
use crate::util::{Protobuf, Xml};
use axum::{routing::get, Router};
use axum_response_cache::CacheLayer;
use tracing::{instrument, trace};

// /mh/gameplayconfig
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(gameplay_config))
        .layer(CacheLayer::with_lifespan(3600))
}

#[instrument]
async fn gameplay_config() -> Result<Protobuf<GameplayConfigResponse>, Xml> {
    trace!("got mayhem/gameplay_config request");
    let gameplay_config_response: GameplayConfigResponse = serde_json::from_str(
        crate::assets::GAMEPLAY_CONFIG,
    )
    .map_err(Xml::internal_error().log_with_message("failed to decode GameplayConfigResponse"))?;

    Ok(Protobuf(gameplay_config_response))
}
