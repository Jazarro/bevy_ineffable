use std::fmt::{Display, Formatter};
use std::io::Error;

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use ron::de::SpannedError;

use crate::config::input_config::InputConfig;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InputConfigRonLoader;

impl AssetLoader for InputConfigRonLoader {
    type Asset = InputConfig;
    type Settings = ();
    type Error = CustomAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<InputConfig>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["input.ron"]
    }
}

/// Possible errors that can be produced by [`InputConfigRonLoader`]
#[derive(Debug)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    Io(Error),
    /// A [RON](ron) Error
    RonSpannedError(SpannedError),
}

impl std::error::Error for CustomAssetLoaderError {}

impl From<SpannedError> for CustomAssetLoaderError {
    fn from(value: SpannedError) -> Self {
        CustomAssetLoaderError::RonSpannedError(value)
    }
}

impl From<Error> for CustomAssetLoaderError {
    fn from(value: Error) -> Self {
        CustomAssetLoaderError::Io(value)
    }
}

impl Display for CustomAssetLoaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomAssetLoaderError::Io(err) => Display::fmt(err, f),
            CustomAssetLoaderError::RonSpannedError(err) => Display::fmt(err, f),
        }
    }
}
