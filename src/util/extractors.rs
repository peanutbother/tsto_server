use super::{error::ErrorMessage, Xml};
use crate::{json_error, xml_response};
use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    RequestPartsExt,
};
use std::collections::HashMap;

#[macro_export]
macro_rules! extract {
    ($value:ident : $value_type:ty) => {
        let axum::Extension::<$value_type>($value) = dioxus::prelude::extract().await?;
    };
    ($($value:ident : $value_type:ty),+ $(,)?) => {
        $($crate::extract!($value : $value_type);)+
    };
}

pub struct NucleusToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for NucleusToken {
    type Rejection = Xml;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        Ok(Self(
            headers
                .get("nucleus_token")
                .or(headers.get("mh_auth_params"))
                .ok_or((
                    400_u16,
                    r#"<?xml version="1.0" encoding="UTF-8"?>
                        <error code="400" type="MISSING_VALUE" field="nucleus_token"/>"#,
                ))?
                .to_str()
                .map_err(|_| Xml::internal_error())?
                .replace(" ", "+"),
        ))
    }
}

pub struct LandUpdateToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for LandUpdateToken {
    type Rejection = Xml;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        Ok(Self(
            headers
                .get("land-update-token")
                .map(|h| h.to_str().unwrap_or_default().to_owned())
                .ok_or(xml_response!(
                    400_u16,
                    "Invalid WholeLandToken for specified MayhemId"
                ))?,
        ))
    }
}

pub struct AccessToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AccessToken {
    type Rejection = ErrorMessage;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(query) = parts
            .extract::<Query<HashMap<String, String>>>()
            .await
            .map_err(|_| json_error!())?;

        let headers = &parts.headers;

        Ok(Self(
            headers
                .get("access_token")
                .map(|h| h.to_str().ok().unwrap_or_default().to_owned())
                .or(query.get("access_token").map(|a| a.to_owned()))
                .ok_or_else(|| json_error!("Invalid Authorization"))?
                .replace(" ", "+"),
        ))
    }
}

pub struct Authorization(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Authorization {
    type Rejection = ErrorMessage;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        Ok(Self(
            headers
                .get("authorization")
                .ok_or(json_error!("missing authorization header"))?
                .to_str()
                .map_err(|_| json_error!("invalid authorization header"))?
                .split(" ")
                .last()
                .ok_or(json_error!("invalid authorization header"))?
                .to_owned(),
        ))
    }
}
