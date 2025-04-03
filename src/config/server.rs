use crate::util::DIRECTORIES;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use tracing::debug;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServerOptions {
    pub port: u16,
    pub default_donuts: u32,
    pub uid_start: u64,
    pub mid_start: u64,
    pub dlc_folder: String,
    pub dlc_routes: Vec<String>,
    pub database: String,
    pub server_address: String,
    pub log_assets: bool,
}

impl ServerOptions {
    pub fn new() -> Self {
        read_toml().expect("reading/writing config succeeds")
    }

    pub fn dlc_folder(&self) -> PathBuf {
        let mut path = DIRECTORIES.data_dir().to_path_buf();
        path.push(self.dlc_folder.clone());

        path
    }

    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let parent = path.parent().expect("parent path is valid utf-8");

        if !parent.exists() {
            create_dir_all(parent)?;
        }

        Ok(std::fs::write(path, toml::to_string_pretty(self)?)?)
    }
}

fn read_toml() -> anyhow::Result<ServerOptions> {
    let mut path = DIRECTORIES.config_dir().to_path_buf();
    path.push("server.toml");

    if let Ok(content) = std::fs::read_to_string(&path) {
        debug!("local config exists");
        Ok(toml::from_str::<ServerOptions>(&content).map(|mut opts| {
            if opts.server_address.ends_with("/") {
                // strip trailing slash
                opts.server_address =
                    opts.server_address[..opts.server_address.len() - 1].to_owned()
            }

            opts
        })?)
    } else {
        debug!("local config does not exist. default creating config");
        let server_options = ServerOptions::default();

        server_options.save(&path)?;

        Ok(server_options)
    }
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            port: 8080,
            default_donuts: 0,
            uid_start: 1000000000000,
            mid_start: 3042000000000000,
            dlc_folder: "dlc".to_owned(),
            dlc_routes: vec!["/gameassets".to_owned()],
            database: "server.db".to_owned(),
            server_address: "http://127.0.0.1".to_owned(),
            log_assets: cfg!(debug_assertions),
        }
    }
}
