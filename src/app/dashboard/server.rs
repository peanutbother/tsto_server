#[cfg(feature = "server")]
use crate::app::controllers::dashboard::DashboardController;
use crate::app::models::dashboard::*;
#[cfg(feature = "server")]
use crate::extract;
use dioxus::prelude::*;

#[server]
pub async fn set_address(address: String) -> Result<bool, ServerFnError> {
    DashboardController::set_address(address)?;
    Ok(true)
}

#[server]
pub async fn set_port(port: u16) -> Result<bool, ServerFnError> {
    DashboardController::set_port(port)?;
    Ok(true)
}

#[server]
pub async fn get_config() -> Result<ServerConfigResponse, ServerFnError> {
    extract!(controller: DashboardController);

    Ok(controller.get_config()?)
}

#[server]
pub async fn set_config(
    config: ServerConfigResponse,
) -> Result<
    (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ),
    ServerFnError,
> {
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
}

#[server]
pub async fn get_events() -> Result<EventsResponse, ServerFnError> {
    Ok(DashboardController::get_events())
}

#[server]
pub async fn get_event() -> Result<(u64, String), ServerFnError> {
    Ok(DashboardController::default().get_event()?)
}

#[server]
pub async fn set_event(ts: u64) -> Result<(), ServerFnError> {
    Ok(DashboardController::set_event(ts)?)
}

#[server]
pub async fn get_dlc_path() -> Result<String, ServerFnError> {
    Ok(DashboardController::get_dlc_path())
}

#[server]
pub async fn set_dlc_path(path: String) -> Result<(), ServerFnError> {
    Ok(DashboardController::set_dlc_path(path)?)
}

#[server]
pub async fn get_health() -> Result<StatusResponse, ServerFnError> {
    Ok(DashboardController::get_status())
}

#[server]
pub async fn get_lobby_time() -> Result<u128, ServerFnError> {
    Ok(DashboardController::get_lobby_time()?)
}

#[server]
pub async fn get_players() -> Result<u64, ServerFnError> {
    extract!(controller: DashboardController);

    Ok(controller.get_players().await?)
}
