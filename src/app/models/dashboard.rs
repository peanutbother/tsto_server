#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CreditsResponse {
    pub credits: Vec<Credit>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Credit {
    pub name: String,
    pub comment: String,
    pub source: Option<String>,
}

impl Credit {
    pub fn new(name: impl AsRef<str>, comment: impl AsRef<str>, source: Option<String>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            comment: comment.as_ref().to_owned(),
            source,
        }
    }
}

#[derive(
    Debug, Default, Clone, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize, PartialEq,
)]
pub enum Status {
    #[default]
    Offline,
    Online,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StatusResponse {
    pub uptime: u64,
    pub status: Status,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ServerConfigResponse {
    pub server_address: String,
    pub port: u16,
    pub default_donuts: u32,
    pub dlc_folder: String,
    pub current_event: u64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct EventsResponse {
    pub events: Vec<(u64, String)>,
    pub active: u64,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ServerLog {
    pub level: String,
    pub timestamp: String,
    pub fields: ServerLogFields,
    pub target: String,
    pub filename: String,
}

use std::fmt::Display;
impl Display for ServerLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} {} {}",
            self.timestamp, self.level, self.fields.message
        ))
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ServerLogFields {
    pub message: String,
}
