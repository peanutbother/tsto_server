use std::fmt::Debug;

use crate::{
    app::models::proxy::{
        EmailLoginCodeType, GeoAgeRequirements, LinkResponse, Persona, PersonaDetailResponse,
        PersonaListResponse, PersonaStatus, PidGamePersonaMapping, PidGamePersonaMappings,
    },
    database::Database,
    json_error,
    util::error::ErrorMessage,
};
use rand::random_range;
use tracing::{debug, error, instrument, warn};

#[derive(Debug, thiserror::Error)]
pub enum ProxyControllerError {
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    #[error("Invalid email provided")]
    InvalidEmail,
    #[error("Tokens do not match")]
    TokenMismatch,
    #[error("No user could be found with that UserId")]
    NotFound,
    #[error("failed to execute query")]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<ProxyControllerError> for u16 {
    fn from(value: ProxyControllerError) -> Self {
        match value {
            ProxyControllerError::InvalidCredentials
            | ProxyControllerError::InvalidEmail
            | ProxyControllerError::TokenMismatch => 400,
            ProxyControllerError::NotFound => 404,
            ProxyControllerError::Database(_) | ProxyControllerError::Unknown(_) => 500,
        }
    }
}

impl From<ProxyControllerError> for ErrorMessage {
    fn from(value: ProxyControllerError) -> Self {
        tracing::error!("{value}");

        match value {
            ProxyControllerError::InvalidCredentials
            | ProxyControllerError::InvalidEmail
            | ProxyControllerError::TokenMismatch
            | ProxyControllerError::NotFound => {
                json_error!(value.into(), format!("{value}"))
            }
            ProxyControllerError::Database(_) => json_error!(),
            ProxyControllerError::Unknown(_) => json_error!(),
        }
    }
}

