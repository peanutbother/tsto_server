use crate::{
    app::controllers::mayhem::MayhemController,
    config::DIRECTIONS,
    protos::{
        com::ea::simpsons::client::{log::ClientLogMessage, metrics::ClientMetricsMessage},
        data::{
            ClientConfigResponse, CurrencyData, DeleteTokenRequest, DeleteTokenResponse,
            ExtraLandMessage, ExtraLandResponse, GetFriendDataRequest, GetFriendDataResponse,
            LandMessage, WholeLandTokenResponse,
        },
    },
    util::{
        extractors::{LandUpdateToken, NucleusToken},
        Protobuf, Xml,
    },
};
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    routing::{get, post, put},
    Extension, Router,
};
use axum_response_cache::CacheLayer;
use std::collections::HashMap;
use tracing::{debug, instrument, trace, warn};

// /mh/games
pub fn create_router() -> Router {
    let friend_data = Router::new()
        .route("/", get(friend_data))
        .route("/origin", get(friend_data_origin))
        .layer(CacheLayer::with_lifespan(60));

    let bg_gameserver_plugin = Router::new()
        .route(
            "/protoClientConfig/",
            get(client_config).layer(CacheLayer::with_lifespan(3600)),
        )
        .route("/protoWholeLandToken/:mayhem_id/", post(whole_land_token))
        .route(
            "/checkToken/:mayhem_id/protoWholeLandToken/",
            get(check_token),
        )
        .route(
            "/deleteToken/:mayhem_id/protoWholeLandToken/",
            post(delete_token),
        )
        .route("/protoland/:land_id/", get(get_protoland))
        .route("/protoland/:land_id/", put(put_protoland))
        .route("/protoland/:land_id/", post(post_protoland))
        .route("/protocurrency/:land_id/", get(proto_currency))
        .route(
            "/extraLandUpdate/:land_id/protoland/",
            post(extra_land_update),
        )
        .route("/event/:land_id/protoland/", get(proto_event))
        .route("/trackinglog/", post(trackinglog))
        .route("/trackingmetrics/", post(trackingmetrics))
        .nest("/friendData", friend_data);

    Router::new()
        .route("/lobby/time", get(lobby_time))
        // /mh/bg_gameserver_plugin
        .nest(&DIRECTIONS.mh_route(), bg_gameserver_plugin)
}

#[instrument]
// /mh/bg_gameserver_plugin/protoClientConfig/
async fn client_config() -> Result<Protobuf<ClientConfigResponse>, Xml> {
    trace!("got mayhem/client_config request");
    Ok(Protobuf(MayhemController::client_config().await?))
}

#[instrument(skip(controller))]
// /mh/bg_gameserver_plugin/protoWholeLandToken/:mayhem_id
async fn whole_land_token(
    headers: HeaderMap,
    Path(mayhem_id): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    NucleusToken(token): NucleusToken,
    Extension(controller): Extension<MayhemController>,
) -> Result<Protobuf<WholeLandTokenResponse>, Xml> {
    trace!("got mayhem/whole_land_token request");
    let force = query
        .get("force")
        .map(|v| matches!(v.as_str(), "1" | "true"))
        .unwrap_or_default();

    Ok(Protobuf(
        controller
            .whole_land_token(force, &token, &mayhem_id)
            .await?,
    ))
}

#[instrument(skip(controller))]
// /mh/bg_gameserver_plugin/checkToken/:mayhem_id/protoWholeLandToken/
async fn check_token(
    headers: HeaderMap,
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    Extension(controller): Extension<MayhemController>,
) -> Result<Protobuf<WholeLandTokenResponse>, Xml> {
    trace!("got mayhem/check_token request");
    Ok(Protobuf(controller.check_token(&token, &mayhem_id).await?))
}

#[instrument(skip(controller))]
// /mh/bg_gameserver_plugin/deleteToken/:mayhem_id/protoWholeLandToken/
async fn delete_token(
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    Extension(controller): Extension<MayhemController>,
    Protobuf(DeleteTokenRequest { token: proto_token }): Protobuf<DeleteTokenRequest>,
) -> Result<Protobuf<DeleteTokenResponse>, Xml> {
    trace!("got mayhem/delete_token request");
    Ok(Protobuf(
        controller
            .delete_token(&token, &proto_token.unwrap_or_default(), &mayhem_id)
            .await?,
    ))
}

#[instrument(skip(controller))]
// /mh/bg_gameserver_plugin/protoland/:land_id
async fn get_protoland(
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    LandUpdateToken(land_update_token): LandUpdateToken,
    Extension(controller): Extension<MayhemController>,
) -> Result<Protobuf<LandMessage>, Xml> {
    trace!("got mayhem/get_protoland request");
    Ok(Protobuf(
        controller
            .get_protoland(&mayhem_id, &token, &land_update_token.to_owned())
            .await?,
    ))
}

