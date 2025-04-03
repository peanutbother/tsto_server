use prost::Message;

tonic::include_proto!("include_all");

pub trait MessageFromPath<E>
where
    Self: prost::Message,
    Self: Default,
    E: From<std::io::Error>,
    E: From<prost::EncodeError>,
    E: From<prost::DecodeError>,
{
    fn load(path: impl AsRef<std::path::Path>) -> Result<Self, E> {
        use std::io::Read;
        let mut file = std::fs::File::open(&path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;
        Ok(Self::decode(&buffer[..])?)
    }

    fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), E> {
        use prost::Message;
        use std::io::Write;
        let mut file = std::fs::File::create(&path)?;
        let mut buffer = vec![];

        Message::encode(self, &mut buffer)?;
        file.write_all(&buffer[..])?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CurrencyError {
    #[error("failed to read/write file")]
    File(#[from] std::io::Error),
    #[error("failed to encode LandMessage")]
    EncodeError(#[from] prost::EncodeError),
    #[error("failed to decode LandMessage")]
    DecodeError(#[from] prost::DecodeError),
}

#[derive(Debug, thiserror::Error)]
pub enum LandError {
    #[error("failed to read/write file")]
    File(#[from] std::io::Error),
    #[error("failed to encode LandMessage")]
    EncodeError(#[from] prost::EncodeError),
    #[error("failed to decode LandMessage")]
    DecodeError(#[from] prost::DecodeError),
}

impl data::LandMessage {
    pub fn new(id: &String) -> Result<Self, LandError> {
        let mut land = Self::decode(crate::assets::STARTER_LAND).map_err(LandError::DecodeError)?;
        land.id = Some(id.to_owned());

        Ok(land)
    }
}

impl MessageFromPath<CurrencyError> for data::CurrencyData {}
impl MessageFromPath<LandError> for data::LandMessage {}
