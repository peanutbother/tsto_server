use crate::app::controllers::mayhem::MayhemController;
use crate::protos::data::UsersResponseMessage;
use crate::util::extractors::NucleusToken;
use crate::util::{Protobuf, Xml};
use crate::xml_response;
use axum::routing::get;
use axum::{extract::Query, routing::put, Extension, Router};
use axum_response_cache::CacheLayer;
use std::collections::HashMap;
use tracing::{instrument, trace};

// /mh/users
pub fn create_router() -> Router {
    Router::new()
        .route("/", put(set_user))
        .route("/", get(get_user).layer(CacheLayer::with_lifespan(3600)))
}

#[instrument(skip(controller))]
async fn set_user(
    Query(query): Query<HashMap<String, String>>,
    NucleusToken(token): NucleusToken,
    Extension(controller): Extension<MayhemController>,
) -> Result<Protobuf<UsersResponseMessage>, Xml> {
    trace!("got mayhem/set_user request");

    let application_user_id = query
        .get("applicationUserId")
        .ok_or(xml_response!("applicationUserId"))?;

    Ok(Protobuf(
        controller.set_user(&token, application_user_id).await?,
    ))
}

#[instrument(skip(controller))]
async fn get_user(
    Query(query): Query<HashMap<String, String>>,
    NucleusToken(token): NucleusToken,
    Extension(controller): Extension<MayhemController>,
) -> Result<Xml, Xml> {
    trace!("got mayhem/get_user request");

    let application_user_id = query
        .get("applicationUserId")
        .ok_or(xml_response!("applicationUserId"))?;

    Ok(Xml(
        200,
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Resources>
                <URI>/users/{}</URI>
            </Resources>"#,
            controller.get_user(&token, application_user_id).await?,
        ),
    ))
}
