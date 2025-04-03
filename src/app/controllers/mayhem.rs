use crate::protos::MessageFromPath;
use crate::protos::{CurrencyError, LandError};
use crate::util::Xml;
use crate::{
    config::OPTIONS,
    database::Database,
    protos::data::{
        ClientConfigResponse, CurrencyData, DeleteTokenResponse, ExtraLandMessage,
        ExtraLandResponse, LandMessage, TokenData, UserIndirectData, UsersResponseMessage,
        WholeLandTokenResponse,
    },
    util::millis_from_unix_epoch,
};
use crate::{util::DIRECTORIES, xml_response};
use std::fs::create_dir_all;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum MayhemControllerError {
    #[error("Invalid AccessToken for specified MayhemId")]
    InvalidAccessToken,
    #[error("Invalid WholeLandToken for specified MayhemId")]
    InvalidWholeLandToken,
    #[error("Resource Already Exists")]
    ResourceAlreadyExists,
    #[error("Resource Does Not Exist")]
    ResourceNotExists,
    #[error("failed to decode json")]
    JSONDecodeError(#[from] serde_json::Error),
    #[error(transparent)]
    ProtoCurrencyError(#[from] CurrencyError),
    #[error(transparent)]
    ProtoLandError(#[from] LandError),
    #[error("failed to execute query")]
    DatabaseError(#[from] sqlx::Error),
    #[error("credentials or id not found")]
    NotFound,
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<MayhemControllerError> for Xml {
    fn from(value: MayhemControllerError) -> Self {
        tracing::error!("{value}");

        match value {
            MayhemControllerError::InvalidAccessToken => {
                xml_response!("Invalid AccessToken for specified MayhemId")
            }
            MayhemControllerError::InvalidWholeLandToken => {
                xml_response!("Invalid WholeLandToken for specified MayhemId")
            }
            MayhemControllerError::ResourceNotExists => {
                xml_response!(404, "No Protoland exists for specified MayhemId")
            }
            MayhemControllerError::ResourceAlreadyExists => {
                xml_response!("RESOURCE_ALREADY_EXISTS")
            }
            MayhemControllerError::NotFound => {
                xml_response!(404, "Invalid AccessToken for specified MayhemId")
            }
            MayhemControllerError::JSONDecodeError(_)
            | MayhemControllerError::ProtoCurrencyError(_)
            | MayhemControllerError::ProtoLandError(_)
            | MayhemControllerError::DatabaseError(_)
            | MayhemControllerError::IO(_)
            | MayhemControllerError::Time(_)
            | MayhemControllerError::Unknown(_) => Xml::internal_error(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MayhemController {
    db: Database,
}

impl Default for MayhemController {
    fn default() -> Self {
        Self {
            db: crate::database::DATABASE
                .get()
                .expect("database is initialized")
                .clone(),
        }
    }
}

impl MayhemController {
    #[instrument]
    pub async fn client_config() -> Result<ClientConfigResponse, MayhemControllerError> {
        let client_config_response: ClientConfigResponse =
            serde_json::from_str(crate::assets::CLIENT_CONFIG)?;

        // client_config_response.items.iter_mut().for_each(|c| {
        //     match c.name.as_ref().map_or("", |c| c.as_str()) {
        //         "ServerSaveInterval" => {
        //             c.value = Some("0.01".to_owned());
        //         }
        //         _ => {}
        //     }
        // });

        Ok(client_config_response)
    }

    #[instrument(skip(self))]
    // /mh/bg_gameserver_plugin/protoWholeLandToken/:mayhem_id
    pub async fn whole_land_token(
        &self,
        force: bool,
        token: &String,
        mayhem_id: &String,
    ) -> Result<WholeLandTokenResponse, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token, whole_land_token
            FROM users
            WHERE mayhem_id = ?"#;
        const UPDATE_QUERY: &str = r#"
            UPDATE users
            SET whole_land_token = ?
            WHERE mayhem_id = ?"#;

        match sqlx::query_as::<_, (String, Option<String>)>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok((user_access_token, whole_land_token)) => {
                debug!("user found: {mayhem_id}");

                if user_access_token != *token {
                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                if whole_land_token.is_some() && !force {
                    return Err(MayhemControllerError::ResourceAlreadyExists);
                }

                let new_token = Uuid::new_v4();

                sqlx::query(UPDATE_QUERY)
                    .bind(new_token.to_string())
                    .bind(mayhem_id)
                    .fetch_optional(db)
                    .await?;

                let whole_land_token_response = WholeLandTokenResponse {
                    token: Some(new_token.to_string()),
                    conflict: None,
                };

                Ok(whole_land_token_response)
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    // /mh/bg_gameserver_plugin/checkToken/:mayhem_id/protoWholeLandToken/
    pub async fn check_token(
        &self,
        token: &String,
        mayhem_id: &String,
    ) -> Result<WholeLandTokenResponse, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token, whole_land_token
            FROM users
            WHERE mayhem_id = ?"#;

        match sqlx::query_as::<_, (String, String)>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok((user_access_token, whole_land_token)) => {
                debug!("user found: {mayhem_id}");

                if user_access_token != *token {
                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                let whole_land_token_response = WholeLandTokenResponse {
                    token: Some(whole_land_token),
                    conflict: None,
                };

                Ok(whole_land_token_response)
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    // /mh/bg_gameserver_plugin/deleteToken/:mayhem_id/protoWholeLandToken/
    pub async fn delete_token(
        &self,
        header_token: &String,
        proto_token: &String,
        mayhem_id: &String,
    ) -> Result<DeleteTokenResponse, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token, whole_land_token
            FROM users
            WHERE mayhem_id = ?"#;
        const UPDATE_QUERY: &str = r#"
            UPDATE users
            SET whole_land_token = ?
            WHERE mayhem_id = ?"#;

        match sqlx::query_as::<_, (String, String)>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok((user_access_token, whole_land_token)) => {
                debug!("user found: {mayhem_id}");

                if user_access_token != *header_token {
                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                if whole_land_token != *proto_token {
                    return Ok(DeleteTokenResponse {
                        result: Some(false),
                    });
                }

                sqlx::query(UPDATE_QUERY)
                    .bind("")
                    .bind(mayhem_id)
                    .fetch_optional(db)
                    .await?;

                Ok(DeleteTokenResponse { result: Some(true) })
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    // /mh/bg_gameserver_plugin/protoland/{landId}
    pub async fn get_protoland(
        &self,
        mayhem_id: &String,
        header_token: &String,
        land_update_token: &String,
    ) -> Result<LandMessage, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token, whole_land_token
            FROM users
            WHERE mayhem_id = ?"#;

        match sqlx::query_as::<_, (String, String)>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok((user_access_token, whole_land_token)) => {
                debug!("user found: {mayhem_id}");

                if user_access_token != *header_token {
                    warn!("token mismatch: {user_access_token} != {header_token}");

                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                if whole_land_token != *land_update_token {
                    warn!("token mismatch: {whole_land_token} != {land_update_token}");

                    return Err(MayhemControllerError::InvalidWholeLandToken);
                }

                let mut path = DIRECTORIES.config_dir().to_path_buf();
                path.push(format!("{mayhem_id}/land.pb"));

                if !path.exists() {
                    warn!("path does not exist: {path:?}");

                    create_dir_all(path.parent().expect("data dir exists"))?;

                    info!("creating {mayhem_id} save {path:?}");

                    let land = LandMessage::new(mayhem_id)?;
                    land.save(path)?;

                    return Ok(land);
                }

                let mut land = LandMessage::load(&path)?;

                if land.id != Some(mayhem_id.to_owned()) {
                    warn!("saved id mismatch. overwriting: {mayhem_id}");

                    land.id = Some(mayhem_id.to_owned());
                }

                Ok(land)
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument]
    // /mh/bg_gameserver_plugin/protoland/{landId}
    pub async fn update_protoland(
        &self,
        mayhem_id: &String,
        header_token: &String,
        land_update_token: &String,
        land_message: &LandMessage,
        force: bool,
    ) -> Result<(), MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token, whole_land_token
            FROM users
            WHERE mayhem_id = ?"#;

        match sqlx::query_as::<_, (String, String)>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok((user_access_token, whole_land_token)) => {
                debug!("user found: {mayhem_id}");

                if user_access_token != *header_token {
                    warn!("token mismatch: {user_access_token} != {header_token}");

                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                if whole_land_token != *land_update_token {
                    warn!("token mismatch: {user_access_token} != {land_update_token}");

                    return Err(MayhemControllerError::InvalidWholeLandToken);
                }

                let mut path = DIRECTORIES.config_dir().to_path_buf();
                path.push(format!("{mayhem_id}/land.pb"));

                if !path.exists() {
                    if !force {
                        return Err(MayhemControllerError::ResourceNotExists);
                    }

                    create_dir_all(path.parent().expect("data dir exists"))?;
                }

                land_message.save(&path)?;

                Ok(())
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    // /mh/games/bg_gameserver_plugin/protocurrency/{mayhem_id}
    pub async fn proto_currency(
        &self,
        mayhem_id: &String,
        header_token: &String,
    ) -> Result<CurrencyData, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token
            FROM users
            WHERE mayhem_id = ?"#;

        match sqlx::query_scalar::<_, String>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok(access_token) => {
                debug!("user found: {mayhem_id}");

                if access_token != *header_token {
                    warn!("token mismatch: {header_token} != {access_token}");

                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                let mut path = DIRECTORIES.config_dir().to_path_buf();
                path.push(format!("{mayhem_id}/currency.pb"));

                let currency = if !path.exists() {
                    info!("creating {mayhem_id} save {path:?}");

                    create_dir_all(path.parent().expect("data dir exists"))
                        .map_err(|e| MayhemControllerError::Unknown(e.into()))?;

                    let epoch = millis_from_unix_epoch()
                        .map_err(|e| MayhemControllerError::Unknown(e.into()))?
                        as i64;

                    let config = OPTIONS.take();
                    let data = CurrencyData {
                        id: Some(mayhem_id.clone()),
                        vc_total_purchased: Some(0),
                        vc_total_awarded: Some(config.default_donuts as i32),
                        vc_balance: Some(config.default_donuts as i32),
                        created_at: Some(epoch),
                        updated_at: Some(epoch),
                        unverified: None,
                    };

                    data.save(&path)?;

                    data
                } else {
                    debug!("currency exists: {mayhem_id}");

                    CurrencyData::load(&path)?
                };

                debug!("loaded currency: {currency:?}");

                Ok(currency)
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("currency not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn extra_land_update(
        &self,
        mayhem_id: &String,
        land_update_token: &String,
        header_token: &String,
        extra_land_message: &ExtraLandMessage,
    ) -> Result<ExtraLandResponse, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
            SELECT user_access_token, whole_land_token
            FROM users
            WHERE mayhem_id = ?"#;

        match sqlx::query_as::<_, (String, String)>(QUERY)
            .bind(mayhem_id)
            .fetch_one(db)
            .await
        {
            Ok((user_access_token, whole_land_token)) => {
                debug!("user found: {mayhem_id}");

                if user_access_token != *header_token {
                    warn!("token mismatch: {header_token} != {user_access_token}");

                    return Err(MayhemControllerError::InvalidAccessToken);
                }
                if whole_land_token != *land_update_token {
                    warn!("token mismatch: {land_update_token} != {whole_land_token}");

                    return Err(MayhemControllerError::InvalidWholeLandToken);
                }

                let mut path = DIRECTORIES.config_dir().to_path_buf();
                path.push(format!("{mayhem_id}/currency.pb"));

                let currency = if !path.exists() {
                    info!("creating {mayhem_id} save {path:?}");

                    create_dir_all(path.parent().expect("data dir exists"))
                        .map_err(|e| MayhemControllerError::Unknown(e.into()))?;

                    let epoch = millis_from_unix_epoch()
                        .map_err(|e| MayhemControllerError::Unknown(e.into()))?
                        as i64;
                    let config = OPTIONS.take();

                    let data = CurrencyData {
                        id: Some(mayhem_id.clone()),
                        vc_total_purchased: Some(0),
                        vc_total_awarded: Some(config.default_donuts as i32),
                        vc_balance: Some(config.default_donuts as i32),
                        created_at: Some(epoch),
                        updated_at: Some(epoch),
                        unverified: None,
                    };

                    data.save(&path)?;

                    data
                } else {
                    debug!("currency exists: {mayhem_id}");
                    CurrencyData::load(&path)?
                };

                let mut donut_delta = 0;
                let mut processed_currency_delta = vec![];
                for delta in extra_land_message.currency_delta.iter() {
                    donut_delta += delta.amount.unwrap_or(0);

                    processed_currency_delta.push(delta.clone());
                }

                let new_total = currency.vc_total_awarded.unwrap_or_default() + donut_delta;

                let epoch = millis_from_unix_epoch()
                    .map_err(|e| MayhemControllerError::Unknown(e.into()))?
                    as i64;

                CurrencyData {
                    id: currency.id,
                    vc_total_purchased: currency.vc_total_purchased,
                    vc_total_awarded: Some(new_total),
                    vc_balance: Some(new_total),
                    created_at: currency.created_at,
                    updated_at: Some(epoch),
                    unverified: None,
                }
                .save(&path)?;

                Ok(ExtraLandResponse {
                    processed_currency_delta,
                    ..Default::default()
                })
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    // /mh/users
    pub async fn set_user(
        &self,
        header_token: &String,
        application_user_id: &String,
    ) -> Result<UsersResponseMessage, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"
        SELECT mayhem_id, user_access_token, session_key
        FROM users
        WHERE user_id = ?"#;

        match sqlx::query_as::<_, (u64, String, String)>(QUERY)
            .bind(application_user_id)
            .fetch_one(db)
            .await
        {
            Ok((mayhem_id, user_access_token, session_key)) => {
                debug!("user found: {application_user_id}: {mayhem_id:?}, {user_access_token:?}, {session_key:?}");

                if user_access_token != *header_token {
                    warn!("token mismatch: {header_token} != {user_access_token}");

                    return Err(MayhemControllerError::InvalidAccessToken);
                }

                let message = UsersResponseMessage {
                    user: Some(UserIndirectData {
                        user_id: Some(mayhem_id.to_string()),
                        telemetry_id: Some(420.to_string()),
                    }),
                    token: Some(TokenData {
                        session_key: Some("".to_owned()),
                        expiration_date: None,
                    }),
                };

                Ok(message)
            }
            Err(sqlx::Error::RowNotFound) => {
                debug!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    #[instrument(skip(self))]
    // /mh/users
    pub async fn get_user(
        &self,
        header_token: &String,
        application_user_id: &String,
    ) -> Result<String, MayhemControllerError> {
        let db = &self.db;

        const QUERY: &str = r#"SELECT mayhem_id FROM users WHERE user_access_token = ?"#;

        match sqlx::query_scalar::<_, u64>(QUERY)
            .bind(header_token)
            .fetch_one(db)
            .await
        {
            Ok(mayhem_id) => {
                debug!("user found: {application_user_id}: {mayhem_id:?}");

                Ok(mayhem_id.to_string())
            }
            Err(sqlx::Error::RowNotFound) => {
                debug!("user not found");

                Err(MayhemControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(MayhemControllerError::DatabaseError(e))
            }
        }
    }

    // TODO change to setting
    pub fn get_lobby_time() -> Result<u128, MayhemControllerError> {
        Ok(crate::util::millis_from_unix_epoch()?)
    }
}
