use axum::Router;

pub mod config;
pub mod games;
pub mod link;
pub mod telemetry;
pub mod users;
pub mod userstats;

// /mh/
pub fn create_router() -> Router {
    Router::new()
        .nest("/games", games::create_router())
        .nest("/gameplayconfig", config::create_router())
        .nest("/users", users::create_router())
        .nest("/userstats", userstats::create_router())
        .nest("/link", link::create_router())
        .nest("/clienttelemetry", telemetry::create_router())
}
