//! urdf loarder for robots. Should create a
//! unique urdf resource for models to read from.

use bevy::asset::LoadedAsset;
use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use thiserror::Error;
use urdf_rs::Robot;

//contains the machinery required to load urdfs
pub struct UrdfLoaderPlugin;

impl Plugin for UrdfLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_asset::<Urdf>()
        .init_asset_loader::<UrdfLoader>()
        ;
    }
}

#[derive(Default)]
pub struct UrdfLoader;

#[derive(Asset, TypePath, Debug, Clone)]
pub struct Urdf {
    pub robot: Robot,
}

/// Possible errors that can be produced by [`UrdfLoaderError`]
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum UrdfLoaderError {
    #[error("Failed to load Urdf")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse urdf")]
    ParsingError,
}


impl AssetLoader for UrdfLoader {
    type Asset = Urdf;
    type Settings = ();
    type Error = UrdfLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let urdf = load_urdf(&bytes)?;
            Ok(urdf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}

pub fn load_urdf<'a> (
    bytes: &'a [u8],
) -> Result<Urdf, UrdfLoaderError>{
    if let Some(res) = std::str::from_utf8(bytes)
        .ok()
        .and_then(|utf| urdf_rs::read_from_string(utf).ok())
    {
        Ok(Urdf{robot: res})
    } else {
        Err(UrdfLoaderError::ParsingError)
    }
}

// async fn load_urdf<'a, 'b>(
//     bytes: &'a [u8],
// ) -> Result<Urdf, UrdfError> {
//     if let Some(res) = std::str::from_utf8(bytes)
//         .ok()
//         .and_then(|utf| urdf_rs::read_from_string(utf).ok())
//     {
//         Ok(Urdf {robot: res})
//     } else {
//         return Err(UrdfError::ParsingError);
//     }
// }

/// Weather this urdf is loaded or not. 
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LoadState {
    #[default]
    Unloaded,
    Loaded,
}