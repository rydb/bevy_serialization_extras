pub(crate) mod loader;
pub mod resources;
pub mod urdf;
pub mod visual;

use std::fs::File;
use std::io::Write;

use crate::plugins::SerializeManyAsOneFor;
use crate::traits::LazySerialize;
// use crate::systems::split_open_self;
// use crate::systems::split_open_self_children;
use crate::urdf::loader::UrdfLoaderPlugin;
use bevy_app::prelude::*;
use bevy_asset::io::file::FileAssetReader;
use bevy_asset::io::AssetSource;
use bevy_asset::prelude::*;
use bevy_asset::AssetApp;
use bevy_derive::Deref;
use bevy_reflect::TypePath;
use derive_more::derive::From;
use resources::CachedUrdf;
use urdf_rs::Robot;

pub const PACKAGE: &str = "package";

#[derive(Asset, TypePath, From, Deref, Debug, Clone, Default)]
pub struct UrdfWrapper(pub Urdf);

impl LazySerialize for UrdfWrapper {
    fn serialize(&self, name: String, folder_path: String) -> Result<(), anyhow::Error> {
        //let path = PathBuf::new()
        let urdf_as_string = urdf_rs::write_to_string(&self.0 .0)?;
        let mut file = File::create(folder_path + &name + ".xml")?;
        let _ = file.write(urdf_as_string.as_bytes());
        Ok(())
    }
    // fn serialize(&self, name: String) -> Result<(), anyhow::Error> {
    //     //let path = PathBuf::new()
    //     let urdf_as_string = urdf_rs::write_to_string(&self.0 .0)?;
    //     let mut file = File::create(name + ".xml")?;
    //     let _ = file.write(urdf_as_string.as_bytes());
    //     Ok(())
    // }
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

/// plugin that contains everything required for a urdf -> bevy conversion
///
/// NOTE: !!! .dae is not supported! If a .dae support plugin gets added, make an issue, and it can be added.
/// In the meantime, use .obj!!!
///
pub struct UrdfSerializationPlugin;

impl Plugin for UrdfSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        // .register_type::<CachedUrdf>()
        .add_plugins(UrdfLoaderPlugin)
        .insert_resource(CachedUrdf::default())
        //.add_plugins(SerializeManyAsOneFor::<UrdfWrapper>::default())
        //.add_plugins(SerializeManyAsOneFor::<LinkQuery, UrdfWrapper>::default())
        // .add_systems(Update, split_open_self_children::<LinksNJoints>)
        // .add_systems(Update, split_open_self::<UrdfJoint>)
        // .add_systems(Update, split_open_self_children::<Visuals>)
        //.add_systems(Update, split_open_self::<GltfNodeWrapper>)
        ;
    }
}
