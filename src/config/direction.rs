use super::OPTIONS;
use crate::{
    app::models::direction::{Direction, DirectionToml, HashMapToVec, KVPair},
    util::{relative_path, DIRECTORIES},
};
use std::{env, fs::create_dir_all};

impl Direction {
    pub fn new() -> Self {
        read_config().expect("config is valid")
    }
    pub fn mh_route(&self) -> String {
        format!("/{}", self.mayhem_game_code)
    }
}

fn read_toml() -> anyhow::Result<DirectionToml> {
    let mut path = if OPTIONS.take().portable {
        relative_path().map_err(anyhow::Error::from)?
    } else {
        DIRECTORIES.config_local_dir().to_path_buf()
    };
    path.push("directions.toml");

    if !path.exists() {
        let parent = path.parent().expect("path is valid utf-8");
        if !parent.exists() {
            create_dir_all(parent)?;
        }

        let config = DirectionToml::default();
        std::fs::write(path, toml::to_string_pretty(&config)?)?;

        return Ok(config);
    }

    let content = std::fs::read_to_string(path)?;

    Ok(toml::from_str(&content)?)
}

pub fn read_config() -> anyhow::Result<Direction> {
    let base: Direction = serde_json::from_str(crate::assets::DIRECTION_ROW_CONFIG)?;
    let overrides = read_toml().unwrap_or_default();

    Ok(Direction {
        client_secret: overrides.client_secret.unwrap_or(base.client_secret),
        mayhem_game_code: overrides.mayhem_game_code.unwrap_or(base.mayhem_game_code),
        server_api_version: env!("CARGO_PKG_VERSION").to_owned(),
        telemetry_freq: overrides.telemetry_freq.unwrap_or(base.telemetry_freq),
        poll_intervals: overrides
            .poll_intervals
            .map(<Vec<KVPair> as HashMapToVec<_, _, _>>::map)
            .unwrap_or(base.poll_intervals),
        server_data: overrides
            .server_data
            .map(<Vec<KVPair> as HashMapToVec<_, _, _>>::map)
            .unwrap_or(base.server_data)
            .into_iter()
            .map(rewrite_server_address)
            .collect(),
        ..base
    })
}

fn rewrite_server_address(mut kv: KVPair) -> KVPair {
    let config = OPTIONS.take().clone();

    if kv.value.is_empty() {
        kv.value = match config.port {
            80 | 443 => format!("{}/", config.server_address.clone()),
            _ => {
                format!("{}:{}/", config.server_address, config.port)
            }
        };
    }

    kv
}
