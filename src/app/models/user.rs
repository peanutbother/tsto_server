use crate::util::secs_from_unix_epoch;
use std::collections::BTreeMap;

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum AuthResponse {
    Anonymous(LoginResponse),
    EA(LoginResponse),
}

#[derive(Debug, Default, serde::Serialize)]
pub struct LoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lnglv_token: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub id_token: Token,
    pub refresh_token: String,
    pub refresh_token_expires_in: u64,
    pub token_type: String,
}

impl TokenResponse {
    pub fn new(access_token: String, id: String, anonymous: bool) -> Self {
        Self {
            access_token,
            // TODO Never expires (for now)
            expires_in: Token::EXPIRATION,
            id_token: Token(id, anonymous),
            // TODO implement refresh token
            refresh_token: "NotImplemented".to_owned(),
            // TODO expire token and set proper expiration time
            refresh_token_expires_in: Token::EXPIRATION,
            token_type: Token::TYPE.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Token(String, bool);

impl Token {
    // Thank you tehfens!
    const KEY: &str = "2Tok8RykmQD41uWDv5mI7JTZ7NIhcZAIPtiBm4Z5";
    // About 68 years
    const EXPIRATION: u64 = 0x7FFFFFFF;
    const AUD: &str = "simpsons4-android-client";
    const ISS: &str = "accounts.ea.com";
    // TODO Probably not important, so we should be able to use it for mobile_ea_account too
    pub const PID_TYPE_EA: &str = "AUTHENTICATOR_EA_ACCOUNT";
    pub const PID_TYPE_ANONYMOUS: &str = "AUTHENTICATOR_ANONYMOUS";
    const AUTH_TIME: u64 = 0;
    const TYPE: &str = "Bearer";

    pub fn sign(&self) -> anyhow::Result<String> {
        use hmac::{Hmac, Mac};
        use jwt::SignWithKey;
        use sha2::Sha256;

        let mut claims = BTreeMap::new();
        let key: Hmac<Sha256> = Hmac::new_from_slice(Token::KEY.as_bytes())?;
        let auth_time = Token::AUTH_TIME.to_string();
        let unix_epoch = secs_from_unix_epoch()?;

        let iat = &unix_epoch.to_string();
        let exp = &(unix_epoch + Token::EXPIRATION).to_string();

        claims.insert("aud", Token::AUD);
        claims.insert("iss", Token::ISS);
        claims.insert("iat", iat);
        claims.insert("exp", exp);
        claims.insert("pid_id", &self.0);
        claims.insert("user_id", &self.0);
        claims.insert("persona_id", &self.0);
        claims.insert(
            "pid_type",
            if self.1 {
                Token::PID_TYPE_ANONYMOUS
            } else {
                Token::PID_TYPE_EA
            },
        );
        claims.insert("auth_time", &auth_time);

        Ok(claims.sign_with_key(&key)?)
    }
}

impl serde::Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let token = self
            .sign()
            .map_err(|_| serde::ser::Error::custom("failed to create jwt"))?;

        serializer.serialize_str(&token)
    }
}

#[derive(Debug)]
pub struct HeaderCheckOptions {
    pub check_underage: bool,
    pub include_authenticators: bool,
    pub include_stopprocess: bool,
    pub include_tid: bool,
}

#[cfg(feature = "server")]
impl From<axum::http::HeaderMap> for HeaderCheckOptions {
    fn from(headers: axum::http::HeaderMap) -> HeaderCheckOptions {
        HeaderCheckOptions {
            check_underage: headers
                .get("x-check-underage")
                .map(|v| v.to_str().unwrap_or_default())
                .unwrap_or_default()
                == "true",
            include_authenticators: headers
                .get("x-include-authenticators")
                .map(|v| v.to_str().unwrap_or_default())
                .unwrap_or_default()
                == "true",
            include_stopprocess: headers
                .get("x-include-stopprocess")
                .map(|v| v.to_str().unwrap_or_default())
                .unwrap_or_default()
                == "true",
            include_tid: headers
                .get("x-include-tid")
                .map(|v| v.to_str().unwrap_or_default())
                .unwrap_or_default()
                == "true",
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct TokenInfoResponse {
    client_id: String, // Always the same
    expires_in: u64,   // About 68 years
    persona_id: String,
    pid_id: String,
    pid_type: String,
    scope: String, // Always this for anonymous accounts
    user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_underage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticators: Option<Vec<Authenticator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telemetry_id: Option<String>,
}

impl TokenInfoResponse {
    pub fn new(id: u64) -> Self {
        Self {
            persona_id: id.to_string(),
            pid_id: id.to_string(),
            user_id: id.to_string(),
            ..Default::default()
        }
    }
    pub fn set_underage(&mut self, value: Option<bool>) {
        self.is_underage = value.or(Some(false));
    }
    pub fn set_authenticators(&mut self, value: Option<Vec<Authenticator>>) {
        self.authenticators = value.or(Some(vec![Authenticator::anonymous(self.user_id.clone())]));
    }
    pub fn set_stop_process(&mut self, value: Option<String>) {
        self.stop_process = value.or(Some("OFF".to_owned()));
    }
    pub fn set_telemetry_id(&mut self, value: Option<String>) {
        self.telemetry_id = value.or(Some(self.user_id.clone()));
    }
    pub fn set_pid_type(&mut self, value: Option<String>) {
        self.pid_type = value
            .or(Some(self.pid_type.clone()))
            .unwrap_or(Token::PID_TYPE_ANONYMOUS.to_owned());
    }
}

impl Default for TokenInfoResponse {
    fn default() -> Self {
        Self {
            client_id: "simpsons4-android-client".to_owned(), // Always the same
            expires_in: Token::EXPIRATION, // About 68 years
            persona_id: Default::default(),
            pid_id: Default::default(),
            pid_type: Token::PID_TYPE_ANONYMOUS.to_owned(),
            // Always this for anonymous accounts
            scope:"offline basic.antelope.links.bulk openid signin antelope-rtm-readwrite search.identity basic.antelope basic.identity basic.persona antelope-inbox-readwrite".to_owned(), 
            user_id: Default::default(),
            is_underage: None,
            authenticators: None,
            stop_process: None,
            telemetry_id: None
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Authenticator {
    authenticator_pid_id: String,
    authenticator_type: String,
}
impl Authenticator {
    pub fn anonymous(id: String) -> Self {
        Self {
            authenticator_pid_id: id,
            authenticator_type: Token::PID_TYPE_ANONYMOUS.to_owned(),
        }
    }
    pub fn ea(id: String) -> Self {
        Self {
            authenticator_pid_id: id,
            authenticator_type: Token::PID_TYPE_EA.to_owned(),
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticatorLoginType {
    #[default]
    MobileAnonymous,
    MobileEaAccount,
}

impl TryFrom<String> for AuthenticatorLoginType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "mobile_anonymous" => Ok(Self::MobileAnonymous),
            "mobile_ea_account" => Ok(Self::MobileEaAccount),
            _ => Err(anyhow::anyhow!("invalid login type")),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EALoginRequest {
    pub email: String,
    pub cred: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceIdResponse {
    // Not important, so it can be random
    pub device_id: String,
    pub result_code: usize,
    pub server_api_version: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UidResponse {
    // Not important, so it can be random
    pub uid: usize,
    pub result_code: usize,
    pub server_api_version: String,
}
