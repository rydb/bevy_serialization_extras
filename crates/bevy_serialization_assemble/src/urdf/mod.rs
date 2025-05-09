pub(crate) mod loader;
pub mod resources;
pub mod urdf;
pub mod visual;

use crate::traits::AssetLoadSettings;
use bytemuck::TransparentWrapper;
use ref_cast::RefCast;
// use crate::systems::split_open_self;
// use crate::systems::split_open_self_children;
use crate::urdf::loader::UrdfLoaderPlugin;
use bevy_app::prelude::*;
use bevy_asset::AssetApp;
use bevy_asset::io::AssetSource;
use bevy_asset::io::file::FileAssetReader;
use bevy_asset::prelude::*;
use bevy_derive::Deref;
use bevy_reflect::TypePath;
use derive_more::derive::From;
use urdf_rs::Robot;

pub const PACKAGE: &str = "package";

#[derive(Asset, TypePath, From, Deref, Debug, Clone, Default, TransparentWrapper)]
#[repr(transparent)]
pub struct UrdfWrapper(pub Urdf);

impl AssetLoadSettings for UrdfWrapper {
    type LoadSettingsType = ();

    fn load_settings() -> Option<Self::LoadSettingsType> {
        None
    }
}

#[derive(Asset, TypePath, From, Deref, Debug, Clone)]
pub struct Urdf(pub Robot);

impl Default for Urdf {
    fn default() -> Self {
        Self(Robot {
            name: "DEFAULT_IN_CASE_OF_ERROR".to_owned(),
            links: Vec::new(),
            joints: Vec::new(),
            materials: Vec::new(),
        })
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

pub struct UrdfSerializationPlugin;

impl Plugin for UrdfSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        // .register_type::<CachedUrdf>()
        .add_plugins(UrdfLoaderPlugin)
        //.add_plugins(SerializeManyAsOneFor::<UrdfWrapper>::default())
        //.add_plugins(SerializeManyAsOneFor::<LinkQuery, UrdfWrapper>::default())
        // .add_systems(Update, split_open_self_children::<LinksNJoints>)
        // .add_systems(Update, split_open_self::<UrdfJoint>)
        // .add_systems(Update, split_open_self_children::<Visuals>)
        //.add_systems(Update, split_open_self::<GltfNodeWrapper>)
        ;
    }
}
