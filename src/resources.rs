use std::{any::TypeId, marker::PhantomData, collections::VecDeque};
use bevy::{prelude::Resource, transform::components::Transform, asset::{AssetId, Asset}};
use moonshine_save::{save::SaveFilter, prelude::LoadFromFileRequest};
use std::collections::HashMap;
use moonshine_save::prelude::SaveIntoFileRequest;
use std::path::Path;

/// keeps track of number of times refresh request has been sent. For ui utils.
#[derive(Resource, Default)]
pub struct RefreshCounter {
    pub counter: usize
}

#[derive(Default, Clone)]
pub struct AssetSpawnRequest<T: Asset> {
    pub item_id: AssetId<T>,
    pub position: Transform,
    pub failed_load_attempts: u64,
}
#[derive(Resource, Default, Clone)]
pub struct AssetSpawnRequestQueue<T: Asset> {
    pub requests: VecDeque<AssetSpawnRequest<T>>,
}

// impl<T> Default for ResLoadRequests<T> {
//     fn default() -> Self {
//         Self {
//             requests: VecDeque::new(),
//             requests_are_for: PhantomData,
//         }
//     }
// }

/// Resource version of moonshine-save's [`SaveFilter`]. 
#[derive(Resource, Default, Clone)]
pub struct SerializeFilter {
    pub filter: SaveFilter
}

#[derive(Resource)]
pub struct SaveRequest {
    pub path: String,
}

impl SaveIntoFileRequest for SaveRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

#[derive(Resource)]
pub struct LoadRequest {
    pub path: String,
}

impl LoadFromFileRequest for LoadRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

/// contains the state of the type registry since the last [`SaveRequest`]/refresh.
#[derive(Resource, Default)]
pub struct TypeRegistryOnSave {
    pub registry: HashMap<TypeId, String>,
}

/// contains the components marked to saved since last save/refresh.
#[derive(Resource, Default)]
pub struct ComponentsOnSave {
    pub components: HashMap<TypeId, String>
}

#[derive(Resource)]
pub struct ShowSerializable {
    pub check: bool
}

impl Default for ShowSerializable {
    fn default() -> Self {
        Self {
            check: false
        }
    }
}

#[derive(Resource)]
pub struct ShowUnserializable {
    pub check: bool
}

impl Default for ShowUnserializable {
    fn default() -> Self {
        Self { check: true }
    }
}