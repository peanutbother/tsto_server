#[cfg(feature = "server")]
mod credits;
pub mod error;
#[cfg(feature = "server")]
pub mod extractors;
#[cfg(feature = "server")]
pub mod protobuf;
#[cfg(feature = "server")]
pub mod xml;

#[cfg(feature = "server")]
pub use r#mod::*;
#[cfg(feature = "server")]
mod r#mod {
    pub use super::credits::CREDITS;
    pub use super::error::ErrorMessage;
    pub use super::protobuf::Protobuf;
    pub use super::xml::Xml;
    use directories::ProjectDirs;
    use lazy_static::lazy_static;
    use std::time::SystemTime;

    lazy_static! {
        pub static ref UPTIME: SystemTime = SystemTime::now();
        pub static ref DIRECTORIES: ProjectDirs =
            ProjectDirs::from("de", "peanutbother", "tsto_server").unwrap();
    }

    pub fn secs_from_unix_epoch() -> Result<u64, std::time::SystemTimeError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
    }

    pub fn millis_from_unix_epoch() -> Result<u128, std::time::SystemTimeError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
    }
}
