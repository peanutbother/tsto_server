use crate::util::Xml;
use axum::{routing::post, Router};
use axum_response_cache::CacheLayer;
use tracing::{instrument, trace};

// /mh/link
pub fn create_router() -> Router {
    Router::new()
        .route("/:mayhem_id/users", post(users))
        .layer(CacheLayer::with_lifespan(3600))
}

#[instrument]
// Ignore the request, because i don't know what it's used for yet
async fn users() -> Xml {
    trace!("got mayhem/link request");
    Xml::ok(
        r#"<?xml version="1.0" encoding="UTF-8"?>
            <Resources>
                <URI>OK</URI>
            </Resources>"#,
    )
}
