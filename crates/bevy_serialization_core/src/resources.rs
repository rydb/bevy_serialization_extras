use bevy_derive::{Deref, DerefMut};
use bevy_reflect::Reflect;
use bevy_render::camera::{CameraMainTextureUsages, CameraRenderGraph, Exposure};
use bevy_render::prelude::*;
use bevy_transform::components::Transform;
// use moonshine_save::FilePath;
use moonshine_save::save::SaveInput;
use moonshine_save::GetFilePath;
use std::collections::HashMap;
use std::path::Path;
use std::{any::TypeId, collections::VecDeque};

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;

/// keeps track of number of times refresh request has been sent. For ui utils.
#[derive(Resource, Default)]
pub struct RefreshCounter {
    pub counter: usize,
}

#[derive(Clone)]
pub enum RequestFrom<T: Asset> {
    ///path of asset relative to main.rs of bevy project.
    ///
    /// E.G:
    ///
    ///If `bob.stl` is in `~/project/assets/models/bob.stl`. Then this should be set to `"models/bob.stl"`
    AssetServerPath(String),
    //AssetId(AssetId<T>),
    AssetHandle(Handle<T>),
}

impl<T: Asset> From<String> for RequestFrom<T> {
    fn from(value: String) -> Self {
        Self::AssetServerPath(value)
    }
}

impl<T: Asset> Default for RequestFrom<T> {
    fn default() -> Self {
        Self::AssetServerPath(
            "don't use default for RequestFrom enum or you will get this!".to_owned(),
        )
    }
}

/// spawn request for assets that are "all-in-one" rather then composed
/// of seperate components.
///
/// E.G: Robots/Urdfs are spawned through this.
#[derive(Default, Clone)]
pub struct AssetSpawnRequest<T: Asset> {
    pub source: RequestFrom<T>,
    pub position: Transform,
    pub failed_load_attempts: u64,
}

#[derive(Resource, Default, Clone)]
pub struct AssetSpawnRequestQueue<T: Asset> {
    pub requests: VecDeque<AssetSpawnRequest<T>>,
}

/// Resource version of moonshine-save's [`SaveFilter`].
#[derive(Resource, Clone, DerefMut, Deref)]
pub struct SerializeFilter(pub SaveInput);
impl Default for SerializeFilter {
    fn default() -> Self {
        // Due to: https://github.com/Zeenobit/moonshine_save/issues/16
        // components that do not implement reflect break save/load.
        // this is just an a default list of unimplemented components to skip serializing over to stop this breakage.
        // make a pr to fix this if this issue is resolved.
        Self({
            // Due to bevy_scene taking `self` and not `&mut self`, to stop partial move errors, It requires... this.. for initialization.
            let filter = SaveInput::default();
            let mut new_filter = SaveInput::default();

            new_filter.components = filter
                .components
                .clone()
                .deny::<CameraMainTextureUsages>()
                .deny::<CameraRenderGraph>()
                .deny::<Exposure>()
                .deny::<Mesh3d>()
                //.deny::<InheritedVisibility>()
                
                ;
            new_filter
        })
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SaveRequest {
    pub path: String,
}

impl GetFilePath for SaveRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct LoadRequest {
    pub path: String,
}

impl GetFilePath for LoadRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

/// contains the state of the type registry since the last [`SaveRequest`]/refresh.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct TypeRegistryOnSave {
    pub registry: HashMap<TypeId, String>,
}

/// contains the components marked to saved since last save/refresh.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ComponentsOnSave {
    pub components: HashMap<TypeId, String>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ShowSerializable {
    pub check: bool,
}

impl Default for ShowSerializable {
    fn default() -> Self {
        Self { check: false }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ShowUnserializable {
    pub check: bool,
}

impl Default for ShowUnserializable {
    fn default() -> Self {
        Self { check: true }
    }
}
