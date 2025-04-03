use crate::{
    app::{
        controllers::user::UserController,
        models::user::{AuthResponse, AuthenticatorLoginType, TokenInfoResponse, TokenResponse},
    },
    json_error,
    util::{error::ErrorMessage, extractors::AccessToken},
};
use axum::{
    extract::Query,
    http::HeaderMap,
    routing::{get, post},
    Extension, Json, Router,
};
use std::collections::HashMap;
use tracing::trace;

// /connect
pub fn create_router() -> Router {
    Router::new()
        .route("/auth", get(auth))
        .route("/token", post(token))
        .route("/tokeninfo", get(token_info))
}

#[tracing::instrument(skip(controller))]
async fn auth(
    Query(query): Query<HashMap<String, String>>,
    Extension(controller): Extension<UserController>,
) -> Result<Json<AuthResponse>, ErrorMessage> {
    trace!("got connect/auth request");
    let response_type = query
        .get("response_type")
        .map(|q| q.split(" ").map(|s| s.to_owned()).collect::<Vec<String>>())
        .ok_or_else(|| json_error!("Missing Response Type"))?;

    let authenticator_login_type = query
        .get("authenticator_login_type")
        .ok_or_else(|| json_error!("Missing Login Type"))
        .and_then(|a| {
            AuthenticatorLoginType::try_from(a.to_owned()).map_err(|e| {
                tracing::warn!("{e}");
                json_error!("invalid authenticator type")
            })
        })?;

    let sig = query.get("sig");

    Ok(Json(
        controller
            .auth(&response_type, authenticator_login_type, sig)
            .await?,
    ))
}

#[tracing::instrument(skip(controller))]
async fn token(
    Query(query): Query<HashMap<String, String>>,
    Extension(controller): Extension<UserController>,
) -> Result<Json<TokenResponse>, ErrorMessage> {
    trace!("got connect/token request");
    let code = query.get("code").ok_or(json_error!("missing code param"))?;

    Ok(Json(controller.token(code).await?))
}

#[tracing::instrument(skip(controller))]
async fn token_info(
    headers: HeaderMap,
    AccessToken(access_token): AccessToken,
    Extension(controller): Extension<UserController>,
) -> Result<Json<TokenInfoResponse>, ErrorMessage> {
    trace!("got connect/token_info request");
    Ok(axum::Json(
        controller
            .token_info(&access_token, &headers.into())
            .await?,
    ))
}
