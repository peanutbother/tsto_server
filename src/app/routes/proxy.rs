use std::collections::HashMap;

use crate::util::error::ErrorMessage;
use crate::util::extractors::Authorization;
use crate::{
    app::{
        controllers::proxy::ProxyController,
        models::proxy::{
            EmailLoginRequest, GeoAgeRequirements, LinkResponse, PersonaDetailResponse,
            PersonaListResponse,
        },
    },
    json_error,
};
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_response_cache::CacheLayer;
use tracing::{instrument, trace};

// /proxy/identity
pub fn create_router() -> Router {
    let router = Router::new()
        .route("/pids//personas", get(fallback))
        .route("/pids/:pid/personas", get(persona_list))
        .route("/pids/me/personas/:pid", get(persona_detail))
        .route(
            "/geoagerequirements",
            get(age).layer(CacheLayer::with_lifespan(3600)),
        )
        .route("/progreg/code", post(code))
        .route("/links", get(links));

    Router::new().nest("/identity", router)
}

#[instrument]
async fn fallback() -> Result<Json<PersonaListResponse>, ErrorMessage> {
    Err(ErrorMessage {
        message: "not_found".to_owned(),
        error_description: Some("no mediator found".to_owned()),
        code: 404,
    })
}

#[instrument(skip(controller))]
async fn persona_list(
    Path(user_id): Path<String>,
    Authorization(token): Authorization,
    Extension(controller): Extension<ProxyController>,
    headers: HeaderMap,
) -> Result<Json<PersonaListResponse>, ErrorMessage> {
    trace!("got proxy/persona_list request");

    Ok(Json(
        controller.persona_list(&token.to_owned(), &user_id).await?,
    ))
}

#[instrument(skip(controller))]
async fn persona_detail(
    Path(user_id): Path<String>,
    Authorization(token): Authorization,
    Extension(controller): Extension<ProxyController>,
    headers: HeaderMap,
) -> Result<Json<PersonaDetailResponse>, ErrorMessage> {
    trace!("got proxy/persona_detail request");

    Ok(Json(
        controller
            .persona_detail(&token.to_owned(), &user_id)
            .await?,
    ))
}

#[instrument]
async fn age() -> Result<Json<GeoAgeRequirements>, String> {
    Ok(Json(ProxyController::age().await?))
}

#[instrument(skip(controller))]
async fn code(
    Extension(controller): Extension<ProxyController>,
    Authorization(token): Authorization,
    headers: HeaderMap,
    body: String,
) -> Result<(), ErrorMessage> {
    trace!("got proxy/code request");
    let EmailLoginRequest { email, code_type } =
        serde_json::from_str(&body).map_err(|_| json_error!("Invalid Request Body"))?;

    Ok(controller.code(&token, &email, &code_type).await?)
}

#[instrument(skip(controller))]
async fn links(
    Extension(controller): Extension<ProxyController>,
    Query(query): Query<HashMap<String, String>>,
    Authorization(token): Authorization,
    headers: HeaderMap,
    body: String,
) -> Result<Json<LinkResponse>, ErrorMessage> {
    trace!("got proxy/links request");
    let persona_namespace = query
        .get("personaNamespace")
        .cloned()
        .unwrap_or("gsp-redcrow-simpsons4".to_owned());

    Ok(Json(controller.links(&token, &persona_namespace).await?))
}
