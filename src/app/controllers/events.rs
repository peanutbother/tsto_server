// use crate::database::Database;
use crate::{json_error, util::error::ErrorMessage};
use tracing::{error, instrument, warn};

#[derive(Debug, thiserror::Error)]
pub enum EventControllerError {
    #[error("No event could be found with that timestamp")]
    NotFound,
    #[error("failed to execute db query")]
    Database(#[from] sqlx::Error),
}

impl From<EventControllerError> for u16 {
    fn from(value: EventControllerError) -> Self {
        match value {
            EventControllerError::NotFound => 404,
            EventControllerError::Database(_) => 500,
        }
    }
}

impl From<EventControllerError> for ErrorMessage {
    fn from(value: EventControllerError) -> ErrorMessage {
        tracing::error!("{value}");

        match value {
            EventControllerError::NotFound => json_error!(value.into(), format!("{value}")),
            EventControllerError::Database(_) => json_error!(),
        }
    }
}

impl From<EventControllerError> for axum::Json<ErrorMessage> {
    fn from(value: EventControllerError) -> Self {
        axum::Json(value.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventController {}

impl EventController {
    #[instrument(skip(self))]
    pub fn set_event(&self, ts: u64) -> Result<(), EventControllerError> {
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn get_event(&self) -> Result<(u64, String), EventControllerError> {
        let event = crate::app::models::events::TSTO_EVENTS[0];

        Ok((event.0, event.1.to_owned()))
    }
}
