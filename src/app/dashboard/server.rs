use crate::app::models::{
    auth::{Role, User},
    dashboard::*,
};
#[cfg(feature = "server")]
use crate::{
    app::controllers::{
        auth::{AuthControllerError, Session},
        dashboard::DashboardController,
    },
    extract, require_auth,
};
use dioxus::prelude::*;

#[server]
pub async fn set_address(address: String) -> Result<bool, ServerFnError> {
    require_auth!(Role::Owner, session => {
        DashboardController::set_address(address)?;
        Ok(true)
    });
}

#[server]
pub async fn set_port(port: u16) -> Result<bool, ServerFnError> {
    require_auth!(Role::Owner, session => {
        DashboardController::set_port(port)?;
        Ok(true)
    });
}

#[server]
pub async fn get_config() -> Result<ServerConfigResponse, ServerFnError> {
    require_auth!(Role::Owner, session => {
        extract!(controller: DashboardController);
        Ok(controller.get_config()?)
    });
}

type ConfigResult = (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

#[server]
pub async fn set_config(config: ServerConfigResponse) -> Result<ConfigResult, ServerFnError> {
    require_auth!(Role::Owner, session => {
        let set_address = DashboardController::set_address(config.server_address);
        let set_port = DashboardController::set_port(config.port);
        let set_default_donuts = DashboardController::set_default_donuts(config.default_donuts);
        let set_dlc_path = DashboardController::set_dlc_path(config.dlc_folder);
        let set_event = DashboardController::set_event(config.current_event);

        Ok((
            set_address.err().map(|e| format!("{e}")),
            set_port.err().map(|e| format!("{e}")),
            set_default_donuts.err().map(|e| format!("{e}")),
            set_dlc_path.err().map(|e| format!("{e}")),
            set_event.err().map(|e| format!("{e}")),
        ))
    });
}

#[server]
pub async fn get_events() -> Result<EventsResponse, ServerFnError> {
    require_auth!(Role::Operator, session => {
        Ok(DashboardController::get_events())
    });
}

#[server]
pub async fn get_event() -> Result<(u64, String), ServerFnError> {
    require_auth!(Role::User, session => {
        Ok(DashboardController::default().get_event()?)
    });
}

#[server]
pub async fn set_event(ts: u64) -> Result<(), ServerFnError> {
    require_auth!(Role::Operator, session => {
        Ok(DashboardController::set_event(ts)?)
    });
}

#[server]
pub async fn get_dlc_path() -> Result<String, ServerFnError> {
    require_auth!(Role::Owner, session => {
        Ok(DashboardController::get_dlc_path())
    });
}

#[server]
pub async fn set_dlc_path(path: String) -> Result<(), ServerFnError> {
    require_auth!(Role::Owner, session => {
        Ok(DashboardController::set_dlc_path(path)?)
    });
}

#[server]
pub async fn get_health() -> Result<StatusResponse, ServerFnError> {
    require_auth!(Role::User, session => {
        Ok(DashboardController::get_status())
    });
}

#[server]
pub async fn get_lobby_time() -> Result<u128, ServerFnError> {
    require_auth!(Role::User, session => {
        Ok(DashboardController::get_lobby_time()?)
    });
}

#[server]
pub async fn get_players() -> Result<u64, ServerFnError> {
    require_auth!(Role::User, session => {
        extract!(controller: DashboardController);
        Ok(controller.get_players().await?)
    });
}

#[server]
pub async fn get_role() -> Result<Role, ServerFnError> {
    require_auth!(session => {
        Ok(session.user.as_ref().unwrap().role.clone())
    });
}

#[server]
pub async fn login(creds: Credentials) -> Result<Option<User>, ServerFnError> {
    extract!(mut session: Session);

    let user = session.authenticate(creds).await?;
    if let Some(user) = user.as_ref() {
        session.login(user).await?;
    }

    Ok(session.user)
}

#[server]
pub async fn get_login() -> Result<Option<User>, ServerFnError> {
    extract!(session: Session);

    Ok(session.user)
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    extract!(mut session: Session);

    session.logout().await?;

    Ok(())
}
