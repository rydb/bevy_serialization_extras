pub(crate) mod loader;
pub mod resources;
pub mod urdf;
pub mod visual;

use crate::plugins::SerializeManyAsOneFor;
use crate::urdf::loader::UrdfLoaderPlugin;
use bevy_app::prelude::*;
use bevy_asset::io::file::FileAssetReader;
use bevy_asset::io::AssetSource;
use bevy_asset::prelude::*;
use bevy_asset::AssetApp;
use bevy_reflect::TypePath;
use resources::CachedUrdf;
use urdf::LinkQuery;
use urdf_rs::Robot;

pub const PACKAGE: &str = "package";

#[derive(Asset, TypePath, Debug, Clone)]
pub struct Urdf {
    pub robot: Robot,
}

impl Default for Urdf {
    fn default() -> Self {
        Self {
            robot: Robot {
                name: "DEFAULT_IN_CASE_OF_ERROR".to_owned(),
                links: Vec::new(),
                joints: Vec::new(),
                materials: Vec::new(),
            },
        }
    }
}

/// asset sources for urdf. Needs to be loaded before [`DefaultPlugins`]
pub struct AssetSourcesUrdfPlugin {
    // path to folder that `package://`` leads to
    pub assets_folder_local_path: String,
}

impl Plugin for AssetSourcesUrdfPlugin {
    fn build(&self, app: &mut App) {
        let path = self.assets_folder_local_path.clone();
        app.register_asset_source(
            PACKAGE,
            AssetSource::build().with_reader(move || Box::new(FileAssetReader::new(path.clone()))),
        );
    }
}

/// plugin that contains everything required for a urdf -> bevy conversion
///
/// NOTE: !!! .dae is not supported! If a .dae support plugin gets added, make an issue, and it can be added.
/// In the meantime, use .obj!!!
///
pub struct UrdfSerializationPlugin;

impl Plugin for UrdfSerializationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CachedUrdf>()
            .add_plugins(UrdfLoaderPlugin)
            .insert_resource(CachedUrdf::default())
            .add_plugins(SerializeManyAsOneFor::<LinkQuery, Urdf>::default());
    }
}
