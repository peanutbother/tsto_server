use crate::{app::models::direction::Direction, config::DIRECTIONS};
use tracing::{debug, instrument};

#[derive(Debug, Default, Clone)]
pub struct DirectionController;

impl DirectionController {
    #[instrument]
    pub async fn by_package(platform: &String, package_id: &String) -> Direction {
        debug!("direction for {package_id}({platform}) requested");

        Self::direction(platform).await
    }

    #[instrument]
    pub async fn by_bundle(platform: &String, bundle_id: &String) -> Direction {
        debug!("direction for {bundle_id}({platform}) requested");

        Self::direction(platform).await
    }

    #[instrument]
    async fn direction(platform: &String) -> Direction {
        let mut dir = DIRECTIONS.clone();

        dir.client_id = format!("simpsons4-{platform}-client");
        dir.mdm_app_key = format!("simpsons4-{platform}");

        dir
    }
}