impl From<ProxyControllerError> for axum::Json<ErrorMessage> {
    fn from(value: ProxyControllerError) -> Self {
        axum::Json(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct ProxyController {
    db: Database,
}

impl Default for ProxyController {
    fn default() -> Self {
        Self {
            db: crate::database::DATABASE
                .get()
                .expect("database is initialized")
                .clone(),
        }
    }
}

impl ProxyController {
    #[instrument(skip(self))]
    pub async fn persona_list(
        &self,
        token: &String,
        user_id: &String,
    ) -> Result<PersonaListResponse, ProxyControllerError> {
        let db = &self.db;

        const USER_BY_UID_QUERY: &str = r#"SELECT user_email, user_name, user_access_token, creation_date FROM users WHERE user_id = ?"#;

        match sqlx::query_as::<_, (Option<String>, Option<String>, String, String)>(
            USER_BY_UID_QUERY,
        )
        .bind(user_id)
        .fetch_one(db)
        .await
        {
            Ok((email, user_name, access_token, date)) => {
                debug!("user found: {user_id}");

                if access_token != *token {
                    warn!("token mismatch: {access_token} != {token}");

                    return Err(ProxyControllerError::TokenMismatch);
                }

                Ok(PersonaListResponse::new(vec![Persona::from_data(
                    user_id, date,
                )
                .with_email(email.is_some())
                .with_name(user_name)]))
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(ProxyControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(ProxyControllerError::Database(e))
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn persona_detail(
        &self,
        token: &String,
        user_id: &String,
    ) -> Result<PersonaDetailResponse, ProxyControllerError> {
        let db = &self.db;

        const USER_BY_UID_QUERY: &str = r#"SELECT user_email, user_name, user_access_token, creation_date FROM users WHERE user_id = ?"#;

        match sqlx::query_as::<_, (Option<String>, Option<String>, String, String)>(
            USER_BY_UID_QUERY,
        )
        .bind(user_id)
        .fetch_one(db)
        .await
        {
            Ok((email, user_name, access_token, date)) => {
                debug!("user found: {user_id}");

                if access_token != *token {
                    warn!("token mismatch: {access_token} != {token}");

                    return Err(ProxyControllerError::TokenMismatch);
                }

                Ok(PersonaDetailResponse {
                    persona: Persona::from_data(user_id, date)
                        .with_email(email.is_some())
                        .with_name(user_name),
                })
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(ProxyControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(ProxyControllerError::Database(e))
            }
        }
    }

    // TODO: Replace hardcoded values with dynamic ones
    #[instrument]
    pub async fn age() -> Result<GeoAgeRequirements, String> {
        Ok(GeoAgeRequirements::default())
    }

    #[instrument(skip(self))]
    pub async fn code(
        &self,
        token: impl AsRef<str> + Debug,
        email: impl AsRef<str> + Debug,
        code_type: &EmailLoginCodeType,
    ) -> Result<(), ProxyControllerError> {
        let db = &self.db;

        const USER_BY_TOKEN_QUERY: &str = "SELECT COUNT() FROM users WHERE user_access_token = ?";
        const USER_BY_EMAIL_QUERY: &str = "SELECT COUNT() FROM users WHERE user_email = ?";
        const UPDATE_CODE_BY_EMAIL_QUERY: &str =
            "UPDATE users SET user_verification_code = ? WHERE user_email = ?";

        let token = token.as_ref();
        let email = email.as_ref();

        if token.is_empty() {
            return Err(ProxyControllerError::InvalidCredentials);
        }
        if email.is_empty() {
            return Err(ProxyControllerError::InvalidEmail);
        }

        match sqlx::query_scalar::<_, u64>(USER_BY_TOKEN_QUERY)
            .bind(token)
            .fetch_one(db)
            .await
        {
            Ok(count) => {
                if count == 0 {
                    return Err(ProxyControllerError::InvalidCredentials);
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                return Err(ProxyControllerError::InvalidCredentials);
            }
            Err(e) => {
                error!("{e}");
                return Err(ProxyControllerError::Database(e));
            }
        }

        match sqlx::query_scalar::<_, u64>(USER_BY_EMAIL_QUERY)
            .bind(email)
            .fetch_one(db)
            .await
        {
            Ok(count) => {
                if count == 0 {
                    return Err(ProxyControllerError::InvalidEmail);
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                return Err(ProxyControllerError::InvalidEmail);
            }
            Err(e) => {
                error!("{e}");
                return Err(ProxyControllerError::Database(e));
            }
        }

        let code = random_range(1000_u32..99999_u32);

        debug!(r#"generated code "{code}" for "{email}""#);

        sqlx::query(UPDATE_CODE_BY_EMAIL_QUERY)
            .bind(code)
            .bind(email)
            .execute(db)
            .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn links(
        &self,
        token: impl AsRef<str> + Debug,
        persona_namespace: impl AsRef<str> + Debug,
    ) -> Result<LinkResponse, ProxyControllerError> {
        let db = &self.db;

        const USER_BY_TOKEN: &str = r#"SELECT user_id FROM users WHERE user_access_token = ?"#;

        if token.as_ref().is_empty() || persona_namespace.as_ref().is_empty() {
            return Err(ProxyControllerError::InvalidCredentials);
        }

        match sqlx::query_scalar::<_, String>(USER_BY_TOKEN)
            .bind(token.as_ref())
            .fetch_one(db)
            .await
        {
            Ok(user_id) => Ok(LinkResponse {
                pid_game_persona_mappings: PidGamePersonaMappings {
                    pid_game_persona_mapping: vec![PidGamePersonaMapping {
                        new_created: false,
                        persona_id: user_id.clone(),
                        persona_namespace: persona_namespace.as_ref().to_owned(),
                        pid_game_persona_mapping_id: user_id.clone(),
                        pid_id: user_id,
                        status: PersonaStatus::Active,
                    }],
                },
            }),
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");

                Err(ProxyControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");

                Err(ProxyControllerError::Database(e))
            }
        }
    }
}
