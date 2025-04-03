use crate::{
    app::models::user::{
        AuthResponse, AuthenticatorLoginType, DeviceIdResponse, EALoginRequest, HeaderCheckOptions,
        LoginResponse, Token, TokenInfoResponse, TokenResponse, UidResponse,
    },
    config::OPTIONS,
    database::Database,
    json_error,
    util::error::ErrorMessage,
};
use base64::{engine::general_purpose, Engine};
use rand::RngCore;
use tracing::{debug, error, instrument, warn};

#[derive(Debug, thiserror::Error)]
pub enum UserControllerError {
    #[error("unknown authenticator type")]
    Authenticator,
    #[error("No user could be found with that UserAccessCode")]
    NotFound,
    #[error("Email verification is currently disabled")]
    SMTPNotEnabled,
    #[error("No user could be found with that email")]
    WrongEmail,
    #[error("missing signature")]
    MissingSignature,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid signature")]
    InvalidVerificationCode,
    #[error("invalid verification code")]
    InvalidSignatureCred(#[from] std::num::ParseIntError),
    #[error("failed to execute query")]
    Database(#[from] sqlx::Error),
}

impl From<UserControllerError> for u16 {
    fn from(value: UserControllerError) -> Self {
        match value {
            UserControllerError::Authenticator
            | UserControllerError::MissingSignature
            | UserControllerError::WrongEmail
            | UserControllerError::SMTPNotEnabled
            | UserControllerError::InvalidSignature
            | UserControllerError::InvalidVerificationCode
            | UserControllerError::InvalidSignatureCred(_) => 400,
            UserControllerError::NotFound => 404,
            UserControllerError::Database(_) => 500,
        }
    }
}

impl From<UserControllerError> for ErrorMessage {
    fn from(value: UserControllerError) -> ErrorMessage {
        tracing::error!("{value}");

        match value {
            UserControllerError::Authenticator
            | UserControllerError::MissingSignature
            | UserControllerError::WrongEmail
            | UserControllerError::SMTPNotEnabled
            | UserControllerError::InvalidSignature
            | UserControllerError::InvalidVerificationCode
            | UserControllerError::NotFound
            | UserControllerError::InvalidSignatureCred(_) => {
                json_error!(value.into(), format!("{value}"))
            }
            UserControllerError::Database(_) => json_error!(),
        }
    }
}

impl From<UserControllerError> for axum::Json<ErrorMessage> {
    fn from(value: UserControllerError) -> Self {
        axum::Json(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct UserController {
    db: Database,
}

impl Default for UserController {
    fn default() -> Self {
        Self {
            db: crate::database::DATABASE
                .get()
                .expect("database is initialized")
                .clone(),
        }
    }
}

impl UserController {
    #[instrument(skip(self))]
    pub async fn auth(
        &self,
        response_type: &Vec<String>,
        authenticator_login_type: AuthenticatorLoginType,
        signature: Option<&String>,
    ) -> Result<AuthResponse, UserControllerError> {
        let db = &self.db;

        const LAST_USER_QUERY: &str =
            r#"SELECT mayhem_id, user_id from users ORDER BY user_id DESC LIMIT 1"#;
        const NEW_USER_QUERY: &str = r#"INSERT INTO users (user_id, mayhem_id, user_access_token, user_access_code) VALUES (?, ?, ?, ?)"#;
        const USER_BY_EMAIL_QUERY: &str = r#"SELECT user_access_token, user_access_code, user_verification_code FROM users WHERE user_email = ?"#;

        match authenticator_login_type {
            AuthenticatorLoginType::MobileAnonymous => {
                let (mayhem_id, user_id): (Option<u64>, Option<u64>) =
                    sqlx::query_as(LAST_USER_QUERY)
                        .fetch_optional(db)
                        .await?
                        .unwrap_or_default();

                let config = OPTIONS.take().clone();
                let new_uid = user_id.unwrap_or(config.uid_start) + 1;
                let new_mid = mayhem_id.unwrap_or(config.mid_start) + 1;
                let new_access_token = UserController::generate_secret("AT".to_owned(), new_uid);
                let new_access_code = UserController::generate_secret("AC".to_owned(), new_uid);

                debug!(
                    "requested {authenticator_login_type:?} for {new_mid} type: {response_type:?}"
                );

                let mut response = LoginResponse::default();

                if response_type.iter().any(|t| t == "code") {
                    response.code = Some(new_access_code.clone());
                }
                if response_type.iter().any(|t| t == "lnglv_token") {
                    response.lnglv_token = Some(new_access_token.clone());
                }

                sqlx::query(NEW_USER_QUERY)
                    .bind(new_uid.to_string())
                    .bind(new_mid.to_string())
                    .bind(new_access_token)
                    .bind(new_access_code)
                    .fetch_optional(db)
                    .await?;

                Ok(AuthResponse::Anonymous(response))
            }
            AuthenticatorLoginType::MobileEaAccount => {
                let sig = signature.ok_or(UserControllerError::MissingSignature)?;

                let (body, _) = sig.split_once(".").unwrap_or_default();
                let body = base64::engine::general_purpose::STANDARD
                    .decode(body.to_owned() + "==")
                    .map_err(|e| {
                        error!("{e}");
                        UserControllerError::InvalidSignature
                    })?;
                let body = String::from_utf8(body).map_err(|e| {
                    error!("{e}");
                    UserControllerError::InvalidSignature
                })?;
                let EALoginRequest { email, cred } = serde_json::from_str(&body).map_err(|e| {
                    error!("{e}");
                    UserControllerError::InvalidSignature
                })?;
                let cred = cred.parse::<u32>()?;

                match sqlx::query_as::<_, (String, String, u32)>(USER_BY_EMAIL_QUERY)
                    .bind(email)
                    .fetch_one(db)
                    .await
                {
                    Ok((user_access_token, user_access_code, user_verification_code)) => {
                        if user_verification_code != cred {
                            return Err(UserControllerError::InvalidVerificationCode);
                        }

                        let mut response = LoginResponse::default();

                        if response_type.iter().any(|v| v == "code") {
                            response.code = Some(user_access_code)
                        }
                        if response_type.iter().any(|v| v == "lnglv_token") {
                            response.lnglv_token = Some(user_access_token)
                        }

                        Ok(AuthResponse::EA(response))
                    }
                    Err(sqlx::Error::RowNotFound) => Err(UserControllerError::WrongEmail),
                    Err(e) => {
                        tracing::error!("{e}");

                        Err(UserControllerError::Database(e))
                    }
                }
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn token(&self, code: &String) -> Result<TokenResponse, UserControllerError> {
        let db = &self.db;

        const USER_BY_CODE_QUERY: &str =
            "SELECT user_id, user_access_token, user_verification_code FROM users WHERE user_access_code = ?";

        match sqlx::query_as::<_, (u64, String, Option<u32>)>(USER_BY_CODE_QUERY)
            .bind(code)
            .fetch_one(db)
            .await
        {
            Ok((user_id, user_access_token, uvc)) => {
                debug!("user found: {user_id}");
                Ok(TokenResponse::new(
                    user_access_token,
                    user_id.to_string(),
                    uvc.is_none(),
                ))
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");
                Err(UserControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");
                Err(UserControllerError::Database(e))
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn token_info(
        &self,
        access_token: &String,
        header_options: &HeaderCheckOptions,
    ) -> Result<TokenInfoResponse, UserControllerError> {
        let db = &self.db;

        const USER_BY_TOKEN_QUERY: &str =
            "SELECT user_id, user_verification_code FROM users WHERE user_access_token = ?";

        match sqlx::query_as::<_, (u64, Option<u32>)>(USER_BY_TOKEN_QUERY)
            .bind(access_token)
            .fetch_one(db)
            .await
        {
            Ok((user_id, uvc)) => {
                debug!("user found: {user_id}");

                let mut token_info_response = TokenInfoResponse::new(user_id);

                if header_options.check_underage {
                    token_info_response.set_underage(None);
                }
                if header_options.include_authenticators {
                    token_info_response.set_authenticators(None);
                }
                if header_options.include_stopprocess {
                    token_info_response.set_stop_process(None);
                }
                if header_options.include_tid {
                    // TODO change api since `NONE` applies default which is *with* tid which is unintuitive
                    token_info_response.set_telemetry_id(None);
                }

                if uvc.is_some() {
                    token_info_response.set_pid_type(Some(Token::PID_TYPE_EA.to_owned()));
                }

                Ok(token_info_response)
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("user not found");
                Err(UserControllerError::NotFound)
            }
            Err(e) => {
                error!("{e}");
                Err(UserControllerError::Database(e))
            }
        }
    }

    fn generate_secret(prefix: String, id: u64) -> String {
        use base64::{engine::general_purpose, Engine as _};

        let mut rng = rand::rng();
        let bytes: &mut [u8; 16] = &mut [0; 16];

        rng.fill_bytes(bytes);
        format!("{prefix}{}{id}", general_purpose::STANDARD.encode(&bytes))
    }

    #[instrument]
    pub fn get_device_id() -> DeviceIdResponse {
        let mut rng = rand::rng();
        let bytes: &mut [u8; 16] = &mut [0; 16];

        rng.fill_bytes(bytes);

        DeviceIdResponse {
            // Not important, so it can be random
            device_id: general_purpose::STANDARD.encode(&bytes),
            result_code: 0,
            server_api_version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }

    #[instrument]
    pub fn validate_device_id(device_id: String) -> Result<DeviceIdResponse, UserControllerError> {
        Ok(DeviceIdResponse {
            device_id,
            result_code: 0,
            server_api_version: env!("CARGO_PKG_VERSION").to_owned(),
        })
    }

    #[instrument]
    pub fn get_anon_uid() -> UidResponse {
        UidResponse {
            // This UID is not checked to be correct, so it's static for simplicity
            uid: 1000000000000,
            result_code: 0,
            server_api_version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }

    #[instrument(skip(self))]
    pub async fn count(&self) -> Result<u64, UserControllerError> {
        Ok(sqlx::query_scalar("SELECT COUNT() FROM users")
            .fetch_one(&self.db)
            .await?)
    }
}
