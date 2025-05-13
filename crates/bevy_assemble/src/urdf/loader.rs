//! urdf loarder for robots. Should create a
//! unique urdf resource for models to read from.

use bevy_app::prelude::*;
use bevy_asset::{
    AssetLoader, AsyncWriteExt, LoadContext, io::Reader, prelude::*, saver::AssetSaver,
};
use bevy_state::prelude::States;
use thiserror::Error;

use super::*;

//contains the machinery required to load urdfs
pub struct UrdfLoaderPlugin;

impl Plugin for UrdfLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Urdf>().init_asset_loader::<UrdfLoader>()
        // app.register_asset_saver
        ;
    }
}

#[derive(Default)]
pub struct UrdfLoader;

/// Possible errors that can be produced by [`UrdfLoaderError`]
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum UrdfLoaderError {
    #[error("Failed to load Urdf")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse urdf")]
    ParsingError(String),
}

// impl From<Error> for UrdfLoaderError

impl AssetLoader for UrdfLoader {
    type Asset = Urdf;

    type Settings = ();

    type Error = UrdfLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let urdf = load_urdf(&bytes)?;
        Ok(urdf)
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}

pub fn load_urdf<'a>(bytes: &'a [u8]) -> Result<Urdf, UrdfLoaderError> {
    // if let Some(res) = std::str::from_utf8(bytes)
    //     .ok()
    //     .and_then(|utf| urdf_rs::read_from_string(utf).ok())
    // {
    //     Ok(Urdf { robot: res })
    // } else {
    //     Err(UrdfLoaderError::ParsingError(""))
    // }
    let res = std::str::from_utf8(bytes);
    match res {
        Ok(res) => match urdf_rs::read_from_string(res) {
            Ok(urdf) => Ok(Urdf(urdf)),
            Err(err) => Err(UrdfLoaderError::ParsingError(err.to_string())),
        },
        Err(err) => Err(UrdfLoaderError::ParsingError(err.to_string())),
    }
    // match std::str::from_utf8(bytes) {
    //     Ok(_) => todo!(),
    //     Err(_) => todo!(),
    // }
}

/// Weather this urdf is loaded or not.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LoadState {
    #[default]
    Unloaded,
    // Loaded,
}

/// Possible errors that can be produced by [`UrdfLoaderError`]
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum UrdfSaveError {
    #[error("Failed to save Urdf")]
    Io(#[from] std::io::Error),
    #[error("erorr saving urdf")]
    UrdfError(#[from] urdf_rs::UrdfError),
}

#[derive(Default)]
pub struct UrdfSaver;

#[derive(Default)]
pub struct UrdfSettings;

impl AssetSaver for UrdfSaver {
    type Asset = Urdf;
    type Settings = ();
    type OutputLoader = UrdfLoader;
    type Error = UrdfSaveError;

    fn save(
        &self,
        writer: &mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'_, Self::Asset>,
        _settings: &Self::Settings,
    ) -> impl bevy_tasks::ConditionalSendFuture<Output = Result<(), Self::Error>> {
        async move {
            let urdf_as_string = urdf_rs::write_to_string(&asset.0)?;
            let bytes = urdf_as_string.as_bytes();

            writer.write_all(bytes).await?;

            Ok(())
        }
    }
}