#[instrument(skip(controller))]
// /mh/bg_gameserver_plugin/protoland/land_id
async fn post_protoland(
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    LandUpdateToken(land_update_token): LandUpdateToken,
    Extension(controller): Extension<MayhemController>,
    Protobuf(land_message): Protobuf<LandMessage>,
    // land_message: Bytes,
) -> Result<Xml, Xml> {
    trace!("got mayhem/post_protoland request");
    Ok(controller
        .update_protoland(
            &mayhem_id,
            &token,
            &land_update_token.to_owned(),
            &land_message,
            true,
        )
        .await
        .map(|_| {
            Xml::ok(
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><WholeLandUpdateResponse/>"#,
            )
        })?)
}

#[instrument(skip(controller))]
// /mh/bg_gameserver_plugin/protoland/:land_id
async fn put_protoland(
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    LandUpdateToken(land_update_token): LandUpdateToken,
    Extension(controller): Extension<MayhemController>,
    Protobuf(land_message): Protobuf<LandMessage>,
    // land_message: Bytes,
) -> Result<Protobuf<LandMessage>, Xml> {
    trace!("got mayhem/put_protoland request");
    Ok(controller
        .update_protoland(
            &mayhem_id,
            &token,
            &land_update_token.to_owned(),
            &land_message,
            false,
        )
        .await
        .map(|_| Protobuf(land_message))?)
}

#[instrument(skip(controller))]
async fn proto_currency(
    headers: HeaderMap,
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    Extension(controller): Extension<MayhemController>,
) -> Result<Protobuf<CurrencyData>, Xml> {
    trace!("got mayhem/proto_currency request");
    Ok(Protobuf(
        controller.proto_currency(&mayhem_id, &token).await?,
    ))
}

#[instrument(skip(controller))]
async fn extra_land_update(
    headers: HeaderMap,
    Path(mayhem_id): Path<String>,
    NucleusToken(token): NucleusToken,
    LandUpdateToken(land_update_token): LandUpdateToken,
    Extension(controller): Extension<MayhemController>,
    Protobuf(extra_land_message): Protobuf<ExtraLandMessage>,
) -> Result<Protobuf<ExtraLandResponse>, Xml> {
    trace!("got mayhem/extra_land_update request");

    Ok(Protobuf(
        controller
            .extra_land_update(
                &mayhem_id,
                &land_update_token.to_owned(),
                &token,
                &extra_land_message,
            )
            .await?,
    ))
}

#[instrument]
async fn proto_event(
    headers: HeaderMap,
    Protobuf(event): Protobuf<crate::protos::data::EventsMessage>,
) -> Result<Protobuf<()>, Xml> {
    trace!("got mayhem/proto_event request");

    Ok(Protobuf(()))
}

#[instrument(skip(log_message))]
async fn trackinglog(Protobuf(log_message): Protobuf<ClientLogMessage>) -> Result<Xml, Xml> {
    warn!(
        target: concat!(env!("CARGO_PKG_NAME"),"::on_clientlog"),
        source = log_message.source,
        message = log_message.text,
    );

    Ok(Xml::ok(
        r#"<?xml version="1.0" encoding="UTF-8"?>
            <Resources>
                <URI>OK</URI>
            </Resources>"#,
    ))
}

#[instrument]
async fn trackingmetrics(
    headers: HeaderMap,
    Protobuf(metrics): Protobuf<ClientMetricsMessage>,
) -> Result<Xml, Xml> {
    debug!(target: concat!(env!("CARGO_PKG_NAME"),"::on_metrics"), "{metrics:?}");

    Ok(Xml::ok(
        r#"<?xml version="1.0" encoding="UTF-8"?>
            <Resources>
                <URI>OK</URI>
            </Resources>"#,
    ))
}

#[instrument]
async fn lobby_time() -> Result<Xml, Xml> {
    trace!("got mayhem/lobby_time request");
    let epoch = MayhemController::get_lobby_time().map_err(|_| Xml::internal_error())?;

    Ok(Xml::ok(format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <Time><epochMilliseconds>{}</epochMilliseconds></Time>"#,
        epoch
    )))
}

#[instrument]
// TODO implement friend data
async fn friend_data(
    Protobuf(_req): Protobuf<GetFriendDataRequest>,
) -> Result<Protobuf<GetFriendDataResponse>, Xml> {
    trace!("got mayhem/friend_data request");

    Ok(Protobuf(GetFriendDataResponse {
        friend_data: vec![],
        error: None,
    }))
}

#[instrument]
// TODO implement friend data
async fn friend_data_origin(
    Protobuf(_req): Protobuf<GetFriendDataRequest>,
) -> Result<Protobuf<GetFriendDataResponse>, Xml> {
    trace!("got mayhem/friend_data_origin request");

    Ok(Protobuf(GetFriendDataResponse {
        friend_data: vec![],
        error: None,
    }))
}
