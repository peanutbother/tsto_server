#[cfg(feature = "server")]
pub static CLIENT_CONFIG: &str = include_str!("./configs/ClientConfig.json");
#[cfg(feature = "server")]
pub static GAMEPLAY_CONFIG: &str = include_str!("./configs/GameplayConfig.json");
#[cfg(feature = "server")]
pub static DIRECTION_ROW_CONFIG: &str = include_str!("./directions/com.ea.game.simpsons4_row.json");
#[cfg(feature = "server")]
pub const STARTER_LAND: &[u8] = include_bytes!("./starter_land.pb");

pub struct Locales {
    pub de: &'static str,
    pub en_us: &'static str,
}

pub static LOCALES: Locales = Locales {
    de: include_str!("./locales/de.ftl"),
    en_us: include_str!("./locales/en-US.ftl"),
};
