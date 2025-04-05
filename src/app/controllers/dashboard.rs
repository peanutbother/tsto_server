use super::{
    events::{EventController, EventControllerError},
    mayhem::{MayhemController, MayhemControllerError},
    user::{UserController, UserControllerError},
};
use crate::{
    app::models::{
        dashboard::{
            CreditsResponse, EventsResponse, ServerConfigResponse, Status, StatusResponse,
        },
        events::TSTO_EVENTS,
    },
    config::OPTIONS,
    util::{relative_path, DIRECTORIES, UPTIME},
};
use tracing::instrument;

#[derive(Debug, thiserror::Error)]
pub enum DashboardControllerError {
    #[error(transparent)]
    EventController(#[from] EventControllerError),
    #[error(transparent)]
    UserController(#[from] UserControllerError),
    #[error(transparent)]
    MayhemController(#[from] MayhemControllerError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Default, Clone)]
pub struct DashboardController {
    events: EventController,
    users: UserController,
}

impl DashboardController {
    #[instrument]
    pub fn get_credits() -> CreditsResponse {
        CreditsResponse {
            credits: crate::util::CREDITS.clone(),
        }
    }

    #[instrument]
    pub fn get_status() -> StatusResponse {
        StatusResponse {
            uptime: UPTIME.elapsed().unwrap_or_default().as_secs(),
            status: Status::Online,
        }
    }

    #[instrument(skip(self))]
    pub async fn get_players(&self) -> Result<u64, DashboardControllerError> {
        Ok(self.users.count().await?)
    }

    #[instrument(skip(self))]
    pub fn get_config(&self) -> Result<ServerConfigResponse, DashboardControllerError> {
        let options = OPTIONS.take().clone();

        Ok(ServerConfigResponse {
            server_address: options.server_address,
            port: options.port,
            default_donuts: options.default_donuts,
            dlc_folder: options.dlc_folder,
            current_event: self.get_event()?.0,
        })
    }

    #[instrument]
    pub fn get_events() -> EventsResponse {
        EventsResponse {
            events: TSTO_EVENTS
                .iter()
                .map(|(k, v)| (*k, v.to_string()))
                .collect(),
            active: 0,
        }
    }

    // TODO implement
    #[instrument(skip(self))]
    pub fn get_event(&self) -> Result<(u64, String), DashboardControllerError> {
        Ok(self.events.get_event()?)
    }

    // TODO implement
    #[instrument]
    pub fn set_event(_ts: u64) -> Result<(), DashboardControllerError> {
        Ok(())
    }

    #[instrument]
    pub fn get_dlc_path() -> String {
        OPTIONS.take().dlc_folder.clone()
    }

    #[instrument]
    pub fn set_dlc_path(dlc_path: String) -> Result<(), DashboardControllerError> {
        let mut path = if OPTIONS.take().portable {
            relative_path().map_err(anyhow::Error::from)?
        } else {
            DIRECTORIES.config_local_dir().to_path_buf()
        };
        path.push("server.toml");

        let mut server_options = OPTIONS.take();
        server_options.dlc_folder = dlc_path;
        server_options.save(path)?;

        Ok(())
    }

    #[instrument]
    pub fn set_default_donuts(donuts: u32) -> Result<(), DashboardControllerError> {
        let mut path = if OPTIONS.take().portable {
            relative_path().map_err(anyhow::Error::from)?
        } else {
            DIRECTORIES.config_local_dir().to_path_buf()
        };
        path.push("server.toml");

        let mut server_options = OPTIONS.take();
        server_options.default_donuts = donuts;
        server_options.save(path)?;

        Ok(())
    }

    #[instrument]
    pub fn set_port(port: u16) -> Result<(), DashboardControllerError> {
        let mut path = if OPTIONS.take().portable {
            relative_path().map_err(anyhow::Error::from)?
        } else {
            DIRECTORIES.config_local_dir().to_path_buf()
        };
        path.push("server.toml");

        let mut server_options = OPTIONS.take();
        server_options.port = port;
        server_options.save(path)?;

        Ok(())
    }

    #[instrument]
    pub fn set_address(address: String) -> Result<(), DashboardControllerError> {
        let mut path = if OPTIONS.take().portable {
            relative_path().map_err(anyhow::Error::from)?
        } else {
            DIRECTORIES.config_local_dir().to_path_buf()
        };
        path.push("server.toml");

        let mut server_options = OPTIONS.take();
        server_options.server_address = address;
        server_options.save(path)?;

        Ok(())
    }

    #[instrument]
    pub fn get_lobby_time() -> Result<u128, DashboardControllerError> {
        Ok(MayhemController::get_lobby_time()?)
    }
}
