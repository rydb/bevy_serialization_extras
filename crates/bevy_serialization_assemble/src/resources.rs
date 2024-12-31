use std::collections::VecDeque;

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;

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

#[derive(Resource, Default, Clone)]
pub struct AssetSpawnRequestQueue<T: Asset> {
    pub requests: VecDeque<AssetSpawnRequest<T>>,
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
