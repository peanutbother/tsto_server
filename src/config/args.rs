use clap::Parser;

#[derive(Debug, Parser)]
#[command(version = env!("CARGO_PKG_VERSION"), about)]
pub struct Args {
    #[arg(
        long,
        short = 'P',
        help = "Enable portable mode",
        long_help = "Enables portable mode which uses relative paths to store config and data"
    )]
    pub portable: bool,
    #[arg(long, short = 'p', help = "Set server port")]
    pub port: Option<u16>,
    #[arg(long, short = 'D', help = "Set database path")]
    pub database: Option<String>,
    #[arg(long, short, help = "Set dlc path to serve")]
    pub dlc_folder: Option<String>,
    #[arg(long, short = 'a', help = "Set server address")]
    pub server_address: Option<String>,
    #[arg(
        long,
        short = 'l',
        help = "Enable asset logging",
        long_help = "Enables logging of assets. This can be very noisy when new clients download dlcs and is disabled by default"
    )]
    pub log_assets: Option<bool>,
}
