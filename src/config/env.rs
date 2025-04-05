use super::{args::Args, server::ServerOptions};

#[derive(Debug, Default)]
pub struct EnvOptions {
    port: Option<u16>,
    dlc_folder: Option<String>,
    database: Option<String>,
    server_address: Option<String>,
    log_assets: Option<bool>,
}

impl EnvOptions {
    /// parses environment variables.
    /// curently server parses the following args:
    ///
    /// `DATABASE`, `DLC_FOLDER`, `LOG_ASSETS`*, `PORT`, `SERVER_ADDRESS`
    ///
    /// *`LOG_ASSETS` will be parsed as truthy if the value equals either to `true` (case ignored) or to `1`
    pub fn parse() -> anyhow::Result<EnvOptions> {
        let env = std::env::vars();
        let mut options = EnvOptions::default();

        for (key, value) in env {
            match key.as_ref() {
                "DATABASE" => options.database = Some(value),
                "DLC_FOLDER" => options.dlc_folder = Some(value),
                "LOG_ASSETS" => {
                    options.log_assets =
                        Some(value.as_str().eq_ignore_ascii_case("true") || value.as_str() == "1");
                }
                "PORT" => options.port = Some(value.parse()?),
                "SERVER_ADDRESS" => options.server_address = Some(value),
                _ => {}
            }
        }

        Ok(options)
    }

    /// merges loaded server options with cli arguments and env args.
    /// env args are prioritized over configuration but cli arguments take precedence
    pub fn merge(&self, options: ServerOptions, args: Args) -> ServerOptions {
        ServerOptions {
            port: args.port.or(self.port).unwrap_or(options.port),
            dlc_folder: args
                .dlc_folder
                .or(self.dlc_folder.clone())
                .unwrap_or(options.dlc_folder),
            database: args
                .database
                .or(self.database.clone())
                .unwrap_or(options.database),
            server_address: args
                .server_address
                .or(self.server_address.clone())
                .unwrap_or(options.server_address),
            log_assets: args
                .log_assets
                .or(self.log_assets)
                .unwrap_or(options.log_assets),
            ..options
        }
    }
}
